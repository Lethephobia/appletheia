use appletheia::application::event::EventEnvelope;
use appletheia::application::projection::Projector;
use appletheia::application::read_model::MaterializationEventContext;
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
    ) -> Result<(), Self::Error> {
        if event.is_for_aggregate::<Account>() {
            let domain_event = event.try_to_domain_event::<Account>()?;
            let account_id = domain_event.aggregate_id();

            match domain_event.payload() {
                AccountEventPayload::Opened {
                    owner,
                    name,
                    description,
                    currency_id,
                } => {
                    self.account_fragment_writer
                        .upsert_account(
                            uow,
                            event_context,
                            AccountFragmentUpsert {
                                id: account_id,
                                owner: *owner,
                                name: name.clone(),
                                description: description.clone(),
                                currency_id: *currency_id,
                                balance: CurrencyAmount::zero(),
                                reserved_balance: CurrencyAmount::zero(),
                                status: MaterializedAccountStatus::Active,
                            },
                        )
                        .await?;
                }
                AccountEventPayload::OwnershipTransferred { owner } => {
                    self.account_fragment_writer
                        .update_account_owner(uow, event_context, account_id, *owner)
                        .await?;
                }
                AccountEventPayload::NameChanged { name } => {
                    self.account_fragment_writer
                        .update_account_name(uow, event_context, account_id, name.clone())
                        .await?;
                }
                AccountEventPayload::DescriptionChanged { description } => {
                    self.account_fragment_writer
                        .update_account_description(
                            uow,
                            event_context,
                            account_id,
                            description.clone(),
                        )
                        .await?;
                }
                AccountEventPayload::Deposited { amount } => {
                    self.account_fragment_writer
                        .increase_balance(uow, event_context, account_id, *amount)
                        .await?;
                }
                AccountEventPayload::Withdrawn { amount } => {
                    self.account_fragment_writer
                        .decrease_balance(uow, event_context, account_id, *amount)
                        .await?;
                }
                AccountEventPayload::FundsReserved { amount } => {
                    self.account_fragment_writer
                        .reserve_balance(uow, event_context, account_id, *amount)
                        .await?;
                }
                AccountEventPayload::ReservedFundsReleased { amount } => {
                    self.account_fragment_writer
                        .release_reserved_balance(uow, event_context, account_id, *amount)
                        .await?;
                }
                AccountEventPayload::ReservedFundsCommitted { amount } => {
                    self.account_fragment_writer
                        .commit_reserved_balance(uow, event_context, account_id, *amount)
                        .await?;
                }
                AccountEventPayload::Frozen => {
                    self.account_fragment_writer
                        .update_account_status(
                            uow,
                            event_context,
                            account_id,
                            MaterializedAccountStatus::Frozen,
                        )
                        .await?;
                }
                AccountEventPayload::Thawed => {
                    self.account_fragment_writer
                        .update_account_status(
                            uow,
                            event_context,
                            account_id,
                            MaterializedAccountStatus::Active,
                        )
                        .await?;
                }
                AccountEventPayload::Closed => {
                    self.account_fragment_writer
                        .delete_account(uow, event_context, account_id)
                        .await?;
                }
            }
        }

        Ok(())
    }
}
