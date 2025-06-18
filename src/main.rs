/*use actix::prelude::*;
use actix_web::{web, App, HttpServer};
use std::io;
use std::sync::Arc;
use tracing::info;
use tracing_subscriber::EnvFilter;
use venom::api::controller::{analyze_collection, AppState};
use venom::application::actor::collection::CollectionActor;
use venom::domain::collection::Collection;
use venom::{
    application::{
        actor::sbom::SbomActor,
        bus::event::{EventBus, SubscribeCollectionEvent},
    },
    config::VulmanConfig,
    infrastructure::generator::sbom::syft::SyftSbomGenerator,
};

#[actix::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive("venom=info".parse().unwrap()))
        .init();

    info!("Starting Vulman...");

    let config = VulmanConfig::load()
        .map_err(|e| {
            io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("Error loading settings: {e}"),
            )
            //})?;
        })
        .unwrap();

    let syft_generator = SyftSbomGenerator::new(&config.sboms_path).unwrap();
    let sbom_actor = SbomActor::new(Arc::new(syft_generator)).start();

    let sbom_recipient = sbom_actor.recipient();

    let event_bus = EventBus::default().start();

    event_bus
        .send(SubscribeCollectionEvent(sbom_recipient))
        .await
        .unwrap();

    let initial_collection = Collection::new("test", vec![]).unwrap();
    //
    // Spawn CollectionActor dinámicamente
    let coll_actor = CollectionActor {
        collection: initial_collection.clone(),
        event_bus: event_bus.clone(),
    }
    .start();

    let state = AppState {
        collection_actor: coll_actor.clone(),
    };

    let _ = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(state.clone()))
            .service(analyze_collection)
    })
    .bind((config.server.host, config.server.port))
    .expect("Failed binding server")
    .run()
    .await;
}
*/
fn main() {
    println!("Hello rust!");
}
