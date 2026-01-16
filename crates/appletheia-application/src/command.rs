pub mod command_dispatch_error;
pub mod command_dispatcher;
pub mod command_failure_report;
pub mod command_handler;
pub mod command_hash;
pub mod command_hasher;
pub mod command_name;
pub mod command_name_owned;
pub mod command_name_owned_error;
pub mod command_outbox;
pub mod default_command_dispatcher;
pub mod default_command_hasher;

pub use command_dispatch_error::CommandDispatchError;
pub use command_dispatcher::CommandDispatcher;
pub use command_failure_report::CommandFailureReport;
pub use command_handler::CommandHandler;
pub use command_hash::{CommandHash, CommandHashError};
pub use command_hasher::CommandHasher;
pub use command_name::CommandName;
pub use command_name_owned::CommandNameOwned;
pub use command_name_owned_error::CommandNameOwnedError;
pub use command_outbox::*;
pub use default_command_dispatcher::DefaultCommandDispatcher;
pub use default_command_hasher::DefaultCommandHasher;

use serde::Serialize;

pub trait Command: Serialize + Send + 'static {
    const COMMAND_NAME: CommandName;
}
