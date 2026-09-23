use std::fmt::{self, Display};

use base64::{Engine, engine::general_purpose::STANDARD};

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

impl Display for CloudEventAttributeValue {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Boolean(value) => Display::fmt(value, formatter),
            Self::Integer(value) => Display::fmt(value, formatter),
            Self::String(value) => Display::fmt(value, formatter),
            Self::Binary(value) => formatter.write_str(&STANDARD.encode(value)),
            Self::Uri(value) => Display::fmt(value, formatter),
            Self::UriReference(value) => Display::fmt(value, formatter),
            Self::Timestamp(value) => Display::fmt(value, formatter),
        }
    }
}
