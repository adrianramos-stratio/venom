use crate::{
    application::aggregate::component::cmd::{ComponentCommand, ComponentCommandKind},
    domain::{
        component::{Component, ComponentError},
        shared::aggregate::EventSourcedAggregate,
    },
};
use actix::{Actor, Context, Handler};

pub struct ComponentActor {
    pub state: Component,
}

impl ComponentActor {
    #[must_use]
    pub const fn new(state: Component) -> Self {
        Self { state }
    }
}

impl Actor for ComponentActor {
    type Context = Context<Self>;
}

impl Handler<ComponentCommand> for ComponentActor {
    type Result = Result<(), ComponentError>;

    fn handle(&mut self, cmd: ComponentCommand, _ctx: &mut Context<Self>) -> Self::Result {
        let ComponentCommand { id, kind } = cmd;

        if !self.state.id().eq(&id) {
            return Err(ComponentError::InconsistentIds(
                id.to_string(),
                self.state.id().to_string(),
            ));
        }

        let event = match kind {
            ComponentCommandKind::AssignSbom(sbom) => self.state.assign_sbom(sbom.clone()),
            ComponentCommandKind::Register => Err(ComponentError::AlreadyRegistered(id.clone())),
        }?;

        tracing::info!("Persist event");
        let result = self.state.apply(&event);
        tracing::info!("Emit event");
        result
    }
}
