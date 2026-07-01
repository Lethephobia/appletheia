use serde::{Deserialize, Serialize};

/// Describes progress for the currency old image object deletion saga.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum CurrencyOldImageObjectDeletionSagaStatus {
    DeleteRequested,
    Skipped,
}
