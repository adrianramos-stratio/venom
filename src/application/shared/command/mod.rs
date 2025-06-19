pub mod bus;
pub mod handler;
pub mod registry;

pub use bus::CommandBus;
pub use handler::HandlesCommand;
pub use registry::RegistersCommands;

use std::any::Any;

use std::fmt::Debug;

/// Marker trait for any command in the system.
///
/// Think of a command as an "instruction" sent to the system.
/// This trait allows dynamic dispatch while retaining type identity via `TypeId`.
pub trait AppCommand: Any + Send + Sync + Debug {
    /// Required to support dynamic downcasting of boxed trait objects.
    fn as_any(self: Box<Self>) -> Box<dyn Any + Send>;
}

/// Blanket implementation
impl<T: Any + Send + Sync + Debug> AppCommand for T {
    fn as_any(self: Box<Self>) -> Box<dyn Any + Send> {
        self
    }
}
