use crate::command::{Command, CommandOptions, CommandOwned, CommandOwnedError};
use crate::outbox::command::CommandEnvelope;
use crate::request_context::RequestContext;
use serde::{Deserialize, Serialize};

/// Describes follow-up work to schedule after a handled command failure.
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub enum CommandFailureReaction {
    #[default]
    None,
    FollowUpCommands(Vec<CommandOwned>),
}

impl CommandFailureReaction {
    /// Returns a reaction that performs no follow-up work.
    pub fn new() -> Self {
        Self::None
    }

    /// Appends a follow-up command to this reaction.
    pub fn push<C>(&mut self, command: &C, options: CommandOptions) -> Result<(), CommandOwnedError>
    where
        C: Command,
    {
        let command = CommandOwned::try_from_command(command, options)?;
        match self {
            Self::None => {
                *self = Self::FollowUpCommands(vec![command]);
            }
            Self::FollowUpCommands(commands) => {
                commands.push(command);
            }
        }

        Ok(())
    }

    /// Returns the serialized follow-up commands when present.
    pub fn follow_up_commands(&self) -> &[CommandOwned] {
        match self {
            Self::None => &[],
            Self::FollowUpCommands(commands) => commands,
        }
    }

    /// Converts the reaction into outbox-ready command envelopes.
    pub fn into_command_envelopes(self, request_context: &RequestContext) -> Vec<CommandEnvelope> {
        match self {
            Self::None => Vec::new(),
            Self::FollowUpCommands(commands) => commands
                .into_iter()
                .map(|command| command.into_command_envelope(request_context))
                .collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};

    use super::CommandFailureReaction;
    use crate::command::{Command, CommandOptions};
    use crate::request_context::{CorrelationId, MessageId, Principal, RequestContext};

    #[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
    struct FirstCommand {}

    impl Command for FirstCommand {
        const NAME: crate::command::CommandName = crate::command::CommandName::new("first");
    }

    #[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
    struct SecondCommand {}

    impl Command for SecondCommand {
        const NAME: crate::command::CommandName = crate::command::CommandName::new("second");
    }

    #[test]
    fn builds_envelopes_for_multiple_follow_up_commands() {
        let mut reaction = CommandFailureReaction::new();
        reaction
            .push(&FirstCommand {}, CommandOptions::default())
            .expect("first");
        reaction
            .push(&SecondCommand {}, CommandOptions::default())
            .expect("second");
        let request_context = RequestContext::new(
            CorrelationId::from(uuid::Uuid::now_v7()),
            MessageId::new(),
            Principal::System,
        )
        .expect("request context should be valid");

        let envelopes = reaction.into_command_envelopes(&request_context);

        assert_eq!(envelopes.len(), 2);
        assert_eq!(envelopes[0].command_name.to_string(), "first");
        assert_eq!(envelopes[1].command_name.to_string(), "second");
        assert_eq!(envelopes[0].correlation_id, request_context.correlation_id);
        assert_eq!(envelopes[1].correlation_id, request_context.correlation_id);
        assert_eq!(
            envelopes[0].options.failure_reaction,
            CommandFailureReaction::None
        );
        assert_eq!(
            envelopes[1].options.failure_reaction,
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
    fn serializes_and_deserializes_follow_up_commands() {
        let mut reaction = CommandFailureReaction::new();
        reaction
            .push(&FirstCommand {}, CommandOptions::default())
            .expect("first");

        let json = serde_json::to_value(&reaction).expect("serialize");
        let decoded: CommandFailureReaction = serde_json::from_value(json).expect("deserialize");

        assert_eq!(decoded, reaction);
    }
}
