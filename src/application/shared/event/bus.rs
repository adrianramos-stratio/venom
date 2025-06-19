use crate::application::shared::event::{error::EventBusError, Event};

pub trait EventBus: Send + Sync {
    fn publish<E: Event + Clone + 'static>(&self, event: E) -> Result<(), EventBusError>;
}
