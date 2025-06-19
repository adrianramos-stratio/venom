use crate::domain::collection::id::CollectionId;
use crate::domain::component::id::ComponentId;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CollectionEvent {
    /// A new collection was created with an initial set of components
    CollectionCreated {
        collection_id: CollectionId,
        initial_components: Vec<ComponentId>,
    },

    /// A component was added to the collection (present in new set but not in previous)
    ComponentAdded {
        collection_id: CollectionId,
        component_id: ComponentId,
    },

    /// A component was removed from the collection (present in old set but not in new)
    ComponentDropped {
        collection_id: CollectionId,
        component_id: ComponentId,
    },
}
