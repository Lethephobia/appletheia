use appletheia_application::{
    CloudEventAttributeStringError, CloudEventDataContentTypeError, CloudEventDataSchemaError,
    CloudEventError, CloudEventExtensionNameError, CloudEventIdError, CloudEventSourceError,
    CloudEventSpecVersionError, CloudEventSubjectError, CloudEventTimeError, CloudEventTypeError,
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum PubsubCloudEventCodecError {
    #[error("missing CloudEvents attribute: {0} (only binary encoding is supported)")]
    MissingAttribute(&'static str),
    #[error(transparent)]
    SpecVersion(#[from] CloudEventSpecVersionError),
    #[error(transparent)]
    Id(#[from] CloudEventIdError),
    #[error(transparent)]
    Source(#[from] CloudEventSourceError),
    #[error(transparent)]
    Type(#[from] CloudEventTypeError),
    #[error(transparent)]
    Subject(#[from] CloudEventSubjectError),
    #[error(transparent)]
    Time(#[from] CloudEventTimeError),
    #[error(transparent)]
    DataSchema(#[from] CloudEventDataSchemaError),
    #[error(transparent)]
    DataContentType(#[from] CloudEventDataContentTypeError),
    #[error(transparent)]
    ExtensionName(#[from] CloudEventExtensionNameError),
    #[error(transparent)]
    AttributeString(#[from] CloudEventAttributeStringError),
    #[error(transparent)]
    CloudEvent(#[from] CloudEventError),
    #[error(transparent)]
    Json(#[from] serde_json::Error),
}
