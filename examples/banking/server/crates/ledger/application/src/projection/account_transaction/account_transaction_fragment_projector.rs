use appletheia::application::event::EventEnvelope;
use appletheia::application::projection::Projector;
use appletheia::application::read_model::{MaterializationEventContext, ReadModelFragmentChange};
use appletheia::domain::AggregateId;
use banking_ledger_domain::currency_issuance::{CurrencyIssuance, CurrencyIssuanceEventPayload};
use banking_ledger_domain::deposit::{Deposit, DepositEventPayload};
use banking_ledger_domain::transfer::{Transfer, TransferEventPayload};
use banking_ledger_domain::withdrawal::{
    Withdrawal, WithdrawalEventPayload, WithdrawalFailureReason,
};

use super::{AccountTransactionFragmentProjectorError, AccountTransactionFragmentProjectorSpec};
use crate::projection::{
    AccountTransactionCurrencyIssuanceIssuedRecord, AccountTransactionDirection,
    AccountTransactionFragment, AccountTransactionFragmentInsert, AccountTransactionFragmentKind,
    AccountTransactionFragmentWriter, AccountTransactionId, AccountTransactionStatus,
    AccountTransactionTransferRequestedRecord,
};

/// Projects ledger events into account transaction fragment read models.
pub struct AccountTransactionFragmentProjector<W>
where
    W: AccountTransactionFragmentWriter,
{
    account_transaction_fragment_writer: W,
}

impl<W> AccountTransactionFragmentProjector<W>
where
    W: AccountTransactionFragmentWriter,
{
    pub fn new(account_transaction_fragment_writer: W) -> Self {
        Self {
            account_transaction_fragment_writer,
        }
    }
}

impl<W> Projector for AccountTransactionFragmentProjector<W>
where
    W: AccountTransactionFragmentWriter,
{
    type Spec = AccountTransactionFragmentProjectorSpec;
    type Fragment = AccountTransactionFragment;
    type Uow = W::Uow;
    type Error = AccountTransactionFragmentProjectorError;

    async fn project(
        &self,
        uow: &mut Self::Uow,
        event_context: MaterializationEventContext,
        event: &EventEnvelope,
    ) -> Result<Vec<ReadModelFragmentChange<Self::Fragment>>, Self::Error> {
        let mut fragment_changes = Vec::new();
        if event.is_for_aggregate::<Deposit>() {
            let domain_event = event.try_into_domain_event::<Deposit>()?;
            let deposit_id = domain_event.aggregate_id();
            let transaction_id = AccountTransactionId::from(deposit_id.value());

            match domain_event.payload() {
                DepositEventPayload::Requested {
                    account_id, amount, ..
                } => {
                    if let Some(fragment) = self
                        .account_transaction_fragment_writer
                        .insert_account_transaction(
                            uow,
                            event_context,
                            AccountTransactionFragmentInsert {
                                transaction_id,
                                account_id: *account_id,
                                counterparty_account_id: None,
                                amount: *amount,
                                direction: AccountTransactionDirection::Incoming,
                                kind: AccountTransactionFragmentKind::Deposit,
                                status: AccountTransactionStatus::Pending,
                            },
                        )
                        .await?
                    {
                        fragment_changes
                            .push(ReadModelFragmentChange::try_from_fragment(&fragment)?);
                    }
                }
                DepositEventPayload::Completed => {
                    if let Some(fragment) = self
                        .account_transaction_fragment_writer
                        .update_account_transaction_status(
                            uow,
                            event_context,
                            transaction_id,
                            AccountTransactionStatus::Completed,
                        )
                        .await?
                    {
                        fragment_changes
                            .push(ReadModelFragmentChange::try_from_fragment(&fragment)?);
                    }
                }
                DepositEventPayload::Failed { .. } => {
                    if let Some(fragment) = self
                        .account_transaction_fragment_writer
                        .update_account_transaction_status(
                            uow,
                            event_context,
                            transaction_id,
                            AccountTransactionStatus::Failed,
                        )
                        .await?
                    {
                        fragment_changes
                            .push(ReadModelFragmentChange::try_from_fragment(&fragment)?);
                    }
                }
                _ => {}
            }
        } else if event.is_for_aggregate::<Withdrawal>() {
            let domain_event = event.try_into_domain_event::<Withdrawal>()?;
            let withdrawal_id = domain_event.aggregate_id();
            let transaction_id = AccountTransactionId::from(withdrawal_id.value());

            match domain_event.payload() {
                WithdrawalEventPayload::Requested {
                    account_id, amount, ..
                } => {
                    if let Some(fragment) = self
                        .account_transaction_fragment_writer
                        .insert_account_transaction(
                            uow,
                            event_context,
                            AccountTransactionFragmentInsert {
                                transaction_id,
                                account_id: *account_id,
                                counterparty_account_id: None,
                                amount: *amount,
                                direction: AccountTransactionDirection::Outgoing,
                                kind: AccountTransactionFragmentKind::Withdrawal,
                                status: AccountTransactionStatus::Pending,
                            },
                        )
                        .await?
                    {
                        fragment_changes
                            .push(ReadModelFragmentChange::try_from_fragment(&fragment)?);
                    }
                }
                WithdrawalEventPayload::Completed => {
                    if let Some(fragment) = self
                        .account_transaction_fragment_writer
                        .update_account_transaction_status(
                            uow,
                            event_context,
                            transaction_id,
                            AccountTransactionStatus::Completed,
                        )
                        .await?
                    {
                        fragment_changes
                            .push(ReadModelFragmentChange::try_from_fragment(&fragment)?);
                    }
                }
                WithdrawalEventPayload::Failed { reason } => {
                    let status = match reason {
                        WithdrawalFailureReason::FundsReserveRejected
                        | WithdrawalFailureReason::TokenTransferRejected => {
                            AccountTransactionStatus::Failed
                        }
                        WithdrawalFailureReason::ReservedFundsReleaseRejected
                        | WithdrawalFailureReason::ReservedFundsCommitRejected => {
                            AccountTransactionStatus::RequiresReview
                        }
                    };
                    if let Some(fragment) = self
                        .account_transaction_fragment_writer
                        .update_account_transaction_status(
                            uow,
                            event_context,
                            transaction_id,
                            status,
                        )
                        .await?
                    {
                        fragment_changes
                            .push(ReadModelFragmentChange::try_from_fragment(&fragment)?);
                    }
                }
                _ => {}
            }
        } else if event.is_for_aggregate::<Transfer>() {
            let domain_event = event.try_into_domain_event::<Transfer>()?;
            let transfer_id = domain_event.aggregate_id();

            match domain_event.payload() {
                TransferEventPayload::Requested {
                    from_account_id,
                    to_account_id,
                    amount,
                    ..
                } => {
                    if let Some(fragment) = self
                        .account_transaction_fragment_writer
                        .record_transfer_requested(
                            uow,
                            event_context,
                            AccountTransactionTransferRequestedRecord {
                                id: transfer_id,
                                correlation_id: event.correlation_id,
                                from_account_id: *from_account_id,
                                to_account_id: *to_account_id,
                                amount: *amount,
                            },
                        )
                        .await?
                    {
                        fragment_changes
                            .push(ReadModelFragmentChange::try_from_fragment(&fragment)?);
                    }
                }
                TransferEventPayload::Completed => {
                    let fragments = self
                        .account_transaction_fragment_writer
                        .complete_transfer(
                            uow,
                            event_context,
                            transfer_id,
                            AccountTransactionId::from(event.event_id.value()),
                        )
                        .await?;
                    for fragment in fragments {
                        fragment_changes
                            .push(ReadModelFragmentChange::try_from_fragment(&fragment)?);
                    }
                }
                TransferEventPayload::Failed { reason } => {
                    if let Some(fragment) = self
                        .account_transaction_fragment_writer
                        .fail_transfer(uow, event_context, transfer_id, *reason)
                        .await?
                    {
                        fragment_changes
                            .push(ReadModelFragmentChange::try_from_fragment(&fragment)?);
                    }
                }
                _ => {}
            }
        } else {
            let domain_event = event.try_into_domain_event::<CurrencyIssuance>()?;
            let currency_issuance_id = domain_event.aggregate_id();

            match domain_event.payload() {
                CurrencyIssuanceEventPayload::Issued {
                    destination_account_id,
                    currency_id,
                    amount,
                    ..
                } => {
                    self.account_transaction_fragment_writer
                        .record_currency_issuance_issued(
                            uow,
                            event_context,
                            AccountTransactionCurrencyIssuanceIssuedRecord {
                                id: currency_issuance_id,
                                destination_account_id: *destination_account_id,
                                currency_id: *currency_id,
                                amount: *amount,
                            },
                        )
                        .await?;
                }
                CurrencyIssuanceEventPayload::Completed => {
                    if let Some(fragment) = self
                        .account_transaction_fragment_writer
                        .complete_currency_issuance(
                            uow,
                            event_context,
                            currency_issuance_id,
                            AccountTransactionId::from(event.event_id.value()),
                        )
                        .await?
                    {
                        fragment_changes
                            .push(ReadModelFragmentChange::try_from_fragment(&fragment)?);
                    }
                }
                CurrencyIssuanceEventPayload::Failed { .. } => {
                    self.account_transaction_fragment_writer
                        .fail_currency_issuance(uow, event_context, currency_issuance_id)
                        .await?;
                }
                _ => {}
            }
        }

        Ok(fragment_changes)
    }
}
