pub mod context;
pub mod event;
pub mod id;
pub mod sbom;

use crate::domain::component::context::ExecutionContext;
use crate::domain::component::event::ComponentEvent;
use crate::domain::component::id::ComponentId;
use crate::domain::component::sbom::Sbom;

use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Component {
    id: ComponentId,
    sbom: Option<Sbom>,
    context: Option<ExecutionContext>,
    deprecated: bool,
}

impl Component {
    /// Register a new component from its ID.
    pub fn register(id: ComponentId) -> (Self, ComponentEvent) {
        let component = Self {
            id: id.clone(),
            sbom: None,
            context: None,
            deprecated: false,
        };

        let event = ComponentEvent::ComponentRegistered { component_id: id };

        (component, event)
    }

    /// Mark this component as deprecated.
    pub fn deprecate(&mut self) -> Result<ComponentEvent, ComponentError> {
        if self.deprecated {
            return Err(ComponentError::AlreadyDeprecated(self.id.clone()));
        }

        self.deprecated = true;

        Ok(ComponentEvent::ComponentDeprecated {
            component_id: self.id.clone(),
        })
    }

    /// Assign the first SBOM to the component.
    pub fn assign_sbom(&mut self, sbom: Sbom) -> Result<ComponentEvent, ComponentError> {
        if self.sbom.is_some() {
            return Err(ComponentError::SbomAlreadyAssigned(self.id.clone()));
        }

        self.sbom = Some(sbom.clone());

        Ok(ComponentEvent::SbomAssigned {
            component_id: self.id.clone(),
            sbom,
        })
    }

    /// Assign the first execution context to the component.
    pub fn assign_execution_context(
        &mut self,
        context: ExecutionContext,
    ) -> Result<ComponentEvent, ComponentError> {
        if self.context.is_some() {
            return Err(ComponentError::ExecutionContextAlreadyAssigned(
                self.id.clone(),
            ));
        }

        self.context = Some(context.clone());

        Ok(ComponentEvent::ExecutionContextAssigned {
            component_id: self.id.clone(),
            context,
        })
    }

    /// Replace the current execution context with a new one.
    pub fn replace_execution_context(
        &mut self,
        context: ExecutionContext,
    ) -> Result<ComponentEvent, ComponentError> {
        self.context = Some(context.clone());

        Ok(ComponentEvent::ExecutionContextReplaced {
            component_id: self.id.clone(),
            context,
        })
    }

    /// Apply an event to reconstruct the component state (Event Sourcing)
    pub fn apply(&mut self, event: &ComponentEvent) {
        match event {
            ComponentEvent::ComponentRegistered { .. } => {}
            ComponentEvent::ComponentDeprecated { .. } => {
                self.deprecated = true;
            }
            ComponentEvent::SbomAssigned { sbom, .. } => {
                self.sbom = Some(sbom.clone());
            }
            ComponentEvent::ExecutionContextAssigned { context, .. }
            | ComponentEvent::ExecutionContextReplaced { context, .. } => {
                self.context = Some(context.clone());
            }
        }
    }

    // Getters (read-only access)

    pub fn id(&self) -> &ComponentId {
        &self.id
    }

    pub fn sbom(&self) -> Option<&Sbom> {
        self.sbom.as_ref()
    }

    pub fn context(&self) -> Option<&ExecutionContext> {
        self.context.as_ref()
    }

    pub fn is_deprecated(&self) -> bool {
        self.deprecated
    }
}

#[derive(Debug, Error, Clone)]
pub enum ComponentError {
    #[error("Component `{0}` is already deprecated")]
    AlreadyDeprecated(ComponentId),

    #[error("Component `{0}` already has an SBOM assigned")]
    SbomAlreadyAssigned(ComponentId),

    #[error("Component `{0}` already has an execution context assigned")]
    ExecutionContextAlreadyAssigned(ComponentId),
}
