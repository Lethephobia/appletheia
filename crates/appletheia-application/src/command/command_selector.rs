use crate::command::CommandName;
use crate::messaging::Selector as MessageSelector;
use crate::outbox::command::CommandEnvelope;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct CommandSelector {
    pub command_name: CommandName,
}

impl CommandSelector {
    pub const fn new(command_name: CommandName) -> Self {
        Self { command_name }
    }

    pub fn matches(&self, command: &CommandEnvelope) -> bool {
        command.command_name.value() == self.command_name.value()
    }
}

impl MessageSelector<CommandEnvelope> for CommandSelector {
    fn matches(&self, message: &CommandEnvelope) -> bool {
        message.command_name.value() == self.command_name.value()
    }
}
