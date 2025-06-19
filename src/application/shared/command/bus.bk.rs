use std::any::TypeId;
use std::collections::HashMap;
use std::sync::Arc;

use super::command::AppCommand;
use super::handler::{CommandHandler, FnHandler, HandlesCommand};
use super::registry::RegistersCommands;

/// Central bus for dispatching commands to their registered handlers.
///
/// Internally uses a type-erased map from command type to handler instance.
#[derive(Default)]
pub struct CommandBus {
    routes: HashMap<TypeId, Box<dyn CommandHandler>>,
}

impl CommandBus {
    /// Registers a handler for a specific command type `C`.
    ///
    /// This allows dynamic dispatch of `C` by storing a boxed, type-erased handler.
    /// Internally uses `Arc` to ensure the handler lives long enough.
    pub fn register_handler<C, H>(&mut self, handler: H)
    where
        C: AppCommand + Clone + 'static,
        H: HandlesCommand<C> + 'static,
    {
        let handler = Arc::new(handler);

        let wrapped = FnHandler::new({
            let handler = handler.clone();
            move |cmd: C| {
                let handler = handler.clone();
                Box::pin(async move { handler.handle(cmd).await })
            }
        });

        let type_id = TypeId::of::<C>();
        tracing::trace!("Registering {type_id:?}");

        self.routes.insert(type_id, Box::new(wrapped));
    }

    /// High-level method for actors that want to register multiple command types.
    ///
    /// This enables doing `bus.register(supervisor)` when the actor implements `RegistersCommands`.
    pub fn register<T: RegistersCommands>(&mut self, actor: T) {
        actor.register_with(self);
    }

    pub fn dispatch(&self, cmd: Box<dyn AppCommand>) {
        let type_id = (*cmd).type_id();
        tracing::trace!("Dispatching: {:?}", type_id);

        if let Some(handler) = self.routes.get(&type_id) {
            tracing::trace!("Found handler for {:?}", type_id);

            // Clona el Arc para poder moverlo al future
            let handler = handler.clone_box();
            actix::spawn(async move {
                let _ = handler.handle(cmd).await;
            });
        } else {
            tracing::warn!("No handler registered for command {:?}", type_id);
        }
    }

    /*   pub async fn dispatch(&self, cmd: Box<dyn AppCommand>) -> Result<(), String> {
        // IMPORTANT (*cmd) to get concrete Box<AppCommand> TypeId instead of Box<dyn AppCommand> TypeId
        let type_id = (*cmd).type_id();
        tracing::trace!("Dispatching: {:?}", type_id);
        if let Some(handler) = self.routes.get(&type_id) {
            tracing::trace!("Found handler for {:?}", type_id);
            handler.handle(cmd).await
        } else {
            Err("No handler registered for command".into())
        }
    }
    */
}
