use appletheia::application::command::{CommandFailureReaction, CommandOptions};
use appletheia::application::event::EventEnvelope;
use appletheia::application::saga::{Saga, SagaInstance, SagaSpec};
use appletheia::domain::Aggregate;
use banking_ledger_domain::account::{Account, AccountEventPayload};
use banking_ledger_domain::transfer::{Transfer, TransferEventPayload};

use super::{TransferSagaError, TransferSagaSpec, TransferSagaState, TransferSagaStatus};
use crate::command::{
    AccountCommitReservedFundsCommand, AccountDepositCommand, AccountReleaseReservedFundsCommand,
    AccountReserveFundsCommand, TransferCompleteCommand, TransferFailCommand,
};

/// Coordinates the transfer flow.
pub struct TransferSaga;

impl Saga for TransferSaga {
    type Spec = TransferSagaSpec;
    type Error = TransferSagaError;

    fn on_event(
        &self,
        instance: &mut SagaInstance<<Self::Spec as SagaSpec>::State>,
        event: &EventEnvelope,
    ) -> Result<(), Self::Error> {
        if event.aggregate_type.value() == Transfer::TYPE.value() {
            let transfer_event = event.try_into_domain_event::<Transfer>()?;
            match transfer_event.payload() {
                TransferEventPayload::Requested {
                    id,
                    from_account_id,
                    to_account_id,
                    amount,
                } => {
                    *instance.state_mut() = Some(TransferSagaState::new(
                        *id,
                        *from_account_id,
                        *to_account_id,
                        *amount,
                    ));

                    instance.append_command(
                        event,
                        &AccountReserveFundsCommand {
                            account_id: *from_account_id,
                            amount: *amount,
                        },
                        CommandOptions {
                            failure_reaction: {
                                let mut reaction = CommandFailureReaction::new();
                                reaction.push(
                                    &TransferFailCommand { transfer_id: *id },
                                    CommandOptions::default(),
                                )?;
                                reaction
                            },
                            ..CommandOptions::default()
                        },
                    )?;
                }
                TransferEventPayload::Completed => {
                    if let Some(state) = instance.state_mut().as_mut() {
                        state.status = TransferSagaStatus::Completed;
                    }
                    instance.succeed();
                }
                TransferEventPayload::Failed => {
                    if let Some(state) = instance.state_mut().as_mut() {
                        state.status = TransferSagaStatus::Failed;
                    }
                    instance.fail();
                }
                _ => {}
            }

            return Ok(());
        }

        if event.aggregate_type.value() == Account::TYPE.value() {
            let account_event = event.try_into_domain_event::<Account>()?;
            match account_event.payload() {
                AccountEventPayload::FundsReserved { .. } => {
                    let (transfer_id, from_account_id, to_account_id, amount) = {
                        let Some(state) = instance.state_mut().as_mut() else {
                            return Ok(());
                        };
                        state.status = TransferSagaStatus::FundsReserved;
                        (
                            state.transfer_id,
                            state.from_account_id,
                            state.to_account_id,
                            state.amount,
                        )
                    };

                    instance.append_command(
                        event,
                        &AccountDepositCommand {
                            account_id: to_account_id,
                            amount,
                        },
                        CommandOptions {
                            failure_reaction: {
                                let mut reaction = CommandFailureReaction::new();
                                reaction.push(
                                    &AccountReleaseReservedFundsCommand {
                                        account_id: from_account_id,
                                        amount,
                                    },
                                    CommandOptions {
                                        failure_reaction: {
                                            let mut reaction = CommandFailureReaction::new();
                                            reaction.push(
                                                &TransferFailCommand { transfer_id },
                                                CommandOptions::default(),
                                            )?;
                                            reaction
                                        },
                                        ..CommandOptions::default()
                                    },
                                )?;
                                reaction
                            },
                            ..CommandOptions::default()
                        },
                    )?;
                }
                AccountEventPayload::Deposited { .. } => {
                    let (from_account_id, amount) = {
                        let Some(state) = instance.state_mut().as_mut() else {
                            return Ok(());
                        };
                        state.status = TransferSagaStatus::Deposited;
                        (state.from_account_id, state.amount)
                    };

                    instance.append_command(
                        event,
                        &AccountCommitReservedFundsCommand {
                            account_id: from_account_id,
                            amount,
                        },
                        CommandOptions::default(),
                    )?;
                }
                AccountEventPayload::ReservedFundsReleased { .. } => {
                    let transfer_id = {
                        let Some(state) = instance.state_mut().as_mut() else {
                            return Ok(());
                        };
                        state.status = TransferSagaStatus::ReservedFundsReleased;
                        state.transfer_id
                    };

                    instance.append_command(
                        event,
                        &TransferFailCommand { transfer_id },
                        CommandOptions::default(),
                    )?;
                }
                AccountEventPayload::ReservedFundsCommitted { .. } => {
                    let transfer_id = {
                        let Some(state) = instance.state_mut().as_mut() else {
                            return Ok(());
                        };
                        state.status = TransferSagaStatus::ReservedFundsCommitted;
                        state.transfer_id
                    };

                    instance.append_command(
                        event,
                        &TransferCompleteCommand { transfer_id },
                        CommandOptions::default(),
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
    use appletheia::application::command::{CommandFailureReaction, CommandOptions};
    use appletheia::application::event::{
        AggregateIdValue, AggregateTypeOwned, EventEnvelope, EventNameOwned, EventSequence,
        SerializedEventPayload,
    };
    use appletheia::application::request_context::{
        ActorRef, CausationId, CorrelationId, MessageId, Principal, RequestContext,
    };
    use appletheia::application::saga::{Saga, SagaInstance, SagaNameOwned, SagaSpec, SagaStatus};
    use appletheia::domain::{Aggregate, AggregateId, EventId, EventOccurredAt, EventPayload};
    use banking_iam_domain::{User, UserId};
    use banking_ledger_domain::account::{Account, AccountEventPayload, AccountId};
    use banking_ledger_domain::core::CurrencyAmount;
    use banking_ledger_domain::transfer::{Transfer, TransferEventPayload, TransferId};

    use super::{TransferSaga, TransferSagaSpec, TransferSagaState, TransferSagaStatus};
    use crate::command::{
        AccountCommitReservedFundsCommand, AccountDepositCommand,
        AccountReleaseReservedFundsCommand, AccountReserveFundsCommand, TransferCompleteCommand,
        TransferFailCommand,
    };

    fn request_context(correlation_id: CorrelationId) -> RequestContext {
        let subject =
            appletheia::application::authorization::AggregateRef::from_id::<User>(UserId::new());

        RequestContext::new(
            correlation_id,
            MessageId::new(),
            ActorRef::Subject {
                subject: subject.clone(),
            },
            Principal::Authenticated { subject },
        )
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

    #[test]
    fn transfer_requested_appends_account_reserve_funds_command() {
        let saga = TransferSaga;
        let correlation_id = CorrelationId::from(uuid::Uuid::now_v7());
        let from_account_id = AccountId::new();
        let to_account_id = AccountId::new();
        let transfer_id = TransferId::new();
        let amount = CurrencyAmount::new(100);
        let mut instance = SagaInstance::<TransferSagaState>::new(
            SagaNameOwned::from(TransferSagaSpec::DESCRIPTOR.name),
            correlation_id,
        );

        saga.on_event(
            &mut instance,
            &transfer_event_envelope(
                correlation_id,
                transfer_id,
                TransferEventPayload::Requested {
                    id: transfer_id,
                    from_account_id,
                    to_account_id,
                    amount,
                },
            ),
        )
        .expect("saga should succeed");

        assert_eq!(instance.uncommitted_commands().len(), 1);
        assert_eq!(
            instance.state.as_ref().map(|state| &state.status),
            Some(&TransferSagaStatus::Requested)
        );
        let command = instance.uncommitted_commands()[0]
            .try_into_command::<AccountReserveFundsCommand>()
            .expect("command should deserialize");
        assert_eq!(
            command,
            AccountReserveFundsCommand {
                account_id: from_account_id,
                amount,
            }
        );
        assert_eq!(
            instance.uncommitted_commands()[0].options.failure_reaction,
            {
                let mut reaction = CommandFailureReaction::new();
                reaction
                    .push(
                        &TransferFailCommand { transfer_id },
                        CommandOptions::default(),
                    )
                    .expect("reaction should serialize");
                reaction
            }
        );
    }

    #[test]
    fn success_path_appends_expected_follow_up_commands_and_succeeds() {
        let saga = TransferSaga;
        let correlation_id = CorrelationId::from(uuid::Uuid::now_v7());
        let from_account_id = AccountId::new();
        let to_account_id = AccountId::new();
        let transfer_id = TransferId::new();
        let amount = CurrencyAmount::new(100);
        let mut instance = SagaInstance::<TransferSagaState>::new(
            SagaNameOwned::from(TransferSagaSpec::DESCRIPTOR.name),
            correlation_id,
        );

        saga.on_event(
            &mut instance,
            &transfer_event_envelope(
                correlation_id,
                transfer_id,
                TransferEventPayload::Requested {
                    id: transfer_id,
                    from_account_id,
                    to_account_id,
                    amount,
                },
            ),
        )
        .expect("requested should succeed");
        let reserve = instance.uncommitted_commands()[0]
            .try_into_command::<AccountReserveFundsCommand>()
            .expect("command should deserialize");
        assert_eq!(
            instance.state.as_ref().map(|state| &state.status),
            Some(&TransferSagaStatus::Requested)
        );
        assert_eq!(
            reserve,
            AccountReserveFundsCommand {
                account_id: from_account_id,
                amount,
            }
        );
        assert_eq!(
            instance.uncommitted_commands()[0].options.failure_reaction,
            {
                let mut reaction = CommandFailureReaction::new();
                reaction
                    .push(
                        &TransferFailCommand { transfer_id },
                        CommandOptions::default(),
                    )
                    .expect("reaction should serialize");
                reaction
            }
        );

        instance.clear_uncommitted_commands();
        saga.on_event(
            &mut instance,
            &account_event_envelope(
                correlation_id,
                from_account_id,
                AccountEventPayload::FundsReserved { amount },
            ),
        )
        .expect("funds reserved should succeed");
        let deposit = instance.uncommitted_commands()[0]
            .try_into_command::<AccountDepositCommand>()
            .expect("command should deserialize");
        assert_eq!(
            instance.state.as_ref().map(|state| &state.status),
            Some(&TransferSagaStatus::FundsReserved)
        );
        assert_eq!(
            deposit,
            AccountDepositCommand {
                account_id: to_account_id,
                amount,
            }
        );
        assert_eq!(
            instance.uncommitted_commands()[0].options.failure_reaction,
            {
                let mut reaction = CommandFailureReaction::new();
                reaction
                    .push(
                        &AccountReleaseReservedFundsCommand {
                            account_id: from_account_id,
                            amount,
                        },
                        CommandOptions {
                            failure_reaction: {
                                let mut reaction = CommandFailureReaction::new();
                                reaction
                                    .push(
                                        &TransferFailCommand { transfer_id },
                                        CommandOptions::default(),
                                    )
                                    .expect("reaction should serialize");
                                reaction
                            },
                            ..CommandOptions::default()
                        },
                    )
                    .expect("reaction should serialize");
                reaction
            }
        );

        instance.clear_uncommitted_commands();
        saga.on_event(
            &mut instance,
            &account_event_envelope(
                correlation_id,
                to_account_id,
                AccountEventPayload::Deposited { amount },
            ),
        )
        .expect("deposited should succeed");
        let commit = instance.uncommitted_commands()[0]
            .try_into_command::<AccountCommitReservedFundsCommand>()
            .expect("command should deserialize");
        assert_eq!(
            instance.state.as_ref().map(|state| &state.status),
            Some(&TransferSagaStatus::Deposited)
        );
        assert_eq!(
            commit,
            AccountCommitReservedFundsCommand {
                account_id: from_account_id,
                amount,
            }
        );
        assert_eq!(
            instance.uncommitted_commands()[0].options.failure_reaction,
            CommandFailureReaction::None
        );

        instance.clear_uncommitted_commands();
        saga.on_event(
            &mut instance,
            &account_event_envelope(
                correlation_id,
                from_account_id,
                AccountEventPayload::ReservedFundsCommitted { amount },
            ),
        )
        .expect("reserved funds committed should succeed");
        let complete = instance.uncommitted_commands()[0]
            .try_into_command::<TransferCompleteCommand>()
            .expect("command should deserialize");
        assert_eq!(complete, TransferCompleteCommand { transfer_id });
        assert_eq!(
            instance.state.as_ref().map(|state| &state.status),
            Some(&TransferSagaStatus::ReservedFundsCommitted)
        );

        instance.clear_uncommitted_commands();
        saga.on_event(
            &mut instance,
            &transfer_event_envelope(correlation_id, transfer_id, TransferEventPayload::Completed),
        )
        .expect("completed should succeed");

        assert_eq!(instance.status, SagaStatus::Succeeded);
    }

    #[test]
    fn reserved_funds_released_appends_transfer_fail_command() {
        let saga = TransferSaga;
        let correlation_id = CorrelationId::from(uuid::Uuid::now_v7());
        let from_account_id = AccountId::new();
        let to_account_id = AccountId::new();
        let transfer_id = TransferId::new();
        let amount = CurrencyAmount::new(100);
        let mut instance = SagaInstance::<TransferSagaState>::new(
            SagaNameOwned::from(TransferSagaSpec::DESCRIPTOR.name),
            correlation_id,
        );

        *instance.state_mut() = Some(TransferSagaState {
            from_account_id,
            to_account_id,
            amount,
            transfer_id,
            status: TransferSagaStatus::FundsReserved,
        });

        saga.on_event(
            &mut instance,
            &account_event_envelope(
                correlation_id,
                from_account_id,
                AccountEventPayload::ReservedFundsReleased { amount },
            ),
        )
        .expect("reserved funds released should succeed");

        let fail = instance.uncommitted_commands()[0]
            .try_into_command::<TransferFailCommand>()
            .expect("command should deserialize");
        assert_eq!(fail, TransferFailCommand { transfer_id });
        assert_eq!(
            instance.state.as_ref().map(|state| &state.status),
            Some(&TransferSagaStatus::ReservedFundsReleased)
        );
    }

    #[test]
    fn failed_transfer_marks_saga_failed() {
        let saga = TransferSaga;
        let correlation_id = CorrelationId::from(uuid::Uuid::now_v7());
        let transfer_id = TransferId::new();
        let mut instance = SagaInstance::<TransferSagaState>::new(
            SagaNameOwned::from(TransferSagaSpec::DESCRIPTOR.name),
            correlation_id,
        );

        saga.on_event(
            &mut instance,
            &transfer_event_envelope(correlation_id, transfer_id, TransferEventPayload::Failed),
        )
        .expect("failed should succeed");

        assert_eq!(instance.status, SagaStatus::Failed);
    }
}
