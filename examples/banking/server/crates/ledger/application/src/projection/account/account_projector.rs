use appletheia::application::event::EventEnvelope;
use appletheia::application::projection::Projector;
use banking_ledger_domain::account::{Account, AccountEventPayload, AccountStatus};
use banking_ledger_domain::core::CurrencyAmount;

use super::{AccountProjectorError, AccountProjectorSpec};
use crate::projection::{AccountProjectionStore, AccountProjectionUpsert};

/// Projects account events into normalized account projections.
pub struct AccountProjector<VS>
where
    VS: AccountProjectionStore,
{
    projection_store: VS,
}

impl<VS> AccountProjector<VS>
where
    VS: AccountProjectionStore,
{
    pub fn new(projection_store: VS) -> Self {
        Self { projection_store }
    }
}

impl<VS> Projector for AccountProjector<VS>
where
    VS: AccountProjectionStore,
{
    type Spec = AccountProjectorSpec;
    type Uow = VS::Uow;
    type Error = AccountProjectorError;

    async fn project(&self, uow: &mut Self::Uow, event: &EventEnvelope) -> Result<(), Self::Error> {
        let domain_event = event.try_into_domain_event::<Account>()?;
        let account_id = domain_event.aggregate_id();

        match domain_event.payload() {
            AccountEventPayload::Opened {
                owner,
                name,
                currency_id,
                ..
            } => {
                self.projection_store
                    .upsert(
                        uow,
                        AccountProjectionUpsert {
                            id: account_id,
                            owner: *owner,
                            name: name.clone(),
                            currency_id: *currency_id,
                            balance: CurrencyAmount::zero(),
                            reserved_balance: CurrencyAmount::zero(),
                            status: AccountStatus::Active,
                        },
                        event.event_sequence,
                    )
                    .await?;
            }
            AccountEventPayload::OwnershipTransferred { owner } => {
                self.projection_store
                    .update_owner(uow, account_id, *owner, event.event_sequence)
                    .await?;
            }
            AccountEventPayload::NameChanged { name } => {
                self.projection_store
                    .update_name(uow, account_id, name.clone(), event.event_sequence)
                    .await?;
            }
            AccountEventPayload::Deposited { amount } => {
                self.projection_store
                    .increase_balance(uow, account_id, *amount, event.event_sequence)
                    .await?;
            }
            AccountEventPayload::Withdrawn { amount } => {
                self.projection_store
                    .decrease_balance(uow, account_id, *amount, event.event_sequence)
                    .await?;
            }
            AccountEventPayload::FundsReserved { amount } => {
                self.projection_store
                    .move_balance_to_reserved(uow, account_id, *amount, event.event_sequence)
                    .await?;
            }
            AccountEventPayload::ReservedFundsReleased { amount } => {
                self.projection_store
                    .move_reserved_to_balance(uow, account_id, *amount, event.event_sequence)
                    .await?;
            }
            AccountEventPayload::ReservedFundsCommitted { amount } => {
                self.projection_store
                    .decrease_reserved(uow, account_id, *amount, event.event_sequence)
                    .await?;
            }
            AccountEventPayload::Frozen => {
                self.projection_store
                    .update_status(uow, account_id, AccountStatus::Frozen, event.event_sequence)
                    .await?;
            }
            AccountEventPayload::Thawed => {
                self.projection_store
                    .update_status(uow, account_id, AccountStatus::Active, event.event_sequence)
                    .await?;
            }
            AccountEventPayload::Closed => {
                self.projection_store
                    .update_status(uow, account_id, AccountStatus::Closed, event.event_sequence)
                    .await?;
            }
            AccountEventPayload::OwnershipTransferRejected { .. }
            | AccountEventPayload::NameChangeRejected { .. }
            | AccountEventPayload::DepositRejected { .. }
            | AccountEventPayload::WithdrawRejected { .. }
            | AccountEventPayload::FundsReserveRejected { .. }
            | AccountEventPayload::ReservedFundsReleaseRejected { .. }
            | AccountEventPayload::ReservedFundsCommitRejected { .. }
            | AccountEventPayload::FreezeRejected { .. }
            | AccountEventPayload::ThawRejected { .. }
            | AccountEventPayload::CloseRejected { .. } => {}
        }

        Ok(())
    }
}
