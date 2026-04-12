use crate::command::{Command, CommandNameOwned, CommandOptions};
use crate::outbox::command::{CommandEnvelope, SerializedCommand};
use crate::request_context::{CausationId, MessageId, RequestContext};
use serde::{Deserialize, Serialize};

use super::CommandRequestOwnedError;

/// Represents an owned, serialized command request with execution options.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct CommandRequestOwned {
    command_name: CommandNameOwned,
    serialized_command: SerializedCommand,
    options: CommandOptions,
}

impl CommandRequestOwned {
    pub fn try_from_command<C>(
        command: &C,
        options: CommandOptions,
    ) -> Result<Self, CommandRequestOwnedError>
    where
        C: Command,
    {
        Ok(Self {
            command_name: CommandNameOwned::from(C::NAME),
            serialized_command: SerializedCommand::new(serde_json::to_value(command)?)?,
            options,
        })
    }

    /// Builds a `CommandEnvelope` using request-scoped causation metadata.
    pub fn into_command_envelope(self, request_context: &RequestContext) -> CommandEnvelope {
        CommandEnvelope {
            command_name: self.command_name,
            command: self.serialized_command,
            correlation_id: request_context.correlation_id,
            message_id: MessageId::new(),
            causation_id: CausationId::from(request_context.message_id),
            options: self.options,
        }
    }
}

#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};

    use super::CommandRequestOwned;
    use crate::command::{Command, CommandFailureReaction, CommandOptions, CommandRequest};
    use crate::request_context::{
        CausationId, CorrelationId, MessageId, Principal, RequestContext,
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
        let command_request_owned = CommandRequestOwned::try_from_command(
            &TestCommand {
                value: "ok".to_owned(),
            },
            CommandOptions::default(),
        )
        .expect("serialize");
        let request_context = RequestContext::new(
            CorrelationId::from(uuid::Uuid::now_v7()),
            MessageId::new(),
            Principal::System,
        )
        .expect("request context should be valid");
        let envelope = command_request_owned.into_command_envelope(&request_context);

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
        assert_eq!(
            envelope.options.failure_reaction,
            CommandFailureReaction::None
        );
    }

    #[test]
    fn new_preserves_failure_reaction() {
        let failure_reaction =
            CommandFailureReaction::follow_up_command(CommandRequest::new(TestCommand {
                value: "follow".to_owned(),
            }))
            .expect("reaction should serialize");
        let command_request_owned = CommandRequestOwned::try_from_command(
            &TestCommand {
                value: "primary".to_owned(),
            },
            CommandOptions {
                failure_reaction: failure_reaction.clone(),
                ..CommandOptions::default()
            },
        )
        .expect("follow-up should serialize");
        let request_context = RequestContext::new(
            CorrelationId::from(uuid::Uuid::now_v7()),
            MessageId::new(),
            Principal::System,
        )
        .expect("request context should be valid");

        let envelope = command_request_owned.into_command_envelope(&request_context);

        assert_eq!(envelope.options.failure_reaction, failure_reaction);
    }
}
