use thiserror::Error;

/// Common errors related to event dispatching and subscription.
#[derive(Error, Debug)]
pub enum EventBusError {
    #[error("Listener registration failed: {0}")]
    RegistrationError(String),

    #[error("Event dispatch failed: {0}")]
    DispatchError(String),
}
