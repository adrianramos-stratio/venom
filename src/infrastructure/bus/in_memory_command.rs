use actix::prelude::*;
use async_trait::async_trait;
use std::any::type_name;

use crate::application::aggregate::component::supervisor::{
    ComponentSupervisor, RegisterComponent,
};
use crate::application::bus::command_bus::{AppCommand, CommandBus, CommandDispatchError};

pub struct InMemoryCommandBus {
    component_supervisor: Addr<ComponentSupervisor>,
}

impl InMemoryCommandBus {
    pub fn new(component_supervisor: Addr<ComponentSupervisor>) -> Self {
        Self {
            component_supervisor,
        }
    }
}

#[async_trait]
impl CommandBus for InMemoryCommandBus {
    async fn dispatch(&self, command: Box<dyn AppCommand>) -> Result<(), CommandDispatchError> {
        // Comprobamos si es un RegisterComponent
        if let Some(cmd) = command.as_any().downcast_ref::<RegisterComponent>() {
            self.component_supervisor.do_send(cmd.clone());
            return Ok(());
        }

        // Añadir más comandos aquí:
        // if let Some(cmd) = command.as_any().downcast_ref::<OtroComando>() { ... }

        Err(CommandDispatchError::NoHandler(
            type_name::<Box<dyn AppCommand>>().to_string(),
        ))
    }
}
