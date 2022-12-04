use super::subscription::NotificationSubscription;
use crate::{
    ptm,
    res::CtrResult,
    srv::{enable_notifications, receive_notification},
    Handle,
};
use alloc::{vec, vec::Vec};

pub type NotificationHandlerResult = CtrResult;
pub type NotificationHandler = fn(u32) -> NotificationHandlerResult;

#[derive(Debug, PartialEq, Eq)]
pub enum NotificationType {
    /// A subscribed notification was handled.
    HandledSubscribed,
    /// A termination request was received.
    Termination,
    /// A notification was not handled.
    None,
}

/// Manages notification subscriptions
pub struct NotificationManager {
    handle: Handle,
    notification_subscriptions: Vec<NotificationSubscription>,
}

impl NotificationManager {
    pub fn new() -> CtrResult<Self> {
        let handle = enable_notifications()?;

        Ok(Self {
            handle,
            notification_subscriptions: vec![],
        })
    }

    pub fn subscribe(
        &mut self,
        notification_id: ptm::NotificationId,
        handler: NotificationHandler,
    ) -> CtrResult {
        let notification_subscription = NotificationSubscription::new(notification_id, handler)?;
        self.notification_subscriptions
            .push(notification_subscription);
        Ok(())
    }

    pub fn get_handle(&self) -> &Handle {
        &self.handle
    }

    /// Attempts to receive a notification and handle it with a previously provided subscription handler.
    pub fn handle_notification(&self) -> CtrResult<NotificationType> {
        let notification_id = receive_notification()?;

        if notification_id == ptm::NotificationId::Termination {
            return Ok(NotificationType::Termination);
        }

        let found_subscription = self
            .notification_subscriptions
            .iter()
            .find(|subscription| subscription.id == notification_id);

        if let Some(subscription) = found_subscription {
            subscription.handle_request()?;
            return Ok(NotificationType::HandledSubscribed);
        }

        Ok(NotificationType::None)
    }
}
