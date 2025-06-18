use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CollectionId(String);

impl CollectionId {
    /// Create a new `CollectionId` from a string-like input.
    ///
    /// The provided ID must not be empty and must follow naming rules defined by the domain.
    ///
    /// # Errors
    ///
    /// Returns [`CollectionIdError::Empty`] if the provided ID string is empty.
    pub fn new(id: impl Into<String>) -> Result<Self, CollectionIdError> {
        let s = id.into();
        if s.trim().is_empty() {
            return Err(CollectionIdError::Empty);
        }
        Ok(Self(s))
    }

    #[must_use]
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
