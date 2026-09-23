use std::error::Error;

use super::{CloudEvent, CloudEventSource, CloudEventTypePrefix};

/// Defines conversion of a message to and from CloudEvents.
pub trait PublishableMessage: Send + Sync {
    type Error: Error + Send + Sync + 'static;

    /// Encodes the message without consuming it, preserving identity across retries.
    fn try_to_cloud_event(
        &self,
        source: &CloudEventSource,
        type_prefix: Option<&CloudEventTypePrefix>,
    ) -> Result<CloudEvent, Self::Error>;

    /// Reconstructs the message after validating its CloudEvent representation.
    fn try_from_cloud_event(
        event: &CloudEvent,
        type_prefix: Option<&CloudEventTypePrefix>,
    ) -> Result<Self, Self::Error>
    where
        Self: Sized;
}
