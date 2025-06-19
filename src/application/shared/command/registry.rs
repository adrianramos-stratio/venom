use super::bus::CommandBus;

/// Trait for self-registration of handlers in the command bus.
///
/// Any actor (e.g. supervisor) implementing this can declare the commands
/// it wants to handle and register them dynamically with the bus.
pub trait RegistersCommands {
    fn register_with(self, bus: &mut CommandBus);
}
