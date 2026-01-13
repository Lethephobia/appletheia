pub mod command_dispatch_error;
pub mod command_dispatcher;
pub mod command_failure_report;
pub mod command_handler;
pub mod command_name;
pub mod default_command_dispatcher;
pub mod default_request_hasher;
pub mod request_hasher;

pub use command_dispatch_error::CommandDispatchError;
pub use command_dispatcher::CommandDispatcher;
pub use command_failure_report::CommandFailureReport;
pub use command_handler::CommandHandler;
pub use command_name::CommandName;
pub use default_command_dispatcher::DefaultCommandDispatcher;
pub use default_request_hasher::DefaultRequestHasher;
pub use request_hasher::RequestHasher;

use serde::Serialize;

pub trait Command: Serialize + Send + 'static {
    const COMMAND_NAME: CommandName;
}
