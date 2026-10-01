pub mod pagination;

mod materialization_event_context;
mod read_model_fragment;
mod read_model_fragment_name;
mod read_model_fragment_name_owned;
mod read_model_fragment_name_owned_error;
mod read_model_name;
mod read_model_name_owned;
mod read_model_name_owned_error;

pub use materialization_event_context::*;
pub use read_model_fragment::*;
pub use read_model_fragment_name::*;
pub use read_model_fragment_name_owned::*;
pub use read_model_fragment_name_owned_error::*;
pub use read_model_name::*;
pub use read_model_name_owned::*;
pub use read_model_name_owned_error::*;

/// Defines a serializable read model returned by a query.
pub trait ReadModel: serde::Serialize + Send + Sync {
    /// Identifies the read model.
    const NAME: ReadModelName;
}

impl<R> ReadModel for Option<R>
where
    R: ReadModel,
{
    const NAME: ReadModelName = R::NAME;
}
