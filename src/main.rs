use std::str::FromStr;

use actix::prelude::*;
use std::sync::Arc;
use tracing::info;
use tracing_subscriber::EnvFilter;
use venom::infrastructure::bus::in_memory_event::InMemoryEventBus;
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

    info!("🚀 Starting system with CommandBus");

    let mut cmd_bus = CommandBus::default();
    let event_bus = Arc::new(InMemoryEventBus::default());

    let supervisor = ComponentSupervisor::new(event_bus).start();
    cmd_bus.register(supervisor);

    let id = ComponentId::from_str("docker.io/library/nginx:1.21").unwrap();
    //let cmd = RegisterComponent { id };
    let cmd = Box::new(ComponentCommand {
        id,
        kind: ComponentCommandKind::Register,
    });

    //let result = bus.dispatch(Box::new(cmd)).await;
    let result = cmd_bus.dispatch(cmd);

    match result {
        Ok(()) => info!("✅ Command dispatched successfully."),
        Err(e) => info!("❌ Command dispatch failed: {e}"),
    }

    actix::clock::sleep(std::time::Duration::from_secs(1)).await;
}
