use thiserror::Error;

#[derive(Debug, Error)]
pub enum CurrencyOldImageObjectDeletionSagaError {
    #[error("unexpected currency old image object deletion saga event")]
    UnexpectedEvent,
}
