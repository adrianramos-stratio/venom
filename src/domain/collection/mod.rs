pub mod event;
pub mod id;

use crate::domain::collection::event::CollectionEvent;
use crate::domain::collection::id::CollectionId;
use crate::domain::component::id::ComponentId;
use crate::domain::shared::aggregate::EventSourcedAggregate;
use std::collections::HashSet;
use std::convert::TryFrom;
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Collection {
    id: CollectionId,
    components: HashSet<ComponentId>,
}

impl Collection {
    /// Emit an event to create a new collection with a given ID and initial set of components.
    ///
    /// This is the only valid way to construct a `Collection` aggregate,
    /// and must be followed by applying the returned event.
    ///
    /// # Errors
    ///
    /// Returns [`CollectionError::Empty`] if the initial component set is empty.
    pub fn create(
        id: CollectionId,
        initial: HashSet<ComponentId>,
    ) -> Result<CollectionEvent, CollectionError> {
        (!initial.is_empty())
            .then(|| CollectionEvent::CollectionCreated {
                collection_id: id,
                initial_components: initial.into_iter().collect(),
            })
            .ok_or(CollectionError::Empty)
    }

    /// Emit events to update the collection by replacing its entire set of components.
    ///
    /// Generates a list of [`CollectionEvent::ComponentAdded`] and [`CollectionEvent::ComponentDropped`]
    /// to reflect the difference between the current and the new set.
    ///
    /// # Errors
    ///
    /// Returns [`CollectionError::Empty`] if the provided set of components is empty.
    pub fn replace_components(
        &self,
        new_components: &HashSet<ComponentId>,
    ) -> Result<Vec<CollectionEvent>, CollectionError> {
        if new_components.is_empty() {
            return Err(CollectionError::Empty);
        }

        let dropped = self
            .components
            .difference(new_components)
            .cloned()
            .map(|component_id| CollectionEvent::ComponentDropped {
                collection_id: self.id.clone(),
                component_id,
            });

        let added = new_components
            .difference(&self.components)
            .cloned()
            .map(|component_id| CollectionEvent::ComponentAdded {
                collection_id: self.id.clone(),
                component_id,
            });

        Ok(dropped.chain(added).collect())
    }

    #[must_use]
    pub const fn id(&self) -> &CollectionId {
        &self.id
    }

    #[must_use]
    pub const fn components(&self) -> &HashSet<ComponentId> {
        &self.components
    }
}

impl TryFrom<&CollectionEvent> for Collection {
    type Error = CollectionError;

    fn try_from(event: &CollectionEvent) -> Result<Self, Self::Error> {
        match event {
            CollectionEvent::CollectionCreated {
                collection_id,
                initial_components,
            } => Ok(Self {
                id: collection_id.clone(),
                components: initial_components.iter().cloned().collect(),
            }),
            _ => Err(CollectionError::InvalidInitialEvent),
        }
    }
}

impl EventSourcedAggregate<CollectionEvent, CollectionError> for Collection {
    fn from_initial_event(event: &CollectionEvent) -> Result<Self, CollectionError> {
        Self::try_from(event)
    }

    fn apply(&mut self, event: &CollectionEvent) -> Result<(), CollectionError> {
        match event {
            CollectionEvent::CollectionCreated { .. } => {
                Err(CollectionError::CreatedEventNotAllowed)
            }
            CollectionEvent::ComponentAdded { component_id, .. } => {
                self.components.insert(component_id.clone());
                Ok(())
            }
            CollectionEvent::ComponentDropped { component_id, .. } => {
                self.components.remove(component_id);
                Ok(())
            }
        }
    }

    fn invalid_initial_event() -> CollectionError {
        CollectionError::InvalidInitialEvent
    }
}

#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum CollectionError {
    #[error("Collection cannot be empty")]
    Empty,

    #[error("CollectionCreated cannot be applied to an existing Collection")]
    CreatedEventNotAllowed,

    #[error("Only CollectionCreated can be used to initialize a Collection")]
    InvalidInitialEvent,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::collection::event::CollectionEvent;
    use crate::domain::collection::id::CollectionId;
    use crate::domain::component::id::ComponentId;
    use crate::domain::shared::aggregate::EventSourcedAggregate;
    use std::collections::HashSet;
    use std::str::FromStr;

    fn dummy_id() -> CollectionId {
        CollectionId::new("test-collection").unwrap()
    }

    fn comp(id: &str) -> ComponentId {
        ComponentId::from_str(format!("registry.test/namespace/{id}:v0").as_str()).unwrap()
    }

    fn initial_components() -> HashSet<ComponentId> {
        vec![comp("a"), comp("b"), comp("c")].into_iter().collect()
    }

    #[test]
    fn create_should_emit_event_with_initial_components() {
        let id = dummy_id();
        let components = initial_components();
        let event = Collection::create(id.clone(), components.clone()).unwrap();

        match event {
            CollectionEvent::CollectionCreated {
                collection_id,
                initial_components,
            } => {
                assert_eq!(collection_id, id);
                assert_eq!(initial_components.len(), 3);
            }
            _ => panic!("Expected CollectionCreated"),
        }
    }

    #[test]
    fn create_should_fail_with_empty_set() {
        let id = dummy_id();
        let result = Collection::create(id, HashSet::new());
        assert!(matches!(result, Err(CollectionError::Empty)));
    }

    #[test]
    fn try_from_collection_created_should_create_instance() {
        let id = dummy_id();
        let comps = initial_components();
        let event = CollectionEvent::CollectionCreated {
            collection_id: id.clone(),
            initial_components: comps.iter().cloned().collect(),
        };

        let collection = Collection::try_from(&event).unwrap();
        assert_eq!(collection.id(), &id);
        assert_eq!(collection.components().len(), 3);
    }

    #[test]
    fn try_from_non_initial_event_should_fail() {
        let id = dummy_id();
        let event = CollectionEvent::ComponentAdded {
            collection_id: id.clone(),
            component_id: comp("z"),
        };

        let result = Collection::try_from(&event);
        assert!(matches!(result, Err(CollectionError::InvalidInitialEvent)));
    }

    #[test]
    fn replace_components_should_emit_added_and_dropped_events() {
        let id = dummy_id();
        let initial: HashSet<ComponentId> = vec![comp("a"), comp("b")].into_iter().collect();
        let created = CollectionEvent::CollectionCreated {
            collection_id: id.clone(),
            initial_components: initial.iter().cloned().collect(),
        };

        let collection = Collection::from_initial_event(&created).unwrap();
        let new_comps: HashSet<ComponentId> = vec![comp("b"), comp("c")].into_iter().collect();

        let events = collection.replace_components(&new_comps).unwrap();
        assert_eq!(events.len(), 2);

        assert!(events.iter().any(|e| matches!(
            e,
            CollectionEvent::ComponentDropped { component_id, .. } if component_id.name() == "a"
        )));
        assert!(events.iter().any(|e| matches!(
            e,
            CollectionEvent::ComponentAdded { component_id, .. } if component_id.name() == "c"
        )));
    }

    #[test]
    fn apply_component_added_and_dropped_should_mutate_state() {
        let id = dummy_id();
        let comps = initial_components();
        let created = CollectionEvent::CollectionCreated {
            collection_id: id.clone(),
            initial_components: comps.iter().cloned().collect(),
        };

        let mut collection = Collection::from_initial_event(&created).unwrap();
        let add = CollectionEvent::ComponentAdded {
            collection_id: id.clone(),
            component_id: comp("z"),
        };
        let drop = CollectionEvent::ComponentDropped {
            collection_id: id.clone(),
            component_id: comp("a"),
        };

        collection.apply(&add).unwrap();
        collection.apply(&drop).unwrap();

        assert!(collection.components().contains(&comp("z")));
        assert!(!collection.components().contains(&comp("a")));
    }

    #[test]
    fn apply_collection_created_should_fail_on_existing_state() {
        let id = dummy_id();
        let comps = initial_components();
        let event = CollectionEvent::CollectionCreated {
            collection_id: id,
            initial_components: comps.iter().cloned().collect(),
        };

        let mut collection = Collection::from_initial_event(&event).unwrap();
        let result = collection.apply(&event);
        assert!(matches!(
            result,
            Err(CollectionError::CreatedEventNotAllowed)
        ));
    }

    #[test]
    fn rehydrate_should_rebuild_full_state() {
        let id = dummy_id();

        let e1 = CollectionEvent::CollectionCreated {
            collection_id: id.clone(),
            initial_components: vec![comp("a"), comp("b")],
        };

        let e2 = CollectionEvent::ComponentAdded {
            collection_id: id.clone(),
            component_id: comp("c"),
        };

        let e3 = CollectionEvent::ComponentDropped {
            collection_id: id.clone(),
            component_id: comp("a"),
        };

        let collection = Collection::rehydrate(&[e1, e2, e3]).unwrap();

        assert_eq!(collection.components().len(), 2);
        assert!(collection.components().contains(&comp("b")));
        assert!(collection.components().contains(&comp("c")));
        assert!(!collection.components().contains(&comp("a")));
    }
}
