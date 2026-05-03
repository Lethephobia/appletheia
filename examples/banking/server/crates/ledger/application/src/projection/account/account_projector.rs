use appletheia::application::event::EventEnvelope;
use appletheia::application::projection::Projector;
use banking_ledger_domain::account::{Account, AccountEventPayload, AccountStatus};
use banking_ledger_domain::core::CurrencyAmount;

use super::{AccountProjectorError, AccountProjectorSpec};
use crate::view::{AccountViewStore, AccountViewUpsert};

/// Projects account events into normalized account views.
pub struct AccountProjector<VS>
where
    VS: AccountViewStore,
{
    view_store: VS,
}

impl<VS> AccountProjector<VS>
where
    VS: AccountViewStore,
{
    pub fn new(view_store: VS) -> Self {
        Self { view_store }
    }
}

impl<VS> Projector for AccountProjector<VS>
where
    VS: AccountViewStore,
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
                self.view_store
                    .upsert(
                        uow,
                        AccountViewUpsert {
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
                self.view_store
                    .update_owner(uow, account_id, *owner, event.event_sequence)
                    .await?;
            }
            AccountEventPayload::NameChanged { name } => {
                self.view_store
                    .update_name(uow, account_id, name.clone(), event.event_sequence)
                    .await?;
            }
            AccountEventPayload::Deposited { amount } => {
                self.view_store
                    .increase_balance(uow, account_id, *amount, event.event_sequence)
                    .await?;
            }
            AccountEventPayload::Withdrawn { amount } => {
                self.view_store
                    .decrease_balance(uow, account_id, *amount, event.event_sequence)
                    .await?;
            }
            AccountEventPayload::FundsReserved { amount } => {
                self.view_store
                    .move_balance_to_reserved(uow, account_id, *amount, event.event_sequence)
                    .await?;
            }
            AccountEventPayload::ReservedFundsReleased { amount } => {
                self.view_store
                    .move_reserved_to_balance(uow, account_id, *amount, event.event_sequence)
                    .await?;
            }
            AccountEventPayload::ReservedFundsCommitted { amount } => {
                self.view_store
                    .decrease_reserved(uow, account_id, *amount, event.event_sequence)
                    .await?;
            }
            AccountEventPayload::Frozen => {
                self.view_store
                    .update_status(uow, account_id, AccountStatus::Frozen, event.event_sequence)
                    .await?;
            }
            AccountEventPayload::Thawed => {
                self.view_store
                    .update_status(uow, account_id, AccountStatus::Active, event.event_sequence)
                    .await?;
            }
            AccountEventPayload::Closed => {
                self.view_store
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
