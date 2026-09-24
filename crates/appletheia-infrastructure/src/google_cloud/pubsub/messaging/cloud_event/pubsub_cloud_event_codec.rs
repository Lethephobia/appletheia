use std::collections::HashMap;

use appletheia_application::{
    CloudEvent, CloudEventAttributeString, CloudEventAttributeValue, CloudEventData,
    CloudEventExtensionName, CloudEventSpecVersion,
};
use google_cloud_pubsub::model::Message;

use super::PubsubCloudEventCodecError;

/// Pub/Sub binary encoding using `ce-datacontenttype`, as shown in the Google binding draft examples.
/// <https://github.com/googleapis/google-cloudevents/blob/main/docs/spec/pubsub.md#314-examples>
pub(crate) struct PubsubCloudEventCodec;

impl PubsubCloudEventCodec {
    pub(crate) fn encode(event: &CloudEvent) -> Result<Message, PubsubCloudEventCodecError> {
        let mut attributes = HashMap::from([
            (
                "ce-specversion".to_owned(),
                event.spec_version().to_string(),
            ),
            ("ce-id".to_owned(), event.id().to_string()),
            ("ce-source".to_owned(), event.source().to_string()),
            ("ce-type".to_owned(), event.event_type().to_string()),
        ]);
        if let Some(subject) = event.subject() {
            attributes.insert("ce-subject".to_owned(), subject.to_string());
        }
        if let Some(time) = event.time() {
            attributes.insert("ce-time".to_owned(), time.to_string());
        }
        if let Some(schema) = event.data_schema() {
            attributes.insert("ce-dataschema".to_owned(), schema.to_string());
        }
        if let Some(content_type) = event.data_content_type() {
            attributes.insert("ce-datacontenttype".to_owned(), content_type.to_string());
        }
        for (name, value) in event.extensions().iter() {
            attributes.insert(format!("ce-{name}"), value.to_string());
        }
        let data = match event.data() {
            None => Vec::new(),
            Some(CloudEventData::Binary(bytes)) => bytes.clone(),
            Some(CloudEventData::Text(text)) => text.as_bytes().to_vec(),
            Some(CloudEventData::Json(value)) => serde_json::to_vec(value)?,
        };
        Ok(Message::new()
            .set_attributes(attributes)
            .set_data(data)
            .set_ordering_key(
                event
                    .partition_key()
                    .map(|key| key.to_string())
                    .unwrap_or_default(),
            ))
    }

    /// Binary payloads remain opaque. Extensions have no wire type tags and are
    /// retained as strings. Pub/Sub cannot distinguish absent data from empty bytes.
    pub(crate) fn decode(message: &Message) -> Result<CloudEvent, PubsubCloudEventCodecError> {
        let attributes = &message.attributes;
        match attributes
            .get("ce-specversion")
            .ok_or(PubsubCloudEventCodecError::MissingAttribute(
                "ce-specversion",
            ))?
            .parse()?
        {
            CloudEventSpecVersion::V1_0 => {}
        }
        let mut event = CloudEvent::new(
            attributes
                .get("ce-id")
                .ok_or(PubsubCloudEventCodecError::MissingAttribute("ce-id"))?
                .parse()?,
            attributes
                .get("ce-source")
                .ok_or(PubsubCloudEventCodecError::MissingAttribute("ce-source"))?
                .parse()?,
            attributes
                .get("ce-type")
                .ok_or(PubsubCloudEventCodecError::MissingAttribute("ce-type"))?
                .parse()?,
        );
        if let Some(subject) = attributes.get("ce-subject") {
            event = event.with_subject(subject.parse()?);
        }
        if let Some(time) = attributes.get("ce-time") {
            event = event.with_time(time.parse()?);
        }
        if let Some(schema) = attributes.get("ce-dataschema") {
            event = event.with_data_schema(schema.parse()?);
        }
        let content_type = attributes
            .get("ce-datacontenttype")
            .map(|value| value.parse())
            .transpose()?;
        let data = if message.data.is_empty() {
            None
        } else {
            Some(CloudEventData::Binary(message.data.to_vec()))
        };
        event = event.try_with_data(data, content_type)?;
        for (key, value) in attributes {
            match key.as_str() {
                "ce-specversion" | "ce-id" | "ce-source" | "ce-type" | "ce-subject" | "ce-time"
                | "ce-dataschema" | "ce-datacontenttype" => continue,
                _ => {}
            }
            if let Some(name) = key.strip_prefix("ce-") {
                event.insert_extension(
                    CloudEventExtensionName::new(name.to_owned())?,
                    CloudEventAttributeValue::String(CloudEventAttributeString::new(
                        value.clone(),
                    )?),
                )?;
            }
        }
        Ok(event)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use appletheia_application::CloudEventDataContentType;
    use appletheia_application::{
        CommandCloudEventCodec, CommandFailureCloudEventCodec, EventCloudEventCodec,
        ReadModelInvalidationCloudEventCodec,
    };
    use serde_json::json;

    fn event() -> CloudEvent {
        CloudEvent::new(
            "event-1".parse().unwrap(),
            "/events".parse().unwrap(),
            "example.changed".parse().unwrap(),
        )
    }

    #[test]
    fn encodes_binary_context_and_native_partition_key() {
        let original = event()
            .with_subject("item/1".parse().unwrap())
            .with_time("2026-09-18T12:00:00Z".parse().unwrap())
            .with_data_schema("https://example.com/schema".parse().unwrap())
            .with_partition_key("item-1".parse().unwrap())
            .try_with_data(
                Some(CloudEventData::Json(json!({"value": 1}))),
                Some(CloudEventDataContentType::json()),
            )
            .unwrap();
        let message = PubsubCloudEventCodec::encode(&original).unwrap();
        assert_eq!(message.attributes["ce-specversion"], "1.0");
        assert_eq!(message.attributes["ce-datacontenttype"], "application/json");
        assert!(!message.attributes.contains_key("Content-Type"));
        assert!(!message.attributes.contains_key("content-type"));
        assert_eq!(message.attributes["ce-partitionkey"], "item-1");
        assert_eq!(message.ordering_key, "item-1");
        assert_eq!(&message.data[..], br#"{"value":1}"#);
        let decoded = PubsubCloudEventCodec::decode(&message).unwrap();
        assert_eq!(decoded.id(), original.id());
        assert_eq!(decoded.source(), original.source());
        assert_eq!(decoded.event_type(), original.event_type());
        assert_eq!(decoded.subject(), original.subject());
        assert_eq!(decoded.time(), original.time());
        assert_eq!(decoded.data_schema(), original.data_schema());
        assert_eq!(decoded.data_content_type(), original.data_content_type());
        assert_eq!(decoded.partition_key(), original.partition_key());
        assert_eq!(
            decoded.data(),
            Some(&CloudEventData::Binary(br#"{"value":1}"#.to_vec()))
        );
    }

    #[test]
    fn decodes_binary_message_without_using_broker_identity() {
        let message = Message::new()
            .set_message_id("broker-42")
            .set_ordering_key("broker-order")
            .set_attributes([
                ("ce-specversion", "1.0"),
                ("ce-id", "external-event"),
                ("ce-source", "/external"),
                ("ce-type", "external.changed"),
                ("ce-datacontenttype", "application/octet-stream"),
                ("ce-attempt", "42"),
                ("unrelated", "ignored"),
            ])
            .set_data(vec![0, 255, 128]);
        let decoded = PubsubCloudEventCodec::decode(&message).unwrap();
        assert_eq!(decoded.id().as_str(), "external-event");
        assert_eq!(decoded.partition_key(), None);
        assert_eq!(
            decoded.data(),
            Some(&CloudEventData::Binary(vec![0, 255, 128]))
        );
        assert_eq!(
            decoded.data_content_type().unwrap().as_str(),
            "application/octet-stream"
        );
        assert_eq!(decoded.extensions().len(), 1);
        assert_eq!(
            decoded.extensions().get(&"attempt".parse().unwrap()),
            Some(&CloudEventAttributeValue::String("42".parse().unwrap()))
        );
    }

    #[test]
    fn encodes_typed_extensions_as_canonical_strings() {
        let mut original = event();
        let values = [
            ("enabled", CloudEventAttributeValue::Boolean(true), "true"),
            ("attempt", CloudEventAttributeValue::Integer(-42), "-42"),
            (
                "opaque",
                CloudEventAttributeValue::Binary(vec![255, 0]),
                "/wA=",
            ),
            (
                "text",
                CloudEventAttributeValue::String("false".parse().unwrap()),
                "false",
            ),
            (
                "uri",
                CloudEventAttributeValue::Uri("https://example.com".parse().unwrap()),
                "https://example.com",
            ),
            (
                "reference",
                CloudEventAttributeValue::UriReference("/items/1".parse().unwrap()),
                "/items/1",
            ),
            (
                "timestamp",
                CloudEventAttributeValue::Timestamp("2026-09-18T12:00:00Z".parse().unwrap()),
                "2026-09-18T12:00:00+00:00",
            ),
        ];
        for (name, value, _) in &values {
            original
                .insert_extension(name.parse().unwrap(), value.clone())
                .unwrap();
        }
        let message = PubsubCloudEventCodec::encode(&original).unwrap();
        let decoded = PubsubCloudEventCodec::decode(&message).unwrap();
        for (name, _, encoded) in values {
            assert_eq!(message.attributes[&format!("ce-{name}")], encoded);
            assert_eq!(
                decoded.extensions().get(&name.parse().unwrap()),
                Some(&CloudEventAttributeValue::String(encoded.parse().unwrap()))
            );
        }
    }

    #[test]
    fn preserves_payload_bytes_and_json_null() {
        for (data, content_type, expected) in [
            (
                CloudEventData::Text("こんにちは".to_owned()),
                Some("text/plain".parse().unwrap()),
                "こんにちは".as_bytes().to_vec(),
            ),
            (CloudEventData::Json(json!(null)), None, b"null".to_vec()),
            (CloudEventData::Binary(vec![255, 0]), None, vec![255, 0]),
        ] {
            let original = event().try_with_data(Some(data), content_type).unwrap();
            let message = PubsubCloudEventCodec::encode(&original).unwrap();
            assert_eq!(&message.data[..], expected);
            let decoded = PubsubCloudEventCodec::decode(&message).unwrap();
            assert_eq!(decoded.data(), Some(&CloudEventData::Binary(expected)));
        }
        let empty = PubsubCloudEventCodec::encode(&event()).unwrap();
        assert!(
            PubsubCloudEventCodec::decode(&empty)
                .unwrap()
                .data()
                .is_none()
        );
    }

    #[test]
    fn rejects_missing_invalid_and_unsupported_context() {
        for attribute in ["ce-specversion", "ce-id", "ce-source", "ce-type"] {
            let mut message = PubsubCloudEventCodec::encode(&event()).unwrap();
            message.attributes.remove(attribute);
            assert!(
                matches!(PubsubCloudEventCodec::decode(&message), Err(PubsubCloudEventCodecError::MissingAttribute(name)) if name == attribute)
            );
        }
        for (name, value) in [
            ("ce-specversion", "2.0"),
            ("ce-id", ""),
            ("ce-source", "%"),
            ("ce-type", ""),
            ("ce-time", "invalid"),
            ("ce-dataschema", "/relative"),
            ("ce-partitionkey", ""),
            ("ce-Invalid", "value"),
        ] {
            let mut message = PubsubCloudEventCodec::encode(&event()).unwrap();
            message.attributes.insert(name.to_owned(), value.to_owned());
            assert!(PubsubCloudEventCodec::decode(&message).is_err(), "{name}");
        }
    }

    #[test]
    fn binary_round_trip_preserves_all_envelope_mappings() {
        use appletheia_application::{
            CommandEnvelope, CommandFailureEnvelope, EventEnvelope, ReadModelInvalidationEnvelope,
        };
        use serde_json::json;
        use uuid::Uuid;

        let id = Uuid::now_v7();
        let context = json!({"correlation_id": id, "message_id": id, "actor": {"type": "system"}});
        let origin = json!({"saga_name": "transfer", "saga_instance_id": id, "step": "debit"});
        let source = "urn:banking".parse().unwrap();
        let command: CommandEnvelope = serde_json::from_value(json!({
            "command_name": "debit", "command": {"amount": 10}, "message_id": id,
            "correlation_id": id, "causation_id": id, "saga_origin": origin, "options": {}
        }))
        .unwrap();
        let failure: CommandFailureEnvelope = serde_json::from_value(json!({
            "failure_id": Uuid::now_v7(), "command_message_id": id, "command_name": "debit",
            "origin": origin, "terminal_reason": "non_retryable", "attempt_count": 1,
            "correlation_id": id, "causation_id": id, "failed_at": "2026-09-23T12:00:00Z"
        }))
        .unwrap();
        let event: EventEnvelope = serde_json::from_value(json!({
            "event_id": id, "event_sequence": 9223372036854775807_i64, "aggregate_type": "account", "aggregate_id": id,
            "aggregate_version": 9223372036854775807_i64, "event_name": "debited", "payload": {"type": "debited", "data": {"amount": 10}},
            "occurred_at": "2026-09-23T12:00:00Z", "correlation_id": id, "causation_id": id, "context": context
        })).unwrap();
        let invalidation: ReadModelInvalidationEnvelope = serde_json::from_value(json!({
            "invalidation_id": Uuid::now_v7(), "source_event_id": id, "source_event_sequence": 9223372036854775807_i64,
            "source_projector_name": "account", "source_event_occurred_at": "2026-09-23T12:00:00Z", "correlation_id": id, "causation_id": id,
            "invalidated_partitions": [{"fragment_name": "account", "key": id}]
        })).unwrap();
        let command_wire = PubsubCloudEventCodec::encode(
            &CommandCloudEventCodec::encode(&command, &source, None).unwrap(),
        )
        .unwrap();
        let failure_wire = PubsubCloudEventCodec::encode(
            &CommandFailureCloudEventCodec::encode(&failure, &source, None).unwrap(),
        )
        .unwrap();
        let event_wire = PubsubCloudEventCodec::encode(
            &EventCloudEventCodec::encode(&event, &source, None).unwrap(),
        )
        .unwrap();
        let invalidation_wire = PubsubCloudEventCodec::encode(
            &ReadModelInvalidationCloudEventCodec::encode(&invalidation, &source, None).unwrap(),
        )
        .unwrap();
        assert_eq!(
            CommandCloudEventCodec::decode(
                &PubsubCloudEventCodec::decode(&command_wire).unwrap(),
                None
            )
            .unwrap(),
            command
        );
        assert_eq!(
            CommandFailureCloudEventCodec::decode(
                &PubsubCloudEventCodec::decode(&failure_wire).unwrap(),
                None
            )
            .unwrap(),
            failure
        );
        assert_eq!(
            EventCloudEventCodec::decode(
                &PubsubCloudEventCodec::decode(&event_wire).unwrap(),
                None
            )
            .unwrap(),
            event
        );
        assert_eq!(
            ReadModelInvalidationCloudEventCodec::decode(
                &PubsubCloudEventCodec::decode(&invalidation_wire).unwrap(),
                None
            )
            .unwrap(),
            invalidation
        );
    }
}
