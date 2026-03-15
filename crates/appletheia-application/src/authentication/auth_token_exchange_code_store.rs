use crate::unit_of_work::UnitOfWork;

use super::{
    AuthTokenExchangeCodeHash, AuthTokenExchangeCodeRecord, AuthTokenExchangeCodeStoreError,
};

/// Persists and consumes auth token exchange codes.
#[allow(async_fn_in_trait)]
pub trait AuthTokenExchangeCodeStore: Send + Sync {
    /// The unit of work type used by the store.
    type Uow: UnitOfWork;

    /// Saves a newly issued exchange code record.
    async fn save(
        &self,
        uow: &mut Self::Uow,
        record: &AuthTokenExchangeCodeRecord,
    ) -> Result<(), AuthTokenExchangeCodeStoreError>;

    /// Atomically consumes the record associated with `code_hash`.
    async fn consume_by_code_hash(
        &self,
        uow: &mut Self::Uow,
        code_hash: &AuthTokenExchangeCodeHash,
    ) -> Result<AuthTokenExchangeCodeRecord, AuthTokenExchangeCodeStoreError>;
}
