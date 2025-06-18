use crate::application::aggregate::component::cmd::AssignSbomToComponent;
use crate::application::aggregate::component::supervisor::ComponentSupervisor;
use crate::application::service::sbom_generator::SbomGenerator;
use crate::domain::component::event::ComponentEvent;
use actix::prelude::*;

pub struct SbomGenerationSaga {
    pub generator: Box<dyn SbomGenerator>,
    pub supervisor: Addr<ComponentSupervisor>,
}

impl Actor for SbomGenerationSaga {
    type Context = Context<Self>;
}

#[derive(Message, Debug)]
#[rtype(result = "()")]
pub struct HandleComponentRegistered(pub ComponentEvent);

impl Handler<HandleComponentRegistered> for SbomGenerationSaga {
    type Result = ();

    fn handle(&mut self, msg: HandleComponentRegistered, _: &mut Context<Self>) {
        tracing::info!("Handling msg {msg:?}");
        if let ComponentEvent::ComponentRegistered { component_id } = msg.0 {
            match self.generator.generate(&component_id) {
                Ok(sbom) => {
                    tracing::info!("SBOM generated successfully");
                    self.supervisor.do_send(AssignSbomToComponent {
                        id: component_id,
                        sbom,
                    });
                }
                Err(err) => {
                    tracing::error!(
                        "SBOM generation failed for component {}: {}",
                        component_id,
                        err
                    );
                }
            }
        }
    }
}

impl Supervised for SbomGenerationSaga {
    fn restarting(&mut self, _ctx: &mut <Self as Actor>::Context) {
        tracing::warn!("SbomGenerationSaga is restarting");
    }
}
