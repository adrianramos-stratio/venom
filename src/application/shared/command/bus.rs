use std::any::TypeId;
use std::collections::HashMap;
use std::sync::Arc;

use super::command::AppCommand;
use super::handler::{CommandHandler, FnHandler, HandlesCommand};
use super::registry::RegistersCommands;

/// Central bus responsible for dispatching commands to their handlers.
///
/// This is the backbone of the Command-side of our CQRS architecture. It supports:
/// - Dynamic registration of handlers for command types
/// - Type-erased routing using `TypeId`
/// - Asynchronous, fire-and-forget dispatch using Actix
#[derive(Default)]
pub struct CommandBus {
    /// Routes associate a command type (via `TypeId`) with its corresponding handler.
    ///
    /// We use `Arc` so handlers can be cloned and moved across threads safely.
    routes: HashMap<TypeId, Arc<dyn CommandHandler>>,
}

impl CommandBus {
    /// Registers a handler for a specific command type `C`.
    ///
    /// Internally, wraps the handler in an `FnHandler`, type-erases it,
    /// and stores it in a `HashMap<TypeId, Arc<dyn CommandHandler>>`.
    ///
    /// # Type Parameters
    /// - `C`: the concrete type of the command
    /// - `H`: the type that handles the command
    ///
    /// `C` must be `'static` because we use dynamic dispatch and async execution.
    pub fn register_handler<C, H>(&mut self, handler: H)
    where
        C: AppCommand + Clone + 'static,
        H: HandlesCommand<C> + 'static,
    {
        let type_id = TypeId::of::<C>();
        tracing::trace!("Registering TypeId: {:?}", type_id);

        let wrapped = Arc::new(FnHandler::new({
            let handler = Arc::new(handler);
            move |cmd: C| {
                let handler = handler.clone();
                Box::pin(async move { handler.handle(cmd).await })
            }
        }));

        self.routes.insert(type_id, wrapped);
    }

    /// Registers all command handlers provided by a struct that implements `RegistersCommands`.
    ///
    /// This allows grouped registration, e.g. from a component supervisor or saga.
    pub fn register<T: RegistersCommands>(&mut self, actor: T) {
        actor.register_with(self);
    }

    /// Dispatches a command by looking up its type in the `routes` and executing its handler.
    ///
    /// This is fire-and-forget: the command is sent and processed asynchronously via `actix::spawn`.
    /// The caller does **not** wait for completion, but we **do check upfront** if a handler exists.
    ///
    /// Returns `Ok(())` if the command was accepted for dispatch,
    /// or `Err(...)` if no handler was registered for its type.
    pub fn dispatch(&self, cmd: Box<dyn AppCommand>) -> Result<(), String> {
        let type_id = (*cmd).type_id();
        tracing::trace!("Dispatching: {:?}", type_id);

        if let Some(handler) = self.routes.get(&type_id) {
            let handler = handler.clone(); // Clone Arc to move into async task

            tracing::info!("🚀 Launching handler for command with TypeId {:?}", type_id);
            // Spawn the command execution in the background.
            actix::spawn(async move {
                if let Err(err) = handler.handle(cmd).await {
                    tracing::warn!("Command handler returned error: {}", err);
                }
            });

            Ok(())
        } else {
            Err(format!("No handler registered for command: {:?}", type_id))
        }
    }
}
