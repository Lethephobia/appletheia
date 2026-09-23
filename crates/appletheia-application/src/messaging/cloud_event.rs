pub mod cloud_event_attribute_string;
pub mod cloud_event_attribute_string_error;
pub mod cloud_event_attribute_uri;
pub mod cloud_event_attribute_uri_error;
pub mod cloud_event_attribute_uri_reference;
pub mod cloud_event_attribute_uri_reference_error;
pub mod cloud_event_attribute_value;
pub mod cloud_event_consumer;
pub mod cloud_event_consumer_error;
pub mod cloud_event_data;
pub mod cloud_event_data_content_type;
pub mod cloud_event_data_content_type_error;
pub mod cloud_event_data_schema;
pub mod cloud_event_data_schema_error;
pub mod cloud_event_delivery;
pub mod cloud_event_delivery_error;
pub mod cloud_event_error;
pub mod cloud_event_extension_name;
pub mod cloud_event_extension_name_error;
pub mod cloud_event_extensions;
pub mod cloud_event_extensions_error;
pub mod cloud_event_id;
pub mod cloud_event_id_error;
pub mod cloud_event_partition_key;
pub mod cloud_event_partition_key_error;
pub mod cloud_event_publisher;
pub mod cloud_event_publisher_error;
pub mod cloud_event_selector;
pub mod cloud_event_source;
pub mod cloud_event_source_error;
pub mod cloud_event_spec_version;
pub mod cloud_event_spec_version_error;
pub mod cloud_event_subject;
pub mod cloud_event_subject_error;
pub mod cloud_event_subscriber;
pub mod cloud_event_subscriber_error;
pub mod cloud_event_time;
pub mod cloud_event_time_error;
pub mod cloud_event_type;
pub mod cloud_event_type_error;
pub mod cloud_event_type_prefix;
pub mod cloud_event_type_prefix_error;

pub use cloud_event_attribute_string::*;
pub use cloud_event_attribute_string_error::*;
pub use cloud_event_attribute_uri::*;
pub use cloud_event_attribute_uri_error::*;
pub use cloud_event_attribute_uri_reference::*;
pub use cloud_event_attribute_uri_reference_error::*;
pub use cloud_event_attribute_value::*;
pub use cloud_event_consumer::*;
pub use cloud_event_consumer_error::*;
pub use cloud_event_data::*;
pub use cloud_event_data_content_type::*;
pub use cloud_event_data_content_type_error::*;
pub use cloud_event_data_schema::*;
pub use cloud_event_data_schema_error::*;
pub use cloud_event_delivery::*;
pub use cloud_event_delivery_error::*;
pub use cloud_event_error::*;
pub use cloud_event_extension_name::*;
pub use cloud_event_extension_name_error::*;
pub use cloud_event_extensions::*;
pub use cloud_event_extensions_error::*;
pub use cloud_event_id::*;
pub use cloud_event_id_error::*;
pub use cloud_event_partition_key::*;
pub use cloud_event_partition_key_error::*;
pub use cloud_event_publisher::*;
pub use cloud_event_publisher_error::*;
pub use cloud_event_selector::*;
pub use cloud_event_source::*;
pub use cloud_event_source_error::*;
pub use cloud_event_spec_version::*;
pub use cloud_event_spec_version_error::*;
pub use cloud_event_subject::*;
pub use cloud_event_subject_error::*;
pub use cloud_event_subscriber::*;
pub use cloud_event_subscriber_error::*;
pub use cloud_event_time::*;
pub use cloud_event_time_error::*;
pub use cloud_event_type::*;
pub use cloud_event_type_error::*;
pub use cloud_event_type_prefix::*;
pub use cloud_event_type_prefix_error::*;

/// A validated, transport-independent CloudEvent. Wire codecs are separate.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CloudEvent {
    spec_version: CloudEventSpecVersion,
    id: CloudEventId,
    source: CloudEventSource,
    event_type: CloudEventType,
    subject: Option<CloudEventSubject>,
    time: Option<CloudEventTime>,
    data_content_type: Option<CloudEventDataContentType>,
    data_schema: Option<CloudEventDataSchema>,
    extensions: CloudEventExtensions,
    data: Option<CloudEventData>,
}

impl CloudEvent {
    pub const SPEC_VERSION: CloudEventSpecVersion = CloudEventSpecVersion::V1_0;

    pub fn new(id: CloudEventId, source: CloudEventSource, event_type: CloudEventType) -> Self {
        Self {
            spec_version: Self::SPEC_VERSION,
            id,
            source,
            event_type,
            subject: None,
            time: None,
            data_content_type: None,
            data_schema: None,
            extensions: CloudEventExtensions::new(),
            data: None,
        }
    }

    pub fn with_subject(mut self, subject: CloudEventSubject) -> Self {
        self.subject = Some(subject);
        self
    }

    pub fn with_time(mut self, time: CloudEventTime) -> Self {
        self.time = Some(time);
        self
    }

    pub fn with_data_schema(mut self, data_schema: CloudEventDataSchema) -> Self {
        self.data_schema = Some(data_schema);
        self
    }

    /// Changes payload and content type together, after validating their compatibility.
    /// Absent content type implies JSON for Json/Text, but leaves Binary uninterpreted.
    pub fn replace_data(
        &mut self,
        data: Option<CloudEventData>,
        content_type: Option<CloudEventDataContentType>,
    ) -> Result<(), CloudEventError> {
        let is_json = content_type
            .as_ref()
            .is_none_or(CloudEventDataContentType::is_json);
        if matches!(&data, Some(CloudEventData::Json(_)) if !is_json)
            || matches!(&data, Some(CloudEventData::Text(_)) if is_json)
        {
            return Err(CloudEventError::IncompatibleDataContentType);
        }
        self.data = data;
        self.data_content_type = content_type;
        Ok(())
    }

    pub fn insert_extension(
        &mut self,
        name: CloudEventExtensionName,
        value: CloudEventAttributeValue,
    ) -> Result<Option<CloudEventAttributeValue>, CloudEventError> {
        Ok(self.extensions.insert(name, value)?)
    }

    pub fn remove_extension(
        &mut self,
        name: &CloudEventExtensionName,
    ) -> Option<CloudEventAttributeValue> {
        self.extensions.remove(name)
    }

    pub fn with_partition_key(mut self, key: CloudEventPartitionKey) -> Self {
        self.extensions.insert_partition_key(key);
        self
    }

    pub fn partition_key(&self) -> Option<CloudEventPartitionKey> {
        self.extensions.partition_key()
    }

    pub fn spec_version(&self) -> CloudEventSpecVersion {
        self.spec_version
    }

    pub fn id(&self) -> &CloudEventId {
        &self.id
    }

    pub fn source(&self) -> &CloudEventSource {
        &self.source
    }

    pub fn event_type(&self) -> &CloudEventType {
        &self.event_type
    }

    pub fn subject(&self) -> Option<&CloudEventSubject> {
        self.subject.as_ref()
    }

    pub fn time(&self) -> Option<CloudEventTime> {
        self.time
    }

    pub fn data_content_type(&self) -> Option<&CloudEventDataContentType> {
        self.data_content_type.as_ref()
    }

    pub fn data_schema(&self) -> Option<&CloudEventDataSchema> {
        self.data_schema.as_ref()
    }

    pub fn extensions(&self) -> &CloudEventExtensions {
        &self.extensions
    }

    pub fn data(&self) -> Option<&CloudEventData> {
        self.data.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use chrono::{TimeZone, Utc};
    use serde_json::Value;

    use super::*;
    use crate::messaging::{
        CloudEventAttributeString, CloudEventAttributeUri, CloudEventAttributeUriReference,
    };

    fn event() -> CloudEvent {
        CloudEvent::new(
            "event-1".parse().unwrap(),
            "/accounts".parse().unwrap(),
            "account.updated".parse().unwrap(),
        )
    }

    #[test]
    fn validates_context_strings_without_restricting_payload_strings() {
        for value in [
            "\n",
            "\u{7f}",
            "\u{85}",
            "\u{fdd0}",
            "\u{1ffff}",
            "\u{10ffff}",
        ] {
            assert!(CloudEventAttributeString::new(value.to_owned()).is_err());
            assert!(CloudEventId::new(value.to_owned()).is_err());
        }
        assert!("".parse::<CloudEventAttributeString>().is_ok());
        assert!("".parse::<CloudEventId>().is_err());
        assert!(" ".parse::<CloudEventId>().is_ok());
        assert!("日本語😀".parse::<CloudEventSubject>().is_ok());
        let mut message = event();
        message
            .replace_data(Some(CloudEventData::Json(Value::String("\n".into()))), None)
            .unwrap();
    }

    #[test]
    fn distinguishes_uri_reference_from_absolute_uri() {
        for value in ["/accounts/1", "../accounts", "?page=1", "#fragment"] {
            assert!(value.parse::<CloudEventSource>().is_ok());
            assert!(value.parse::<CloudEventDataSchema>().is_err());
        }
        for value in ["urn:example:schema", "https://example.com/schema"] {
            assert!(value.parse::<CloudEventDataSchema>().is_ok());
            assert!(value.parse::<CloudEventAttributeUri>().is_ok());
        }
        for value in [
            "https://example.com/a b",
            "%zz",
            "https://[invalid",
            "日本語",
        ] {
            assert!(value.parse::<CloudEventSource>().is_err(), "{value}");
        }
        assert!("".parse::<CloudEventSource>().is_err());
        assert!("".parse::<CloudEventAttributeUriReference>().is_ok());
        assert!(
            "https://example.com/schema#part"
                .parse::<CloudEventDataSchema>()
                .is_err()
        );
    }

    #[test]
    fn rejects_reserved_extension_names_but_not_long_names() {
        for value in [
            "",
            "CorrelationId",
            "correlation_id",
            "é",
            "data",
            "id",
            "specversion",
            "datacontenttype",
            "data_base64",
        ] {
            assert!(value.parse::<CloudEventExtensionName>().is_err(), "{value}");
        }
        assert!(
            "anextensionnamelongerthantwentycharacters"
                .parse::<CloudEventExtensionName>()
                .is_ok()
        );
    }

    #[test]
    fn checks_partition_key_through_both_entry_points() {
        let mut message = event().with_partition_key("account:1".parse().unwrap());
        let name = "partitionkey".parse::<CloudEventExtensionName>().unwrap();
        for value in [
            CloudEventAttributeValue::Integer(1),
            CloudEventAttributeValue::String("".parse().unwrap()),
        ] {
            assert!(message.insert_extension(name.clone(), value).is_err());
            assert_eq!(message.partition_key().unwrap().as_str(), "account:1");
        }
        message
            .insert_extension(
                name.clone(),
                CloudEventAttributeValue::String("account:2".parse().unwrap()),
            )
            .unwrap();
        assert_eq!(message.partition_key().unwrap().as_str(), "account:2");
        assert_eq!(message.extensions().len(), 1);
        message.remove_extension(&name);
        assert!(message.partition_key().is_none());
    }

    #[test]
    fn failed_payload_change_preserves_both_payload_and_content_type() {
        let mut message = event();
        message
            .replace_data(
                Some(CloudEventData::Text("<xml/>".into())),
                Some("application/xml".parse().unwrap()),
            )
            .unwrap();
        let original = message.clone();
        assert!(
            message
                .replace_data(
                    Some(CloudEventData::Json(Value::Null)),
                    Some("application/xml".parse().unwrap())
                )
                .is_err()
        );
        assert_eq!(message, original);
        assert!(
            message
                .replace_data(Some(CloudEventData::Text("{}".into())), None)
                .is_err()
        );
        assert_eq!(message, original);
    }

    #[test]
    fn distinguishes_absent_null_and_binary_payloads() {
        let mut message = event();
        assert!(message.data().is_none());
        message
            .replace_data(Some(CloudEventData::Json(Value::Null)), None)
            .unwrap();
        assert_eq!(message.data(), Some(&CloudEventData::Json(Value::Null)));
        message
            .replace_data(Some(CloudEventData::Binary(vec![0, 255])), None)
            .unwrap();
        assert!(message.data_content_type().is_none());
        message
            .replace_data(None, Some("application/json".parse().unwrap()))
            .unwrap();
        assert!(message.data().is_none());
        assert!(message.data_content_type().is_some());
    }

    #[test]
    fn recognizes_json_media_types_and_parameters() {
        for value in [
            "application/json",
            "Application/JSON; charset=utf-8",
            "application/problem+json",
            "Application/Problem+JSON; charset=utf-8",
        ] {
            let content_type = value.parse::<CloudEventDataContentType>().unwrap();
            assert!(content_type.is_json());
            let mut message = event();
            message
                .replace_data(
                    Some(CloudEventData::Json(Value::Bool(true))),
                    Some(content_type),
                )
                .unwrap();
        }
        for value in ["", "text/plain\r\nx: y", "text/\u{7f}plain"] {
            assert!(
                value.parse::<CloudEventDataContentType>().is_err(),
                "{value}"
            );
        }
        // Match the existing content-type VOs: do not enforce media-type grammar.
        for value in ["custom", "application/", "*/*"] {
            let content_type = value.parse::<CloudEventDataContentType>().unwrap();
            assert_eq!(content_type.as_str(), value);
            assert!(!content_type.is_json());
        }
        assert!(
            !"application/json-seq"
                .parse::<CloudEventDataContentType>()
                .unwrap()
                .is_json()
        );
    }

    #[test]
    fn serde_cannot_bypass_value_validation() {
        assert!(serde_json::from_str::<CloudEventId>(r#""""#).is_err());
        assert!(serde_json::from_str::<CloudEventSource>(r#""bad uri""#).is_err());
        assert!(serde_json::from_str::<CloudEventExtensionName>(r#""data""#).is_err());
        assert!(serde_json::from_str::<CloudEventSpecVersion>(r#""1.0.2""#).is_err());
        let id: CloudEventId = "stable-id".parse().unwrap();
        assert_eq!(
            serde_json::from_str::<CloudEventId>(&serde_json::to_string(&id).unwrap()).unwrap(),
            id
        );
    }

    #[test]
    fn normalizes_timestamp_offsets_and_rejects_non_rfc3339_values() {
        let timestamp: CloudEventTime = "2026-09-17T09:00:00+09:00".parse().unwrap();
        assert_eq!(timestamp.to_string(), "2026-09-17T00:00:00+00:00");
        assert!("2026-09-17".parse::<CloudEventTime>().is_err());
        assert!(CloudEventTime::new(Utc.with_ymd_and_hms(10000, 1, 1, 0, 0, 0).unwrap()).is_err());
    }
}
