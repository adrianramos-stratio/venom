use crate::domain::component::{id::ComponentId, sbom::Sbom, ComponentError};
use actix::Message;

#[derive(Message, Debug, Clone)]
#[rtype(result = "Result<(), ComponentError>")]
pub struct ComponentCommand {
    pub id: ComponentId,
    pub kind: ComponentCommandKind,
}

#[derive(Debug, Clone)]
pub enum ComponentCommandKind {
    Register,
    AssignSbom(Sbom),
}
