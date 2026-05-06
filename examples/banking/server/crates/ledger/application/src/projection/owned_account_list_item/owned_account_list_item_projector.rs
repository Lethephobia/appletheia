use appletheia::application::event::EventEnvelope;
use appletheia::application::projection::Projector;
use banking_ledger_domain::account::{Account, AccountEventPayload};
use banking_ledger_domain::core::CurrencyAmount;
use banking_ledger_domain::currency::{Currency, CurrencyEventPayload};

use super::{OwnedAccountListItemProjectorError, OwnedAccountListItemProjectorSpec};
use crate::read_model::{OwnedAccountListItemStatus, OwnedAccountListItemWriter};

/// Projects account and currency events into owned account list item read models.
pub struct OwnedAccountListItemProjector<W>
where
    W: OwnedAccountListItemWriter,
{
    writer: W,
}

impl<W> OwnedAccountListItemProjector<W>
where
    W: OwnedAccountListItemWriter,
{
    pub fn new(writer: W) -> Self {
        Self { writer }
    }
}

impl<W> Projector for OwnedAccountListItemProjector<W>
where
    W: OwnedAccountListItemWriter,
{
    type Spec = OwnedAccountListItemProjectorSpec;
    type Uow = W::Uow;
    type Error = OwnedAccountListItemProjectorError;

    async fn project(&self, uow: &mut Self::Uow, event: &EventEnvelope) -> Result<(), Self::Error> {
        if event.is_for_aggregate::<Account>() {
            let domain_event = event.try_into_domain_event::<Account>()?;
            let account_id = domain_event.aggregate_id();

            match domain_event.payload() {
                AccountEventPayload::Opened {
                    owner,
                    name,
                    currency_id,
                    ..
                } => {
                    self.writer
                        .upsert_account(
                            uow,
                            account_id,
                            *owner,
                            name.clone(),
                            *currency_id,
                            CurrencyAmount::zero(),
                            CurrencyAmount::zero(),
                            OwnedAccountListItemStatus::Active,
                            event.event_sequence,
                            event.occurred_at,
                        )
                        .await?;
                }
                AccountEventPayload::OwnershipTransferred { owner } => {
                    self.writer
                        .update_account_owner(
                            uow,
                            account_id,
                            *owner,
                            event.event_sequence,
                            event.occurred_at,
                        )
                        .await?;
                }
                AccountEventPayload::NameChanged { name } => {
                    self.writer
                        .update_account_name(
                            uow,
                            account_id,
                            name.clone(),
                            event.event_sequence,
                            event.occurred_at,
                        )
                        .await?;
                }
                AccountEventPayload::Deposited { amount } => {
                    self.writer
                        .increase_balance(
                            uow,
                            account_id,
                            *amount,
                            event.event_sequence,
                            event.occurred_at,
                        )
                        .await?;
                }
                AccountEventPayload::Withdrawn { amount } => {
                    self.writer
                        .decrease_balance(
                            uow,
                            account_id,
                            *amount,
                            event.event_sequence,
                            event.occurred_at,
                        )
                        .await?;
                }
                AccountEventPayload::FundsReserved { amount } => {
                    self.writer
                        .reserve_balance(
                            uow,
                            account_id,
                            *amount,
                            event.event_sequence,
                            event.occurred_at,
                        )
                        .await?;
                }
                AccountEventPayload::ReservedFundsReleased { amount } => {
                    self.writer
                        .release_reserved_balance(
                            uow,
                            account_id,
                            *amount,
                            event.event_sequence,
                            event.occurred_at,
                        )
                        .await?;
                }
                AccountEventPayload::ReservedFundsCommitted { amount } => {
                    self.writer
                        .commit_reserved_balance(
                            uow,
                            account_id,
                            *amount,
                            event.event_sequence,
                            event.occurred_at,
                        )
                        .await?;
                }
                AccountEventPayload::Frozen => {
                    self.writer
                        .update_account_status(
                            uow,
                            account_id,
                            OwnedAccountListItemStatus::Frozen,
                            event.event_sequence,
                            event.occurred_at,
                        )
                        .await?;
                }
                AccountEventPayload::Thawed => {
                    self.writer
                        .update_account_status(
                            uow,
                            account_id,
                            OwnedAccountListItemStatus::Active,
                            event.event_sequence,
                            event.occurred_at,
                        )
                        .await?;
                }
                AccountEventPayload::Closed => {
                    self.writer
                        .delete_account(uow, account_id, event.event_sequence, event.occurred_at)
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

            return Ok(());
        }

        let domain_event = event.try_into_domain_event::<Currency>()?;
        let currency_id = domain_event.aggregate_id();

        match domain_event.payload() {
            CurrencyEventPayload::Defined {
                symbol,
                name,
                decimals,
                ..
            } => {
                self.writer
                    .upsert_currency(
                        uow,
                        currency_id,
                        symbol.clone(),
                        name.clone(),
                        *decimals,
                        event.event_sequence,
                        event.occurred_at,
                    )
                    .await?;
            }
            CurrencyEventPayload::SymbolChanged { symbol } => {
                self.writer
                    .update_currency_symbol(
                        uow,
                        currency_id,
                        symbol.clone(),
                        event.event_sequence,
                        event.occurred_at,
                    )
                    .await?;
            }
            CurrencyEventPayload::NameChanged { name } => {
                self.writer
                    .update_currency_name(
                        uow,
                        currency_id,
                        name.clone(),
                        event.event_sequence,
                        event.occurred_at,
                    )
                    .await?;
            }
            CurrencyEventPayload::Removed => {
                self.writer
                    .delete_currency(uow, currency_id, event.event_sequence, event.occurred_at)
                    .await?;
            }
            CurrencyEventPayload::OwnershipTransferred { .. }
            | CurrencyEventPayload::OwnershipTransferRejected { .. }
            | CurrencyEventPayload::SymbolChangeRejected { .. }
            | CurrencyEventPayload::NameChangeRejected { .. }
            | CurrencyEventPayload::SupplyIncreased { .. }
            | CurrencyEventPayload::SupplyIncreaseRejected { .. }
            | CurrencyEventPayload::SupplyDecreased { .. }
            | CurrencyEventPayload::SupplyDecreaseRejected { .. }
            | CurrencyEventPayload::Activated
            | CurrencyEventPayload::ActivateRejected { .. }
            | CurrencyEventPayload::Deactivated
            | CurrencyEventPayload::DeactivateRejected { .. }
            | CurrencyEventPayload::RemoveRejected { .. } => {}
        }

        Ok(())
    }
}
