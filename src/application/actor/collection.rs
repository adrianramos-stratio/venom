use crate::{
    application::bus::event::{EventBus, PublishCollectionEvents},
    domain::collection::Collection,
};
use actix::prelude::*;

#[derive(Message)]
#[rtype(result = "()")]
pub enum CollectionCommand {
    ProcessCollection(Collection),
}

pub struct CollectionActor {
    pub collection: Collection,
    pub event_bus: Addr<EventBus>,
}

impl Actor for CollectionActor {
    type Context = Context<Self>;
}

impl Handler<CollectionCommand> for CollectionActor {
    type Result = ();

    fn handle(&mut self, msg: CollectionCommand, _ctx: &mut Self::Context) -> Self::Result {
        match msg {
            CollectionCommand::ProcessCollection(new) => {
                let current = self.collection.clone();
                let events = current.diff(&new);
                self.event_bus.do_send(PublishCollectionEvents(events));
            }
        }
    }
}
