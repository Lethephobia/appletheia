pub mod default_query_dispatcher;
pub mod query_consistency;
pub mod query_dispatcher;
pub mod query_dispatcher_error;
pub mod query_handler;
pub mod query_name;

pub use default_query_dispatcher::DefaultQueryDispatcher;
pub use query_consistency::QueryConsistency;
pub use query_dispatcher::QueryDispatcher;
pub use query_dispatcher::QueryOptions;
pub use query_dispatcher_error::QueryDispatcherError;
pub use query_handler::QueryHandler;
pub use query_name::QueryName;

pub trait Query: Send + 'static {
    const NAME: QueryName;
}
