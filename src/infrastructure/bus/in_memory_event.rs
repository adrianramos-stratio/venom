use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, RwLock};

use tokio::task;

use crate::application::shared::event::{
    bus::EventBus, error::EventBusError, listener::EventListener, Event,
};

// Boxed future type without using futures crate
type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;
type DynEventFn = Arc<dyn Fn(Arc<dyn Any + Send + Sync>) -> BoxFuture<'static, ()> + Send + Sync>;

/// In-memory asynchronous EventBus implementation
pub struct InMemoryEventBus {
    listeners: RwLock<HashMap<TypeId, Vec<DynEventFn>>>,
}

impl Default for InMemoryEventBus {
    fn default() -> Self {
        Self {
            listeners: RwLock::new(HashMap::new()),
        }
    }
}

impl InMemoryEventBus {
    pub fn publish<E>(&self, event: E) -> Result<(), EventBusError>
    where
        E: Event + Clone + 'static,
    {
        tracing::info!("Received event {event:?}");
        let listeners_opt = self
            .listeners
            .read()
            .map_err(|e| EventBusError::DispatchError(format!("Lock poisoned: {}", e)))?
            .get(&TypeId::of::<E>())
            .cloned();

        if let Some(listeners) = listeners_opt {
            tracing::info!("Some listeners found for {event:?}");
            let event = Arc::new(event);
            for listener in listeners {
                let evt = Arc::clone(&event);
                task::spawn(async move {
                    (listener)(evt).await;
                });
            }
        } else {
            tracing::warn!("No listeners found for event {event:?}");
        }

        Ok(())
    }

    pub fn subscribe<E, L>(&self, listener: Arc<L>) -> Result<(), EventBusError>
    where
        E: Event + Clone + 'static,
        L: EventListener<E> + 'static,
    {
        let mut map = self
            .listeners
            .write()
            .map_err(|e| EventBusError::RegistrationError(format!("Lock poisoned: {e}")))?;

        let entry = map.entry(TypeId::of::<E>()).or_default();

        let func: DynEventFn = Arc::new(move |any_evt| {
            let l = Arc::clone(&listener);
            Box::pin(async move {
                match any_evt.downcast_ref::<E>() {
                    Some(e) => {
                        l.on_event(e).await;
                    }
                    None => {
                        tracing::error!(
                            "Failed to downcast event to expected type: {}",
                            std::any::type_name::<E>()
                        );
                    }
                }
            })
        });

        entry.push(func);
        Ok(())
    }
}

impl EventBus for InMemoryEventBus {
    fn publish<E: Event + Clone + 'static>(&self, event: E) -> Result<(), EventBusError> {
        self.publish(event)
    }
}
