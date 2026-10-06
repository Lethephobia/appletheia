use crate::messaging::Selector as MessageSelector;

use super::{CommandFailureEnvelope, CommandName};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct CommandFailureSelector {
    pub command_name: CommandName,
}

impl CommandFailureSelector {
    pub const fn new(command_name: CommandName) -> Self {
        Self { command_name }
    }

    pub fn matches(&self, failure: &CommandFailureEnvelope) -> bool {
        failure.command_name.value() == self.command_name.value()
    }
}

impl MessageSelector<CommandFailureEnvelope> for CommandFailureSelector {
    fn matches(&self, message: &CommandFailureEnvelope) -> bool {
        self.matches(message)
    }
}
