pub mod default_query_dispatcher;
pub mod pagination;
pub mod query_dispatcher;
pub mod query_dispatcher_error;
pub mod query_handler;
pub mod query_name;

pub use default_query_dispatcher::*;
pub use query_dispatcher::*;
pub use query_dispatcher_error::*;
pub use query_handler::*;
pub use query_name::*;

pub trait Query: Send + 'static {
    const NAME: QueryName;
}
