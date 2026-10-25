use super::CloudEventsPubsubReadModelChangeCodecError;
use crate::cloud_events::{CloudEventSource, CloudEventTypePrefix};
use crate::google_cloud::pubsub::messaging::{PubsubMessageCodec, PubsubMessageCodecError};
use appletheia_application::PublishableMessage;
use appletheia_application::read_model::{
    ReadModelChange, ReadModelChangeEnvelope, ReadModelChangeId, ReadModelChangeSelector,
    ReadModelTypeOwned,
};
use appletheia_domain::EventId;
use chrono::{DateTime, Utc};
use google_cloud_pubsub::model::Message;
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct CloudEventsPubsubReadModelChangeCodec {
    source: CloudEventSource,
    cloud_event_type_prefix: Option<CloudEventTypePrefix>,
}

impl CloudEventsPubsubReadModelChangeCodec {
    pub fn new(
        source: CloudEventSource,
        cloud_event_type_prefix: Option<CloudEventTypePrefix>,
    ) -> Self {
        Self {
            source,
            cloud_event_type_prefix,
        }
    }

    pub fn encode_id(change_id: ReadModelChangeId) -> String {
        change_id.to_string()
    }

    pub fn decode_id(
        change_id: &str,
    ) -> Result<ReadModelChangeId, CloudEventsPubsubReadModelChangeCodecError> {
        Ok(ReadModelChangeId::try_from(change_id.parse::<Uuid>()?)?)
    }

    pub fn encode_type(
        cloud_event_type_prefix: Option<&CloudEventTypePrefix>,
        read_model_type: &ReadModelTypeOwned,
    ) -> String {
        match cloud_event_type_prefix {
            Some(prefix) => format!("{prefix}.{read_model_type}.changed"),
            None => format!("{read_model_type}.changed"),
        }
    }

    pub fn decode_type(
        event_type: &str,
        cloud_event_type_prefix: Option<&CloudEventTypePrefix>,
    ) -> Result<ReadModelTypeOwned, CloudEventsPubsubReadModelChangeCodecError> {
        let name = match cloud_event_type_prefix {
            Some(prefix) => event_type
                .strip_prefix(&format!("{prefix}."))
                .ok_or(CloudEventsPubsubReadModelChangeCodecError::TypePrefixMismatch)?,
            None => event_type,
        };
        Ok(name
            .strip_suffix(".changed")
            .ok_or(CloudEventsPubsubReadModelChangeCodecError::InvalidMetadata(
                "type",
            ))?
            .parse()?)
    }
}

impl PubsubMessageCodec for CloudEventsPubsubReadModelChangeCodec {
    type Message = ReadModelChangeEnvelope;
    type Selector = ReadModelChangeSelector;

    fn encode(&self, envelope: &Self::Message) -> Result<Message, PubsubMessageCodecError> {
        (|| -> Result<Message, CloudEventsPubsubReadModelChangeCodecError> {
            let ordering_key = envelope.ordering_key().to_string();
            let attributes: HashMap<String, String> = [
                ("ce-specversion", "1.0".to_owned()),
                ("ce-source", self.source.to_string()),
                ("ce-id", Self::encode_id(envelope.change_id)),
                (
                    "ce-type",
                    Self::encode_type(
                        self.cloud_event_type_prefix.as_ref(),
                        &envelope.change.read_model_ref().read_model_type,
                    ),
                ),
                ("ce-subject", ordering_key.clone()),
                (
                    "ce-time",
                    DateTime::<Utc>::from(envelope.changed_at).to_rfc3339(),
                ),
                ("ce-causationid", envelope.source_event_id.to_string()),
                ("ce-datacontenttype", "application/json".to_owned()),
                ("ce-partitionkey", ordering_key.clone()),
            ]
            .into_iter()
            .map(|(key, value)| (key.to_owned(), value))
            .collect();
            Ok(Message::new()
                .set_attributes(attributes)
                .set_data(serde_json::to_vec(&envelope.change)?)
                .set_ordering_key(ordering_key))
        })()
        .map_err(|source| PubsubMessageCodecError::Encode(Box::new(source)))
    }

    fn decode(&self, message: &Message) -> Result<Self::Message, PubsubMessageCodecError> {
        (|| -> Result<Self::Message, CloudEventsPubsubReadModelChangeCodecError> {
            if message
                .attributes
                .get("ce-specversion")
                .map(String::as_str)
                .ok_or(
                    CloudEventsPubsubReadModelChangeCodecError::MissingAttribute("ce-specversion"),
                )?
                != "1.0"
            {
                return Err(CloudEventsPubsubReadModelChangeCodecError::UnsupportedSpecVersion);
            }
            message
                .attributes
                .get("ce-source")
                .map(String::as_str)
                .ok_or(CloudEventsPubsubReadModelChangeCodecError::MissingAttribute("ce-source"))?
                .parse::<CloudEventSource>()?;
            let content_type = message
                .attributes
                .get("ce-datacontenttype")
                .map(String::as_str)
                .ok_or(
                    CloudEventsPubsubReadModelChangeCodecError::MissingAttribute(
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
                return Err(CloudEventsPubsubReadModelChangeCodecError::InvalidContentType);
            }
            let change: ReadModelChange = serde_json::from_slice(&message.data)?;
            let read_model_type = Self::decode_type(
                message
                    .attributes
                    .get("ce-type")
                    .map(String::as_str)
                    .ok_or(
                        CloudEventsPubsubReadModelChangeCodecError::MissingAttribute("ce-type"),
                    )?,
                self.cloud_event_type_prefix.as_ref(),
            )?;
            if read_model_type != change.read_model_ref().read_model_type {
                return Err(CloudEventsPubsubReadModelChangeCodecError::InvalidMetadata(
                    "type",
                ));
            }
            let envelope = ReadModelChangeEnvelope {
                change_id: Self::decode_id(
                    message.attributes.get("ce-id").map(String::as_str).ok_or(
                        CloudEventsPubsubReadModelChangeCodecError::MissingAttribute("ce-id"),
                    )?,
                )?,
                source_event_id: EventId::try_from(
                    message
                        .attributes
                        .get("ce-causationid")
                        .map(String::as_str)
                        .ok_or(
                            CloudEventsPubsubReadModelChangeCodecError::MissingAttribute(
                                "ce-causationid",
                            ),
                        )?
                        .parse::<Uuid>()?,
                )?,
                changed_at: DateTime::parse_from_rfc3339(
                    message
                        .attributes
                        .get("ce-time")
                        .map(String::as_str)
                        .ok_or(
                            CloudEventsPubsubReadModelChangeCodecError::MissingAttribute("ce-time"),
                        )?,
                )?
                .with_timezone(&Utc)
                .into(),
                change,
            };
            let ordering_key = envelope.ordering_key();
            if message
                .attributes
                .get("ce-subject")
                .map(String::as_str)
                .ok_or(CloudEventsPubsubReadModelChangeCodecError::MissingAttribute("ce-subject"))?
                != ordering_key.as_str()
            {
                return Err(CloudEventsPubsubReadModelChangeCodecError::InvalidMetadata(
                    "subject",
                ));
            }
            if message
                .attributes
                .get("ce-partitionkey")
                .map(String::as_str)
                .ok_or(
                    CloudEventsPubsubReadModelChangeCodecError::MissingAttribute("ce-partitionkey"),
                )?
                != ordering_key.as_str()
            {
                return Err(CloudEventsPubsubReadModelChangeCodecError::InvalidMetadata(
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
        (|| -> Result<String, CloudEventsPubsubReadModelChangeCodecError> {
            let event_type = Self::encode_type(
                self.cloud_event_type_prefix.as_ref(),
                &selector.read_model_type.into(),
            );
            Ok(format!(
                "attributes.{} = {}",
                serde_json::to_string("ce-type")?,
                serde_json::to_string(&event_type)?
            ))
        })()
        .map_err(|source| PubsubMessageCodecError::EncodeSelector(Box::new(source)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use appletheia_application::messaging::Selector;
    use appletheia_application::read_model::{ReadModelRef, ReadModelType};

    #[test]
    fn round_trip_preserves_change_identity_and_routing() {
        for prefix in [None, Some("com.example".parse().unwrap())] {
            let codec = CloudEventsPubsubReadModelChangeCodec::new(
                "urn:example:read-models".parse().unwrap(),
                prefix,
            );
            let envelope = ReadModelChangeEnvelope::new(
                EventId::new(),
                ReadModelChange::new(ReadModelRef {
                    read_model_type: "users".parse().unwrap(),
                    id: "provider/subject:123".into(),
                }),
            );
            let wire = codec.encode(&envelope).unwrap();
            assert_eq!(codec.decode(&wire).unwrap(), envelope);
            assert_eq!(wire.attributes["ce-subject"], "users/provider/subject:123");
            assert_eq!(wire.attributes["ce-partitionkey"], wire.ordering_key);
            assert_eq!(
                wire.attributes["ce-causationid"],
                envelope.source_event_id.to_string()
            );
            assert_eq!(wire.attributes["ce-id"], envelope.change_id.to_string());
            assert_eq!(
                serde_json::from_slice::<serde_json::Value>(&wire.data).unwrap(),
                serde_json::json!({"type": "users", "id": "provider/subject:123"})
            );
            let selector = ReadModelChangeSelector::new(ReadModelType::new("users"));
            assert!(selector.matches(&envelope));
            assert_eq!(
                codec.encode_selector(&selector).unwrap(),
                format!(
                    "attributes.\"ce-type\" = {}",
                    serde_json::to_string(&wire.attributes["ce-type"]).unwrap()
                )
            );
            let mut extended = wire.clone();
            extended
                .attributes
                .insert("ce-extra".into(), "ignored".into());
            assert_eq!(codec.decode(&extended).unwrap(), envelope);
            for name in [
                "ce-specversion",
                "ce-id",
                "ce-source",
                "ce-type",
                "ce-subject",
                "ce-time",
                "ce-causationid",
                "ce-partitionkey",
                "ce-datacontenttype",
            ] {
                let mut missing = wire.clone();
                missing.attributes.remove(name);
                assert!(codec.decode(&missing).is_err(), "{name}");
            }
            for (name, value) in [
                ("ce-specversion", "2.0"),
                ("ce-id", "not-a-uuid"),
                ("ce-type", "other.changed"),
                ("ce-subject", "users/other"),
                ("ce-time", "invalid"),
                ("ce-causationid", "invalid"),
                ("ce-partitionkey", "wrong"),
                ("ce-datacontenttype", "text/plain"),
            ] {
                let mut invalid = wire.clone();
                invalid.attributes.insert(name.into(), value.into());
                assert!(codec.decode(&invalid).is_err(), "{name}");
            }
        }
    }

    #[test]
    fn dynamic_resource_type_validation_and_filter_escaping() {
        assert!(
            serde_json::from_str::<ReadModelChange>(r#"{"type":"invalid/type","id":"1"}"#).is_err()
        );
        let codec = CloudEventsPubsubReadModelChangeCodec::new(
            "urn:example".parse().unwrap(),
            Some(r#"com.quoted"\value"#.parse().unwrap()),
        );
        let selector = ReadModelChangeSelector::new(ReadModelType::new("users"));
        assert_eq!(
            codec.encode_selector(&selector).unwrap(),
            r#"attributes."ce-type" = "com.quoted\"\\value.users.changed""#
        );
    }
}
