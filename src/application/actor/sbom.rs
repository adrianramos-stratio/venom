use actix::fut;
use actix::prelude::*;
use std::sync::Arc;
use tokio::task;
use tracing::error;

use crate::domain::{collection::CollectionEvent, sbom::SbomGenerator};

pub struct SbomActor {
    pub generator: Arc<dyn SbomGenerator + Send + Sync>,
}

impl SbomActor {
    pub fn new(generator: Arc<dyn SbomGenerator + Send + Sync>) -> Self {
        Self { generator }
    }
}

impl Actor for SbomActor {
    type Context = Context<Self>;
}

impl Handler<CollectionEvent> for SbomActor {
    type Result = ResponseActFuture<Self, ()>;

    fn handle(&mut self, msg: CollectionEvent, _ctx: &mut Self::Context) -> Self::Result {
        match msg {
            CollectionEvent::ComponentAdded(component) => {
                let generator = Arc::clone(&self.generator);

                // Envolvemos un future en una tarea de bloqueo
                let fut = async move {
                    task::spawn_blocking(move || {
                        if let Err(e) = generator.generate_sbom(&component) {
                            error!("Could not generate SBOM for `{}`: {}", component.name, e);
                        }
                    })
                    .await
                    .unwrap_or_else(|e| {
                        error!("Failed to spawn task for SBOM generation: {e}");
                    });
                };

                Box::pin(
                    fut::wrap_future(fut).map(|_result, _actor: &mut Self, _ctx| {
                        tracing::info!("SBOM generation task completed.");
                    }),
                )
            }
            _ => {
                tracing::info!("Sbom already exists");
                Box::pin(fut::ready(()))
            }
        }
    }
}
