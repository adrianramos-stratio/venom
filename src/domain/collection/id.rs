use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CollectionId(String);

impl CollectionId {
    pub fn new(id: impl Into<String>) -> Result<Self, CollectionIdError> {
        let s = id.into();
        if s.trim().is_empty() {
            return Err(CollectionIdError::Empty);
        }
        Ok(Self(s))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for CollectionId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum CollectionIdError {
    #[error("Collection ID cannot be empty")]
    Empty,
}
