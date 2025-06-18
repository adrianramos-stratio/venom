use super::actor::ComponentActor;
use super::cmd::{AssignSbom, AssignSbomToComponent, ComponentRegisteredAck, RegisterComponent};
use crate::domain::component::Component;
use actix::prelude::*;
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
