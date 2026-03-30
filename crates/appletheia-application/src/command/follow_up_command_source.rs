use crate::command::Command;

/// Wraps a typed command so `FollowUpCommand` conversion can use `TryFrom`.
#[derive(Clone, Copy, Debug)]
pub struct FollowUpCommandSource<'a, C>
where
    C: Command,
{
    command: &'a C,
}

impl<'a, C> FollowUpCommandSource<'a, C>
where
    C: Command,
{
    /// Creates a wrapper for a typed follow-up command source.
    pub fn new(command: &'a C) -> Self {
        Self { command }
    }

    /// Returns the wrapped typed command.
    pub fn command(&self) -> &'a C {
        self.command
    }
}
