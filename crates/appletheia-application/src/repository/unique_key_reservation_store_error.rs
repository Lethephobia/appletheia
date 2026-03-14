use std::error::Error;

use appletheia_domain::aggregate::{AggregateType, UniqueKey, UniqueValue};
use thiserror::Error;

/// Errors returned by `UniqueKeyReservationStore`.
#[derive(Debug, Error)]
pub enum UniqueKeyReservationStoreError {
    #[error(
        "unique key is already reserved: aggregate_type={aggregate_type}, key={namespace}, value={normalized_key}"
    )]
    Conflict {
        aggregate_type: AggregateType,
        namespace: UniqueKey,
        normalized_key: String,
    },

    #[error("unique key mismatch: expected={expected}, actual={actual}")]
    NamespaceMismatch {
        expected: UniqueKey,
        actual: UniqueKey,
    },

    #[error("duplicate unique key in request: key={namespace}, value={normalized_key}")]
    DuplicateKey {
        namespace: UniqueKey,
        normalized_key: String,
    },

    #[error("persistence error: {0}")]
    Persistence(#[source] Box<dyn Error + Send + Sync + 'static>),
}

impl UniqueKeyReservationStoreError {
    /// Creates a conflict error for the given aggregate-type / unique-key / value triplet.
    pub fn conflict(
        aggregate_type: AggregateType,
        namespace: UniqueKey,
        value: &UniqueValue,
    ) -> Self {
        Self::Conflict {
            aggregate_type,
            namespace,
            normalized_key: value.normalized_key(),
        }
    }

    /// Creates a duplicate-key error for the given key.
    pub fn duplicate_key(namespace: UniqueKey, value: &UniqueValue) -> Self {
        Self::DuplicateKey {
            namespace,
            normalized_key: value.normalized_key(),
        }
    }
}
