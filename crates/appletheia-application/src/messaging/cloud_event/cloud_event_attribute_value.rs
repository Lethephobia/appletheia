use super::{
    CloudEventAttributeString, CloudEventAttributeUri, CloudEventAttributeUriReference,
    CloudEventTime,
};

/// A typed context value, independent of any wire format.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CloudEventAttributeValue {
    Boolean(bool),
    Integer(i32),
    String(CloudEventAttributeString),
    Binary(Vec<u8>),
    Uri(CloudEventAttributeUri),
    UriReference(CloudEventAttributeUriReference),
    Timestamp(CloudEventTime),
}
