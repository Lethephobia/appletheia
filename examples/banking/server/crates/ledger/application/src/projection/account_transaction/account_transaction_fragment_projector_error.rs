use appletheia::application::event::EventEnvelopeError;
use appletheia::application::read_model::ReadModelFragmentChangeError;
use thiserror::Error;

use crate::projection::AccountTransactionFragmentWriterError;

/// Error returned while projecting account transaction fragments.
#[derive(Debug, Error)]
pub enum AccountTransactionFragmentProjectorError {
    #[error(transparent)]
    EventEnvelope(#[from] EventEnvelopeError),

    #[error(transparent)]
    Writer(#[from] AccountTransactionFragmentWriterError),

    #[error(transparent)]
    FragmentChange(#[from] ReadModelFragmentChangeError),
}
