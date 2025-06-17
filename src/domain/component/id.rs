use std::{fmt, str::FromStr};
use thiserror::Error;
use tracing::warn;

const DEFAULT_REGISTRY: &str = "docker.io";
const LATEST_TAG: &str = "latest";

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ComponentId {
    registry: String,
    namespace: Option<String>,
    name: String,
    tag: String,
}

impl FromStr for ComponentId {
    type Err = ComponentIdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.trim().is_empty() {
            return Err(ComponentIdError::InvalidFormat(s.to_string()));
        }

        // Extract optional tag, if present
        let (reference, tag) = match s.rsplit_once(':') {
            Some((left, right)) if !right.contains('/') => (left, right.to_string()),
            _ => (s, LATEST_TAG.to_string()),
        };

        if tag.eq_ignore_ascii_case(LATEST_TAG) {
            warn!("{s} uses '{LATEST_TAG}' as a tag, which is discouraged.");
        }

        let segments: Vec<&str> = reference.split('/').collect();
        let is_registry = |s: &str| s.contains('.') || s.contains(':') || s == "localhost";

        let component = match segments.as_slice() {
            // Single-part: image name
            [name] => Self {
                registry: DEFAULT_REGISTRY.to_string(),
                namespace: None,
                name: (*name).to_string(),
                tag,
            },

            // Registry + image name (no namespace)
            [registry, name] if is_registry(registry) => Self {
                registry: (*registry).to_string(),
                namespace: None,
                name: (*name).to_string(),
                tag,
            },

            // Namespace + image name
            [namespace, name] => Self {
                registry: DEFAULT_REGISTRY.to_string(),
                namespace: Some((*namespace).to_string()),
                name: (*name).to_string(),
                tag,
            },

            // Registry + namespace(s) + image name
            [registry, middle @ .., name] if is_registry(registry) => Self {
                registry: (*registry).to_string(),
                namespace: Some(middle.join("/")),
                name: (*name).to_string(),
                tag,
            },

            // Implicit registry + nested namespace + name
            [middle @ .., name] => Self {
                registry: DEFAULT_REGISTRY.to_string(),
                namespace: Some(middle.join("/")),
                name: (*name).to_string(),
                tag,
            },

            _ => return Err(ComponentIdError::InvalidFormat(s.to_string())),
        };

        if component.name.is_empty() {
            return Err(ComponentIdError::InvalidFormat(s.to_string()));
        }

        Ok(component)
    }
}

impl fmt::Display for ComponentId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.namespace {
            Some(ns) => write!(f, "{}/{}/{}:{}", self.registry, ns, self.name, self.tag),
            None => write!(f, "{}/{}:{}", self.registry, self.name, self.tag),
        }
    }
}

#[derive(Debug, Error, Clone)]
pub enum ComponentIdError {
    #[error("Invalid component id format: {0}")]
    InvalidFormat(String),
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    fn comp(input: &str) -> ComponentId {
        ComponentId::from_str(input).unwrap()
    }

    #[test]
    fn test_defaults_to_docker_io_and_library() {
        let c = comp("nginx");
        assert_eq!(c.registry, "docker.io");
        assert_eq!(c.namespace, None);
        assert_eq!(c.name, "nginx");
        assert_eq!(c.tag, "latest");
    }

    #[test]
    fn test_custom_tag_without_registry_or_namespace() {
        let c = comp("nginx:1.21.0");
        assert_eq!(c.registry, "docker.io");
        assert_eq!(c.namespace, None);
        assert_eq!(c.name, "nginx");
        assert_eq!(c.tag, "1.21.0");
    }

    #[test]
    fn test_with_namespace() {
        let c = comp("stratio/nginx:1.2");
        assert_eq!(c.registry, "docker.io");
        assert_eq!(c.namespace, Some("stratio".to_string()));
        assert_eq!(c.name, "nginx");
        assert_eq!(c.tag, "1.2");
    }

    #[test]
    fn test_with_registry_namespace_and_tag() {
        let c = comp("ghcr.io/stratio/nginx:v2.0");
        assert_eq!(c.registry, "ghcr.io");
        assert_eq!(c.namespace, Some("stratio".to_string()));
        assert_eq!(c.name, "nginx");
        assert_eq!(c.tag, "v2.0");
    }

    #[test]
    fn test_with_nested_namespace() {
        let c = comp("ghcr.io/team/a/b/c:v4");
        assert_eq!(c.registry, "ghcr.io");
        assert_eq!(c.namespace, Some("team/a/b".to_string()));
        assert_eq!(c.name, "c");
        assert_eq!(c.tag, "v4");
    }

    #[test]
    fn test_display_roundtrip() {
        let input = "ghcr.io/stratio/nginx:v2.0";
        let c = ComponentId::from_str(input).unwrap();
        assert_eq!(c.to_string(), input);
    }

    #[test]
    fn test_invalid_format_empty() {
        let err = ComponentId::from_str("").unwrap_err();
        assert!(matches!(err, ComponentIdError::InvalidFormat(_)));
    }

    #[test]
    fn test_invalid_format_just_colon() {
        let err = ComponentId::from_str(":").unwrap_err();
        assert!(matches!(err, ComponentIdError::InvalidFormat(_)));
    }
}
