use crate::domain::component::context::ExecutionContext;
use crate::domain::component::id::ComponentId;
use crate::domain::component::sbom::Sbom;

/// Domain events emitted by the `Component` aggregate.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ComponentEvent {
    /// A new component has been registered with the given identifier.
    ComponentRegistered { component_id: ComponentId },

    /// The component has been deprecated and will no longer be used.
    ComponentDeprecated { component_id: ComponentId },

    /// A software bill of materials (SBOM) has been assigned to the component.
    ///
    /// This event is only emitted once per component.
    SbomAssigned {
        component_id: ComponentId,
        sbom: Sbom,
    },

    /// An execution context has been initially assigned to the component.
    ///
    /// This context is a required prerequisite for classification.
    ExecutionContextAssigned {
        component_id: ComponentId,
        context: ExecutionContext,
    },

    /// The execution context has been replaced by a new one.
    ///
    /// This may occur after reclassification or environment changes.
    ExecutionContextReplaced {
        component_id: ComponentId,
        context: ExecutionContext,
    },
}
