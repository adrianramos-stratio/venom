use crate::domain::component::Component;

use actix::Message;
use thiserror::Error;

#[derive(Debug, Clone, PartialEq)]
pub struct Collection {
    pub name: String,
    pub components: Vec<Component>,
}

impl Collection {
    pub fn new(
        name: impl Into<String>,
        components: Vec<Component>,
    ) -> Result<Self, CollectionError> {
        let name = name.into();
        if name.trim().is_empty() {
            return Err(CollectionError::EmptyName);
        }

        Ok(Self { name, components })
    }

    pub fn diff(&self, new: &Collection) -> Vec<CollectionEvent> {
        let old: std::collections::HashSet<_> = self.components.iter().cloned().collect();
        let new: std::collections::HashSet<_> = new.components.iter().cloned().collect();

        new.difference(&old)
            .map(|c| CollectionEvent::ComponentAdded(c.clone()))
            .chain(
                old.difference(&new)
                    .map(|c| CollectionEvent::ComponentDropped(c.clone())),
            )
            .chain(
                new.intersection(&old)
                    .map(|c| CollectionEvent::ComponentUnchanged(c.clone())),
            )
            .collect()
    }
}

#[derive(Debug, Error, Clone, PartialEq)]
pub enum CollectionError {
    #[error("Collection name cannot be empty")]
    EmptyName,
}

#[derive(Debug, Clone, Message)]
#[rtype(result = "()")]
pub enum CollectionEvent {
    ComponentAdded(Component),
    ComponentDropped(Component),
    ComponentUnchanged(Component),
}

pub trait Repository {
    fn list(&self) -> Result<Vec<Collection>, RepositoryError>;
}

#[derive(Debug, Error)]
pub enum RepositoryError {
    #[error("Failed to load collection data: {0}")]
    LoadError(String),

    #[error("Invalid data in repository: {0}")]
    InvalidData(String),
}
