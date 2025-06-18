use std::convert::TryFrom;
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

    pub fn from_url_str(s: &str) -> Result<Self, SbomLocationError> {
        let url =
            Url::parse(s).map_err(|e| SbomLocationError::InvalidUrl(s.into(), e.to_string()))?;
        Ok(Self::new(SbomLocation::remote(url)))
    }

    pub fn from_path_str(s: &str) -> Result<Self, SbomLocationError> {
        let path = PathBuf::from(s);
        if path.as_os_str().is_empty() {
            return Err(SbomLocationError::InvalidPathFormat(s.into()));
        }
        if path.exists() {
            Ok(Self::new(SbomLocation::local(path)))
        } else {
            Err(SbomLocationError::NonExistentPath(s.into()))
        }
    }
}

impl fmt::Display for Sbom {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.location.fmt(f)
    }
}

impl From<Url> for Sbom {
    fn from(url: Url) -> Self {
        Self::new(SbomLocation::remote(url))
    }
}

impl From<PathBuf> for Sbom {
    fn from(path: PathBuf) -> Self {
        Self::new(SbomLocation::local(path))
    }
}

impl TryFrom<&str> for Sbom {
    type Error = SbomLocationError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Sbom::from_url_str(value)
            .or_else(|_| Sbom::from_path_str(value))
            .map_err(|_| SbomLocationError::NotUrlNorPath(value.to_string()))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SbomLocation {
    Local(PathBuf),
    Remote(Url),
}

impl SbomLocation {
    pub fn remote(url: Url) -> Self {
        Self::Remote(url)
    }

    pub fn local(path: impl Into<PathBuf>) -> Self {
        Self::Local(path.into())
    }
}

impl fmt::Display for SbomLocation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SbomLocation::Local(p) => write!(f, "file://{}", p.display()),
            SbomLocation::Remote(url) => write!(f, "{}", url),
        }
    }
}

#[derive(Debug, thiserror::Error, Clone)]
pub enum SbomLocationError {
    #[error("Invalid URL `{0}`: {1}")]
    InvalidUrl(String, String),

    #[error("Path creation failed from `{0}`")]
    InvalidPathFormat(String),

    #[error("Path `{0}` does not exist")]
    NonExistentPath(String),

    #[error("Value `{0}` is neither a valid URL nor a valid path")]
    NotUrlNorPath(String),
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;

    #[test]
    fn test_sbom_from_valid_url_str() {
        let url = "https://example.com/sbom.json";
        let sbom = Sbom::from_url_str(url).expect("Expected Sbom from valid URL");
        assert!(matches!(sbom.location(), SbomLocation::Remote(_)));
    }

    #[test]
    fn test_sbom_from_invalid_url_str() {
        let url = "ht!tp:/::invalid";
        let err = Sbom::from_url_str(url).expect_err("Expected error for invalid URL");
        match err {
            SbomLocationError::InvalidUrl(u, _) => assert_eq!(u, url),
            _ => panic!("Unexpected error type"),
        }
    }

    #[test]
    fn test_sbom_from_existing_path_str() {
        let path = std::env::temp_dir().join("test_sbom.json");
        std::fs::write(&path, "dummy").unwrap();
        let sbom =
            Sbom::from_path_str(path.to_str().unwrap()).expect("Expected Sbom from valid path");
        assert!(matches!(sbom.location(), SbomLocation::Local(_)));
        std::fs::remove_file(&path).unwrap();
    }

    #[test]
    fn test_sbom_from_nonexistent_path_str() {
        let path = "/tmp/this_should_not_exist_123456.json";
        let err = Sbom::from_path_str(path).expect_err("Expected error for non-existent path");
        match err {
            SbomLocationError::NonExistentPath(p) => assert_eq!(p, path),
            _ => panic!("Unexpected error type"),
        }
    }

    #[test]
    fn test_sbom_try_from_valid_url_str() {
        let url = "https://example.com/sbom.json";
        let sbom = Sbom::try_from(url).expect("Expected TryFrom to succeed with valid URL");
        assert!(matches!(sbom.location(), SbomLocation::Remote(_)));
    }

    #[test]
    fn test_sbom_try_from_existing_path_str() {
        let path = std::env::temp_dir().join("test_sbom2.json");
        std::fs::write(&path, "dummy").unwrap();
        let sbom = Sbom::try_from(path.to_str().unwrap())
            .expect("Expected TryFrom to succeed with valid path");
        assert!(matches!(sbom.location(), SbomLocation::Local(_)));
        std::fs::remove_file(&path).unwrap();
    }

    #[test]
    fn test_sbom_from_invalid_path_format() {
        let input = ""; // will yield empty OsStr
        let err = Sbom::from_path_str(input).expect_err("Expected failure for empty path");
        match err {
            SbomLocationError::InvalidPathFormat(p) => assert_eq!(p, input),
            _ => panic!("Unexpected error type"),
        }
    }

    #[test]
    fn test_sbom_try_from_invalid_path_format() {
        let input = "";
        let err = Sbom::try_from(input).expect_err("Expected error for empty path in TryFrom");
        match err {
            SbomLocationError::NotUrlNorPath(p) => assert_eq!(p, input),
            _ => panic!("Unexpected error type"),
        }
    }

    #[test]
    fn test_sbom_try_from_not_url_nor_path() {
        let input = "not-a-valid-url-and-no-file";
        let err = Sbom::try_from(input).expect_err("Expected error for invalid string");
        match err {
            SbomLocationError::NotUrlNorPath(p) => assert_eq!(p, input),
            _ => panic!("Unexpected error type"),
        }
    }
}
