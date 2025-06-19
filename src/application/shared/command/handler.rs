use std::future::Future;
use std::marker::PhantomData;
use std::pin::Pin;
use std::sync::Arc;

use async_trait::async_trait;

use super::AppCommand;

/// Alias for boxed asynchronous future returning a result.
pub type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;

/// Trait for handlers of a specific command type `C`.
#[async_trait]
pub trait HandlesCommand<C>: Send + Sync
where
    C: AppCommand + Clone + 'static,
{
    async fn handle(&self, cmd: C) -> Result<(), String>;
}

/// Trait object for type-erased command handlers.
/// This allows storing handlers in a type map and invoking them via dynamic dispatch.
#[async_trait]
pub trait CommandHandler: Send + Sync {
    async fn handle(&self, cmd: Box<dyn AppCommand>) -> Result<(), String>;
}

/// Wrapper that adapts a concrete handler into a `CommandHandler` via dynamic dispatch.
/// Internally performs a downcast of the boxed command.
pub struct FnHandler<C>
where
    C: AppCommand + Clone + 'static,
{
    handler: Arc<dyn Fn(C) -> BoxFuture<'static, Result<(), String>> + Send + Sync>,
    _marker: PhantomData<C>,
}

impl<C> FnHandler<C>
where
    C: AppCommand + Clone + 'static,
{
    pub fn new<F>(f: F) -> Self
    where
        F: Fn(C) -> BoxFuture<'static, Result<(), String>> + Send + Sync + 'static,
    {
        Self {
            handler: Arc::new(f),
            _marker: PhantomData,
        }
    }
}

#[async_trait]
impl<C> CommandHandler for FnHandler<C>
where
    C: AppCommand + Clone + 'static,
{
    async fn handle(&self, cmd: Box<dyn AppCommand>) -> Result<(), String> {
        // Try to downcast the command to the expected type
        if let Ok(c) = cmd.as_any().downcast::<C>() {
            (self.handler)(*c).await
        } else {
            Err("Command type mismatch".to_string())
        }
    }
}
