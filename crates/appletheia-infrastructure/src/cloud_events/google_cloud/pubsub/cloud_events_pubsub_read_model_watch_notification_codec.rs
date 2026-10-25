use super::CloudEventsPubsubReadModelWatchNotificationCodecError;
use crate::cloud_events::{CloudEventSource, CloudEventTypePrefix};
use crate::google_cloud::pubsub::messaging::{PubsubMessageCodec, PubsubMessageCodecError};
use appletheia_application::PublishableMessage;
use appletheia_application::read_model::ReadModelChangeId;
use appletheia_application::read_model::watch::{
    ReadModelWatchNotification, ReadModelWatchNotificationEnvelope, ReadModelWatchNotificationId,
    ReadModelWatchNotificationSelector,
};
use chrono::{DateTime, Utc};
use google_cloud_pubsub::model::Message;
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct CloudEventsPubsubReadModelWatchNotificationCodec {
    source: CloudEventSource,
    cloud_event_type_prefix: Option<CloudEventTypePrefix>,
}

impl CloudEventsPubsubReadModelWatchNotificationCodec {
    pub fn new(
        source: CloudEventSource,
        cloud_event_type_prefix: Option<CloudEventTypePrefix>,
    ) -> Self {
        Self {
            source,
            cloud_event_type_prefix,
        }
    }

    pub fn encode_id(notification_id: ReadModelWatchNotificationId) -> String {
        notification_id.to_string()
    }

    pub fn decode_id(
        notification_id: &str,
    ) -> Result<ReadModelWatchNotificationId, CloudEventsPubsubReadModelWatchNotificationCodecError>
    {
        Ok(ReadModelWatchNotificationId::try_from(
            notification_id.parse::<Uuid>()?,
        )?)
    }

    pub fn encode_type(cloud_event_type_prefix: Option<&CloudEventTypePrefix>) -> String {
        match cloud_event_type_prefix {
            Some(prefix) => format!("{prefix}.read_model_watch.notified"),
            None => "read_model_watch.notified".to_owned(),
        }
    }

    pub fn decode_type(
        event_type: &str,
        cloud_event_type_prefix: Option<&CloudEventTypePrefix>,
    ) -> Result<(), CloudEventsPubsubReadModelWatchNotificationCodecError> {
        if event_type != Self::encode_type(cloud_event_type_prefix) {
            return Err(
                CloudEventsPubsubReadModelWatchNotificationCodecError::InvalidMetadata("type"),
            );
        }
        Ok(())
    }
}

impl PubsubMessageCodec for CloudEventsPubsubReadModelWatchNotificationCodec {
    type Message = ReadModelWatchNotificationEnvelope;
    type Selector = ReadModelWatchNotificationSelector;

    fn encode(&self, envelope: &Self::Message) -> Result<Message, PubsubMessageCodecError> {
        (|| -> Result<Message, CloudEventsPubsubReadModelWatchNotificationCodecError> {
            let ordering_key = envelope.ordering_key().to_string();
            let attributes: HashMap<String, String> = [
                ("ce-specversion", "1.0".to_owned()),
                ("ce-source", self.source.to_string()),
                ("ce-id", Self::encode_id(envelope.notification_id)),
                (
                    "ce-type",
                    Self::encode_type(self.cloud_event_type_prefix.as_ref()),
                ),
                (
                    "ce-subject",
                    format!(
                        "{}/{}",
                        envelope.notification.target.read_model_type,
                        envelope.notification.target.id
                    ),
                ),
                (
                    "ce-endpointid",
                    envelope.notification.endpoint_id.to_string(),
                ),
                (
                    "ce-time",
                    DateTime::<Utc>::from(envelope.notified_at).to_rfc3339(),
                ),
                ("ce-causationid", envelope.source_change_id.to_string()),
                ("ce-datacontenttype", "application/json".to_owned()),
                ("ce-partitionkey", ordering_key.clone()),
            ]
            .into_iter()
            .map(|(key, value)| (key.to_owned(), value))
            .collect();
            Ok(Message::new()
                .set_attributes(attributes)
                .set_data(serde_json::to_vec(&envelope.notification)?)
                .set_ordering_key(ordering_key))
        })()
        .map_err(|source| PubsubMessageCodecError::Encode(Box::new(source)))
    }

    fn decode(&self, message: &Message) -> Result<Self::Message, PubsubMessageCodecError> {
        (|| -> Result<Self::Message, CloudEventsPubsubReadModelWatchNotificationCodecError> {
            if message
                .attributes
                .get("ce-specversion")
                .map(String::as_str)
                .ok_or(
                    CloudEventsPubsubReadModelWatchNotificationCodecError::MissingAttribute(
                        "ce-specversion",
                    ),
                )?
                != "1.0"
            {
                return Err(
                    CloudEventsPubsubReadModelWatchNotificationCodecError::UnsupportedSpecVersion,
                );
            }
            message
                .attributes
                .get("ce-source")
                .map(String::as_str)
                .ok_or(
                    CloudEventsPubsubReadModelWatchNotificationCodecError::MissingAttribute(
                        "ce-source",
                    ),
                )?
                .parse::<CloudEventSource>()?;
            let content_type = message
                .attributes
                .get("ce-datacontenttype")
                .map(String::as_str)
                .ok_or(
                    CloudEventsPubsubReadModelWatchNotificationCodecError::MissingAttribute(
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
                return Err(
                    CloudEventsPubsubReadModelWatchNotificationCodecError::InvalidContentType,
                );
            }
            let notification: ReadModelWatchNotification = serde_json::from_slice(&message.data)?;
            Self::decode_type(
                message
                    .attributes
                    .get("ce-type")
                    .map(String::as_str)
                    .ok_or(
                        CloudEventsPubsubReadModelWatchNotificationCodecError::MissingAttribute(
                            "ce-type",
                        ),
                    )?,
                self.cloud_event_type_prefix.as_ref(),
            )?;
            let envelope = ReadModelWatchNotificationEnvelope {
                notification_id: Self::decode_id(
                    message.attributes.get("ce-id").map(String::as_str).ok_or(
                        CloudEventsPubsubReadModelWatchNotificationCodecError::MissingAttribute(
                            "ce-id",
                        ),
                    )?,
                )?,
                source_change_id: ReadModelChangeId::try_from(
                    message
                        .attributes
                        .get("ce-causationid")
                        .map(String::as_str)
                        .ok_or(
                            CloudEventsPubsubReadModelWatchNotificationCodecError::MissingAttribute(
                                "ce-causationid",
                            ),
                        )?
                        .parse::<Uuid>()?,
                )?,
                notified_at: DateTime::parse_from_rfc3339(
                    message
                        .attributes
                        .get("ce-time")
                        .map(String::as_str)
                        .ok_or(
                            CloudEventsPubsubReadModelWatchNotificationCodecError::MissingAttribute(
                                "ce-time",
                            ),
                        )?,
                )?
                .with_timezone(&Utc)
                .into(),
                notification,
            };
            let ordering_key = envelope.ordering_key();
            if message
                .attributes
                .get("ce-subject")
                .map(String::as_str)
                .ok_or(
                    CloudEventsPubsubReadModelWatchNotificationCodecError::MissingAttribute(
                        "ce-subject",
                    ),
                )?
                != format!(
                    "{}/{}",
                    envelope.notification.target.read_model_type, envelope.notification.target.id
                )
            {
                return Err(
                    CloudEventsPubsubReadModelWatchNotificationCodecError::InvalidMetadata(
                        "subject",
                    ),
                );
            }
            if message
                .attributes
                .get("ce-endpointid")
                .map(String::as_str)
                .ok_or(
                    CloudEventsPubsubReadModelWatchNotificationCodecError::MissingAttribute(
                        "ce-endpointid",
                    ),
                )?
                != envelope.notification.endpoint_id.to_string()
            {
                return Err(
                    CloudEventsPubsubReadModelWatchNotificationCodecError::InvalidMetadata(
                        "endpointid",
                    ),
                );
            }
            if message
                .attributes
                .get("ce-partitionkey")
                .map(String::as_str)
                .ok_or(
                    CloudEventsPubsubReadModelWatchNotificationCodecError::MissingAttribute(
                        "ce-partitionkey",
                    ),
                )?
                != ordering_key.as_str()
            {
                return Err(
                    CloudEventsPubsubReadModelWatchNotificationCodecError::InvalidMetadata(
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
        (|| -> Result<String, CloudEventsPubsubReadModelWatchNotificationCodecError> {
            let event_type = Self::encode_type(self.cloud_event_type_prefix.as_ref());
            Ok(format!(
                "attributes.{} = {} AND attributes.{} = {}",
                serde_json::to_string("ce-type")?,
                serde_json::to_string(&event_type)?,
                serde_json::to_string("ce-endpointid")?,
                serde_json::to_string(&selector.endpoint_id.to_string())?
            ))
        })()
        .map_err(|source| PubsubMessageCodecError::EncodeSelector(Box::new(source)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use appletheia_application::messaging::Selector;
    use appletheia_application::read_model::ReadModelRef;
    use appletheia_application::read_model::watch::ReadModelWatchEndpointId;

    fn envelope() -> ReadModelWatchNotificationEnvelope {
        ReadModelWatchNotificationEnvelope::new(
            ReadModelChangeId::new(),
            ReadModelWatchNotification::new(
                ReadModelWatchEndpointId::new(),
                ReadModelRef {
                    read_model_type: "user".parse().unwrap(),
                    id: "provider/subject:123".into(),
                },
            ),
        )
    }

    #[test]
    fn round_trip_preserves_notification_and_endpoint_routing() {
        for prefix in [None, Some("com.example".parse().unwrap())] {
            let codec = CloudEventsPubsubReadModelWatchNotificationCodec::new(
                "urn:example:watch".parse().unwrap(),
                prefix,
            );
            let envelope = envelope();
            let wire = codec.encode(&envelope).unwrap();
            assert_eq!(codec.decode(&wire).unwrap(), envelope);
            assert_eq!(
                wire.attributes["ce-id"],
                envelope.notification_id.to_string()
            );
            assert_eq!(
                wire.attributes["ce-causationid"],
                envelope.source_change_id.to_string()
            );
            assert_eq!(wire.attributes["ce-subject"], "user/provider/subject:123");
            assert_eq!(
                wire.attributes["ce-endpointid"],
                envelope.notification.endpoint_id.to_string()
            );
            assert_eq!(wire.attributes["ce-partitionkey"], wire.ordering_key);
            assert_eq!(
                wire.ordering_key,
                envelope.notification.endpoint_id.to_string()
            );
            assert_eq!(
                serde_json::from_slice::<serde_json::Value>(&wire.data).unwrap(),
                serde_json::json!({
                    "endpoint_id": envelope.notification.endpoint_id,
                    "target": {"type": "user", "id": "provider/subject:123"},
                })
            );
            let selector =
                ReadModelWatchNotificationSelector::new(envelope.notification.endpoint_id);
            assert!(selector.matches(&envelope));
            assert_eq!(
                codec.encode_selector(&selector).unwrap(),
                format!(
                    "attributes.\"ce-type\" = {} AND attributes.\"ce-endpointid\" = {}",
                    serde_json::to_string(&wire.attributes["ce-type"]).unwrap(),
                    serde_json::to_string(&envelope.notification.endpoint_id.to_string()).unwrap(),
                )
            );
            let mut extended = wire.clone();
            extended
                .attributes
                .insert("ce-extra".into(), "ignored".into());
            assert_eq!(codec.decode(&extended).unwrap(), envelope);
            for name in [
                "ce-specversion",
                "ce-source",
                "ce-id",
                "ce-type",
                "ce-subject",
                "ce-time",
                "ce-causationid",
                "ce-endpointid",
                "ce-datacontenttype",
                "ce-partitionkey",
            ] {
                let mut missing = wire.clone();
                missing.attributes.remove(name);
                assert!(codec.decode(&missing).is_err(), "{name}");
            }
            for (name, value) in [
                ("ce-specversion", "2.0"),
                ("ce-id", "invalid"),
                ("ce-type", "other.read_model_watch.notified"),
                ("ce-subject", "user/other"),
                ("ce-time", "invalid"),
                ("ce-causationid", "invalid"),
                ("ce-endpointid", "other"),
                ("ce-partitionkey", "wrong"),
                ("ce-datacontenttype", "text/plain"),
            ] {
                let mut invalid = wire.clone();
                invalid.attributes.insert(name.into(), value.into());
                assert!(codec.decode(&invalid).is_err(), "{name}");
            }
            let mut wrong_target = wire.clone();
            wrong_target.data = serde_json::to_vec(&serde_json::json!({
                "endpoint_id": envelope.notification.endpoint_id,
                "target": {"type": "user", "id": "different"},
            }))
            .unwrap()
            .into();
            assert!(codec.decode(&wrong_target).is_err());
            let mut wrong_endpoint = wire.clone();
            wrong_endpoint.data = serde_json::to_vec(&ReadModelWatchNotification::new(
                ReadModelWatchEndpointId::new(),
                envelope.notification.target.clone(),
            ))
            .unwrap()
            .into();
            assert!(codec.decode(&wrong_endpoint).is_err());
        }
    }

    #[test]
    fn selector_escapes_configured_type_prefix() {
        let prefix: CloudEventTypePrefix = "com.quoted\"\\value".parse().unwrap();
        let codec = CloudEventsPubsubReadModelWatchNotificationCodec::new(
            "urn:example:watch".parse().unwrap(),
            Some(prefix),
        );
        let selector = ReadModelWatchNotificationSelector::new(ReadModelWatchEndpointId::new());
        assert_eq!(
            codec.encode_selector(&selector).unwrap(),
            format!(
                r#"attributes."ce-type" = "com.quoted\"\\value.read_model_watch.notified" AND attributes."ce-endpointid" = "{}""#,
                selector.endpoint_id,
            )
        );
    }
}
