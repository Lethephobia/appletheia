#![allow(dead_code, unused_imports)]

use appletheia_application::command::{Command, CommandName};
use appletheia_macros::command;
use serde::{Deserialize, Serialize};

#[command(name = "logout")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
struct LogoutCommand {
    token_id: String,
}

fn assert_command<T: Command>() {}

fn main() {
    assert_command::<LogoutCommand>();
    let _: CommandName = LogoutCommand::NAME;
}
