use crate::application::actor::collection::{CollectionActor, CollectionCommand};
use crate::domain::{collection::Collection, component::Component};
use actix_web::{post, web, HttpResponse, Responder};
use std::str::FromStr;

use serde::Deserialize;

use actix::Addr;

#[derive(Clone)]
pub struct AppState {
    pub collection_actor: Addr<CollectionActor>,
}

#[derive(Debug, Deserialize)]
pub struct CollectionInput {
    pub name: String,
    pub components: Vec<String>,
}

#[post("/analyze")]
async fn analyze_collection(
    input: web::Json<CollectionInput>,
    data: web::Data<AppState>,
) -> impl Responder {
    let components = input
        .components
        .iter()
        .filter_map(|s| Component::from_str(s).ok())
        .collect();

    let collection = match Collection::new(&input.name, components) {
        Ok(c) => c,
        Err(e) => return HttpResponse::BadRequest().body(format!("Invalid collection: {e}")),
    };

    // Enviar comando
    if let Err(e) = data
        .collection_actor
        .send(CollectionCommand::ProcessCollection(collection))
        .await
    {
        return HttpResponse::InternalServerError().body(format!("Actor error: {e}"));
    }

    HttpResponse::Ok().body("Analysis started")
}
