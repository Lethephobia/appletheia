use appletheia::application::unit_of_work::UnitOfWork;
use banking_ledger_domain::payout_destination::{PayoutDestinationId, PayoutDestinationOwner};

use super::PayoutDestinationRelationshipUpdaterError;

#[allow(async_fn_in_trait)]
pub trait PayoutDestinationRelationshipUpdater: Send + Sync {
    type Uow: UnitOfWork;

    async fn upsert_owner(
        &self,
        uow: &mut Self::Uow,
        payout_destination_id: PayoutDestinationId,
        owner: PayoutDestinationOwner,
    ) -> Result<(), PayoutDestinationRelationshipUpdaterError>;
}
