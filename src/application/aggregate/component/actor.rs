use super::cmd::AssignSbom;
use crate::domain::component::{event::ComponentEvent, Component, ComponentError};
use actix::prelude::*;

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

impl Handler<AssignSbom> for ComponentActor {
    type Result = Result<ComponentEvent, ComponentError>;

    fn handle(&mut self, msg: AssignSbom, _ctx: &mut Context<Self>) -> Self::Result {
        self.state.assign_sbom(msg.sbom)
    }
}
