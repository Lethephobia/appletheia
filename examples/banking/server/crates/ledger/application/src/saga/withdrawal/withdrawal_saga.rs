use appletheia::application::event::EventEnvelope;
use appletheia::application::saga::{Saga, SagaInstance, SagaSpec};
use banking_ledger_domain::account::{Account, AccountEventPayload};
use banking_ledger_domain::withdrawal::{
    Withdrawal, WithdrawalEventPayload, WithdrawalFailureReason,
};

use super::{WithdrawalSagaError, WithdrawalSagaSpec, WithdrawalSagaState, WithdrawalSagaStatus};
use crate::command::{
    AccountFundsReserveCommand, AccountReservedFundsCommitCommand,
    AccountReservedFundsReleaseCommand, WithdrawalCompleteCommand, WithdrawalFailCommand,
    WithdrawalTokenTransferCommand,
};

/// Coordinates the withdrawal flow.
pub struct WithdrawalSaga;

impl Saga for WithdrawalSaga {
    type Spec = WithdrawalSagaSpec;
    type Error = WithdrawalSagaError;

    fn on_event(
        &self,
        instance: &mut SagaInstance<<Self::Spec as SagaSpec>::State>,
        event: &EventEnvelope,
    ) -> Result<(), Self::Error> {
        if event.is_for_aggregate::<Withdrawal>() {
            let withdrawal_event = event.try_into_domain_event::<Withdrawal>()?;
            match withdrawal_event.payload() {
                WithdrawalEventPayload::Requested {
                    id,
                    account_id,
                    amount,
                    ..
                } => {
                    *instance.state_mut() =
                        Some(WithdrawalSagaState::new(*id, *account_id, *amount));

                    instance.append_command(
                        event,
                        &AccountFundsReserveCommand {
                            account_id: *account_id,
                            amount: *amount,
                        },
                    )?;
                }
                WithdrawalEventPayload::TokenTransferred { .. } => {
                    let state = instance.state_required_mut()?;
                    let account_id = state.account_id;
                    let amount = state.amount;
                    state.status = WithdrawalSagaStatus::ReservedFundsCommitRequested;

                    instance.append_command(
                        event,
                        &AccountReservedFundsCommitCommand { account_id, amount },
                    )?;
                }
                WithdrawalEventPayload::Completed => {
                    instance.state_required_mut()?.status = WithdrawalSagaStatus::Completed;
                    instance.succeed();
                }
                WithdrawalEventPayload::Failed { .. } => {
                    let state = instance.state_required_mut()?;
                    if matches!(state.status, WithdrawalSagaStatus::TokenTransferRequested) {
                        state.status = WithdrawalSagaStatus::ReservedFundsReleaseRequested;
                        let account_id = state.account_id;
                        let amount = state.amount;
                        instance.append_command(
                            event,
                            &AccountReservedFundsReleaseCommand { account_id, amount },
                        )?;
                    } else {
                        state.status = WithdrawalSagaStatus::Failed;
                        instance.fail();
                    }
                }
                _ => {}
            }

            return Ok(());
        } else if event.is_for_aggregate::<Account>() {
            let account_event = event.try_into_domain_event::<Account>()?;
            match account_event.payload() {
                AccountEventPayload::FundsReserved { .. } => {
                    let state = instance.state_required_mut()?;
                    let withdrawal_id = state.withdrawal_id;
                    state.status = WithdrawalSagaStatus::TokenTransferRequested;

                    instance
                        .append_command(event, &WithdrawalTokenTransferCommand { withdrawal_id })?;
                }
                AccountEventPayload::FundsReserveRejected { .. } => {
                    let state = instance.state_required_mut()?;
                    state.status = WithdrawalSagaStatus::FailRequested;
                    let withdrawal_id = state.withdrawal_id;
                    instance.append_command(
                        event,
                        &WithdrawalFailCommand {
                            withdrawal_id,
                            reason: WithdrawalFailureReason::FundsReserveRejected,
                        },
                    )?;
                }
                AccountEventPayload::ReservedFundsReleased { .. } => {
                    let state = instance.state_required_mut()?;
                    if state.status == WithdrawalSagaStatus::ReservedFundsReleaseRequested {
                        state.status = WithdrawalSagaStatus::ReservedFundsReleased;
                        instance.fail();
                    }
                }
                AccountEventPayload::ReservedFundsReleaseRejected { .. } => {
                    let state = instance.state_required_mut()?;
                    if state.status == WithdrawalSagaStatus::ReservedFundsReleaseRequested {
                        state.status = WithdrawalSagaStatus::ReservedFundsReleaseRejected;
                        instance.fail();
                    }
                }
                AccountEventPayload::ReservedFundsCommitted { .. } => {
                    let state = instance.state_required_mut()?;
                    let withdrawal_id = state.withdrawal_id;
                    state.status = WithdrawalSagaStatus::CompleteRequested;

                    instance.append_command(event, &WithdrawalCompleteCommand { withdrawal_id })?;
                }
                AccountEventPayload::ReservedFundsCommitRejected { .. } => {
                    let state = instance.state_required_mut()?;
                    state.status = WithdrawalSagaStatus::FailRequested;
                    let withdrawal_id = state.withdrawal_id;
                    instance.append_command(
                        event,
                        &WithdrawalFailCommand {
                            withdrawal_id,
                            reason: WithdrawalFailureReason::ReservedFundsCommitRejected,
                        },
                    )?;
                }
                _ => {}
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use appletheia::application::event::{
        AggregateIdValue, AggregateTypeOwned, EventEnvelope, EventNameOwned, EventSequence,
        SerializedEventPayload,
    };
    use appletheia::application::request_context::{
        CausationId, CorrelationId, MessageId, Principal, RequestContext,
    };
    use appletheia::application::saga::{Saga, SagaInstance, SagaNameOwned, SagaSpec, SagaStatus};
    use appletheia::domain::{Aggregate, AggregateId, EventId, EventOccurredAt, EventPayload};
    use banking_iam_domain::{User, UserId};
    use banking_ledger_domain::account::{
        Account, AccountEventPayload, AccountFundsReserveRejectionReason, AccountId,
    };
    use banking_ledger_domain::core::{
        CurrencyAmount, OnchainTransactionId, TokenAccountOwnerAddress,
    };
    use banking_ledger_domain::withdrawal::{
        Withdrawal, WithdrawalEventPayload, WithdrawalFailureReason, WithdrawalId,
    };

    use super::{WithdrawalSaga, WithdrawalSagaSpec, WithdrawalSagaState, WithdrawalSagaStatus};
    use crate::command::{
        AccountFundsReserveCommand, AccountReservedFundsCommitCommand,
        AccountReservedFundsReleaseCommand, WithdrawalCompleteCommand, WithdrawalFailCommand,
        WithdrawalTokenTransferCommand,
    };

    fn request_context(correlation_id: CorrelationId) -> RequestContext {
        let subject =
            appletheia::application::authorization::AggregateRef::from_id::<User>(UserId::new());

        RequestContext::new(
            correlation_id,
            MessageId::new(),
            Principal::Authenticated { subject },
        )
        .expect("request context should be valid")
    }

    fn account_event_envelope(
        correlation_id: CorrelationId,
        account_id: AccountId,
        payload: AccountEventPayload,
    ) -> EventEnvelope {
        EventEnvelope {
            event_sequence: EventSequence::try_from(1).expect("sequence should be valid"),
            event_id: EventId::new(),
            aggregate_type: AggregateTypeOwned::from(Account::TYPE),
            aggregate_id: AggregateIdValue::from(account_id.value()),
            aggregate_version: appletheia::domain::AggregateVersion::try_from(1)
                .expect("version should be valid"),
            event_name: EventNameOwned::from(payload.name()),
            payload: SerializedEventPayload::try_from(
                payload.into_json_value().expect("payload should serialize"),
            )
            .expect("payload should be valid"),
            occurred_at: EventOccurredAt::now(),
            correlation_id,
            causation_id: CausationId::from(MessageId::new()),
            context: request_context(correlation_id),
        }
    }

    fn withdrawal_event_envelope(
        correlation_id: CorrelationId,
        withdrawal_id: WithdrawalId,
        payload: WithdrawalEventPayload,
    ) -> EventEnvelope {
        EventEnvelope {
            event_sequence: EventSequence::try_from(1).expect("sequence should be valid"),
            event_id: EventId::new(),
            aggregate_type: AggregateTypeOwned::from(Withdrawal::TYPE),
            aggregate_id: AggregateIdValue::from(withdrawal_id.value()),
            aggregate_version: appletheia::domain::AggregateVersion::try_from(1)
                .expect("version should be valid"),
            event_name: EventNameOwned::from(payload.name()),
            payload: SerializedEventPayload::try_from(
                payload.into_json_value().expect("payload should serialize"),
            )
            .expect("payload should be valid"),
            occurred_at: EventOccurredAt::now(),
            correlation_id,
            causation_id: CausationId::from(MessageId::new()),
            context: request_context(correlation_id),
        }
    }

    fn token_account_owner_address() -> TokenAccountOwnerAddress {
        TokenAccountOwnerAddress::try_from("11111111111111111111111111111111")
            .expect("token account owner address should be valid")
    }

    #[test]
    fn withdrawal_requested_appends_account_funds_reserve_command() {
        let saga = WithdrawalSaga;
        let correlation_id = CorrelationId::from(uuid::Uuid::now_v7());
        let account_id = AccountId::new();
        let withdrawal_id = WithdrawalId::new();
        let amount = CurrencyAmount::new(100);
        let mut instance = SagaInstance::<WithdrawalSagaState>::new(
            SagaNameOwned::from(WithdrawalSagaSpec::DESCRIPTOR.name),
            correlation_id,
            EventId::new(),
        );

        saga.on_event(
            &mut instance,
            &withdrawal_event_envelope(
                correlation_id,
                withdrawal_id,
                WithdrawalEventPayload::Requested {
                    id: withdrawal_id,
                    account_id,
                    currency_id: banking_ledger_domain::currency::CurrencyId::new(),
                    token_account_owner_address: token_account_owner_address(),
                    amount,
                },
            ),
        )
        .expect("saga should succeed");

        assert_eq!(instance.uncommitted_commands().len(), 1);
        assert_eq!(
            instance.state.as_ref().map(|state| &state.status),
            Some(&WithdrawalSagaStatus::FundsReserveRequested)
        );
        let command = instance.uncommitted_commands()[0]
            .try_into_command::<AccountFundsReserveCommand>()
            .expect("command should deserialize");
        assert_eq!(command, AccountFundsReserveCommand { account_id, amount });
    }

    #[test]
    fn success_path_appends_expected_follow_up_commands_and_succeeds() {
        let saga = WithdrawalSaga;
        let correlation_id = CorrelationId::from(uuid::Uuid::now_v7());
        let account_id = AccountId::new();
        let withdrawal_id = WithdrawalId::new();
        let amount = CurrencyAmount::new(100);
        let onchain_transaction_id =
            OnchainTransactionId::try_from("signature-1").expect("on-chain transaction id valid");
        let mut instance = SagaInstance::<WithdrawalSagaState>::new(
            SagaNameOwned::from(WithdrawalSagaSpec::DESCRIPTOR.name),
            correlation_id,
            EventId::new(),
        );

        saga.on_event(
            &mut instance,
            &withdrawal_event_envelope(
                correlation_id,
                withdrawal_id,
                WithdrawalEventPayload::Requested {
                    id: withdrawal_id,
                    account_id,
                    currency_id: banking_ledger_domain::currency::CurrencyId::new(),
                    token_account_owner_address: token_account_owner_address(),
                    amount,
                },
            ),
        )
        .expect("requested should succeed");
        instance.clear_uncommitted_commands();

        saga.on_event(
            &mut instance,
            &account_event_envelope(
                correlation_id,
                account_id,
                AccountEventPayload::FundsReserved { amount },
            ),
        )
        .expect("funds reserved should succeed");
        let transfer = instance.uncommitted_commands()[0]
            .try_into_command::<WithdrawalTokenTransferCommand>()
            .expect("command should deserialize");
        assert_eq!(transfer, WithdrawalTokenTransferCommand { withdrawal_id });
        assert_eq!(
            instance.state.as_ref().map(|state| &state.status),
            Some(&WithdrawalSagaStatus::TokenTransferRequested)
        );

        instance.clear_uncommitted_commands();
        saga.on_event(
            &mut instance,
            &withdrawal_event_envelope(
                correlation_id,
                withdrawal_id,
                WithdrawalEventPayload::TokenTransferred {
                    onchain_transaction_id,
                },
            ),
        )
        .expect("token transferred should succeed");
        let commit = instance.uncommitted_commands()[0]
            .try_into_command::<AccountReservedFundsCommitCommand>()
            .expect("command should deserialize");
        assert_eq!(
            commit,
            AccountReservedFundsCommitCommand { account_id, amount }
        );
        assert_eq!(
            instance.state.as_ref().map(|state| &state.status),
            Some(&WithdrawalSagaStatus::ReservedFundsCommitRequested)
        );

        instance.clear_uncommitted_commands();
        saga.on_event(
            &mut instance,
            &account_event_envelope(
                correlation_id,
                account_id,
                AccountEventPayload::ReservedFundsCommitted { amount },
            ),
        )
        .expect("reserved funds committed should succeed");
        let complete = instance.uncommitted_commands()[0]
            .try_into_command::<WithdrawalCompleteCommand>()
            .expect("command should deserialize");
        assert_eq!(complete, WithdrawalCompleteCommand { withdrawal_id });

        instance.clear_uncommitted_commands();
        saga.on_event(
            &mut instance,
            &withdrawal_event_envelope(
                correlation_id,
                withdrawal_id,
                WithdrawalEventPayload::Completed,
            ),
        )
        .expect("completed should succeed");

        assert_eq!(instance.status, SagaStatus::Succeeded);
    }

    #[test]
    fn funds_reserve_rejected_appends_withdrawal_fail_command() {
        let saga = WithdrawalSaga;
        let correlation_id = CorrelationId::from(uuid::Uuid::now_v7());
        let account_id = AccountId::new();
        let withdrawal_id = WithdrawalId::new();
        let amount = CurrencyAmount::new(100);
        let mut instance = SagaInstance::<WithdrawalSagaState>::new(
            SagaNameOwned::from(WithdrawalSagaSpec::DESCRIPTOR.name),
            correlation_id,
            EventId::new(),
        );

        saga.on_event(
            &mut instance,
            &withdrawal_event_envelope(
                correlation_id,
                withdrawal_id,
                WithdrawalEventPayload::Requested {
                    id: withdrawal_id,
                    account_id,
                    currency_id: banking_ledger_domain::currency::CurrencyId::new(),
                    token_account_owner_address: token_account_owner_address(),
                    amount,
                },
            ),
        )
        .expect("requested should succeed");
        instance.clear_uncommitted_commands();

        saga.on_event(
            &mut instance,
            &account_event_envelope(
                correlation_id,
                account_id,
                AccountEventPayload::FundsReserveRejected {
                    amount,
                    reason: AccountFundsReserveRejectionReason::InsufficientAvailableBalance,
                },
            ),
        )
        .expect("reserve rejection should succeed");

        let fail = instance.uncommitted_commands()[0]
            .try_into_command::<WithdrawalFailCommand>()
            .expect("command should deserialize");
        assert_eq!(
            fail,
            WithdrawalFailCommand {
                withdrawal_id,
                reason: WithdrawalFailureReason::FundsReserveRejected,
            }
        );
        assert_eq!(
            instance.state.as_ref().map(|state| &state.status),
            Some(&WithdrawalSagaStatus::FailRequested)
        );
    }

    #[test]
    fn failed_after_transfer_requests_reserved_funds_release() {
        let saga = WithdrawalSaga;
        let correlation_id = CorrelationId::from(uuid::Uuid::now_v7());
        let account_id = AccountId::new();
        let withdrawal_id = WithdrawalId::new();
        let amount = CurrencyAmount::new(100);
        let mut instance = SagaInstance::<WithdrawalSagaState>::new(
            SagaNameOwned::from(WithdrawalSagaSpec::DESCRIPTOR.name),
            correlation_id,
            EventId::new(),
        );

        saga.on_event(
            &mut instance,
            &withdrawal_event_envelope(
                correlation_id,
                withdrawal_id,
                WithdrawalEventPayload::Requested {
                    id: withdrawal_id,
                    account_id,
                    currency_id: banking_ledger_domain::currency::CurrencyId::new(),
                    token_account_owner_address: token_account_owner_address(),
                    amount,
                },
            ),
        )
        .expect("requested should succeed");
        instance.clear_uncommitted_commands();

        saga.on_event(
            &mut instance,
            &account_event_envelope(
                correlation_id,
                account_id,
                AccountEventPayload::FundsReserved { amount },
            ),
        )
        .expect("funds reserved should succeed");
        instance.clear_uncommitted_commands();

        saga.on_event(
            &mut instance,
            &withdrawal_event_envelope(
                correlation_id,
                withdrawal_id,
                WithdrawalEventPayload::Failed {
                    reason: WithdrawalFailureReason::TokenTransferRejected,
                },
            ),
        )
        .expect("failed should succeed");

        let release = instance.uncommitted_commands()[0]
            .try_into_command::<AccountReservedFundsReleaseCommand>()
            .expect("command should deserialize");
        assert_eq!(
            release,
            AccountReservedFundsReleaseCommand { account_id, amount }
        );
        assert_eq!(
            instance.state.as_ref().map(|state| &state.status),
            Some(&WithdrawalSagaStatus::ReservedFundsReleaseRequested)
        );
    }
}
