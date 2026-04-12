use crate::command::{Command, CommandFailureReaction, CommandOptions, CommandRequestOwnedError};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CommandRequest<C: Command> {
    pub command: C,
    pub options: CommandOptions,
}

impl<C: Command> CommandRequest<C> {
    pub fn new(command: C) -> Self {
        Self {
            command,
            options: CommandOptions::default(),
        }
    }

    pub fn with_options(command: C, options: CommandOptions) -> Self {
        Self { command, options }
    }

    pub fn with_failure_follow_up<F>(
        command: C,
        follow_up: CommandRequest<F>,
    ) -> Result<Self, CommandRequestOwnedError>
    where
        F: Command,
    {
        Ok(Self::with_options(
            command,
            CommandOptions {
                failure_reaction: CommandFailureReaction::follow_up_command(follow_up)?,
                ..CommandOptions::default()
            },
        ))
    }
}

#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};

    use super::CommandRequest;
    use crate::command::{Command, CommandFailureReaction, CommandOptions};

    #[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
    struct TestCommand {
        value: String,
    }

    impl Command for TestCommand {
        const NAME: crate::command::CommandName = crate::command::CommandName::new("test");
    }

    #[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
    struct FollowUpCommand {
        value: String,
    }

    impl Command for FollowUpCommand {
        const NAME: crate::command::CommandName = crate::command::CommandName::new("follow_up");
    }

    #[test]
    fn with_failure_follow_up_sets_failure_reaction() {
        let follow_up = CommandRequest::new(FollowUpCommand {
            value: "rollback".to_owned(),
        });

        let request = CommandRequest::with_failure_follow_up(
            TestCommand {
                value: "run".to_owned(),
            },
            follow_up,
        )
        .expect("follow-up should serialize");

        assert_eq!(
            request,
            CommandRequest::with_options(
                TestCommand {
                    value: "run".to_owned(),
                },
                CommandOptions {
                    failure_reaction: CommandFailureReaction::follow_up_command(
                        CommandRequest::new(FollowUpCommand {
                            value: "rollback".to_owned(),
                        }),
                    )
                    .expect("follow-up should serialize"),
                    ..CommandOptions::default()
                },
            )
        );
    }
}
