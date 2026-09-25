pub mod cloud_events_pubsub_command_codec;
pub mod cloud_events_pubsub_command_codec_error;
pub mod cloud_events_pubsub_command_failure_codec;
pub mod cloud_events_pubsub_command_failure_codec_error;
pub mod cloud_events_pubsub_event_codec;
pub mod cloud_events_pubsub_event_codec_error;
pub mod cloud_events_pubsub_read_model_invalidation_codec;
pub mod cloud_events_pubsub_read_model_invalidation_codec_error;

pub use cloud_events_pubsub_command_codec::*;
pub use cloud_events_pubsub_command_codec_error::*;
pub use cloud_events_pubsub_command_failure_codec::*;
pub use cloud_events_pubsub_command_failure_codec_error::*;
pub use cloud_events_pubsub_event_codec::*;
pub use cloud_events_pubsub_event_codec_error::*;
pub use cloud_events_pubsub_read_model_invalidation_codec::*;
pub use cloud_events_pubsub_read_model_invalidation_codec_error::*;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cloud_events::CloudEventSource;
    use crate::google_cloud::pubsub::messaging::{PubsubMessageCodec, PubsubMessageCodecError};
    use appletheia_application::{
        CommandEnvelope, CommandFailureEnvelope, CommandName, CommandSelector, EventEnvelope,
        EventSelector, ProjectorName, ReadModelInvalidationEnvelope, SagaName,
    };
    use appletheia_domain::{AggregateType, EventName};
    use serde_json::json;
    use uuid::Uuid;

    #[test]
    fn binary_round_trip_preserves_all_envelope_mappings_and_selector_attributes() {
        let id = Uuid::now_v7();
        let context = json!({"correlation_id": id, "message_id": id, "actor": {"type": "system"}});
        let origin = json!({"saga_name": "transfer", "saga_instance_id": id, "step": "debit"});
        let source: CloudEventSource = "urn:banking".parse().unwrap();
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
        let command_codec = CloudEventsPubsubCommandCodec::new(
            source.clone(),
            Some("com.example".parse().unwrap()),
        );
        let command_wire = command_codec.encode(&command).unwrap();
        assert_eq!(command_codec.decode(&command_wire).unwrap(), command);
        assert_eq!(command_wire.attributes["ce-source"], "urn:banking");
        assert_eq!(
            command_wire.attributes["ce-datacontenttype"],
            "application/json"
        );
        assert_eq!(
            command_wire.attributes["ce-partitionkey"],
            command_wire.ordering_key
        );
        let failure_codec = CloudEventsPubsubCommandFailureCodec::new(
            source.clone(),
            Some("com.example".parse().unwrap()),
        );
        let failure_wire = failure_codec.encode(&failure).unwrap();
        assert_eq!(failure_codec.decode(&failure_wire).unwrap(), failure);
        assert_eq!(failure_wire.attributes["ce-source"], "urn:banking");
        assert_eq!(
            failure_wire.attributes["ce-datacontenttype"],
            "application/json"
        );
        assert_eq!(
            failure_wire.attributes["ce-partitionkey"],
            failure_wire.ordering_key
        );
        let event_codec =
            CloudEventsPubsubEventCodec::new(source.clone(), Some("com.example".parse().unwrap()));
        let event_wire = event_codec.encode(&event).unwrap();
        assert_eq!(event_codec.decode(&event_wire).unwrap(), event);
        assert_eq!(event_wire.attributes["ce-source"], "urn:banking");
        assert_eq!(
            event_wire.attributes["ce-datacontenttype"],
            "application/json"
        );
        assert_eq!(
            event_wire.attributes["ce-partitionkey"],
            event_wire.ordering_key
        );
        let invalidation_codec = CloudEventsPubsubReadModelInvalidationCodec::new(
            source.clone(),
            Some("com.example".parse().unwrap()),
        );
        let invalidation_wire = invalidation_codec.encode(&invalidation).unwrap();
        assert_eq!(
            invalidation_codec.decode(&invalidation_wire).unwrap(),
            invalidation
        );
        assert_eq!(invalidation_wire.attributes["ce-source"], "urn:banking");
        assert_eq!(
            invalidation_wire.attributes["ce-datacontenttype"],
            "application/json"
        );
        assert_eq!(
            invalidation_wire.attributes["ce-partitionkey"],
            invalidation_wire.ordering_key
        );
        assert_eq!(
            serde_json::from_slice::<serde_json::Value>(&event_wire.data).unwrap(),
            *event.payload.value()
        );
        assert_eq!(
            event_codec
                .encode_selector(&EventSelector::from_parts(
                    AggregateType::new("account"),
                    EventName::new("debited")
                ))
                .unwrap(),
            format!(
                "attributes.\"ce-type\" = \"{}\"",
                event_wire.attributes["ce-type"]
            )
        );
        assert_eq!(
            command_codec
                .encode_selector(&CommandSelector::new(CommandName::new("debit")))
                .unwrap(),
            format!(
                "attributes.\"ce-type\" = \"{}\"",
                command_wire.attributes["ce-type"]
            )
        );
        assert_eq!(
            failure_codec
                .encode_selector(&SagaName::new("transfer"))
                .unwrap(),
            format!(
                "attributes.\"ce-saganame\" = \"{}\"",
                failure_wire.attributes["ce-saganame"]
            )
        );
        assert_eq!(
            invalidation_codec
                .encode_selector(&ProjectorName::new("account"))
                .unwrap(),
            format!(
                "attributes.\"ce-sourceprojectorname\" = \"{}\"",
                invalidation_wire.attributes["ce-sourceprojectorname"]
            )
        );
        let mut missing_id = event_wire.clone();
        missing_id.attributes.remove("ce-id");
        let PubsubMessageCodecError::Decode(decode_source) =
            event_codec.decode(&missing_id).unwrap_err()
        else {
            panic!("expected a decode error");
        };
        assert!(matches!(
            decode_source.downcast_ref::<CloudEventsPubsubEventCodecError>(),
            Some(CloudEventsPubsubEventCodecError::MissingAttribute("ce-id"))
        ));
        for name in [
            "ce-specversion",
            "ce-id",
            "ce-source",
            "ce-type",
            "ce-context",
            "ce-partitionkey",
            "ce-datacontenttype",
        ] {
            let mut invalid = event_wire.clone();
            invalid.attributes.remove(name);
            assert!(event_codec.decode(&invalid).is_err(), "{name}");
        }
        for (name, value) in [
            ("ce-specversion", "2.0"),
            ("ce-source", "%"),
            ("ce-subject", "other/id"),
            ("ce-eventsequence", "-1"),
            ("ce-partitionkey", "wrong"),
            ("ce-datacontenttype", "text/plain"),
            ("ce-time", "invalid"),
        ] {
            let mut invalid = event_wire.clone();
            invalid.attributes.insert(name.to_owned(), value.to_owned());
            assert!(event_codec.decode(&invalid).is_err(), "{name}");
        }
        let mut extended_event = event_wire.clone();
        extended_event
            .attributes
            .insert("ce-extra".to_owned(), "value".to_owned());
        assert_eq!(event_codec.decode(&extended_event).unwrap(), event);
        let escaped_prefix_codec = CloudEventsPubsubEventCodec::new(
            source.clone(),
            Some(r#"com.quoted"\value"#.parse().unwrap()),
        );
        assert_eq!(
            escaped_prefix_codec
                .encode_selector(&EventSelector::from_parts(
                    AggregateType::new("account"),
                    EventName::new("debited"),
                ))
                .unwrap(),
            r#"attributes."ce-type" = "com.quoted\"\\value.account.debited""#,
        );
        let wrong_prefix =
            CloudEventsPubsubEventCodec::new(source.clone(), Some("other".parse().unwrap()));
        assert!(wrong_prefix.decode(&event_wire).is_err());
        let mut invalid_failure = failure_wire.clone();
        invalid_failure
            .attributes
            .insert("ce-commandmessageid".to_owned(), Uuid::now_v7().to_string());
        assert!(failure_codec.decode(&invalid_failure).is_err());
        let mut invalid_command = command_wire.clone();
        invalid_command
            .attributes
            .insert("ce-saganame".to_owned(), "other".to_owned());
        assert!(command_codec.decode(&invalid_command).is_err());
        let mut invalid_invalidation = invalidation_wire.clone();
        invalid_invalidation
            .attributes
            .insert("ce-sourceeventid".to_owned(), Uuid::now_v7().to_string());
        assert!(invalidation_codec.decode(&invalid_invalidation).is_err());
    }
}
