pub mod event;
pub mod id;

use std::collections::HashSet;

use crate::domain::collection::event::CollectionEvent;
use crate::domain::collection::id::CollectionId;
use crate::domain::component::id::ComponentId;
use thiserror::Error;

/// Aggregate root representing a logical collection of components.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Collection {
    id: CollectionId,
    components: HashSet<ComponentId>,
}

impl Collection {
    /// Create a new collection with a given id and set of components.
    pub fn new(
        id: CollectionId,
        components: HashSet<ComponentId>,
    ) -> Result<(Self, CollectionEvent), CollectionError> {
        if components.is_empty() {
            return Err(CollectionError::Empty(id.clone()));
        }

        let collection = Self {
            id: id.clone(),
            components: components.clone(),
        };

        let event = CollectionEvent::CollectionCreated {
            collection_id: id,
            initial_components: components.into_iter().collect(),
        };

        Ok((collection, event))
    }

    /// Replace all components with a new set, generating added/dropped events.
    pub fn replace_components(
        &self,
        new_components: HashSet<ComponentId>,
    ) -> Result<(Self, Vec<CollectionEvent>), CollectionError> {
        if new_components.is_empty() {
            return Err(CollectionError::Empty(self.id.clone()));
        }

        let mut events = Vec::new();

        for removed in self.components.difference(&new_components) {
            events.push(CollectionEvent::ComponentDropped {
                collection_id: self.id.clone(),
                component_id: removed.clone(),
            });
        }

        for added in new_components.difference(&self.components) {
            events.push(CollectionEvent::ComponentAdded {
                collection_id: self.id.clone(),
                component_id: added.clone(),
            });
        }

        let updated = Self {
            id: self.id.clone(),
            components: new_components,
        };

        Ok((updated, events))
    }

    /// Apply a single event to mutate the state (used for event sourcing).
    pub fn apply(&mut self, event: &CollectionEvent) {
        match event {
            CollectionEvent::ComponentAdded { component_id, .. } => {
                self.components.insert(component_id.clone());
            }
            CollectionEvent::ComponentDropped { component_id, .. } => {
                self.components.remove(component_id);
            }
            CollectionEvent::CollectionCreated {
                initial_components, ..
            } => {
                self.components = initial_components.iter().cloned().collect();
            }
        }
    }

    /// Read-only accessor for the collection ID.
    pub fn id(&self) -> &CollectionId {
        &self.id
    }

    /// Read-only accessor for component IDs.
    pub fn components(&self) -> &HashSet<ComponentId> {
        &self.components
    }
}

/// Errors that can arise when operating on a Collection aggregate.
#[derive(Debug, Error, Clone)]
pub enum CollectionError {
    #[error("Collection `{0}` cannot be empty")]
    Empty(CollectionId),
}
