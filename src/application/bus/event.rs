use actix::prelude::*;

use crate::domain::collection::event::CollectionEvent;

#[derive(Message)]
#[rtype(result = "()")]
pub struct SubscribeCollectionEvent(pub Recipient<CollectionEvent>);

#[derive(Message, Clone)]
#[rtype(result = "()")]
pub struct PublishCollectionEvents(pub Vec<CollectionEvent>);

#[derive(Default)]
pub struct EventBus {
    pub collection_subscribers: Vec<Recipient<CollectionEvent>>,
}

impl Actor for EventBus {
    type Context = Context<Self>;
}

impl Handler<SubscribeCollectionEvent> for EventBus {
    type Result = ();

    fn handle(&mut self, msg: SubscribeCollectionEvent, _ctx: &mut Self::Context) {
        self.collection_subscribers.push(msg.0);
    }
}

impl Handler<PublishCollectionEvents> for EventBus {
    type Result = ();

    fn handle(&mut self, msg: PublishCollectionEvents, _ctx: &mut Self::Context) {
        let PublishCollectionEvents(events) = msg;
        for subs in &self.collection_subscribers {
            for ev in &events {
                subs.do_send(ev.clone());
            }
        }
    }
}
