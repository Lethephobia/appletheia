use appletheia::application::unit_of_work::UnitOfWork;
use banking_ledger_domain::{CurrencyRegistrarId, CurrencyRegistrarInvitationId, UserId};

use super::CurrencyRegistrarInvitationRelationshipUpdaterError;

#[allow(async_fn_in_trait)]
pub trait CurrencyRegistrarInvitationRelationshipUpdater: Send + Sync {
    type Uow: UnitOfWork;

    async fn upsert_invitee(
        &self,
        uow: &mut Self::Uow,
        invitation_id: CurrencyRegistrarInvitationId,
        invitee_id: UserId,
    ) -> Result<(), CurrencyRegistrarInvitationRelationshipUpdaterError>;

    async fn upsert_registrar(
        &self,
        uow: &mut Self::Uow,
        invitation_id: CurrencyRegistrarInvitationId,
        registrar_id: CurrencyRegistrarId,
    ) -> Result<(), CurrencyRegistrarInvitationRelationshipUpdaterError>;
}
