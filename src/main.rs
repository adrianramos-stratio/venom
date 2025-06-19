use std::io;
use std::str::FromStr;
use venom::config::VenomConfig;

use actix::prelude::*;
use std::sync::{Arc, Mutex};
use tracing::info;
use tracing_subscriber::EnvFilter;
use venom::application::saga::sbom_generation::SbomGenerationSaga;
use venom::infrastructure::bus::in_memory_event::InMemoryEventBus;
use venom::infrastructure::generator::syft::SyftSbomGenerator;
use venom::{
    application::{
        aggregate::component::{
            cmd::{ComponentCommand, ComponentCommandKind},
            supervisor::ComponentSupervisor,
        },
        shared::command::CommandBus,
    },
    domain::component::id::ComponentId,
};

#[actix::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::from_default_env().add_directive("venom=trace".parse().unwrap()),
        )
        .init();

    let config = VenomConfig::load()
        .map_err(|e| {
            io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("Error loading settings: {e}"),
            )
            //})?;
        })
        .unwrap();

    info!("🚀 Starting system with CommandBus");

    let cmd_bus = Arc::new(Mutex::new(CommandBus::default()));
    let event_bus = Arc::new(InMemoryEventBus::default());

    let supervisor = ComponentSupervisor::new(event_bus.clone()).start();
    let generator = Box::new(SyftSbomGenerator::new(&config.sboms_path).unwrap());
    let sbom_saga = SbomGenerationSaga::new(cmd_bus.clone(), generator).start();
    let _ = event_bus.subscribe(Arc::new(sbom_saga));

    // Only mutable for registering
    let mut cmd_bus = cmd_bus.lock().unwrap();
    cmd_bus.register(supervisor);

    let components = vec![
        "docker.io/library/nginx:1.21",
        "docker.io/library/redis:7.2",
        "docker.io/library/postgres:16",
        "docker.io/library/alpine:3.19",
        "docker.io/library/httpd:2.4",
        "docker.io/library/node:20",
        "docker.io/library/python:3.12",
        "docker.io/library/golang:1.22",
        "docker.io/library/rust:1.77",
        "docker.io/library/ubuntu:24.04",
    ];

    for ref_id in components {
        let id = ComponentId::from_str(ref_id).unwrap();
        let cmd = Box::new(ComponentCommand {
            id,
            kind: ComponentCommandKind::Register,
        });

        let result = cmd_bus.dispatch(cmd);
        match result {
            Ok(()) => info!("✅ Dispatched {ref_id}"),
            Err(e) => info!("❌ Failed dispatch for {ref_id}: {e}"),
        }
    }
    actix::clock::sleep(std::time::Duration::from_secs(25)).await;
}
