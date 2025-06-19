use super::actor::ComponentActor;
use super::cmd::{AssignSbom, AssignSbomToComponent, ComponentRegisteredAck, RegisterComponent};
use crate::application::shared::command::CommandBus;
use crate::application::shared::command::HandlesCommand;
use crate::application::shared::command::RegistersCommands;
use crate::domain::component::Component;
use actix::prelude::*;
use async_trait::async_trait;
use std::collections::HashMap;

pub struct ComponentSupervisor {
    components: HashMap<String, Addr<ComponentActor>>,
}

impl ComponentSupervisor {
    pub fn new() -> Self {
        Self {
            components: HashMap::new(),
        }
    }
}

impl Actor for ComponentSupervisor {
    type Context = Context<Self>;
}

impl Handler<RegisterComponent> for ComponentSupervisor {
    type Result = Result<ComponentRegisteredAck, String>;

    fn handle(&mut self, msg: RegisterComponent, _: &mut Context<Self>) -> Self::Result {
        let key = msg.id.to_string();
        if self.components.contains_key(&key) {
            return Err(format!("Component {} already exists", key));
        }

        let event = Component::register(msg.id.clone());
        match Component::try_from(&event) {
            Ok(component) => {
                let addr = ComponentActor::new(component).start();
                self.components.insert(key.clone(), addr);

                Ok(ComponentRegisteredAck { id: msg.id })
            }
            Err(e) => Err(e.to_string()),
        }
    }
}

impl Handler<AssignSbomToComponent> for ComponentSupervisor {
    type Result = ();

    fn handle(&mut self, msg: AssignSbomToComponent, _: &mut Context<Self>) {
        if let Some(actor) = self.components.get(&msg.id.to_string()) {
            let _ = actor.do_send(AssignSbom { sbom: msg.sbom });
        }
    }
}

#[async_trait]
impl HandlesCommand<RegisterComponent> for Addr<ComponentSupervisor> {
    async fn handle(&self, cmd: RegisterComponent) -> Result<(), String> {
        tracing::info!("Inside Handler 1!");
        match self.send(cmd).await {
            Ok(Ok(_ack)) => {
                tracing::info!("RegisterComponent complete!!");
                Ok(())
            }
            Ok(Err(e)) => Err(e),
            Err(mailbox_err) => Err(format!("Mailbox error: {mailbox_err}")),
        }
    }
}

#[async_trait]
impl HandlesCommand<AssignSbomToComponent> for Addr<ComponentSupervisor> {
    async fn handle(&self, cmd: AssignSbomToComponent) -> Result<(), String> {
        tracing::info!("Inside Handler 2!");
        match self.send(cmd).await {
            Ok(()) => {
                tracing::info!("AssignSbomToComponent complete!!");
                Ok(())
            }
            Err(mailbox_err) => Err(format!("Mailbox error: {mailbox_err}")),
        }
    }
}

impl RegistersCommands for Addr<ComponentSupervisor> {
    fn register_with(self, bus: &mut CommandBus) {
        tracing::info!("✅ Registering handler for RegisterComponent");
        bus.register_handler::<RegisterComponent, Self>(self.clone());
        bus.register_handler::<AssignSbomToComponent, Self>(self);
    }
}
