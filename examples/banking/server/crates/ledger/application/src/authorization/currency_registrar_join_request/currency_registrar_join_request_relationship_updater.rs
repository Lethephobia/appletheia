use appletheia::application::unit_of_work::UnitOfWork;
use banking_ledger_domain::{CurrencyRegistrarId, CurrencyRegistrarJoinRequestId, UserId};

use super::CurrencyRegistrarJoinRequestRelationshipUpdaterError;

#[allow(async_fn_in_trait)]
pub trait CurrencyRegistrarJoinRequestRelationshipUpdater: Send + Sync {
    type Uow: UnitOfWork;

    async fn upsert_registrar(
        &self,
        uow: &mut Self::Uow,
        join_request_id: CurrencyRegistrarJoinRequestId,
        registrar_id: CurrencyRegistrarId,
    ) -> Result<(), CurrencyRegistrarJoinRequestRelationshipUpdaterError>;

    async fn upsert_requester(
        &self,
        uow: &mut Self::Uow,
        join_request_id: CurrencyRegistrarJoinRequestId,
        requester_id: UserId,
    ) -> Result<(), CurrencyRegistrarJoinRequestRelationshipUpdaterError>;
}
