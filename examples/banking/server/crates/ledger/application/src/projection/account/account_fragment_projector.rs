use appletheia::application::event::EventEnvelope;
use appletheia::application::projection::Projector;
use appletheia::application::read_model::{MaterializationEventContext, ReadModelFragmentChange};
use banking_ledger_domain::account::{Account, AccountEventPayload};
use banking_ledger_domain::core::CurrencyAmount;

use super::{AccountFragmentProjectorError, AccountFragmentProjectorSpec};
use crate::projection::{
    AccountFragment, AccountFragmentUpsert, AccountFragmentWriter, MaterializedAccountStatus,
};

/// Projects account events into account fragments.
pub struct AccountFragmentProjector<W>
where
    W: AccountFragmentWriter,
{
    account_fragment_writer: W,
}

impl<W> AccountFragmentProjector<W>
where
    W: AccountFragmentWriter,
{
    pub fn new(account_fragment_writer: W) -> Self {
        Self {
            account_fragment_writer,
        }
    }
}

impl<W> Projector for AccountFragmentProjector<W>
where
    W: AccountFragmentWriter,
{
    type Spec = AccountFragmentProjectorSpec;
    type Fragment = AccountFragment;
    type Uow = W::Uow;
    type Error = AccountFragmentProjectorError;

    async fn project(
        &self,
        uow: &mut Self::Uow,
        event_context: MaterializationEventContext,
        event: &EventEnvelope,
    ) -> Result<Vec<ReadModelFragmentChange<Self::Fragment>>, Self::Error> {
        let mut fragment_changes = Vec::new();
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
                    if let Some(fragment) = self
                        .account_fragment_writer
                        .upsert_account(
                            uow,
                            event_context,
                            AccountFragmentUpsert {
                                id: account_id,
                                owner: *owner,
                                name: name.clone(),
                                currency_id: *currency_id,
                                balance: CurrencyAmount::zero(),
                                reserved_balance: CurrencyAmount::zero(),
                                status: MaterializedAccountStatus::Active,
                            },
                        )
                        .await?
                    {
                        fragment_changes
                            .push(ReadModelFragmentChange::try_from_fragment(&fragment)?);
                    }
                }
                AccountEventPayload::OwnershipTransferred { owner } => {
                    if let Some(fragment) = self
                        .account_fragment_writer
                        .update_account_owner(uow, event_context, account_id, *owner)
                        .await?
                    {
                        fragment_changes
                            .push(ReadModelFragmentChange::try_from_fragment(&fragment)?);
                    }
                }
                AccountEventPayload::NameChanged { name } => {
                    if let Some(fragment) = self
                        .account_fragment_writer
                        .update_account_name(uow, event_context, account_id, name.clone())
                        .await?
                    {
                        fragment_changes
                            .push(ReadModelFragmentChange::try_from_fragment(&fragment)?);
                    }
                }
                AccountEventPayload::Deposited { amount } => {
                    if let Some(fragment) = self
                        .account_fragment_writer
                        .increase_balance(uow, event_context, account_id, *amount)
                        .await?
                    {
                        fragment_changes
                            .push(ReadModelFragmentChange::try_from_fragment(&fragment)?);
                    }
                }
                AccountEventPayload::Withdrawn { amount } => {
                    if let Some(fragment) = self
                        .account_fragment_writer
                        .decrease_balance(uow, event_context, account_id, *amount)
                        .await?
                    {
                        fragment_changes
                            .push(ReadModelFragmentChange::try_from_fragment(&fragment)?);
                    }
                }
                AccountEventPayload::FundsReserved { amount } => {
                    if let Some(fragment) = self
                        .account_fragment_writer
                        .reserve_balance(uow, event_context, account_id, *amount)
                        .await?
                    {
                        fragment_changes
                            .push(ReadModelFragmentChange::try_from_fragment(&fragment)?);
                    }
                }
                AccountEventPayload::ReservedFundsReleased { amount } => {
                    if let Some(fragment) = self
                        .account_fragment_writer
                        .release_reserved_balance(uow, event_context, account_id, *amount)
                        .await?
                    {
                        fragment_changes
                            .push(ReadModelFragmentChange::try_from_fragment(&fragment)?);
                    }
                }
                AccountEventPayload::ReservedFundsCommitted { amount } => {
                    if let Some(fragment) = self
                        .account_fragment_writer
                        .commit_reserved_balance(uow, event_context, account_id, *amount)
                        .await?
                    {
                        fragment_changes
                            .push(ReadModelFragmentChange::try_from_fragment(&fragment)?);
                    }
                }
                AccountEventPayload::Frozen => {
                    if let Some(fragment) = self
                        .account_fragment_writer
                        .update_account_status(
                            uow,
                            event_context,
                            account_id,
                            MaterializedAccountStatus::Frozen,
                        )
                        .await?
                    {
                        fragment_changes
                            .push(ReadModelFragmentChange::try_from_fragment(&fragment)?);
                    }
                }
                AccountEventPayload::Thawed => {
                    if let Some(fragment) = self
                        .account_fragment_writer
                        .update_account_status(
                            uow,
                            event_context,
                            account_id,
                            MaterializedAccountStatus::Active,
                        )
                        .await?
                    {
                        fragment_changes
                            .push(ReadModelFragmentChange::try_from_fragment(&fragment)?);
                    }
                }
                AccountEventPayload::Closed => {
                    if self
                        .account_fragment_writer
                        .delete_account(uow, event_context, account_id)
                        .await?
                    {
                        fragment_changes.push(ReadModelFragmentChange::try_removed(&account_id)?);
                    }
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
        }

        Ok(fragment_changes)
    }
}
