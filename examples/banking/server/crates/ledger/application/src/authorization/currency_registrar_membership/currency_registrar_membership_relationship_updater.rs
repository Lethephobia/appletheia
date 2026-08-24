use appletheia::application::unit_of_work::UnitOfWork;
use banking_iam_domain::UserId;
use banking_ledger_domain::currency_registrar::CurrencyRegistrarId;
use banking_ledger_domain::currency_registrar_membership::CurrencyRegistrarMembershipId;

use super::CurrencyRegistrarMembershipRelationshipUpdaterError;

#[allow(async_fn_in_trait)]
pub trait CurrencyRegistrarMembershipRelationshipUpdater: Send + Sync {
    type Uow: UnitOfWork;

    async fn upsert_registrar(
        &self,
        uow: &mut Self::Uow,
        currency_registrar_membership_id: CurrencyRegistrarMembershipId,
        currency_registrar_id: CurrencyRegistrarId,
    ) -> Result<(), CurrencyRegistrarMembershipRelationshipUpdaterError>;

    async fn upsert_member(
        &self,
        uow: &mut Self::Uow,
        currency_registrar_id: CurrencyRegistrarId,
        user_id: UserId,
    ) -> Result<(), CurrencyRegistrarMembershipRelationshipUpdaterError>;

    async fn remove_member(
        &self,
        uow: &mut Self::Uow,
        currency_registrar_id: CurrencyRegistrarId,
        user_id: UserId,
    ) -> Result<(), CurrencyRegistrarMembershipRelationshipUpdaterError>;
}
