pub mod default_query_dispatcher;
pub mod query_consistency;
pub mod query_dispatch_error;
pub mod query_dispatcher;
pub mod query_handler;
pub mod query_name;
pub mod read_model;
pub mod read_your_writes_poll_interval;
pub mod read_your_writes_poll_interval_error;
pub mod read_your_writes_timeout;
pub mod read_your_writes_timeout_error;

pub use default_query_dispatcher::DefaultQueryDispatcher;
pub use query_consistency::QueryConsistency;
pub use query_dispatch_error::QueryDispatchError;
pub use query_dispatcher::QueryDispatcher;
pub use query_dispatcher::QueryOptions;
pub use query_handler::QueryHandler;
pub use query_name::QueryName;
pub use read_model::ReadModel;
pub use read_your_writes_poll_interval::ReadYourWritesPollInterval;
pub use read_your_writes_poll_interval_error::ReadYourWritesPollIntervalError;
pub use read_your_writes_timeout::ReadYourWritesTimeout;
pub use read_your_writes_timeout_error::ReadYourWritesTimeoutError;

pub trait Query: Send + 'static {
    const NAME: QueryName;
}
