use crate::command::{Command, CommandRequest};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SagaTransition<Ctx, Cmd>
where
    Cmd: Command,
{
    pub context: Ctx,
    pub command: Option<CommandRequest<Cmd>>,
}

impl<Ctx, Cmd> SagaTransition<Ctx, Cmd>
where
    Cmd: Command,
{
    pub fn new(context: Ctx, command: CommandRequest<Cmd>) -> Self {
        Self {
            context,
            command: Some(command),
        }
    }

    pub fn no_command(context: Ctx) -> Self {
        Self {
            context,
            command: None,
        }
    }
}
