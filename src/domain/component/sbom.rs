use std::fmt;
use std::path::PathBuf;
use url::Url;

/// Immutable reference to a Software Bill of Materials (SBOM)
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Sbom {
    location: SbomLocation,
}

impl Sbom {
    pub fn new(location: SbomLocation) -> Self {
        Self { location }
    }

    pub fn location(&self) -> &SbomLocation {
        &self.location
    }
}

impl fmt::Display for Sbom {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.location.fmt(f)
    }
}

/// Enum representing possible SBOM storage locations
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SbomLocation {
    /// A local file path
    Local(PathBuf),

    /// A valid remote URI (http, https, s3, etc.)
    Remote(Url),
}

impl SbomLocation {
    pub fn remote(uri: &str) -> Result<Self, SbomLocationError> {
        let parsed = Url::parse(uri)
            .map_err(|e| SbomLocationError::InvalidUri(uri.into(), e.to_string()))?;
        Ok(Self::Remote(parsed))
    }

    pub fn local(path: impl Into<PathBuf>) -> Self {
        Self::Local(path.into())
    }
}

impl fmt::Display for SbomLocation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SbomLocation::Local(p) => write!(f, "file://{}", p.display()),
            SbomLocation::Remote(uri) => write!(f, "{}", uri),
        }
    }
}

#[derive(Debug, thiserror::Error, Clone)]
pub enum SbomLocationError {
    #[error("Invalid URI `{0}`: {1}")]
    InvalidUri(String, String),
}
