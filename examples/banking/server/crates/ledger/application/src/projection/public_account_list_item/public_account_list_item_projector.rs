use appletheia::application::event::EventEnvelope;
use appletheia::application::projection::Projector;
use banking_ledger_domain::account::{Account, AccountEventPayload};
use banking_ledger_domain::currency::{Currency, CurrencyEventPayload};

use super::{PublicAccountListItemProjectorError, PublicAccountListItemProjectorSpec};
use crate::read_model::{PublicAccountListItemStatus, PublicAccountListItemWriter};

/// Projects account and currency events into public account list item read models.
pub struct PublicAccountListItemProjector<W>
where
    W: PublicAccountListItemWriter,
{
    writer: W,
}

impl<W> PublicAccountListItemProjector<W>
where
    W: PublicAccountListItemWriter,
{
    pub fn new(writer: W) -> Self {
        Self { writer }
    }
}

impl<W> Projector for PublicAccountListItemProjector<W>
where
    W: PublicAccountListItemWriter,
{
    type Spec = PublicAccountListItemProjectorSpec;
    type Uow = W::Uow;
    type Error = PublicAccountListItemProjectorError;

    async fn project(&self, uow: &mut Self::Uow, event: &EventEnvelope) -> Result<(), Self::Error> {
        if event.is_for_aggregate::<Account>() {
            let domain_event = event.try_into_domain_event::<Account>()?;
            let account_id = domain_event.aggregate_id();

            match domain_event.payload() {
                AccountEventPayload::Opened {
                    owner, currency_id, ..
                } => {
                    self.writer
                        .upsert_account(
                            uow,
                            account_id,
                            *owner,
                            *currency_id,
                            PublicAccountListItemStatus::Active,
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
                AccountEventPayload::Frozen => {
                    self.writer
                        .update_account_status(
                            uow,
                            account_id,
                            PublicAccountListItemStatus::Frozen,
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
                            PublicAccountListItemStatus::Active,
                            event.event_sequence,
                            event.occurred_at,
                        )
                        .await?;
                }
                AccountEventPayload::Closed => {
                    self.writer
                        .delete_account(uow, account_id, event.event_sequence)
                        .await?;
                }
                AccountEventPayload::OwnershipTransferRejected { .. }
                | AccountEventPayload::NameChanged { .. }
                | AccountEventPayload::NameChangeRejected { .. }
                | AccountEventPayload::Deposited { .. }
                | AccountEventPayload::DepositRejected { .. }
                | AccountEventPayload::Withdrawn { .. }
                | AccountEventPayload::WithdrawRejected { .. }
                | AccountEventPayload::FundsReserved { .. }
                | AccountEventPayload::FundsReserveRejected { .. }
                | AccountEventPayload::ReservedFundsReleased { .. }
                | AccountEventPayload::ReservedFundsReleaseRejected { .. }
                | AccountEventPayload::ReservedFundsCommitted { .. }
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
                    .delete_currency(uow, currency_id, event.event_sequence)
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
