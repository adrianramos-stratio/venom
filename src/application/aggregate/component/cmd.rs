use crate::domain::component::{
    event::ComponentEvent, id::ComponentId, sbom::Sbom, ComponentError,
};
use actix::Message;
use std::fmt;

#[derive(Message, Debug, Clone)]
#[rtype(result = "Result<ComponentRegisteredAck, String>")]
pub struct RegisterComponent {
    pub id: ComponentId,
}

#[derive(Message)]
#[rtype(result = "Result<ComponentEvent, ComponentError>")]
pub struct AssignSbom {
    pub sbom: Sbom,
}

pub struct ComponentRegisteredAck {
    pub id: ComponentId,
}

impl fmt::Debug for ComponentRegisteredAck {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ComponentRegisteredAck({})", self.id)
    }
}

#[derive(Message, Debug, Clone)]
#[rtype(result = "()")]
pub struct AssignSbomToComponent {
    pub id: ComponentId,
    pub sbom: Sbom,
}
