use std::collections::HashMap;
use uuid::Uuid;

use crate::cloud_events::{CloudEventSource, CloudEventTypePrefix};
use crate::google_cloud::pubsub::messaging::{PubsubMessageCodec, PubsubMessageCodecError};
use appletheia_application::command::{CommandEnvelope, CommandNameOwned, SerializedCommand};
use appletheia_application::request_context::{CausationId, CorrelationId, MessageId};
use appletheia_application::saga::SagaCommandOrigin;
use google_cloud_pubsub::model::Message;

use super::CloudEventsPubsubCommandCodecError;

use appletheia_application::CommandSelector;
use appletheia_application::PublishableMessage;

#[derive(Clone, Debug)]
pub struct CloudEventsPubsubCommandCodec {
    source: CloudEventSource,
    cloud_event_type_prefix: Option<CloudEventTypePrefix>,
}

impl CloudEventsPubsubCommandCodec {
    pub fn new(
        source: CloudEventSource,
        cloud_event_type_prefix: Option<CloudEventTypePrefix>,
    ) -> Self {
        Self {
            source,
            cloud_event_type_prefix,
        }
    }

    pub fn encode_id(message_id: MessageId) -> String {
        message_id.to_string()
    }

    pub fn decode_id(
        cloud_event_id: &str,
    ) -> Result<MessageId, CloudEventsPubsubCommandCodecError> {
        Ok(MessageId::from(cloud_event_id.parse::<Uuid>()?))
    }

    pub fn encode_type(
        cloud_event_type_prefix: Option<&CloudEventTypePrefix>,
        command_name: &CommandNameOwned,
    ) -> String {
        let name = command_name.value();

        match cloud_event_type_prefix {
            Some(prefix) => format!("{prefix}.{name}"),
            None => name.to_owned(),
        }
    }

    pub fn decode_type(
        event_type: &str,
        cloud_event_type_prefix: Option<&CloudEventTypePrefix>,
    ) -> Result<CommandNameOwned, CloudEventsPubsubCommandCodecError> {
        let command_name = match cloud_event_type_prefix {
            Some(prefix) => event_type
                .strip_prefix(&format!("{prefix}."))
                .ok_or(CloudEventsPubsubCommandCodecError::TypePrefixMismatch)?,
            None => event_type,
        };
        Ok(command_name.parse()?)
    }
}

impl PubsubMessageCodec for CloudEventsPubsubCommandCodec {
    type Message = CommandEnvelope;
    type Selector = CommandSelector;

    fn encode(&self, envelope: &Self::Message) -> Result<Message, PubsubMessageCodecError> {
        (|| -> Result<Message, CloudEventsPubsubCommandCodecError> {
            let ordering_key = envelope.ordering_key().to_string();
            let mut attributes: HashMap<String, String> = [
                ("ce-specversion", "1.0".to_owned()),
                ("ce-source", self.source.to_string()),
                ("ce-datacontenttype", "application/json".to_owned()),
                ("ce-partitionkey", ordering_key.clone()),
                ("ce-id", Self::encode_id(envelope.message_id)),
                (
                    "ce-type",
                    Self::encode_type(
                        self.cloud_event_type_prefix.as_ref(),
                        &envelope.command_name,
                    ),
                ),
                ("ce-correlationid", envelope.correlation_id.to_string()),
                ("ce-causationid", envelope.causation_id.to_string()),
                ("ce-options", serde_json::to_string(&envelope.options)?),
            ]
            .into_iter()
            .map(|(name, value)| (name.to_owned(), value))
            .collect();
            if let Some(origin) = &envelope.saga_origin {
                attributes.insert("ce-sagaorigin".to_owned(), serde_json::to_string(origin)?);
                attributes.insert("ce-saganame".to_owned(), origin.saga_name.to_string());
            }
            Ok(Message::new()
                .set_attributes(attributes)
                .set_data(serde_json::to_vec(envelope.command.value())?)
                .set_ordering_key(ordering_key))
        })()
        .map_err(|source| PubsubMessageCodecError::Encode(Box::new(source)))
    }

    fn decode(&self, message: &Message) -> Result<Self::Message, PubsubMessageCodecError> {
        (|| -> Result<Self::Message, CloudEventsPubsubCommandCodecError> {
            if message
                .attributes
                .get("ce-specversion")
                .map(String::as_str)
                .ok_or(CloudEventsPubsubCommandCodecError::MissingAttribute(
                    "ce-specversion",
                ))?
                != "1.0"
            {
                return Err(CloudEventsPubsubCommandCodecError::UnsupportedSpecVersion);
            }

            message
                .attributes
                .get("ce-source")
                .map(String::as_str)
                .ok_or(CloudEventsPubsubCommandCodecError::MissingAttribute(
                    "ce-source",
                ))?
                .parse::<CloudEventSource>()?;
            let content_type = message
                .attributes
                .get("ce-datacontenttype")
                .map(String::as_str)
                .ok_or(CloudEventsPubsubCommandCodecError::MissingAttribute(
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
                return Err(CloudEventsPubsubCommandCodecError::InvalidContentType);
            }

            let data: serde_json::Value = serde_json::from_slice(&message.data)?;

            let command_name = Self::decode_type(
                message
                    .attributes
                    .get("ce-type")
                    .map(String::as_str)
                    .ok_or(CloudEventsPubsubCommandCodecError::MissingAttribute(
                        "ce-type",
                    ))?,
                self.cloud_event_type_prefix.as_ref(),
            )?;
            let saga_origin: Option<SagaCommandOrigin> = message
                .attributes
                .get("ce-sagaorigin")
                .map(ToString::to_string)
                .map(|value| serde_json::from_str(&value))
                .transpose()?;
            if message
                .attributes
                .get("ce-saganame")
                .map(ToString::to_string)
                != saga_origin
                    .as_ref()
                    .map(|origin| origin.saga_name.to_string())
            {
                return Err(CloudEventsPubsubCommandCodecError::InvalidMetadata(
                    "saganame/sagaorigin",
                ));
            }
            let envelope = CommandEnvelope {
                command_name,
                command: SerializedCommand::new(data)?,
                message_id: Self::decode_id(
                    message.attributes.get("ce-id").map(String::as_str).ok_or(
                        CloudEventsPubsubCommandCodecError::MissingAttribute("ce-id"),
                    )?,
                )?,
                options: serde_json::from_str(
                    message
                        .attributes
                        .get("ce-options")
                        .map(String::as_str)
                        .ok_or(CloudEventsPubsubCommandCodecError::MissingAttribute(
                            "ce-options",
                        ))?,
                )?,
                saga_origin,
                correlation_id: CorrelationId::from(
                    message
                        .attributes
                        .get("ce-correlationid")
                        .map(String::as_str)
                        .ok_or(CloudEventsPubsubCommandCodecError::MissingAttribute(
                            "ce-correlationid",
                        ))?
                        .parse::<Uuid>()?,
                ),
                causation_id: CausationId::from(MessageId::from(
                    message
                        .attributes
                        .get("ce-causationid")
                        .map(String::as_str)
                        .ok_or(CloudEventsPubsubCommandCodecError::MissingAttribute(
                            "ce-causationid",
                        ))?
                        .parse::<Uuid>()?,
                )),
            };
            if message
                .attributes
                .get("ce-partitionkey")
                .map(String::as_str)
                != Some((envelope.correlation_id.to_string()).as_str())
            {
                return Err(CloudEventsPubsubCommandCodecError::InvalidMetadata(
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
        (|| -> Result<String, CloudEventsPubsubCommandCodecError> {
            let event_type = Self::encode_type(
                self.cloud_event_type_prefix.as_ref(),
                &selector.command_name.into(),
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
    fn id_and_type_conversions_preserve_values_and_reject_invalid_metadata() {
        let id = MessageId::new();
        let encoded_id = CloudEventsPubsubCommandCodec::encode_id(id);
        assert_eq!(
            CloudEventsPubsubCommandCodec::decode_id(&encoded_id).unwrap(),
            id
        );
        assert!(CloudEventsPubsubCommandCodec::decode_id("invalid").is_err());
        let prefix = "com.example".parse().unwrap();
        for cloud_event_type_prefix in [None, Some(&prefix)] {
            let command_name = "transfer".parse().unwrap();
            let encoded =
                CloudEventsPubsubCommandCodec::encode_type(cloud_event_type_prefix, &command_name);
            assert_eq!(
                CloudEventsPubsubCommandCodec::decode_type(&encoded, cloud_event_type_prefix)
                    .unwrap(),
                command_name
            );
        }
        assert!(CloudEventsPubsubCommandCodec::decode_type("wrong.type", None).is_err());
        assert!(CloudEventsPubsubCommandCodec::decode_type("wrong.type", Some(&prefix)).is_err());
    }
}
