pub mod context;
pub mod event;
pub mod id;
pub mod sbom;

use crate::domain::component::context::ExecutionContext;
use crate::domain::component::event::ComponentEvent;
use crate::domain::component::id::ComponentId;
use crate::domain::component::sbom::Sbom;
use crate::domain::shared::aggregate::EventSourcedAggregate;
use std::convert::TryFrom;
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Component {
    id: ComponentId,
    sbom: Option<Sbom>,
    context: Option<ExecutionContext>,
    deprecated: bool,
}

impl Component {
    #[must_use]
    pub const fn new(id: ComponentId) -> Self {
        Self {
            id,
            sbom: None,
            context: None,
            deprecated: false,
        }
    }

    /// Emit event for registering a new component
    #[must_use]
    pub const fn register(id: ComponentId) -> ComponentEvent {
        ComponentEvent::ComponentRegistered { component_id: id }
    }

    /// Emit an event to mark the component as deprecated.
    ///
    /// This operation can only be performed once.
    ///
    /// # Errors
    ///
    /// Returns [`ComponentError::AlreadyDeprecated`] if the component has already been deprecated.
    pub fn deprecate(&self) -> Result<ComponentEvent, ComponentError> {
        if self.deprecated {
            Err(ComponentError::AlreadyDeprecated(self.id.clone()))
        } else {
            Ok(ComponentEvent::ComponentDeprecated {
                component_id: self.id.clone(),
            })
        }
    }

    /// Emit an event to assign the initial SBOM to the component.
    ///
    /// This method ensures that an SBOM can only be assigned once.
    ///
    /// # Errors
    ///
    /// Returns [`ComponentError::SbomAlreadyAssigned`] if the component already has an SBOM.
    pub fn assign_sbom(&self, sbom: Sbom) -> Result<ComponentEvent, ComponentError> {
        if self.sbom.is_some() {
            Err(ComponentError::SbomAlreadyAssigned(self.id.clone()))
        } else {
            Ok(ComponentEvent::SbomAssigned {
                component_id: self.id.clone(),
                sbom,
            })
        }
    }

    /// Emit an event to assign an initial execution context to the component.
    ///
    /// This method fails if the component already has an execution context assigned.
    ///
    /// # Errors
    ///
    /// Returns [`ComponentError::ExecutionContextAlreadyAssigned`] if a context has already been assigned.
    pub fn assign_execution_context(
        &self,
        context: ExecutionContext,
    ) -> Result<ComponentEvent, ComponentError> {
        if self.context.is_some() {
            Err(ComponentError::ExecutionContextAlreadyAssigned(
                self.id.clone(),
            ))
        } else {
            Ok(ComponentEvent::ExecutionContextAssigned {
                component_id: self.id.clone(),
                context,
            })
        }
    }

    /// Emit an event to replace the current execution context of the component.
    ///
    /// # Errors
    ///
    /// Returns [`ComponentError::ExecutionContextNotAssigned`] if no execution context was assigned yet.
    pub fn replace_execution_context(
        &self,
        context: ExecutionContext,
    ) -> Result<ComponentEvent, ComponentError> {
        if self.context.is_none() {
            Err(ComponentError::ExecutionContextNotAssigned(self.id.clone()))
        } else {
            Ok(ComponentEvent::ExecutionContextReplaced {
                component_id: self.id.clone(),
                context,
            })
        }
    }

    // Accessors

    #[must_use]
    pub const fn id(&self) -> &ComponentId {
        &self.id
    }

    #[must_use]
    pub const fn sbom(&self) -> Option<&Sbom> {
        self.sbom.as_ref()
    }

    #[must_use]
    pub const fn context(&self) -> Option<&ExecutionContext> {
        self.context.as_ref()
    }

    #[must_use]
    pub const fn is_deprecated(&self) -> bool {
        self.deprecated
    }
}

impl TryFrom<&ComponentEvent> for Component {
    type Error = ComponentError;

    fn try_from(event: &ComponentEvent) -> Result<Self, Self::Error> {
        match event {
            ComponentEvent::ComponentRegistered { component_id } => {
                Ok(Self::new(component_id.clone()))
            }
            _ => Err(ComponentError::InvalidInitialEvent),
        }
    }
}

impl EventSourcedAggregate<ComponentEvent, ComponentError> for Component {
    fn from_initial_event(event: &ComponentEvent) -> Result<Self, ComponentError> {
        Self::try_from(event)
    }

    fn apply(&mut self, event: &ComponentEvent) -> Result<(), ComponentError> {
        match event {
            ComponentEvent::ComponentRegistered { .. } => {
                Err(ComponentError::RegisteredEventNotAllowed)
            }
            ComponentEvent::ComponentDeprecated { .. } => {
                self.deprecated = true;
                Ok(())
            }
            ComponentEvent::SbomAssigned { sbom, .. } => {
                self.sbom = Some(sbom.clone());
                Ok(())
            }
            ComponentEvent::ExecutionContextAssigned { context, .. }
            | ComponentEvent::ExecutionContextReplaced { context, .. } => {
                self.context = Some(context.clone());
                Ok(())
            }
        }
    }

    fn invalid_initial_event() -> ComponentError {
        ComponentError::InvalidInitialEvent
    }
}

#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum ComponentError {
    #[error("Component `{0}` is already registered")]
    AlreadyRegistered(ComponentId),

    #[error("Component `{0}` is already deprecated")]
    AlreadyDeprecated(ComponentId),

    #[error("Component `{0}` already has an SBOM assigned")]
    SbomAlreadyAssigned(ComponentId),

    #[error("Component `{0}` has no execution context assigned")]
    ExecutionContextNotAssigned(ComponentId),

    #[error("Component `{0}` already has an execution context assigned")]
    ExecutionContextAlreadyAssigned(ComponentId),

    #[error("Event `ComponentRegistered` cannot be applied to an existing aggregate")]
    RegisteredEventNotAllowed,

    #[error("Only `ComponentRegistered` can be used to initialize a Component")]
    InvalidInitialEvent,

    #[error("Inconsistent ids: `{0}` != `{1}`")]
    InconsistentIds(String, String),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::component::context::ExecutionContext;
    use crate::domain::component::event::ComponentEvent;
    use crate::domain::component::id::ComponentId;
    use crate::domain::component::sbom::Sbom;
    use crate::domain::shared::aggregate::EventSourcedAggregate;
    use std::str::FromStr;

    fn dummy_id() -> ComponentId {
        ComponentId::from_str("registry.test/namespace/image:v0").unwrap()
    }

    fn dummy_sbom() -> Sbom {
        Sbom::from_url_str("https://example.com/sbom.json").unwrap()
    }

    fn dummy_context() -> ExecutionContext {
        ExecutionContext::None
    }

    #[test]
    fn register_and_apply_should_build_component_correctly() {
        let id = dummy_id();
        let event = Component::register(id.clone());
        let component = Component::from_initial_event(&event).unwrap();

        assert_eq!(component.id(), &id);
        assert!(!component.is_deprecated());
        assert!(component.sbom().is_none());
        assert!(component.context().is_none());
    }

    #[test]
    fn apply_sbom_assignment() {
        let id = dummy_id();
        let register = Component::register(id.clone());
        let mut component = Component::from_initial_event(&register).unwrap();

        let sbom = dummy_sbom();
        let assign_event = component.assign_sbom(sbom.clone()).unwrap();
        component.apply(&assign_event).unwrap();

        assert_eq!(component.sbom(), Some(&sbom));
    }

    #[test]
    fn assign_sbom_twice_should_fail() {
        let id = dummy_id();
        let register = Component::register(id.clone());
        let mut component = Component::from_initial_event(&register).unwrap();

        let sbom = dummy_sbom();
        let event = component.assign_sbom(sbom.clone()).unwrap();
        component.apply(&event).unwrap();

        let err = component.assign_sbom(sbom).unwrap_err();
        assert!(matches!(err, ComponentError::SbomAlreadyAssigned(_)));
    }

    #[test]
    fn assign_and_replace_execution_context() {
        let id = dummy_id();
        let register = Component::register(id.clone());
        let mut component = Component::from_initial_event(&register).unwrap();

        let ctx1 = dummy_context();
        let event1 = component.assign_execution_context(ctx1.clone()).unwrap();
        component.apply(&event1).unwrap();

        assert_eq!(component.context(), Some(&ctx1));

        let ctx2 = ExecutionContext::None;
        let event2 = component.replace_execution_context(ctx2.clone()).unwrap();
        component.apply(&event2).unwrap();

        assert_eq!(component.context(), Some(&ctx2));
    }

    #[test]
    fn assign_context_twice_should_fail() {
        let id = dummy_id();
        let register = Component::register(id.clone());
        let mut component = Component::from_initial_event(&register).unwrap();

        let ctx = dummy_context();
        let e1 = component.assign_execution_context(ctx.clone()).unwrap();
        component.apply(&e1).unwrap();

        let err = component.assign_execution_context(ctx).unwrap_err();
        assert!(matches!(
            err,
            ComponentError::ExecutionContextAlreadyAssigned(_)
        ));
    }

    #[test]
    fn apply_registered_event_should_fail_on_existing_aggregate() {
        let id = dummy_id();
        let event = Component::register(id.clone());
        let mut component = Component::from_initial_event(&event).unwrap();

        let result = component.apply(&event);
        assert_eq!(
            result.unwrap_err(),
            ComponentError::RegisteredEventNotAllowed
        );
    }

    #[test]
    fn deprecate_should_work_and_fail_on_second_attempt() {
        let id = dummy_id();
        let event = Component::register(id.clone());
        let mut component = Component::from_initial_event(&event).unwrap();

        let deprecate_event = component.deprecate().unwrap();
        component.apply(&deprecate_event).unwrap();
        assert!(component.is_deprecated());

        let err = component.deprecate().unwrap_err();
        assert_eq!(err, ComponentError::AlreadyDeprecated(id));
    }

    #[test]
    fn rehydrate_should_rebuild_complete_component_state() {
        let id = dummy_id();
        let sbom = dummy_sbom();
        let ctx = dummy_context();

        let events = vec![
            ComponentEvent::ComponentRegistered {
                component_id: id.clone(),
            },
            ComponentEvent::SbomAssigned {
                component_id: id.clone(),
                sbom: sbom.clone(),
            },
            ComponentEvent::ExecutionContextAssigned {
                component_id: id.clone(),
                context: ctx.clone(),
            },
            ComponentEvent::ComponentDeprecated {
                component_id: id.clone(),
            },
        ];

        let component = Component::rehydrate(&events).unwrap();

        assert_eq!(component.id(), &id);
        assert_eq!(component.sbom(), Some(&sbom));
        assert_eq!(component.context(), Some(&ctx));
        assert!(component.is_deprecated());
    }
}
