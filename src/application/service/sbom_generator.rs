use crate::domain::component::{id::ComponentId, sbom::Sbom};
use thiserror::Error;

pub trait SbomGenerator: Send + Sync {
    /// Generates a Software Bill of Materials (SBOM) for the given component ID.
    ///
    /// # Errors
    ///
    /// Returns [`SbomGeneratorError`] if the SBOM generation process fails
    /// due to missing tools, filesystem errors, or command execution issues.
    fn generate(&self, id: &ComponentId) -> Result<Sbom, SbomGeneratorError>;
}

#[derive(Debug, Error)]
pub enum SbomGeneratorError {
    #[error("Required tool '{0}' is not available: {1}")]
    ToolUnavailable(String, String),

    #[error("Could not create destination '{0}': {1}")]
    DestinationUnavailable(String, String),

    #[error("Failed to generate SBOM for '{0}': {1}")]
    GenerationFailed(String, String),

    #[error("Destination '{0}' already exists")]
    DestinationAlreadyExists(String),
}
