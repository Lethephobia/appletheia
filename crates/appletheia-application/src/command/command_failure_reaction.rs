use crate::command::{Command, CommandRequest, CommandRequestOwned, CommandRequestOwnedError};
use crate::outbox::command::CommandEnvelope;
use crate::request_context::RequestContext;
use serde::{Deserialize, Serialize};

/// Describes follow-up work to schedule after a handled command failure.
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub enum CommandFailureReaction {
    #[default]
    None,
    FollowUpCommand(Box<CommandRequestOwned>),
}

impl CommandFailureReaction {
    /// Returns a reaction that performs no follow-up work.
    pub fn new() -> Self {
        Self::None
    }

    /// Builds a reaction that schedules one follow-up command.
    pub fn follow_up_command<C>(
        command_request: CommandRequest<C>,
    ) -> Result<Self, CommandRequestOwnedError>
    where
        C: Command,
    {
        let command = CommandRequestOwned::try_from_command(
            &command_request.command,
            command_request.options,
        )?;
        Ok(Self::FollowUpCommand(Box::new(command)))
    }

    /// Returns the serialized follow-up command when present.
    pub fn follow_up_command_ref(&self) -> Option<&CommandRequestOwned> {
        match self {
            Self::None => None,
            Self::FollowUpCommand(command) => Some(command.as_ref()),
        }
    }

    /// Converts the reaction into outbox-ready command envelopes.
    pub fn into_command_envelopes(self, request_context: &RequestContext) -> Vec<CommandEnvelope> {
        match self {
            Self::None => Vec::new(),
            Self::FollowUpCommand(command) => {
                vec![(*command).into_command_envelope(request_context)]
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};

    use super::CommandFailureReaction;
    use crate::command::{Command, CommandRequest};
    use crate::request_context::{CorrelationId, MessageId, Principal, RequestContext};

    #[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
    struct FollowUpCommand {}

    impl Command for FollowUpCommand {
        const NAME: crate::command::CommandName = crate::command::CommandName::new("follow_up");
    }

    #[test]
    fn builds_envelope_for_follow_up_command() {
        let reaction =
            CommandFailureReaction::follow_up_command(CommandRequest::new(FollowUpCommand {}))
                .expect("follow-up");
        let request_context = RequestContext::new(
            CorrelationId::from(uuid::Uuid::now_v7()),
            MessageId::new(),
            Principal::System,
        )
        .expect("request context should be valid");

        let envelopes = reaction.into_command_envelopes(&request_context);

        assert_eq!(envelopes.len(), 1);
        assert_eq!(envelopes[0].command_name.to_string(), "follow_up");
        assert_eq!(envelopes[0].correlation_id, request_context.correlation_id);
        assert_eq!(
            envelopes[0].options.failure_reaction,
            CommandFailureReaction::None
        );
    }

    #[test]
    fn none_reaction_returns_no_envelopes() {
        let request_context = RequestContext::new(
            CorrelationId::from(uuid::Uuid::now_v7()),
            MessageId::new(),
            Principal::System,
        )
        .expect("request context should be valid");

        let envelopes = CommandFailureReaction::None.into_command_envelopes(&request_context);

        assert!(envelopes.is_empty());
    }

    #[test]
    fn serializes_and_deserializes_follow_up_command() {
        let reaction =
            CommandFailureReaction::follow_up_command(CommandRequest::new(FollowUpCommand {}))
                .expect("follow-up");

        let json = serde_json::to_value(&reaction).expect("serialize");
        let decoded: CommandFailureReaction = serde_json::from_value(json).expect("deserialize");

        assert_eq!(decoded, reaction);
    }
}
