use appletheia::application::event::EventSequence;
use appletheia::application::unit_of_work::UnitOfWork;
use banking_ledger_domain::core::CurrencyAmount;
use banking_ledger_domain::currency::{
    CurrencyId, CurrencyName, CurrencyOwner, CurrencyStatus, CurrencySymbol,
};

use super::{CurrencyViewStoreError, CurrencyViewUpsert};

/// Persists normalized currency views for query-side reads.
#[allow(async_fn_in_trait)]
pub trait CurrencyViewStore: Send + Sync {
    type Uow: UnitOfWork;

    async fn upsert(
        &self,
        uow: &mut Self::Uow,
        input: CurrencyViewUpsert,
        event_sequence: EventSequence,
    ) -> Result<(), CurrencyViewStoreError>;

    async fn update_owner(
        &self,
        uow: &mut Self::Uow,
        id: CurrencyId,
        owner: CurrencyOwner,
        event_sequence: EventSequence,
    ) -> Result<(), CurrencyViewStoreError>;

    async fn update_symbol(
        &self,
        uow: &mut Self::Uow,
        id: CurrencyId,
        symbol: CurrencySymbol,
        event_sequence: EventSequence,
    ) -> Result<(), CurrencyViewStoreError>;

    async fn update_name(
        &self,
        uow: &mut Self::Uow,
        id: CurrencyId,
        name: CurrencyName,
        event_sequence: EventSequence,
    ) -> Result<(), CurrencyViewStoreError>;

    async fn increase_supply(
        &self,
        uow: &mut Self::Uow,
        id: CurrencyId,
        amount: CurrencyAmount,
        event_sequence: EventSequence,
    ) -> Result<(), CurrencyViewStoreError>;

    async fn decrease_supply(
        &self,
        uow: &mut Self::Uow,
        id: CurrencyId,
        amount: CurrencyAmount,
        event_sequence: EventSequence,
    ) -> Result<(), CurrencyViewStoreError>;

    async fn update_status(
        &self,
        uow: &mut Self::Uow,
        id: CurrencyId,
        status: CurrencyStatus,
        event_sequence: EventSequence,
    ) -> Result<(), CurrencyViewStoreError>;

    async fn delete(
        &self,
        uow: &mut Self::Uow,
        id: CurrencyId,
        event_sequence: EventSequence,
    ) -> Result<(), CurrencyViewStoreError>;
}
