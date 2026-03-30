use crate::command::{Command, FollowUpCommand, FollowUpCommandSource};
use crate::outbox::command::CommandEnvelope;
use crate::request_context::RequestContext;

use super::CommandFailureReactionError;

/// Describes follow-up work to schedule after a handled command failure.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CommandFailureReaction {
    None,
    FollowUpCommands(Vec<FollowUpCommand>),
}

impl CommandFailureReaction {
    /// Returns a reaction that performs no follow-up work.
    pub fn new() -> Self {
        Self::None
    }

    /// Builds a reaction containing a single follow-up command.
    pub fn with_command<C>(command: &C) -> Result<Self, CommandFailureReactionError>
    where
        C: Command,
    {
        Ok(Self::FollowUpCommands(vec![FollowUpCommand::try_from(
            FollowUpCommandSource::new(command),
        )?]))
    }

    /// Appends a typed follow-up command to this reaction.
    pub fn push_command<C>(&mut self, command: &C) -> Result<(), CommandFailureReactionError>
    where
        C: Command,
    {
        match self {
            Self::None => {
                *self = Self::FollowUpCommands(vec![FollowUpCommand::try_from(
                    FollowUpCommandSource::new(command),
                )?]);
            }
            Self::FollowUpCommands(commands) => {
                commands.push(FollowUpCommand::try_from(FollowUpCommandSource::new(
                    command,
                ))?);
            }
        }

        Ok(())
    }

    /// Returns the serialized follow-up commands when present.
    pub fn follow_up_commands(&self) -> &[FollowUpCommand] {
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

impl Default for CommandFailureReaction {
    fn default() -> Self {
        Self::None
    }
}

#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};

    use super::CommandFailureReaction;
    use crate::command::Command;
    use crate::request_context::{ActorRef, CorrelationId, MessageId, Principal, RequestContext};

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
        let mut reaction = CommandFailureReaction::with_command(&FirstCommand {}).expect("first");
        reaction.push_command(&SecondCommand {}).expect("second");
        let request_context = RequestContext::new(
            CorrelationId::from(uuid::Uuid::now_v7()),
            MessageId::new(),
            ActorRef::System,
            Principal::System,
        );

        let envelopes = reaction.into_command_envelopes(&request_context);

        assert_eq!(envelopes.len(), 2);
        assert_eq!(envelopes[0].command_name.to_string(), "first");
        assert_eq!(envelopes[1].command_name.to_string(), "second");
        assert_eq!(envelopes[0].correlation_id, request_context.correlation_id);
        assert_eq!(envelopes[1].correlation_id, request_context.correlation_id);
    }

    #[test]
    fn none_reaction_returns_no_envelopes() {
        let request_context = RequestContext::new(
            CorrelationId::from(uuid::Uuid::now_v7()),
            MessageId::new(),
            ActorRef::System,
            Principal::System,
        );

        let envelopes = CommandFailureReaction::None.into_command_envelopes(&request_context);

        assert!(envelopes.is_empty());
    }
}
