use crate::domain::component::Component;
use thiserror::Error;

pub trait SbomGenerator: Send + Sync + 'static {
    fn generate_sbom(&self, component: &Component) -> Result<(), GeneratorError>;
}

#[derive(Debug, Error)]
pub enum GeneratorError {
    #[error("Failed to generate SBOM for {0}: {1}")]
    GenerationFailed(String, String),

    #[error("Tool '{0}' is unavailable: {1}")]
    ToolUnavailable(String, String),

    #[error("Destination '{0}' is unavailable: {1}")]
    DestinationUnavailable(String, String),
}
