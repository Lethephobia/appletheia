pub mod default_query_dispatcher;
pub mod query_consistency;
pub mod query_dispatcher;
pub mod query_dispatcher_error;
pub mod query_handler;
pub mod query_name;
pub mod query_options;

pub use default_query_dispatcher::*;
pub use query_consistency::*;
pub use query_dispatcher::*;
pub use query_dispatcher_error::*;
pub use query_handler::*;
pub use query_name::*;
pub use query_options::*;

pub trait Query: Send + 'static {
    const NAME: QueryName;
}
