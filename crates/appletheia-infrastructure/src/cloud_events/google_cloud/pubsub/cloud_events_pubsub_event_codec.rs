use appletheia_domain::{AggregateVersion, EventId};
use chrono::{DateTime, Utc};
use std::collections::HashMap;
use uuid::Uuid;

use crate::cloud_events::{CloudEventSource, CloudEventTypePrefix};
use crate::google_cloud::pubsub::messaging::{PubsubMessageCodec, PubsubMessageCodecError};
use appletheia_application::aggregate::{AggregateIdValue, AggregateTypeOwned};
use appletheia_application::event::{
    EventEnvelope, EventNameOwned, EventSequence, SerializedEventPayload,
};
use appletheia_application::request_context::{CausationId, CorrelationId, MessageId};
use google_cloud_pubsub::model::Message;

use super::CloudEventsPubsubEventCodecError;

use appletheia_application::EventSelector;
use appletheia_application::PublishableMessage;

#[derive(Clone, Debug)]
pub struct CloudEventsPubsubEventCodec {
    source: CloudEventSource,
    cloud_event_type_prefix: Option<CloudEventTypePrefix>,
}

impl CloudEventsPubsubEventCodec {
    pub fn new(
        source: CloudEventSource,
        cloud_event_type_prefix: Option<CloudEventTypePrefix>,
    ) -> Self {
        Self {
            source,
            cloud_event_type_prefix,
        }
    }

    pub fn encode_id(event_id: EventId) -> String {
        event_id.to_string()
    }

    pub fn decode_id(event_id: &str) -> Result<EventId, CloudEventsPubsubEventCodecError> {
        Ok(EventId::try_from(event_id.parse::<Uuid>()?)?)
    }

    pub fn encode_type(
        cloud_event_type_prefix: Option<&CloudEventTypePrefix>,
        aggregate_type: &AggregateTypeOwned,
        event_name: &EventNameOwned,
    ) -> String {
        let name = &format!("{aggregate_type}.{event_name}");

        match cloud_event_type_prefix {
            Some(prefix) => format!("{prefix}.{name}"),
            None => name.to_owned(),
        }
    }

    pub fn decode_type(
        event_type: &str,
        cloud_event_type_prefix: Option<&CloudEventTypePrefix>,
    ) -> Result<(AggregateTypeOwned, EventNameOwned), CloudEventsPubsubEventCodecError> {
        let name = match cloud_event_type_prefix {
            Some(prefix) => event_type
                .strip_prefix(&format!("{prefix}."))
                .ok_or(CloudEventsPubsubEventCodecError::TypePrefixMismatch)?,
            None => event_type,
        };
        let (aggregate_type, event_name) = name
            .split_once('.')
            .ok_or(CloudEventsPubsubEventCodecError::InvalidMetadata("type"))?;
        Ok((aggregate_type.parse()?, event_name.parse()?))
    }
}

impl PubsubMessageCodec for CloudEventsPubsubEventCodec {
    type Message = EventEnvelope;
    type Selector = EventSelector;

    fn encode(&self, envelope: &Self::Message) -> Result<Message, PubsubMessageCodecError> {
        (|| -> Result<Message, CloudEventsPubsubEventCodecError> {
            let ordering_key = envelope.ordering_key().to_string();
            let attributes: HashMap<String, String> = [
                ("ce-specversion", "1.0".to_owned()),
                ("ce-source", self.source.to_string()),
                ("ce-datacontenttype", "application/json".to_owned()),
                ("ce-partitionkey", ordering_key.clone()),
                ("ce-id", Self::encode_id(envelope.event_id)),
                (
                    "ce-type",
                    Self::encode_type(
                        self.cloud_event_type_prefix.as_ref(),
                        &envelope.aggregate_type,
                        &envelope.event_name,
                    ),
                ),
                ("ce-correlationid", envelope.correlation_id.to_string()),
                ("ce-causationid", envelope.causation_id.to_string()),
                (
                    "ce-aggregateversion",
                    envelope.aggregate_version.to_string(),
                ),
                ("ce-eventsequence", envelope.event_sequence.to_string()),
                ("ce-context", serde_json::to_string(&envelope.context)?),
                (
                    "ce-subject",
                    format!("{}/{}", envelope.aggregate_type, envelope.aggregate_id),
                ),
                ("ce-time", {
                    let timestamp: DateTime<Utc> = envelope.occurred_at.into();
                    let encoded_time = timestamp.to_rfc3339();
                    DateTime::parse_from_rfc3339(&encoded_time)?;
                    encoded_time
                }),
            ]
            .into_iter()
            .map(|(name, value)| (name.to_owned(), value))
            .collect();
            Ok(Message::new()
                .set_attributes(attributes)
                .set_data(serde_json::to_vec(envelope.payload.value())?)
                .set_ordering_key(ordering_key))
        })()
        .map_err(|source| PubsubMessageCodecError::Encode(Box::new(source)))
    }

    fn decode(&self, message: &Message) -> Result<Self::Message, PubsubMessageCodecError> {
        (|| -> Result<Self::Message, CloudEventsPubsubEventCodecError> {
            if message
                .attributes
                .get("ce-specversion")
                .map(String::as_str)
                .ok_or(CloudEventsPubsubEventCodecError::MissingAttribute(
                    "ce-specversion",
                ))?
                != "1.0"
            {
                return Err(CloudEventsPubsubEventCodecError::UnsupportedSpecVersion);
            }

            message
                .attributes
                .get("ce-source")
                .map(String::as_str)
                .ok_or(CloudEventsPubsubEventCodecError::MissingAttribute(
                    "ce-source",
                ))?
                .parse::<CloudEventSource>()?;
            let content_type = message
                .attributes
                .get("ce-datacontenttype")
                .map(String::as_str)
                .ok_or(CloudEventsPubsubEventCodecError::MissingAttribute(
                    "ce-datacontenttype",
                ))?;
            let media_type = content_type.split(';').next().unwrap_or_default().trim();
            let is_json = media_type.split_once('/').is_some_and(|(_, subtype)| {
                subtype.eq_ignore_ascii_case("json")
                    || subtype
                        .rsplit_once('+')
                        .is_some_and(|(_, suffix)| suffix.eq_ignore_ascii_case("json"))
            });
            if !is_json {
                return Err(CloudEventsPubsubEventCodecError::InvalidContentType);
            }

            let data: serde_json::Value = serde_json::from_slice(&message.data)?;

            let (aggregate_type, event_name) = Self::decode_type(
                message
                    .attributes
                    .get("ce-type")
                    .map(String::as_str)
                    .ok_or(CloudEventsPubsubEventCodecError::MissingAttribute(
                        "ce-type",
                    ))?,
                self.cloud_event_type_prefix.as_ref(),
            )?;
            let subject = message
                .attributes
                .get("ce-subject")
                .map(String::as_str)
                .ok_or(CloudEventsPubsubEventCodecError::MissingAttribute(
                    "ce-subject",
                ))?;
            let (subject_aggregate_type, aggregate_id) = subject
                .rsplit_once('/')
                .ok_or(CloudEventsPubsubEventCodecError::InvalidMetadata("subject"))?;
            if aggregate_type.value() != subject_aggregate_type {
                return Err(CloudEventsPubsubEventCodecError::InvalidMetadata(
                    "type/subject",
                ));
            }
            let envelope = EventEnvelope {
                event_id: Self::decode_id(
                    message
                        .attributes
                        .get("ce-id")
                        .map(String::as_str)
                        .ok_or(CloudEventsPubsubEventCodecError::MissingAttribute("ce-id"))?,
                )?,
                event_sequence: EventSequence::try_from(
                    message
                        .attributes
                        .get("ce-eventsequence")
                        .map(String::as_str)
                        .ok_or(CloudEventsPubsubEventCodecError::MissingAttribute(
                            "ce-eventsequence",
                        ))?
                        .parse::<i64>()?,
                )?,
                aggregate_type,
                aggregate_id: AggregateIdValue::from(aggregate_id.parse::<Uuid>()?),
                aggregate_version: AggregateVersion::try_from(
                    message
                        .attributes
                        .get("ce-aggregateversion")
                        .map(String::as_str)
                        .ok_or(CloudEventsPubsubEventCodecError::MissingAttribute(
                            "ce-aggregateversion",
                        ))?
                        .parse::<i64>()?,
                )?,
                event_name,
                payload: SerializedEventPayload::try_from(data)?,
                occurred_at: DateTime::parse_from_rfc3339(
                    message
                        .attributes
                        .get("ce-time")
                        .map(String::as_str)
                        .ok_or(CloudEventsPubsubEventCodecError::MissingAttribute(
                            "ce-time",
                        ))?,
                )?
                .with_timezone(&Utc)
                .into(),
                context: serde_json::from_str(
                    message
                        .attributes
                        .get("ce-context")
                        .map(String::as_str)
                        .ok_or(CloudEventsPubsubEventCodecError::MissingAttribute(
                            "ce-context",
                        ))?,
                )?,
                correlation_id: CorrelationId::from(
                    message
                        .attributes
                        .get("ce-correlationid")
                        .map(String::as_str)
                        .ok_or(CloudEventsPubsubEventCodecError::MissingAttribute(
                            "ce-correlationid",
                        ))?
                        .parse::<Uuid>()?,
                ),
                causation_id: CausationId::from(MessageId::from(
                    message
                        .attributes
                        .get("ce-causationid")
                        .map(String::as_str)
                        .ok_or(CloudEventsPubsubEventCodecError::MissingAttribute(
                            "ce-causationid",
                        ))?
                        .parse::<Uuid>()?,
                )),
            };
            if message
                .attributes
                .get("ce-partitionkey")
                .map(String::as_str)
                != Some((format!("{}:{}", envelope.aggregate_type, envelope.aggregate_id)).as_str())
            {
                return Err(CloudEventsPubsubEventCodecError::InvalidMetadata(
                    "partitionkey",
                ));
            }
            Ok(envelope)
        })()
        .map_err(|source| PubsubMessageCodecError::Decode(Box::new(source)))
    }

    fn encode_selector(
        &self,
        selector: &Self::Selector,
    ) -> Result<String, PubsubMessageCodecError> {
        (|| -> Result<String, CloudEventsPubsubEventCodecError> {
            let event_type = Self::encode_type(
                self.cloud_event_type_prefix.as_ref(),
                &selector.aggregate_type.into(),
                &selector.event_name.into(),
            );
            Ok(format!(
                "attributes.{} = {}",
                serde_json::to_string("ce-type")?,
                serde_json::to_string(event_type.as_str())?
            ))
        })()
        .map_err(|source| PubsubMessageCodecError::EncodeSelector(Box::new(source)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn id_round_trip_validates_domain_id() {
        let event_id = EventId::new();
        let encoded = CloudEventsPubsubEventCodec::encode_id(event_id);
        assert_eq!(
            CloudEventsPubsubEventCodec::decode_id(&encoded).unwrap(),
            event_id
        );
        for invalid in ["not-a-uuid", "00000000-0000-0000-0000-000000000000"] {
            assert!(CloudEventsPubsubEventCodec::decode_id(invalid).is_err());
        }
    }

    #[test]
    fn type_round_trip_with_and_without_prefix() {
        let aggregate_type = "bank_account".parse().unwrap();
        let event_name = "created_v1".parse().unwrap();
        let prefix = "com.example.events".parse().unwrap();
        for cloud_event_type_prefix in [None, Some(&prefix)] {
            let encoded = CloudEventsPubsubEventCodec::encode_type(
                cloud_event_type_prefix,
                &aggregate_type,
                &event_name,
            );
            assert_eq!(
                CloudEventsPubsubEventCodec::decode_type(&encoded, cloud_event_type_prefix)
                    .unwrap(),
                (aggregate_type.clone(), event_name.clone()),
            );
        }
    }

    #[test]
    fn decode_type_rejects_invalid_components_and_prefix() {
        for invalid in [
            "account",
            ".created",
            "account.",
            "account.created.extra",
            "Account.created",
        ] {
            assert!(CloudEventsPubsubEventCodec::decode_type(invalid, None).is_err());
        }
        assert!(
            CloudEventsPubsubEventCodec::decode_type(
                "com.other.account.created",
                Some(&"com.example".parse().unwrap()),
            )
            .is_err()
        );
    }
}
