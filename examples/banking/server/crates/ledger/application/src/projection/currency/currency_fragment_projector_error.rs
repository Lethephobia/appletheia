use appletheia::application::event::EventEnvelopeError;
use appletheia::application::read_model::ReadModelFragmentChangeError;
use thiserror::Error;

use crate::projection::{CurrencyFragmentWriterError, MaterializedCurrencyStatusError};

/// Error returned while projecting currency fragments.
#[derive(Debug, Error)]
pub enum CurrencyFragmentProjectorError {
    #[error("currency event envelope failed")]
    EventEnvelope(#[from] EventEnvelopeError),

    #[error("currency fragment writer failed")]
    Writer(#[from] CurrencyFragmentWriterError),

    #[error(transparent)]
    Status(#[from] MaterializedCurrencyStatusError),

    #[error(transparent)]
    FragmentChange(#[from] ReadModelFragmentChangeError),
}
