use crate::command::{Command, CommandNameOwned, FollowUpCommandSource};
use crate::outbox::command::{CommandEnvelope, SerializedCommand};
use crate::request_context::{CausationId, MessageId, RequestContext};

use super::FollowUpCommandError;

/// Represents a serialized follow-up command prepared after a handled command failure.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FollowUpCommand {
    command_name: CommandNameOwned,
    serialized_command: SerializedCommand,
}

impl FollowUpCommand {
    /// Builds a `CommandEnvelope` using request-scoped causation metadata.
    pub fn into_command_envelope(self, request_context: &RequestContext) -> CommandEnvelope {
        CommandEnvelope {
            command_name: self.command_name,
            command: self.serialized_command,
            correlation_id: request_context.correlation_id,
            message_id: MessageId::new(),
            causation_id: CausationId::from(request_context.message_id),
        }
    }
}

impl<C> TryFrom<FollowUpCommandSource<'_, C>> for FollowUpCommand
where
    C: Command,
{
    type Error = FollowUpCommandError;

    fn try_from(source: FollowUpCommandSource<'_, C>) -> Result<Self, Self::Error> {
        let command = source.command();
        let serialized_command = SerializedCommand::new(serde_json::to_value(command)?)?;

        Ok(Self {
            command_name: CommandNameOwned::from(C::NAME),
            serialized_command,
        })
    }
}

#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};

    use super::FollowUpCommand;
    use crate::command::{Command, FollowUpCommandSource};
    use crate::request_context::{
        ActorRef, CausationId, CorrelationId, MessageId, Principal, RequestContext,
    };

    #[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
    struct TestCommand {
        value: String,
    }

    impl Command for TestCommand {
        const NAME: crate::command::CommandName = crate::command::CommandName::new("test");
    }

    #[test]
    fn try_from_command_serializes_command_name_and_payload() {
        let follow_up = FollowUpCommand::try_from(FollowUpCommandSource::new(&TestCommand {
            value: "ok".to_owned(),
        }))
        .expect("serialize");
        let request_context = RequestContext::new(
            CorrelationId::from(uuid::Uuid::now_v7()),
            MessageId::new(),
            ActorRef::System,
            Principal::System,
        );
        let envelope = follow_up.into_command_envelope(&request_context);

        assert_eq!(envelope.command_name.to_string(), "test");
        let decoded = envelope
            .try_into_command::<TestCommand>()
            .expect("command should deserialize");
        assert_eq!(
            decoded,
            TestCommand {
                value: "ok".to_owned()
            }
        );
        assert_eq!(envelope.correlation_id, request_context.correlation_id);
        assert_eq!(
            envelope.causation_id,
            CausationId::from(request_context.message_id)
        );
    }
}
