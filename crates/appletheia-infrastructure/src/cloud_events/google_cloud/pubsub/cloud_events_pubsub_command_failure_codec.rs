use chrono::{DateTime, Utc};
use std::collections::HashMap;
use uuid::Uuid;

use crate::cloud_events::{CloudEventSource, CloudEventTypePrefix};
use crate::google_cloud::pubsub::messaging::{PubsubMessageCodec, PubsubMessageCodecError};
use appletheia_application::command::{
    CommandAttemptCount, CommandFailureEnvelope, CommandFailureId, CommandNameOwned,
};
use appletheia_application::request_context::{CausationId, CorrelationId, MessageId};
use appletheia_application::saga::SagaCommandOrigin;
use google_cloud_pubsub::model::Message;

use super::CloudEventsPubsubCommandFailureCodecError;

use appletheia_application::PublishableMessage;
use appletheia_application::SagaName;

#[derive(Clone, Debug)]
pub struct CloudEventsPubsubCommandFailureCodec {
    source: CloudEventSource,
    cloud_event_type_prefix: Option<CloudEventTypePrefix>,
}

impl CloudEventsPubsubCommandFailureCodec {
    pub fn new(
        source: CloudEventSource,
        cloud_event_type_prefix: Option<CloudEventTypePrefix>,
    ) -> Self {
        Self {
            source,
            cloud_event_type_prefix,
        }
    }

    pub fn encode_id(failure_id: CommandFailureId) -> String {
        failure_id.to_string()
    }

    pub fn decode_id(
        cloud_event_id: &str,
    ) -> Result<CommandFailureId, CloudEventsPubsubCommandFailureCodecError> {
        Ok(CommandFailureId::try_from(cloud_event_id.parse::<Uuid>()?)?)
    }

    pub fn encode_type(
        cloud_event_type_prefix: Option<&CloudEventTypePrefix>,
        command_name: &CommandNameOwned,
    ) -> String {
        let name = &format!("{command_name}.failed");

        match cloud_event_type_prefix {
            Some(prefix) => format!("{prefix}.{name}"),
            None => name.to_owned(),
        }
    }

    pub fn decode_type(
        event_type: &str,
        cloud_event_type_prefix: Option<&CloudEventTypePrefix>,
    ) -> Result<CommandNameOwned, CloudEventsPubsubCommandFailureCodecError> {
        let name = match cloud_event_type_prefix {
            Some(prefix) => event_type
                .strip_prefix(&format!("{prefix}."))
                .ok_or(CloudEventsPubsubCommandFailureCodecError::TypePrefixMismatch)?,
            None => event_type,
        };
        let command_name = name.strip_suffix(".failed").ok_or(
            CloudEventsPubsubCommandFailureCodecError::InvalidMetadata("type"),
        )?;
        Ok(command_name.parse()?)
    }
}

impl PubsubMessageCodec for CloudEventsPubsubCommandFailureCodec {
    type Message = CommandFailureEnvelope;
    type Selector = SagaName;

    fn encode(&self, envelope: &Self::Message) -> Result<Message, PubsubMessageCodecError> {
        (|| -> Result<Message, CloudEventsPubsubCommandFailureCodecError> {
            if envelope.command_message_id.value() != envelope.causation_id.value() {
                return Err(CloudEventsPubsubCommandFailureCodecError::InvalidMetadata(
                    "command_message_id/causationid",
                ));
            }
            let ordering_key = envelope.ordering_key().to_string();
            let attributes: HashMap<String, String> = [
                ("ce-specversion", "1.0".to_owned()),
                ("ce-source", self.source.to_string()),
                ("ce-datacontenttype", "application/json".to_owned()),
                ("ce-partitionkey", ordering_key.clone()),
                ("ce-id", Self::encode_id(envelope.failure_id)),
                (
                    "ce-type",
                    Self::encode_type(
                        self.cloud_event_type_prefix.as_ref(),
                        &envelope.command_name,
                    ),
                ),
                ("ce-correlationid", envelope.correlation_id.to_string()),
                ("ce-causationid", envelope.causation_id.to_string()),
                (
                    "ce-commandmessageid",
                    envelope.command_message_id.to_string(),
                ),
                ("ce-sagaorigin", serde_json::to_string(&envelope.origin)?),
                ("ce-saganame", envelope.origin.saga_name.to_string()),
                ("ce-time", {
                    let timestamp: DateTime<Utc> = envelope.failed_at.into();
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
                    "terminal_reason": envelope.terminal_reason,
                    "attempt_count": envelope.attempt_count,
                }))?)
                .set_ordering_key(ordering_key))
        })()
        .map_err(|source| PubsubMessageCodecError::Encode(Box::new(source)))
    }

    fn decode(&self, message: &Message) -> Result<Self::Message, PubsubMessageCodecError> {
        (|| -> Result<Self::Message, CloudEventsPubsubCommandFailureCodecError> {
            if message
                .attributes
                .get("ce-specversion")
                .map(String::as_str)
                .ok_or(CloudEventsPubsubCommandFailureCodecError::MissingAttribute(
                    "ce-specversion",
                ))?
                != "1.0"
            {
                return Err(CloudEventsPubsubCommandFailureCodecError::UnsupportedSpecVersion);
            }

            message
                .attributes
                .get("ce-source")
                .map(String::as_str)
                .ok_or(CloudEventsPubsubCommandFailureCodecError::MissingAttribute(
                    "ce-source",
                ))?
                .parse::<CloudEventSource>()?;
            let content_type = message
                .attributes
                .get("ce-datacontenttype")
                .map(String::as_str)
                .ok_or(CloudEventsPubsubCommandFailureCodecError::MissingAttribute(
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
                return Err(CloudEventsPubsubCommandFailureCodecError::InvalidContentType);
            }

            let data: serde_json::Value = serde_json::from_slice(&message.data)?;

            let command_name = Self::decode_type(
                message
                    .attributes
                    .get("ce-type")
                    .map(String::as_str)
                    .ok_or(CloudEventsPubsubCommandFailureCodecError::MissingAttribute(
                        "ce-type",
                    ))?,
                self.cloud_event_type_prefix.as_ref(),
            )?;
            let origin: SagaCommandOrigin = serde_json::from_str(
                message
                    .attributes
                    .get("ce-sagaorigin")
                    .map(String::as_str)
                    .ok_or(CloudEventsPubsubCommandFailureCodecError::MissingAttribute(
                        "ce-sagaorigin",
                    ))?,
            )?;
            if origin.saga_name.value()
                != message
                    .attributes
                    .get("ce-saganame")
                    .map(String::as_str)
                    .ok_or(CloudEventsPubsubCommandFailureCodecError::MissingAttribute(
                        "ce-saganame",
                    ))?
            {
                return Err(CloudEventsPubsubCommandFailureCodecError::InvalidMetadata(
                    "saganame/sagaorigin",
                ));
            }
            let envelope = CommandFailureEnvelope {
                failure_id: Self::decode_id(
                    message.attributes.get("ce-id").map(String::as_str).ok_or(
                        CloudEventsPubsubCommandFailureCodecError::MissingAttribute("ce-id"),
                    )?,
                )?,
                command_message_id: MessageId::from(
                    message
                        .attributes
                        .get("ce-commandmessageid")
                        .map(String::as_str)
                        .ok_or(CloudEventsPubsubCommandFailureCodecError::MissingAttribute(
                            "ce-commandmessageid",
                        ))?
                        .parse::<Uuid>()?,
                ),
                command_name,
                origin,
                terminal_reason: serde_json::from_value(
                    data.get("terminal_reason").cloned().ok_or(
                        CloudEventsPubsubCommandFailureCodecError::InvalidMetadata(
                            "terminal_reason",
                        ),
                    )?,
                )?,
                attempt_count: CommandAttemptCount::try_from(serde_json::from_value::<i64>(
                    data.get("attempt_count").cloned().ok_or(
                        CloudEventsPubsubCommandFailureCodecError::InvalidMetadata("attempt_count"),
                    )?,
                )?)?,
                failed_at: DateTime::parse_from_rfc3339(
                    message
                        .attributes
                        .get("ce-time")
                        .map(String::as_str)
                        .ok_or(CloudEventsPubsubCommandFailureCodecError::MissingAttribute(
                            "ce-time",
                        ))?,
                )?
                .with_timezone(&Utc)
                .into(),
                correlation_id: CorrelationId::from(
                    message
                        .attributes
                        .get("ce-correlationid")
                        .map(String::as_str)
                        .ok_or(CloudEventsPubsubCommandFailureCodecError::MissingAttribute(
                            "ce-correlationid",
                        ))?
                        .parse::<Uuid>()?,
                ),
                causation_id: CausationId::from(MessageId::from(
                    message
                        .attributes
                        .get("ce-causationid")
                        .map(String::as_str)
                        .ok_or(CloudEventsPubsubCommandFailureCodecError::MissingAttribute(
                            "ce-causationid",
                        ))?
                        .parse::<Uuid>()?,
                )),
            };
            if envelope.command_message_id.value() != envelope.causation_id.value() {
                return Err(CloudEventsPubsubCommandFailureCodecError::InvalidMetadata(
                    "command_message_id/causationid",
                ));
            }
            if message
                .attributes
                .get("ce-partitionkey")
                .map(String::as_str)
                != Some((envelope.correlation_id.to_string()).as_str())
            {
                return Err(CloudEventsPubsubCommandFailureCodecError::InvalidMetadata(
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
        (|| -> Result<String, CloudEventsPubsubCommandFailureCodecError> {
            Ok(format!(
                "attributes.{} = {}",
                serde_json::to_string("ce-saganame")?,
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
        let id = CommandFailureId::new();
        let encoded_id = CloudEventsPubsubCommandFailureCodec::encode_id(id);
        assert_eq!(
            CloudEventsPubsubCommandFailureCodec::decode_id(&encoded_id).unwrap(),
            id
        );
        assert!(CloudEventsPubsubCommandFailureCodec::decode_id("invalid").is_err());
        let prefix = "com.example".parse().unwrap();
        for cloud_event_type_prefix in [None, Some(&prefix)] {
            let command_name = "transfer".parse().unwrap();
            let encoded = CloudEventsPubsubCommandFailureCodec::encode_type(
                cloud_event_type_prefix,
                &command_name,
            );
            assert_eq!(
                CloudEventsPubsubCommandFailureCodec::decode_type(
                    &encoded,
                    cloud_event_type_prefix
                )
                .unwrap(),
                command_name
            );
        }
        assert!(CloudEventsPubsubCommandFailureCodec::decode_type("wrong.type", None).is_err());
        assert!(
            CloudEventsPubsubCommandFailureCodec::decode_type("wrong.type", Some(&prefix)).is_err()
        );
    }
}
