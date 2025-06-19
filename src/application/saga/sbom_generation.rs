use actix::{Actor, Addr, Context, Handler};
use async_trait::async_trait;
use std::sync::{Arc, Mutex};

use crate::application::aggregate::component::cmd::{ComponentCommand, ComponentCommandKind};
use crate::application::aggregate::component::event::ComponentRegisteredEvent;
use crate::application::service::sbom_generator::SbomGenerator;
use crate::application::shared::command::CommandBus;
use crate::application::shared::event::listener::EventListener;
use crate::application::shared::event::Event;
use crate::domain::component::event::ComponentEvent;

/// Saga actor responsible for reacting to component registration events
pub struct SbomGenerationSaga {
    pub command_bus: Arc<Mutex<CommandBus>>,
    pub generator: Box<dyn SbomGenerator>,
}

impl SbomGenerationSaga {
    pub fn new(command_bus: Arc<Mutex<CommandBus>>, generator: Box<dyn SbomGenerator>) -> Self {
        Self {
            command_bus,
            generator,
        }
    }
}

impl Actor for SbomGenerationSaga {
    type Context = Context<Self>;
}

impl Handler<ComponentRegisteredEvent> for SbomGenerationSaga {
    type Result = ();

    fn handle(
        &mut self,
        event: ComponentRegisteredEvent,
        _ctx: &mut Self::Context,
    ) -> Self::Result {
        tracing::info!("Handling event {event:?}");

        if let ComponentEvent::ComponentRegistered { component_id } = event.as_payload().unwrap() {
            match self.generator.generate(component_id) {
                Ok(sbom) => {
                    tracing::info!("SBOM generated successfully");
                    let cmd_bus = self.command_bus.lock().unwrap();
                    let _ = cmd_bus.dispatch(Box::new(ComponentCommand {
                        id: component_id.clone(),
                        kind: ComponentCommandKind::AssignSbom(sbom),
                    }));
                }
                Err(err) => {
                    tracing::error!("SBOM generation failed for component {component_id}: {err}");
                }
            }
        }
    }
}

#[async_trait]
impl EventListener<ComponentRegisteredEvent> for Addr<SbomGenerationSaga> {
    async fn on_event(&self, event: &ComponentRegisteredEvent) {
        self.do_send(event.clone());
    }
}
