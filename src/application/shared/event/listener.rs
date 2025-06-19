use crate::application::shared::event::Event;
use async_trait::async_trait;

/// Asynchronous trait for subscribing to events of a specific type.
#[async_trait]
pub trait EventListener<E: Event>: Send + Sync {
    /// Handler method invoked when an event of type E is published.
    async fn on_event(&self, event: &E);
}
