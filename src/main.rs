use actix::Actor;
use std::{io, str::FromStr};
use tracing::info;
use tracing_subscriber::EnvFilter;
use venom::{
    application::{
        aggregate::component::{cmd::RegisterComponent, supervisor::ComponentSupervisor},
        saga::sbom_generation::{HandleComponentRegistered, SbomGenerationSaga},
    },
    config::VulmanConfig,
    domain::component::{event::ComponentEvent, id::ComponentId},
    infrastructure::generator::syft::SyftSbomGenerator,
};

#[actix::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::from_default_env().add_directive("venom=debug".parse().unwrap()),
        )
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

    let generator = SyftSbomGenerator::new(&config.sboms_path).unwrap();
    let supervisor = ComponentSupervisor::new().start();
    let saga = SbomGenerationSaga {
        generator: Box::new(generator),
        supervisor: supervisor.clone(),
    }
    .start();

    let id = ComponentId::from_str("docker.io/alpine:3.11").unwrap();

    // Registrar componente (esto lanza al actor supervisor)
    let _ack = supervisor
        .send(RegisterComponent { id: id.clone() })
        .await
        .unwrap();
    let handled = saga
        .send(HandleComponentRegistered(
            ComponentEvent::ComponentRegistered {
                component_id: id.clone(),
            },
        ))
        .await;

    tracing::info!("Sent event: handled = {:?}", handled);
}
