use std::fs;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use tracing::info;

use crate::application::service::sbom_generator::{SbomGenerator, SbomGeneratorError};
use crate::domain::component::{id::ComponentId, sbom::Sbom};

pub struct SyftSbomGenerator {
    base_path: PathBuf,
}

impl SyftSbomGenerator {
    /// Create a new `SyftSbomGenerator`, validating that the `syft` CLI tool is available.
    ///
    /// # Errors
    ///
    /// Returns [`SbomGeneratorError::ToolUnavailable`] if the `syft` tool is not installed or not executable.
    pub fn new(base_path: impl Into<PathBuf>) -> Result<Self, SbomGeneratorError> {
        let status = Command::new("syft")
            .arg("--version")
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status();

        match status {
            Ok(s) if s.success() => Ok(Self {
                base_path: base_path.into(),
            }),
            Ok(_) => Err(SbomGeneratorError::ToolUnavailable(
                "syft".to_string(),
                "syft --version returned non-zero exit code".to_string(),
            )),
            Err(e) => Err(SbomGeneratorError::ToolUnavailable(
                "syft".to_string(),
                format!("syft not found or not executable: {e}"),
            )),
        }
    }
}

impl SbomGenerator for SyftSbomGenerator {
    fn generate(&self, component: &ComponentId) -> Result<Sbom, SbomGeneratorError> {
        let mut sbom_dir = self.base_path.join(component.registry());

        if let Some(ns) = &component.namespace() {
            sbom_dir = sbom_dir.join(ns);
        }

        if !sbom_dir.exists() {
            fs::create_dir_all(&sbom_dir).map_err(|e| {
                SbomGeneratorError::DestinationUnavailable(
                    sbom_dir.as_path().to_string_lossy().to_string(),
                    e.to_string(),
                )
            })?;
        }

        let filename = format!("{}_{}_sbom.json", component.name(), component.tag());
        let sbom_path = sbom_dir.join(filename);

        if sbom_path.exists() {
            return Err(SbomGeneratorError::DestinationAlreadyExists(
                sbom_path.to_string_lossy().to_string(),
            ));
        }

        let target = format!("registry:{component}");

        info!("Generating sbom for {target} in {sbom_path:?}");

        let status = Command::new("syft")
            .arg(&target)
            .arg("--output")
            .arg(format!("cyclonedx-json={}", sbom_path.to_string_lossy()))
            .stdout(Stdio::null()) // suppress stdout
            .stderr(Stdio::null()) // suppress stderr
            .status()
            .map_err(|e| {
                SbomGeneratorError::GenerationFailed(component.to_string(), e.to_string())
            })?;

        if status.success() {
            // Check if output file exists and is non-empty
            match fs::metadata(&sbom_path) {
                Ok(meta) if meta.len() == 0 => {
                    let _ = fs::remove_file(&sbom_path); // remove silently
                    Err(SbomGeneratorError::GenerationFailed(
                        component.to_string(),
                        "SBOM file was created but is empty".to_string(),
                    ))
                }
                Ok(_) => Ok(Sbom::from(sbom_path)),
                Err(e) => Err(SbomGeneratorError::GenerationFailed(
                    component.to_string(),
                    format!("SBOM file metadata check failed: {e}"),
                )),
            }
        } else {
            let _ = fs::remove_file(&sbom_path); // ensure no empty file remains
            Err(SbomGeneratorError::GenerationFailed(
                component.to_string(),
                format!("Syft exited with status {status}"),
            ))
        }
    }
}
