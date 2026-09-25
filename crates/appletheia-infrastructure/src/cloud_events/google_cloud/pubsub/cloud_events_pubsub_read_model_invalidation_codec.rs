use appletheia_domain::EventId;
use chrono::{DateTime, Utc};
use std::collections::HashMap;
use uuid::Uuid;

use crate::cloud_events::{CloudEventSource, CloudEventTypePrefix};
use crate::google_cloud::pubsub::messaging::{PubsubMessageCodec, PubsubMessageCodecError};
use appletheia_application::event::EventSequence;
use appletheia_application::projection::ProjectorNameOwned;
use appletheia_application::read_model::{ReadModelInvalidationEnvelope, ReadModelInvalidationId};
use appletheia_application::request_context::{CausationId, CorrelationId, MessageId};
use google_cloud_pubsub::model::Message;

use super::CloudEventsPubsubReadModelInvalidationCodecError;

use appletheia_application::ProjectorName;
use appletheia_application::PublishableMessage;

#[derive(Clone, Debug)]
pub struct CloudEventsPubsubReadModelInvalidationCodec {
    source: CloudEventSource,
    cloud_event_type_prefix: Option<CloudEventTypePrefix>,
}

impl CloudEventsPubsubReadModelInvalidationCodec {
    pub fn new(
        source: CloudEventSource,
        cloud_event_type_prefix: Option<CloudEventTypePrefix>,
    ) -> Self {
        Self {
            source,
            cloud_event_type_prefix,
        }
    }

    pub fn encode_id(invalidation_id: ReadModelInvalidationId) -> String {
        invalidation_id.to_string()
    }

    pub fn decode_id(
        cloud_event_id: &str,
    ) -> Result<ReadModelInvalidationId, CloudEventsPubsubReadModelInvalidationCodecError> {
        Ok(ReadModelInvalidationId::try_from(
            cloud_event_id.parse::<Uuid>()?,
        )?)
    }

    pub fn encode_type(cloud_event_type_prefix: Option<&CloudEventTypePrefix>) -> String {
        let name = "read_model.invalidated";

        match cloud_event_type_prefix {
            Some(prefix) => format!("{prefix}.{name}"),
            None => name.to_owned(),
        }
    }

    pub fn decode_type(
        event_type: &str,
        cloud_event_type_prefix: Option<&CloudEventTypePrefix>,
    ) -> Result<(), CloudEventsPubsubReadModelInvalidationCodecError> {
        let event_name = match cloud_event_type_prefix {
            Some(prefix) => event_type
                .strip_prefix(&format!("{prefix}."))
                .ok_or(CloudEventsPubsubReadModelInvalidationCodecError::TypePrefixMismatch)?,
            None => event_type,
        };
        if event_name != "read_model.invalidated" {
            return Err(CloudEventsPubsubReadModelInvalidationCodecError::InvalidMetadata("type"));
        }
        Ok(())
    }
}

impl PubsubMessageCodec for CloudEventsPubsubReadModelInvalidationCodec {
    type Message = ReadModelInvalidationEnvelope;
    type Selector = ProjectorName;

    fn encode(&self, envelope: &Self::Message) -> Result<Message, PubsubMessageCodecError> {
        (|| -> Result<Message, CloudEventsPubsubReadModelInvalidationCodecError> {
            if envelope.source_event_id.value() != envelope.causation_id.value() {
                return Err(
                    CloudEventsPubsubReadModelInvalidationCodecError::InvalidMetadata(
                        "source_event_id/causationid",
                    ),
                );
            }
            if envelope.invalidated_partitions.is_empty() {
                return Err(CloudEventsPubsubReadModelInvalidationCodecError::EmptyPartitions);
            }
            let ordering_key = envelope.ordering_key().to_string();
            let attributes: HashMap<String, String> = [
                ("ce-specversion", "1.0".to_owned()),
                ("ce-source", self.source.to_string()),
                ("ce-datacontenttype", "application/json".to_owned()),
                ("ce-partitionkey", ordering_key.clone()),
                ("ce-id", Self::encode_id(envelope.invalidation_id)),
                (
                    "ce-type",
                    Self::encode_type(self.cloud_event_type_prefix.as_ref()),
                ),
                ("ce-correlationid", envelope.correlation_id.to_string()),
                ("ce-causationid", envelope.causation_id.to_string()),
                ("ce-sourceeventid", envelope.source_event_id.to_string()),
                (
                    "ce-sourceeventsequence",
                    envelope.source_event_sequence.to_string(),
                ),
                (
                    "ce-sourceprojectorname",
                    envelope.source_projector_name.to_string(),
                ),
                ("ce-sourceeventoccurredat", {
                    let timestamp: DateTime<Utc> = envelope.source_event_occurred_at.into();
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
                .set_data(serde_json::to_vec(&serde_json::json!({
                    "invalidated_partitions": envelope.invalidated_partitions,
                }))?)
                .set_ordering_key(ordering_key))
        })()
        .map_err(|source| PubsubMessageCodecError::Encode(Box::new(source)))
    }

    fn decode(&self, message: &Message) -> Result<Self::Message, PubsubMessageCodecError> {
        (|| -> Result<Self::Message, CloudEventsPubsubReadModelInvalidationCodecError> {
            if message
                .attributes
                .get("ce-specversion")
                .map(String::as_str)
                .ok_or(
                    CloudEventsPubsubReadModelInvalidationCodecError::MissingAttribute(
                        "ce-specversion",
                    ),
                )?
                != "1.0"
            {
                return Err(
                    CloudEventsPubsubReadModelInvalidationCodecError::UnsupportedSpecVersion,
                );
            }

            message
                .attributes
                .get("ce-source")
                .map(String::as_str)
                .ok_or(
                    CloudEventsPubsubReadModelInvalidationCodecError::MissingAttribute("ce-source"),
                )?
                .parse::<CloudEventSource>()?;
            let content_type = message
                .attributes
                .get("ce-datacontenttype")
                .map(String::as_str)
                .ok_or(
                    CloudEventsPubsubReadModelInvalidationCodecError::MissingAttribute(
                        "ce-datacontenttype",
                    ),
                )?;
            let media_type = content_type.split(';').next().unwrap_or_default().trim();
            let is_json = media_type.split_once('/').is_some_and(|(_, subtype)| {
                subtype.eq_ignore_ascii_case("json")
                    || subtype
                        .rsplit_once('+')
                        .is_some_and(|(_, suffix)| suffix.eq_ignore_ascii_case("json"))
            });
            if !is_json {
                return Err(CloudEventsPubsubReadModelInvalidationCodecError::InvalidContentType);
            }

            let data: serde_json::Value = serde_json::from_slice(&message.data)?;

            Self::decode_type(
                message
                    .attributes
                    .get("ce-type")
                    .map(String::as_str)
                    .ok_or(
                        CloudEventsPubsubReadModelInvalidationCodecError::MissingAttribute(
                            "ce-type",
                        ),
                    )?,
                self.cloud_event_type_prefix.as_ref(),
            )?;
            let envelope = ReadModelInvalidationEnvelope {
                invalidation_id: Self::decode_id(
                    message.attributes.get("ce-id").map(String::as_str).ok_or(
                        CloudEventsPubsubReadModelInvalidationCodecError::MissingAttribute("ce-id"),
                    )?,
                )?,
                source_event_id: EventId::try_from(
                    message
                        .attributes
                        .get("ce-sourceeventid")
                        .map(String::as_str)
                        .ok_or(
                            CloudEventsPubsubReadModelInvalidationCodecError::MissingAttribute(
                                "ce-sourceeventid",
                            ),
                        )?
                        .parse::<Uuid>()?,
                )?,
                source_event_sequence: EventSequence::try_from(
                    message
                        .attributes
                        .get("ce-sourceeventsequence")
                        .map(String::as_str)
                        .ok_or(
                            CloudEventsPubsubReadModelInvalidationCodecError::MissingAttribute(
                                "ce-sourceeventsequence",
                            ),
                        )?
                        .parse::<i64>()?,
                )?,
                source_projector_name: ProjectorNameOwned::new(
                    message
                        .attributes
                        .get("ce-sourceprojectorname")
                        .map(String::as_str)
                        .ok_or(
                            CloudEventsPubsubReadModelInvalidationCodecError::MissingAttribute(
                                "ce-sourceprojectorname",
                            ),
                        )?
                        .to_string(),
                )?,
                source_event_occurred_at: DateTime::parse_from_rfc3339(
                    message
                        .attributes
                        .get("ce-sourceeventoccurredat")
                        .map(String::as_str)
                        .ok_or(
                            CloudEventsPubsubReadModelInvalidationCodecError::MissingAttribute(
                                "ce-sourceeventoccurredat",
                            ),
                        )?,
                )?
                .with_timezone(&Utc)
                .into(),
                invalidated_partitions: serde_json::from_value(
                    data.get("invalidated_partitions").cloned().ok_or(
                        CloudEventsPubsubReadModelInvalidationCodecError::InvalidMetadata(
                            "invalidated_partitions",
                        ),
                    )?,
                )?,
                correlation_id: CorrelationId::from(
                    message
                        .attributes
                        .get("ce-correlationid")
                        .map(String::as_str)
                        .ok_or(
                            CloudEventsPubsubReadModelInvalidationCodecError::MissingAttribute(
                                "ce-correlationid",
                            ),
                        )?
                        .parse::<Uuid>()?,
                ),
                causation_id: CausationId::from(MessageId::from(
                    message
                        .attributes
                        .get("ce-causationid")
                        .map(String::as_str)
                        .ok_or(
                            CloudEventsPubsubReadModelInvalidationCodecError::MissingAttribute(
                                "ce-causationid",
                            ),
                        )?
                        .parse::<Uuid>()?,
                )),
            };
            if envelope.source_event_id.value() != envelope.causation_id.value() {
                return Err(
                    CloudEventsPubsubReadModelInvalidationCodecError::InvalidMetadata(
                        "source_event_id/causationid",
                    ),
                );
            }
            if envelope.invalidated_partitions.is_empty() {
                return Err(CloudEventsPubsubReadModelInvalidationCodecError::EmptyPartitions);
            }
            if message
                .attributes
                .get("ce-partitionkey")
                .map(String::as_str)
                != Some((envelope.source_projector_name.to_string()).as_str())
            {
                return Err(
                    CloudEventsPubsubReadModelInvalidationCodecError::InvalidMetadata(
                        "partitionkey",
                    ),
                );
            }
            Ok(envelope)
        })()
        .map_err(|source| PubsubMessageCodecError::Decode(Box::new(source)))
    }

    fn encode_selector(
        &self,
        selector: &Self::Selector,
    ) -> Result<String, PubsubMessageCodecError> {
        (|| -> Result<String, CloudEventsPubsubReadModelInvalidationCodecError> {
            Ok(format!(
                "attributes.{} = {}",
                serde_json::to_string("ce-sourceprojectorname")?,
                serde_json::to_string(selector.value())?
            ))
        })()
        .map_err(|source| PubsubMessageCodecError::EncodeSelector(Box::new(source)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn id_and_type_conversions_preserve_values_and_reject_invalid_metadata() {
        let id = ReadModelInvalidationId::new();
        let encoded_id = CloudEventsPubsubReadModelInvalidationCodec::encode_id(id);
        assert_eq!(
            CloudEventsPubsubReadModelInvalidationCodec::decode_id(&encoded_id).unwrap(),
            id
        );
        assert!(CloudEventsPubsubReadModelInvalidationCodec::decode_id("invalid").is_err());
        let prefix = "com.example".parse().unwrap();
        for cloud_event_type_prefix in [None, Some(&prefix)] {
            let encoded =
                CloudEventsPubsubReadModelInvalidationCodec::encode_type(cloud_event_type_prefix);
            CloudEventsPubsubReadModelInvalidationCodec::decode_type(
                &encoded,
                cloud_event_type_prefix,
            )
            .unwrap();
        }
        assert!(
            CloudEventsPubsubReadModelInvalidationCodec::decode_type("wrong.type", None).is_err()
        );
        assert!(
            CloudEventsPubsubReadModelInvalidationCodec::decode_type("wrong.type", Some(&prefix))
                .is_err()
        );
    }
}
