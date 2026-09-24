use serde::{Deserialize, Serialize};

use crate::messaging::{OrderingKey, PublishableMessage};
use crate::request_context::{CausationId, CorrelationId, MessageId};
use crate::saga::SagaCommandOrigin;

use super::{Command, CommandEnvelopeError, CommandNameOwned, CommandOptions, SerializedCommand};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct CommandEnvelope {
    pub command_name: CommandNameOwned,
    pub command: SerializedCommand,
    pub correlation_id: CorrelationId,
    pub message_id: MessageId,
    pub causation_id: CausationId,
    pub saga_origin: Option<SagaCommandOrigin>,
    pub options: CommandOptions,
}

impl CommandEnvelope {
    pub fn new<C: Command>(
        command: &C,
        correlation_id: CorrelationId,
        causation_id: CausationId,
        options: CommandOptions,
    ) -> Result<Self, CommandEnvelopeError> {
        Ok(Self {
            command_name: CommandNameOwned::from(C::NAME),
            command: SerializedCommand::new(serde_json::to_value(command)?)?,
            correlation_id,
            message_id: MessageId::new(),
            causation_id,
            saga_origin: None,
            options,
        })
    }

    /// Attaches the saga step that dispatched this command.
    pub fn with_saga_origin(mut self, saga_origin: SagaCommandOrigin) -> Self {
        self.saga_origin = Some(saga_origin);
        self
    }

    pub fn try_to_command<C>(&self) -> Result<C, CommandEnvelopeError>
    where
        C: Command,
    {
        let expected = CommandNameOwned::from(C::NAME);
        if self.command_name != expected {
            return Err(CommandEnvelopeError::CommandNameMismatch {
                expected: expected.to_string(),
                actual: self.command_name.to_string(),
            });
        }

        let json = self.command.value().clone();
        Ok(serde_json::from_value(json)?)
    }
}

impl PublishableMessage for CommandEnvelope {
    fn ordering_key(&self) -> OrderingKey {
        OrderingKey::from(self.correlation_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::messaging::CloudEventAttributeValue;
    use crate::messaging::CommandCloudEventCodec;
    use crate::saga::{SagaInstanceId, SagaNameOwned, SerializedSagaStep};

    #[test]
    fn cloud_event_round_trip_preserves_optional_origin_and_checks_filter_metadata() {
        let message_id = MessageId::new();
        let mut envelope = CommandEnvelope {
            command_name: CommandNameOwned::new("transfer".to_owned()).unwrap(),
            command: SerializedCommand::new(serde_json::json!({"amount": 10})).unwrap(),
            message_id,
            correlation_id: CorrelationId::from(message_id.value()),
            causation_id: CausationId::from(message_id),
            options: CommandOptions::default(),
            saga_origin: None,
        };
        let source = "urn:banking:commands".parse().unwrap();
        let event = CommandCloudEventCodec::encode(&envelope, &source, None).unwrap();
        assert_eq!(event.event_type().as_str(), "transfer");
        assert!(event.subject().is_none());
        assert!(event.time().is_none());
        assert_eq!(
            CommandCloudEventCodec::decode(&event, None).unwrap(),
            envelope
        );
        envelope.saga_origin = Some(SagaCommandOrigin {
            saga_name: SagaNameOwned::new("transfer".to_owned()).unwrap(),
            saga_instance_id: SagaInstanceId::new(),
            step: SerializedSagaStep::try_from(serde_json::json!("debit")).unwrap(),
        });
        let mut with_origin = CommandCloudEventCodec::encode(&envelope, &source, None).unwrap();
        assert_eq!(
            CommandCloudEventCodec::decode(&with_origin, None).unwrap(),
            envelope
        );
        with_origin
            .insert_extension(
                "saganame".parse().unwrap(),
                CloudEventAttributeValue::String(("other".to_owned()).parse().unwrap()),
            )
            .unwrap();
        assert!(CommandCloudEventCodec::decode(&with_origin, None).is_err());
    }
}
