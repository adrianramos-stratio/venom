pub mod context;
pub mod event;
pub mod id;
pub mod sbom;

use context::ExecutionContext;
use id::ComponentId;
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Component {
    pub id: ComponentId,
    pub context: ExecutionContext,
}

#[derive(Debug, Error, Clone)]
pub enum ComponentError {
    #[error("Invalid component reference format: {0}")]
    InvalidFormat(String),
}
