use std::collections::HashMap;
use std::sync::Arc;

use actix::{Actor, Addr, Context, Handler};

use crate::application::aggregate::component::actor::ComponentActor;
use crate::application::aggregate::component::cmd::{ComponentCommand, ComponentCommandKind};
use crate::application::aggregate::component::event::ComponentRegisteredEvent;
use crate::application::shared::command::bus::CommandBus;
use crate::application::shared::command::handler::HandlesCommand;
use crate::application::shared::command::RegistersCommands;
use crate::application::shared::event::bus::EventBus;
use crate::domain::component::id::ComponentId;
use crate::domain::component::{Component, ComponentError};
use crate::domain::shared::aggregate::EventSourcedAggregate;

#[derive(Clone)]
pub struct ComponentSupervisor<EB>
where
    EB: EventBus + Send + Sync + 'static,
{
    children: HashMap<ComponentId, Addr<ComponentActor>>,
    event_bus: Arc<EB>,
}

impl<EB> Actor for ComponentSupervisor<EB>
where
    EB: EventBus + Send + Sync + 'static,
{
    type Context = Context<Self>;
}

impl<EB> ComponentSupervisor<EB>
where
    EB: EventBus + Send + Sync + 'static,
{
    pub fn new(event_bus: Arc<EB>) -> Self {
        Self {
            children: HashMap::new(),
            event_bus,
        }
    }
}

impl<EB> Handler<ComponentCommand> for ComponentSupervisor<EB>
where
    EB: EventBus + Send + Sync + 'static,
{
    type Result = Result<(), ComponentError>;

    fn handle(&mut self, cmd: ComponentCommand, _ctx: &mut Context<Self>) -> Self::Result {
        let ComponentCommand { ref id, ref kind } = cmd;

        if let ComponentCommandKind::Register = kind {
            tracing::info!("Check if new component id is in the journal");
            tracing::info!("Throw an exception if is");
            let event = Component::register(id.clone());
            let component = Component::from_initial_event(&event)?;
            tracing::info!("Persist {component:?} in journal");
            let _ = self.event_bus.publish(ComponentRegisteredEvent::new(event));
            Ok(())
        } else {
            if !self.children.contains_key(id) {
                tracing::info!("Check if id exists in journal");
                let component = Component::new(id.clone());
                let actor = ComponentActor::new(component).start();
                tracing::info!("Register actor addr {actor:?}")
                // self.children.insert(id.clone(), actor);
            }

            if let Some(actor) = self.children.get(id) {
                actor.do_send(cmd.clone());
            }
            Ok(())
        }
    }
}

#[async_trait::async_trait]
impl<EB> HandlesCommand<ComponentCommand> for Addr<ComponentSupervisor<EB>>
where
    EB: EventBus + Send + Sync + 'static,
{
    async fn handle(&self, cmd: ComponentCommand) -> Result<(), String> {
        self.do_send(cmd);
        Ok(())
    }
}

impl<EB> RegistersCommands for Addr<ComponentSupervisor<EB>>
where
    EB: EventBus + Send + Sync + 'static,
{
    fn register_with(self, bus: &mut CommandBus) {
        bus.register_handler::<ComponentCommand, Self>(self);
    }
}
