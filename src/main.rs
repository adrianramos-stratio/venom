use std::str::FromStr;

use actix::prelude::*;
use tracing::info;
use tracing_subscriber::EnvFilter;
use venom::application::aggregate::component::cmd::AssignSbomToComponent;
use venom::domain::component::sbom::Sbom;
use venom::{
    application::{
        aggregate::component::{cmd::RegisterComponent, supervisor::ComponentSupervisor},
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

    info!("🚀 Starting system with CommandBus");

    let mut bus = CommandBus::default();

    let supervisor = ComponentSupervisor::new().start();
    bus.register(supervisor);

    let id = ComponentId::from_str("docker.io/library/nginx:1.21").unwrap();
    //let cmd = RegisterComponent { id };
    let cmd = Box::new(RegisterComponent { id });

    //let result = bus.dispatch(Box::new(cmd)).await;
    let result = bus.dispatch(cmd);

    match result {
        Ok(()) => info!("✅ Command dispatched successfully."),
        Err(e) => info!("❌ Command dispatch failed: {e}"),
    }

    let cmd2 = Box::new(AssignSbomToComponent {
        id: ComponentId::from_str("docker.io/library/nginx:1.21").unwrap(),
        sbom: Sbom::from_url_str("https://raw.githubusercontent.com/CycloneDX/bom-examples/refs/heads/master/SBOM/laravel-7.12.0/bom.1.2.json").unwrap(),
    });

    let result = bus.dispatch(cmd2);

    match result {
        Ok(()) => info!("✅ Second command dispatched successfully."),
        Err(e) => info!("❌ Second command failed: {e}"),
    }

    actix::clock::sleep(std::time::Duration::from_secs(1)).await;
}
