pub mod bus;
pub mod error;
pub mod listener;

use actix::Message;
use std::any::Any;
use std::fmt::Debug;
use std::time::SystemTime;
use uuid::Uuid;

/// Base trait for all events dispatched through the EventBus.
/// Supports message-based delivery, tracing, and payload inspection.
pub trait Event: Debug + Send + Sync + Message<Result = ()> + 'static {
    /// Unique identifier for this event instance
    fn event_id(&self) -> &Uuid;

    /// Timestamp when the event was emitted
    fn date(&self) -> &SystemTime;

    /// Access to the original domain event or application event payload
    fn payload(&self) -> &dyn Any;

    /// Downcast the payload to a concrete type if applicable
    fn as_payload<T: 'static>(&self) -> Option<&T> {
        self.payload().downcast_ref::<T>()
    }
}
