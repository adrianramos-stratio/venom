use crate::application::shared::event::Event;
use crate::domain::component::event::ComponentEvent;

use actix::Message;
use std::any::Any;
use std::fmt::Debug;
use std::time::SystemTime;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct ComponentRegisteredEvent {
    id: Uuid,
    date: SystemTime,
    payload: ComponentEvent,
}

impl ComponentRegisteredEvent {
    pub fn new(payload: ComponentEvent) -> Self {
        Self {
            id: Uuid::new_v4(),
            date: SystemTime::now(),
            payload,
        }
    }
}

impl Message for ComponentRegisteredEvent {
    type Result = ();
}

impl Event for ComponentRegisteredEvent {
    fn event_id(&self) -> &Uuid {
        &self.id
    }
    fn date(&self) -> &SystemTime {
        &self.date
    }
    fn payload(&self) -> &dyn Any {
        &self.payload
    }
}
