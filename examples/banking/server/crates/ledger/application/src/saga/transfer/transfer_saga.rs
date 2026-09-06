use super::{TransferSagaError, TransferSagaSpec, TransferSagaState, TransferSagaStep};
use crate::command::{
    AccountDepositCommand, AccountFundsReserveCommand, AccountReservedFundsCommitCommand,
    AccountReservedFundsReleaseCommand, AccountWithdrawCommand, TransferCompleteCommand,
    TransferFailCommand,
};
use appletheia::application::command::CommandFailureEnvelope;
use appletheia::application::event::EventEnvelope;
use appletheia::application::request_context::CausationId;
use appletheia::application::saga::{Saga, SagaInstance, SagaSpec};
use banking_ledger_domain::account::{Account, AccountEventPayload};
use banking_ledger_domain::transfer::{Transfer, TransferEventPayload, TransferFailureReason};

/// Coordinates the transfer flow.
pub struct TransferSaga;

impl Saga for TransferSaga {
    type Spec = TransferSagaSpec;
    type Step = TransferSagaStep;
    type Error = TransferSagaError;

    fn on_event(
        &self,
        instance: &mut SagaInstance<<Self::Spec as SagaSpec>::State, Self::Step>,
        event: &EventEnvelope,
        step: Option<Self::Step>,
    ) -> Result<(), Self::Error> {
        if event.is_for_aggregate::<Transfer>() {
            let transfer_event = event.try_into_domain_event::<Transfer>()?;
            match transfer_event.payload() {
                TransferEventPayload::Requested {
                    from_account_id,
                    to_account_id,
                    amount,
                    ..
                } if step.is_none() => {
                    *instance.state_mut() = Some(TransferSagaState::new(
                        transfer_event.aggregate_id(),
                        *from_account_id,
                        *to_account_id,
                        *amount,
                    ));

                    instance.append_command(
                        CausationId::from(event.event_id),
                        TransferSagaStep::ReserveFunds,
                        &AccountFundsReserveCommand {
                            account_id: *from_account_id,
                            amount: *amount,
                        },
                    )?;
                }
                TransferEventPayload::Completed if step == Some(TransferSagaStep::Complete) => {
                    instance.succeed();
                }
                TransferEventPayload::Failed { .. } if step == Some(TransferSagaStep::Fail) => {
                    instance.fail();
                }
                _ => {}
            }

            return Ok(());
        } else if event.is_for_aggregate::<Account>() {
            let account_event = event.try_into_domain_event::<Account>()?;
            match account_event.payload() {
                AccountEventPayload::FundsReserved { .. }
                    if step == Some(TransferSagaStep::ReserveFunds) =>
                {
                    let state = instance.state_required_mut()?;
                    let to_account_id = state.to_account_id;
                    let amount = state.amount;

                    instance.append_command(
                        CausationId::from(event.event_id),
                        TransferSagaStep::Deposit,
                        &AccountDepositCommand {
                            account_id: to_account_id,
                            amount,
                        },
                    )?;
                }
                AccountEventPayload::Deposited { .. }
                    if step == Some(TransferSagaStep::Deposit) =>
                {
                    let state = instance.state_required_mut()?;
                    let from_account_id = state.from_account_id;
                    let amount = state.amount;

                    instance.append_command(
                        CausationId::from(event.event_id),
                        TransferSagaStep::CommitFunds,
                        &AccountReservedFundsCommitCommand {
                            account_id: from_account_id,
                            amount,
                        },
                    )?;
                }
                AccountEventPayload::ReservedFundsReleased { .. }
                    if step == Some(TransferSagaStep::ReleaseFunds) =>
                {
                    let state = instance.state_required_mut()?;
                    let transfer_id = state.transfer_id;

                    instance.append_command(
                        CausationId::from(event.event_id),
                        TransferSagaStep::Fail,
                        &TransferFailCommand {
                            transfer_id,
                            reason: TransferFailureReason::DepositRejected,
                        },
                    )?;
                }
                AccountEventPayload::ReservedFundsCommitted { .. }
                    if step == Some(TransferSagaStep::CommitFunds) =>
                {
                    let state = instance.state_required_mut()?;
                    let transfer_id = state.transfer_id;

                    instance.append_command(
                        CausationId::from(event.event_id),
                        TransferSagaStep::Complete,
                        &TransferCompleteCommand { transfer_id },
                    )?;
                }
                AccountEventPayload::Withdrawn { .. }
                    if step == Some(TransferSagaStep::CompensateDeposit) =>
                {
                    let state = instance.state_required_mut()?;
                    let transfer_id = state.transfer_id;

                    instance.append_command(
                        CausationId::from(event.event_id),
                        TransferSagaStep::Fail,
                        &TransferFailCommand {
                            transfer_id,
                            reason: TransferFailureReason::ReservedFundsCommitRejected,
                        },
                    )?;
                }
                _ => {}
            }
        }

        Ok(())
    }

    fn on_command_failed(
        &self,
        instance: &mut SagaInstance<<Self::Spec as SagaSpec>::State, Self::Step>,
        failure: &CommandFailureEnvelope,
        step: Self::Step,
    ) -> Result<(), Self::Error> {
        if step == TransferSagaStep::ReserveFunds {
            let state = instance.state_required_mut()?;
            let transfer_id = state.transfer_id;
            instance.append_command(
                CausationId::from(failure.failure_id),
                TransferSagaStep::Fail,
                &TransferFailCommand {
                    transfer_id,
                    reason: TransferFailureReason::FundsReserveRejected,
                },
            )?;
        } else if step == TransferSagaStep::Deposit {
            let state = instance.state_required_mut()?;
            let from_account_id = state.from_account_id;
            let amount = state.amount;
            instance.append_command(
                CausationId::from(failure.failure_id),
                TransferSagaStep::ReleaseFunds,
                &AccountReservedFundsReleaseCommand {
                    account_id: from_account_id,
                    amount,
                },
            )?;
        } else if step == TransferSagaStep::ReleaseFunds {
            let state = instance.state_required_mut()?;
            let transfer_id = state.transfer_id;
            instance.append_command(
                CausationId::from(failure.failure_id),
                TransferSagaStep::Fail,
                &TransferFailCommand {
                    transfer_id,
                    reason: TransferFailureReason::ReservedFundsReleaseRejected,
                },
            )?;
        } else if step == TransferSagaStep::CommitFunds {
            let state = instance.state_required_mut()?;
            let account_id = state.to_account_id;
            let amount = state.amount;
            instance.append_command(
                CausationId::from(failure.failure_id),
                TransferSagaStep::CompensateDeposit,
                &AccountWithdrawCommand { account_id, amount },
            )?;
        } else if step == TransferSagaStep::CompensateDeposit {
            let state = instance.state_required_mut()?;
            let transfer_id = state.transfer_id;
            instance.append_command(
                CausationId::from(failure.failure_id),
                TransferSagaStep::Fail,
                &TransferFailCommand {
                    transfer_id,
                    reason: TransferFailureReason::ReservedFundsCommitRejected,
                },
            )?;
        } else if matches!(step, TransferSagaStep::Complete | TransferSagaStep::Fail) {
            instance.fail();
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use uuid::Uuid;

    use appletheia::application::command::{
        Command, CommandAttemptCount, CommandEnvelope, CommandFailedAt, CommandFailureEnvelope,
        CommandOptions, CommandTerminalReason,
    };
    use appletheia::application::event::{
        AggregateIdValue, AggregateTypeOwned, EventEnvelope, EventNameOwned, EventSequence,
        SerializedEventPayload,
    };
    use appletheia::application::request_context::{
        CausationId, CorrelationId, MessageId, Principal, RequestContext,
    };
    use appletheia::application::saga::{
        Saga, SagaCommandOrigin, SagaInstance, SagaNameOwned, SagaSpec, SagaStatus,
        SerializedSagaStep,
    };
    use appletheia::domain::{Aggregate, AggregateId, EventId, EventOccurredAt, EventPayload};
    use banking_iam_domain::{User, UserId};
    use banking_ledger_domain::account::{Account, AccountEventPayload, AccountId};
    use banking_ledger_domain::core::CurrencyAmount;
    use banking_ledger_domain::transfer::{
        Transfer, TransferEventPayload, TransferFailureReason, TransferId, TransferNote,
    };

    use super::{TransferSaga, TransferSagaSpec, TransferSagaState, TransferSagaStep};
    use crate::command::{
        AccountDepositCommand, AccountFundsReserveCommand, AccountReservedFundsCommitCommand,
        AccountReservedFundsReleaseCommand, AccountWithdrawCommand, TransferCompleteCommand,
        TransferFailCommand,
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

    fn transfer_event_envelope(
        correlation_id: CorrelationId,
        transfer_id: TransferId,
        payload: TransferEventPayload,
    ) -> EventEnvelope {
        EventEnvelope {
            event_sequence: EventSequence::try_from(1).expect("sequence should be valid"),
            event_id: EventId::new(),
            aggregate_type: AggregateTypeOwned::from(Transfer::TYPE),
            aggregate_id: AggregateIdValue::from(transfer_id.value()),
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

    fn command_failure<C: Command>(
        instance: &SagaInstance<TransferSagaState, TransferSagaStep>,
        step: TransferSagaStep,
        command: &C,
    ) -> CommandFailureEnvelope {
        let origin = SagaCommandOrigin {
            saga_name: instance.saga_name.clone(),
            saga_instance_id: instance.saga_instance_id,
            step: SerializedSagaStep::new(step).expect("step should serialize"),
        };
        let envelope = CommandEnvelope::new(
            command,
            instance.correlation_id,
            CausationId::from(MessageId::new()),
            CommandOptions::default(),
        )
        .expect("command envelope should be valid")
        .with_saga_origin(origin.clone());
        CommandFailureEnvelope::new(
            &envelope,
            origin,
            CommandTerminalReason::NonRetryable,
            CommandAttemptCount::first(),
            CommandFailedAt::now(),
        )
    }

    fn handle_event(
        saga: &TransferSaga,
        instance: &mut SagaInstance<TransferSagaState, TransferSagaStep>,
        envelope: &EventEnvelope,
        step: Option<TransferSagaStep>,
    ) -> Result<(), super::TransferSagaError> {
        saga.on_event(instance, envelope, step)
    }

    #[test]
    fn transfer_requested_with_note_appends_account_funds_reserve_command() {
        let saga = TransferSaga;
        let correlation_id = CorrelationId::from(Uuid::now_v7());
        let from_account_id = AccountId::new();
        let to_account_id = AccountId::new();
        let transfer_id = TransferId::new();
        let amount = CurrencyAmount::new(100);
        let mut instance = SagaInstance::<TransferSagaState, TransferSagaStep>::new(
            SagaNameOwned::from(TransferSagaSpec::DESCRIPTOR.name),
            correlation_id,
            EventId::new(),
        );

        handle_event(
            &saga,
            &mut instance,
            &transfer_event_envelope(
                correlation_id,
                transfer_id,
                TransferEventPayload::Requested {
                    from_account_id,
                    to_account_id,
                    amount,
                    note: Some(
                        TransferNote::try_from("invoice 123")
                            .expect("transfer note should be valid"),
                    ),
                },
            ),
            None,
        )
        .expect("saga should succeed");

        assert_eq!(instance.uncommitted_commands().len(), 1);
        assert_eq!(
            instance.uncommitted_commands()[0]
                .saga_origin
                .as_ref()
                .expect("saga origin")
                .step
                .try_into_step::<TransferSagaStep>()
                .expect("saga step"),
            TransferSagaStep::ReserveFunds
        );
        assert!(instance.dispatched_commands.is_empty());
        let command = instance.uncommitted_commands()[0]
            .try_into_command::<AccountFundsReserveCommand>()
            .expect("command should deserialize");
        assert_eq!(
            command,
            AccountFundsReserveCommand {
                account_id: from_account_id,
                amount,
            }
        );
    }

    #[test]
    fn success_path_appends_expected_follow_up_commands_and_succeeds() {
        let saga = TransferSaga;
        let correlation_id = CorrelationId::from(Uuid::now_v7());
        let from_account_id = AccountId::new();
        let to_account_id = AccountId::new();
        let transfer_id = TransferId::new();
        let amount = CurrencyAmount::new(100);
        let mut instance = SagaInstance::<TransferSagaState, TransferSagaStep>::new(
            SagaNameOwned::from(TransferSagaSpec::DESCRIPTOR.name),
            correlation_id,
            EventId::new(),
        );

        handle_event(
            &saga,
            &mut instance,
            &transfer_event_envelope(
                correlation_id,
                transfer_id,
                TransferEventPayload::Requested {
                    from_account_id,
                    to_account_id,
                    amount,
                    note: None,
                },
            ),
            None,
        )
        .expect("requested should succeed");
        let reserve = instance.uncommitted_commands()[0]
            .try_into_command::<AccountFundsReserveCommand>()
            .expect("command should deserialize");
        assert_eq!(
            reserve,
            AccountFundsReserveCommand {
                account_id: from_account_id,
                amount,
            }
        );

        instance.clear_uncommitted_commands();
        handle_event(
            &saga,
            &mut instance,
            &account_event_envelope(
                correlation_id,
                from_account_id,
                AccountEventPayload::FundsReserved { amount },
            ),
            Some(TransferSagaStep::ReserveFunds),
        )
        .expect("funds reserved should succeed");
        let deposit = instance.uncommitted_commands()[0]
            .try_into_command::<AccountDepositCommand>()
            .expect("command should deserialize");
        assert_eq!(
            deposit,
            AccountDepositCommand {
                account_id: to_account_id,
                amount,
            }
        );

        instance.clear_uncommitted_commands();
        handle_event(
            &saga,
            &mut instance,
            &account_event_envelope(
                correlation_id,
                to_account_id,
                AccountEventPayload::Deposited { amount },
            ),
            Some(TransferSagaStep::Deposit),
        )
        .expect("deposited should succeed");
        let commit = instance.uncommitted_commands()[0]
            .try_into_command::<AccountReservedFundsCommitCommand>()
            .expect("command should deserialize");
        assert_eq!(
            commit,
            AccountReservedFundsCommitCommand {
                account_id: from_account_id,
                amount,
            }
        );

        instance.clear_uncommitted_commands();
        handle_event(
            &saga,
            &mut instance,
            &account_event_envelope(
                correlation_id,
                from_account_id,
                AccountEventPayload::ReservedFundsCommitted { amount },
            ),
            Some(TransferSagaStep::CommitFunds),
        )
        .expect("reserved funds committed should succeed");
        let complete = instance.uncommitted_commands()[0]
            .try_into_command::<TransferCompleteCommand>()
            .expect("command should deserialize");
        assert_eq!(complete, TransferCompleteCommand { transfer_id });

        instance.clear_uncommitted_commands();
        handle_event(
            &saga,
            &mut instance,
            &transfer_event_envelope(correlation_id, transfer_id, TransferEventPayload::Completed),
            Some(TransferSagaStep::Complete),
        )
        .expect("completed should succeed");

        assert_eq!(instance.status, SagaStatus::Succeeded);
    }

    #[test]
    fn reserved_funds_released_appends_transfer_fail_command() {
        let saga = TransferSaga;
        let correlation_id = CorrelationId::from(Uuid::now_v7());
        let from_account_id = AccountId::new();
        let to_account_id = AccountId::new();
        let transfer_id = TransferId::new();
        let amount = CurrencyAmount::new(100);
        let mut instance = SagaInstance::<TransferSagaState, TransferSagaStep>::new(
            SagaNameOwned::from(TransferSagaSpec::DESCRIPTOR.name),
            correlation_id,
            EventId::new(),
        );

        *instance.state_mut() = Some(TransferSagaState {
            from_account_id,
            to_account_id,
            amount,
            transfer_id,
        });

        handle_event(
            &saga,
            &mut instance,
            &account_event_envelope(
                correlation_id,
                from_account_id,
                AccountEventPayload::ReservedFundsReleased { amount },
            ),
            Some(TransferSagaStep::ReleaseFunds),
        )
        .expect("reserved funds released should succeed");

        let fail = instance.uncommitted_commands()[0]
            .try_into_command::<TransferFailCommand>()
            .expect("command should deserialize");
        assert_eq!(
            fail,
            TransferFailCommand {
                transfer_id,
                reason: TransferFailureReason::DepositRejected,
            }
        );
    }

    #[test]
    fn deposit_failure_appends_release_reserved_funds_command() {
        let saga = TransferSaga;
        let correlation_id = CorrelationId::from(Uuid::now_v7());
        let from_account_id = AccountId::new();
        let to_account_id = AccountId::new();
        let transfer_id = TransferId::new();
        let amount = CurrencyAmount::new(100);
        let mut instance = SagaInstance::<TransferSagaState, TransferSagaStep>::new(
            SagaNameOwned::from(TransferSagaSpec::DESCRIPTOR.name),
            correlation_id,
            EventId::new(),
        );

        *instance.state_mut() = Some(TransferSagaState {
            from_account_id,
            to_account_id,
            amount,
            transfer_id,
        });

        let failure = command_failure(
            &instance,
            TransferSagaStep::Deposit,
            &AccountDepositCommand {
                account_id: to_account_id,
                amount,
            },
        );
        saga.on_command_failed(&mut instance, &failure, TransferSagaStep::Deposit)
            .expect("deposit failure should succeed");

        let release = instance.uncommitted_commands()[0]
            .try_into_command::<AccountReservedFundsReleaseCommand>()
            .expect("command should deserialize");
        assert_eq!(
            release,
            AccountReservedFundsReleaseCommand {
                account_id: from_account_id,
                amount,
            }
        );
    }

    #[test]
    fn funds_reserve_failure_appends_transfer_fail_command() {
        let saga = TransferSaga;
        let correlation_id = CorrelationId::from(Uuid::now_v7());
        let from_account_id = AccountId::new();
        let to_account_id = AccountId::new();
        let transfer_id = TransferId::new();
        let amount = CurrencyAmount::new(100);
        let mut instance = SagaInstance::<TransferSagaState, TransferSagaStep>::new(
            SagaNameOwned::from(TransferSagaSpec::DESCRIPTOR.name),
            correlation_id,
            EventId::new(),
        );

        *instance.state_mut() = Some(TransferSagaState {
            from_account_id,
            to_account_id,
            amount,
            transfer_id,
        });

        let failure = command_failure(
            &instance,
            TransferSagaStep::ReserveFunds,
            &AccountFundsReserveCommand {
                account_id: from_account_id,
                amount,
            },
        );
        saga.on_command_failed(&mut instance, &failure, TransferSagaStep::ReserveFunds)
            .expect("funds reservation failure should succeed");

        let fail = instance.uncommitted_commands()[0]
            .try_into_command::<TransferFailCommand>()
            .expect("command should deserialize");
        assert_eq!(
            fail,
            TransferFailCommand {
                transfer_id,
                reason: TransferFailureReason::FundsReserveRejected,
            }
        );
    }

    #[test]
    fn reserved_funds_release_failure_appends_transfer_fail_command() {
        let saga = TransferSaga;
        let correlation_id = CorrelationId::from(Uuid::now_v7());
        let from_account_id = AccountId::new();
        let to_account_id = AccountId::new();
        let transfer_id = TransferId::new();
        let amount = CurrencyAmount::new(100);
        let mut instance = SagaInstance::<TransferSagaState, TransferSagaStep>::new(
            SagaNameOwned::from(TransferSagaSpec::DESCRIPTOR.name),
            correlation_id,
            EventId::new(),
        );

        *instance.state_mut() = Some(TransferSagaState {
            from_account_id,
            to_account_id,
            amount,
            transfer_id,
        });

        let failure = command_failure(
            &instance,
            TransferSagaStep::ReleaseFunds,
            &AccountReservedFundsReleaseCommand {
                account_id: from_account_id,
                amount,
            },
        );
        saga.on_command_failed(&mut instance, &failure, TransferSagaStep::ReleaseFunds)
            .expect("reserved funds release failure should succeed");

        let fail = instance.uncommitted_commands()[0]
            .try_into_command::<TransferFailCommand>()
            .expect("command should deserialize");
        assert_eq!(
            fail,
            TransferFailCommand {
                transfer_id,
                reason: TransferFailureReason::ReservedFundsReleaseRejected,
            }
        );
    }

    #[test]
    fn reserved_funds_commit_failure_appends_deposit_compensation_command() {
        let saga = TransferSaga;
        let correlation_id = CorrelationId::from(Uuid::now_v7());
        let from_account_id = AccountId::new();
        let to_account_id = AccountId::new();
        let transfer_id = TransferId::new();
        let amount = CurrencyAmount::new(100);
        let mut instance = SagaInstance::<TransferSagaState, TransferSagaStep>::new(
            SagaNameOwned::from(TransferSagaSpec::DESCRIPTOR.name),
            correlation_id,
            EventId::new(),
        );

        *instance.state_mut() = Some(TransferSagaState {
            from_account_id,
            to_account_id,
            amount,
            transfer_id,
        });

        let failure = command_failure(
            &instance,
            TransferSagaStep::CommitFunds,
            &AccountReservedFundsCommitCommand {
                account_id: from_account_id,
                amount,
            },
        );
        saga.on_command_failed(&mut instance, &failure, TransferSagaStep::CommitFunds)
            .expect("reserved funds commit failure should succeed");

        let withdraw = instance.uncommitted_commands()[0]
            .try_into_command::<AccountWithdrawCommand>()
            .expect("command should deserialize");
        assert_eq!(
            withdraw,
            AccountWithdrawCommand {
                account_id: to_account_id,
                amount,
            }
        );
    }

    #[test]
    fn withdrawn_after_commit_rejection_appends_transfer_fail_command() {
        let saga = TransferSaga;
        let correlation_id = CorrelationId::from(Uuid::now_v7());
        let from_account_id = AccountId::new();
        let to_account_id = AccountId::new();
        let transfer_id = TransferId::new();
        let amount = CurrencyAmount::new(100);
        let mut instance = SagaInstance::<TransferSagaState, TransferSagaStep>::new(
            SagaNameOwned::from(TransferSagaSpec::DESCRIPTOR.name),
            correlation_id,
            EventId::new(),
        );

        *instance.state_mut() = Some(TransferSagaState {
            from_account_id,
            to_account_id,
            amount,
            transfer_id,
        });

        handle_event(
            &saga,
            &mut instance,
            &account_event_envelope(
                correlation_id,
                to_account_id,
                AccountEventPayload::Withdrawn { amount },
            ),
            Some(TransferSagaStep::CompensateDeposit),
        )
        .expect("withdrawn should succeed");

        let fail = instance.uncommitted_commands()[0]
            .try_into_command::<TransferFailCommand>()
            .expect("command should deserialize");
        assert_eq!(
            fail,
            TransferFailCommand {
                transfer_id,
                reason: TransferFailureReason::ReservedFundsCommitRejected,
            }
        );
    }

    #[test]
    fn failed_transfer_marks_saga_failed() {
        let saga = TransferSaga;
        let correlation_id = CorrelationId::from(Uuid::now_v7());
        let from_account_id = AccountId::new();
        let to_account_id = AccountId::new();
        let transfer_id = TransferId::new();
        let amount = CurrencyAmount::new(100);
        let mut instance = SagaInstance::<TransferSagaState, TransferSagaStep>::new(
            SagaNameOwned::from(TransferSagaSpec::DESCRIPTOR.name),
            correlation_id,
            EventId::new(),
        );

        *instance.state_mut() = Some(TransferSagaState {
            from_account_id,
            to_account_id,
            amount,
            transfer_id,
        });

        handle_event(
            &saga,
            &mut instance,
            &transfer_event_envelope(
                correlation_id,
                transfer_id,
                TransferEventPayload::Failed {
                    reason: TransferFailureReason::FundsReserveRejected,
                },
            ),
            Some(TransferSagaStep::Fail),
        )
        .expect("failed should succeed");

        assert_eq!(instance.status, SagaStatus::Failed);
    }

    #[test]
    fn deposited_event_from_another_step_is_ignored() {
        let saga = TransferSaga;
        let correlation_id = CorrelationId::from(Uuid::now_v7());
        let from_account_id = AccountId::new();
        let to_account_id = AccountId::new();
        let transfer_id = TransferId::new();
        let amount = CurrencyAmount::new(100);
        let mut instance = SagaInstance::<TransferSagaState, TransferSagaStep>::new(
            SagaNameOwned::from(TransferSagaSpec::DESCRIPTOR.name),
            correlation_id,
            EventId::new(),
        );
        *instance.state_mut() = Some(TransferSagaState {
            from_account_id,
            to_account_id,
            amount,
            transfer_id,
        });

        handle_event(
            &saga,
            &mut instance,
            &account_event_envelope(
                correlation_id,
                to_account_id,
                AccountEventPayload::Deposited { amount },
            ),
            Some(TransferSagaStep::ReleaseFunds),
        )
        .expect("unmatched step should be ignored");

        assert!(instance.uncommitted_commands().is_empty());
    }
}
