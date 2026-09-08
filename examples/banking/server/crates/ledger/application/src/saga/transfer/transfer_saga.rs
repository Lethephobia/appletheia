use super::{TransferSagaHandlerError, TransferSagaState, TransferSagaStep};
use crate::command::{
    AccountDepositCommand, AccountFundsReserveCommand, AccountReservedFundsCommitCommand,
    AccountReservedFundsReleaseCommand, AccountWithdrawCommand, TransferCompleteCommand,
    TransferFailCommand,
};
use appletheia::application::saga::SagaError;
use appletheia::application::saga::{Saga, SagaDefinition, SagaDefinitionBuilder, SagaName};
use banking_ledger_domain::account::{Account, AccountEventPayload};
use banking_ledger_domain::transfer::{Transfer, TransferEventPayload, TransferFailureReason};

/// Coordinates the transfer flow.
pub struct TransferSaga;

impl Saga for TransferSaga {
    type State = TransferSagaState;
    type Step = TransferSagaStep;
    type HandlerError = TransferSagaHandlerError;

    fn definition(
        &self,
    ) -> Result<SagaDefinition<'_, Self::State, Self::Step, Self::HandlerError>, SagaError> {
        SagaDefinitionBuilder::<Self::State, Self::Step, Self::HandlerError>::new(SagaName::new(
            "transfer",
        ))
        .add_start_step(TransferSagaStep::ReserveFunds)
        .on::<Transfer>(TransferEventPayload::REQUESTED)
        .handle(|ctx, transfer_event| {
            if let TransferEventPayload::Requested {
                from_account_id,
                to_account_id,
                amount,
                ..
            } = transfer_event.payload()
            {
                ctx.set_state(TransferSagaState::new(
                    transfer_event.aggregate_id(),
                    *from_account_id,
                    *to_account_id,
                    *amount,
                ));

                ctx.append_command(&AccountFundsReserveCommand {
                    account_id: *from_account_id,
                    amount: *amount,
                })?;
            }
            Ok(())
        })
        .add_step(TransferSagaStep::Deposit)
        .on::<Account>(
            TransferSagaStep::ReserveFunds,
            AccountEventPayload::FUNDS_RESERVED,
        )
        .handle(|ctx, _account_event| {
            let state = ctx.state_required_mut()?;
            let to_account_id = state.to_account_id;
            let amount = state.amount;

            ctx.append_command(&AccountDepositCommand {
                account_id: to_account_id,
                amount,
            })?;
            Ok(())
        })
        .add_step(TransferSagaStep::CommitFunds)
        .on::<Account>(TransferSagaStep::Deposit, AccountEventPayload::DEPOSITED)
        .handle(|ctx, _account_event| {
            let state = ctx.state_required_mut()?;
            let from_account_id = state.from_account_id;
            let amount = state.amount;

            ctx.append_command(&AccountReservedFundsCommitCommand {
                account_id: from_account_id,
                amount,
            })?;
            Ok(())
        })
        .add_step(TransferSagaStep::Fail)
        .on::<Account>(
            TransferSagaStep::ReleaseFunds,
            AccountEventPayload::RESERVED_FUNDS_RELEASED,
        )
        .handle(|ctx, _account_event| {
            let state = ctx.state_required_mut()?;
            let transfer_id = state.transfer_id;

            ctx.append_command(&TransferFailCommand {
                transfer_id,
                reason: TransferFailureReason::DepositRejected,
            })?;
            Ok(())
        })
        .add_step(TransferSagaStep::Complete)
        .on::<Account>(
            TransferSagaStep::CommitFunds,
            AccountEventPayload::RESERVED_FUNDS_COMMITTED,
        )
        .handle(|ctx, _account_event| {
            let state = ctx.state_required_mut()?;
            let transfer_id = state.transfer_id;

            ctx.append_command(&TransferCompleteCommand { transfer_id })?;
            Ok(())
        })
        .add_step(TransferSagaStep::Fail)
        .on::<Account>(
            TransferSagaStep::CompensateDeposit,
            AccountEventPayload::WITHDRAWN,
        )
        .handle(|ctx, _account_event| {
            let state = ctx.state_required_mut()?;
            let transfer_id = state.transfer_id;

            ctx.append_command(&TransferFailCommand {
                transfer_id,
                reason: TransferFailureReason::ReservedFundsCommitRejected,
            })?;
            Ok(())
        })
        .add_failure_step(TransferSagaStep::Fail)
        .on(TransferSagaStep::ReserveFunds)
        .handle(|ctx, _failure| {
            let state = ctx.state_required_mut()?;
            let transfer_id = state.transfer_id;
            ctx.append_command(&TransferFailCommand {
                transfer_id,
                reason: TransferFailureReason::FundsReserveRejected,
            })?;
            Ok(())
        })
        .add_failure_step(TransferSagaStep::ReleaseFunds)
        .on(TransferSagaStep::Deposit)
        .handle(|ctx, _failure| {
            let state = ctx.state_required_mut()?;
            let from_account_id = state.from_account_id;
            let amount = state.amount;
            ctx.append_command(&AccountReservedFundsReleaseCommand {
                account_id: from_account_id,
                amount,
            })?;
            Ok(())
        })
        .add_failure_step(TransferSagaStep::Fail)
        .on(TransferSagaStep::ReleaseFunds)
        .handle(|ctx, _failure| {
            let state = ctx.state_required_mut()?;
            let transfer_id = state.transfer_id;
            ctx.append_command(&TransferFailCommand {
                transfer_id,
                reason: TransferFailureReason::ReservedFundsReleaseRejected,
            })?;
            Ok(())
        })
        .add_failure_step(TransferSagaStep::CompensateDeposit)
        .on(TransferSagaStep::CommitFunds)
        .handle(|ctx, _failure| {
            let state = ctx.state_required_mut()?;
            let account_id = state.to_account_id;
            let amount = state.amount;
            ctx.append_command(&AccountWithdrawCommand { account_id, amount })?;
            Ok(())
        })
        .add_failure_step(TransferSagaStep::Fail)
        .on(TransferSagaStep::CompensateDeposit)
        .handle(|ctx, _failure| {
            let state = ctx.state_required_mut()?;
            let transfer_id = state.transfer_id;
            ctx.append_command(&TransferFailCommand {
                transfer_id,
                reason: TransferFailureReason::ReservedFundsCommitRejected,
            })?;
            Ok(())
        })
        .build()
        .map_err(SagaError::from)
    }
}

#[cfg(test)]
mod tests {
    use super::TransferSagaHandlerError;
    use appletheia::application::authorization::AggregateRef;
    use appletheia::application::saga::SagaRouteError;
    use appletheia::application::saga::{SagaContext, SagaRoute};
    use appletheia::domain::AggregateVersion;
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
        Saga, SagaCommandOrigin, SagaInstance, SagaNameOwned, SerializedSagaStep,
    };
    use appletheia::domain::{Aggregate, AggregateId, EventId, EventOccurredAt, EventPayload};
    use banking_iam_domain::{User, UserId};
    use banking_ledger_domain::account::{Account, AccountEventPayload, AccountId};
    use banking_ledger_domain::core::CurrencyAmount;
    use banking_ledger_domain::transfer::{
        Transfer, TransferEventPayload, TransferFailureReason, TransferId, TransferNote,
    };

    use super::{TransferSaga, TransferSagaState, TransferSagaStep};
    use crate::command::{
        AccountDepositCommand, AccountFundsReserveCommand, AccountReservedFundsCommitCommand,
        AccountReservedFundsReleaseCommand, AccountWithdrawCommand, TransferCompleteCommand,
        TransferFailCommand,
    };

    fn request_context(correlation_id: CorrelationId) -> RequestContext {
        let subject = AggregateRef::from_id::<User>(UserId::new());

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
            aggregate_version: AggregateVersion::try_from(1).expect("version should be valid"),
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
            aggregate_version: AggregateVersion::try_from(1).expect("version should be valid"),
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

    fn handle_command_failure(
        saga: &TransferSaga,
        instance: &mut SagaInstance<TransferSagaState, TransferSagaStep>,
        failure: &CommandFailureEnvelope,
        step: TransferSagaStep,
    ) -> Result<(), TransferSagaHandlerError> {
        let definition = saga.definition().expect("valid saga definition");
        let Some(SagaRoute::OnCommandFailed {
            step: route_step,
            handler,
            ..
        }) = definition.find_command_failure_route(step)
        else {
            panic!("failure route");
        };
        let mut context =
            SagaContext::new(instance, CausationId::from(failure.failure_id), *route_step);
        handler(&mut context, failure)
    }

    fn handle_event(
        saga: &TransferSaga,
        instance: &mut SagaInstance<TransferSagaState, TransferSagaStep>,
        envelope: &EventEnvelope,
        step: Option<TransferSagaStep>,
    ) -> Result<bool, SagaRouteError<TransferSagaHandlerError>> {
        let definition = saga.definition().expect("valid saga definition");
        let Some(
            SagaRoute::StartsOn {
                step: route_step,
                handler,
                ..
            }
            | SagaRoute::OnEvent {
                step: route_step,
                handler,
                ..
            },
        ) = definition.find_event_route(envelope, step)
        else {
            return Ok(false);
        };
        let mut context =
            SagaContext::new(instance, CausationId::from(envelope.event_id), *route_step);
        handler(&mut context, envelope)?;
        Ok(true)
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
            SagaNameOwned::from(
                TransferSaga
                    .definition()
                    .expect("valid saga definition")
                    .name(),
            ),
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
    fn success_path_appends_expected_follow_up_commands() {
        let saga = TransferSaga;
        let correlation_id = CorrelationId::from(Uuid::now_v7());
        let from_account_id = AccountId::new();
        let to_account_id = AccountId::new();
        let transfer_id = TransferId::new();
        let amount = CurrencyAmount::new(100);
        let mut instance = SagaInstance::<TransferSagaState, TransferSagaStep>::new(
            SagaNameOwned::from(
                TransferSaga
                    .definition()
                    .expect("valid saga definition")
                    .name(),
            ),
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

        assert!(instance.uncommitted_commands().is_empty());
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
            SagaNameOwned::from(
                TransferSaga
                    .definition()
                    .expect("valid saga definition")
                    .name(),
            ),
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
            SagaNameOwned::from(
                TransferSaga
                    .definition()
                    .expect("valid saga definition")
                    .name(),
            ),
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
        handle_command_failure(&saga, &mut instance, &failure, TransferSagaStep::Deposit)
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
            SagaNameOwned::from(
                TransferSaga
                    .definition()
                    .expect("valid saga definition")
                    .name(),
            ),
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
        handle_command_failure(
            &saga,
            &mut instance,
            &failure,
            TransferSagaStep::ReserveFunds,
        )
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
            SagaNameOwned::from(
                TransferSaga
                    .definition()
                    .expect("valid saga definition")
                    .name(),
            ),
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
        handle_command_failure(
            &saga,
            &mut instance,
            &failure,
            TransferSagaStep::ReleaseFunds,
        )
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
            SagaNameOwned::from(
                TransferSaga
                    .definition()
                    .expect("valid saga definition")
                    .name(),
            ),
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
        handle_command_failure(
            &saga,
            &mut instance,
            &failure,
            TransferSagaStep::CommitFunds,
        )
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
            SagaNameOwned::from(
                TransferSaga
                    .definition()
                    .expect("valid saga definition")
                    .name(),
            ),
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
    fn failed_transfer_does_not_require_a_completion_route() {
        let saga = TransferSaga;
        let correlation_id = CorrelationId::from(Uuid::now_v7());
        let from_account_id = AccountId::new();
        let to_account_id = AccountId::new();
        let transfer_id = TransferId::new();
        let amount = CurrencyAmount::new(100);
        let mut instance = SagaInstance::<TransferSagaState, TransferSagaStep>::new(
            SagaNameOwned::from(
                TransferSaga
                    .definition()
                    .expect("valid saga definition")
                    .name(),
            ),
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

        assert!(instance.uncommitted_commands().is_empty());
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
            SagaNameOwned::from(
                TransferSaga
                    .definition()
                    .expect("valid saga definition")
                    .name(),
            ),
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
