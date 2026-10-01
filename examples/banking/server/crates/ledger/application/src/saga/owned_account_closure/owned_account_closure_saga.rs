use appletheia::application::saga::{
    Saga, SagaDefinition, SagaDefinitionBuilder, SagaError, SagaName,
};
use banking_iam_domain::{Organization, OrganizationEventPayload, User, UserEventPayload};
use banking_ledger_domain::account::{Account, AccountEventPayload, AccountOwner};
use banking_ledger_domain::owned_account_closure::{
    OwnedAccountClosure, OwnedAccountClosureEventPayload,
};

use super::{
    OwnedAccountClosureSagaHandlerError, OwnedAccountClosureSagaState, OwnedAccountClosureSagaStep,
};
use crate::command::{
    AccountCloseCommand, OwnedAccountClosureFailedRecordCommand, OwnedAccountClosureScanCommand,
    OwnedAccountClosureStartCommand, OwnedAccountClosureSucceededRecordCommand,
};

/// Enumerates a removed owner's accounts without waiting for earlier close attempts to finish.
pub struct OwnedAccountClosureSaga;

impl Saga for OwnedAccountClosureSaga {
    type State = OwnedAccountClosureSagaState;
    type Step = OwnedAccountClosureSagaStep;
    type HandlerError = OwnedAccountClosureSagaHandlerError;

    fn definition(
        &self,
    ) -> Result<SagaDefinition<'_, Self::State, Self::Step, Self::HandlerError>, SagaError> {
        SagaDefinitionBuilder::<Self::State, Self::Step, Self::HandlerError>::new(SagaName::new(
            "owned_account_closure",
        ))
        .add_start_step(OwnedAccountClosureSagaStep::Start)
        .on::<User>(UserEventPayload::REMOVED)
        .handle(|ctx, event| {
            ctx.append_command(&OwnedAccountClosureStartCommand {
                owner: AccountOwner::User(event.aggregate_id()),
            })?;
            Ok(())
        })
        .add_start_step(OwnedAccountClosureSagaStep::Start)
        .on::<Organization>(OrganizationEventPayload::REMOVED)
        .handle(|ctx, event| {
            ctx.append_command(&OwnedAccountClosureStartCommand {
                owner: AccountOwner::Organization(event.aggregate_id()),
            })?;
            Ok(())
        })
        .add_step(OwnedAccountClosureSagaStep::Scan)
        .on::<OwnedAccountClosure>(
            OwnedAccountClosureSagaStep::Start,
            OwnedAccountClosureEventPayload::STARTED,
        )
        .handle(|ctx, event| {
            let owned_account_closure_id = event.aggregate_id();
            ctx.set_state(OwnedAccountClosureSagaState {
                owned_account_closure_id,
            });
            ctx.append_command(&OwnedAccountClosureScanCommand {
                owned_account_closure_id,
            })?;
            Ok(())
        })
        .add_step(OwnedAccountClosureSagaStep::Scan)
        .on::<OwnedAccountClosure>(
            OwnedAccountClosureSagaStep::Scan,
            OwnedAccountClosureEventPayload::SCANNED,
        )
        .handle(|ctx, event| {
            if let OwnedAccountClosureEventPayload::Scanned {
                next_cursor: Some(_),
            } = event.payload()
            {
                ctx.append_command(&OwnedAccountClosureScanCommand {
                    owned_account_closure_id: event.aggregate_id(),
                })?;
            }
            Ok(())
        })
        .add_step(OwnedAccountClosureSagaStep::CloseAccount)
        .on::<OwnedAccountClosure>(
            OwnedAccountClosureSagaStep::Scan,
            OwnedAccountClosureEventPayload::REQUESTED,
        )
        .handle(|ctx, event| {
            if let OwnedAccountClosureEventPayload::Requested { account_id } = event.payload() {
                ctx.append_command(&AccountCloseCommand {
                    account_id: *account_id,
                })?;
            }
            Ok(())
        })
        .add_step(OwnedAccountClosureSagaStep::RecordSucceeded)
        .on::<Account>(
            OwnedAccountClosureSagaStep::CloseAccount,
            AccountEventPayload::CLOSED,
        )
        .handle(|ctx, event| {
            let owned_account_closure_id = ctx.state_required()?.owned_account_closure_id;
            ctx.append_command(&OwnedAccountClosureSucceededRecordCommand {
                owned_account_closure_id,
                account_id: event.aggregate_id(),
            })?;
            Ok(())
        })
        .add_failure_step(OwnedAccountClosureSagaStep::RecordFailed)
        .on(OwnedAccountClosureSagaStep::CloseAccount)
        .handle(|ctx, failure| {
            let command = failure.try_to_command::<AccountCloseCommand>()?;
            let owned_account_closure_id = ctx.state_required()?.owned_account_closure_id;
            ctx.append_command(&OwnedAccountClosureFailedRecordCommand {
                owned_account_closure_id,
                account_id: command.account_id,
            })?;
            Ok(())
        })
        .build()
        .map_err(SagaError::from)
    }
}

#[cfg(test)]
mod tests {
    use super::OwnedAccountClosureSagaHandlerError;
    use appletheia::application::saga::{SagaContext, SagaRoute};
    use appletheia::domain::AggregateVersion;
    use uuid::Uuid;

    use appletheia::application::aggregate::{AggregateIdValue, AggregateRef, AggregateTypeOwned};
    use appletheia::application::event::{
        EventEnvelope, EventNameOwned, EventSequence, SerializedEventPayload,
    };
    use appletheia::application::request_context::{
        CausationId, CorrelationId, MessageId, Principal, RequestContext,
    };
    use appletheia::application::saga::{Saga, SagaInstance, SagaNameOwned};
    use appletheia::domain::{
        Aggregate, AggregateId, AggregateType, EventId, EventOccurredAt, EventPayload,
    };
    use banking_iam_domain::{User, UserEventPayload, UserId};
    use banking_ledger_domain::account::{AccountId, AccountOwner};
    use banking_ledger_domain::owned_account_closure::{
        OwnedAccountClosure, OwnedAccountClosureEventPayload, OwnedAccountClosureId,
    };

    use super::{
        OwnedAccountClosureSaga, OwnedAccountClosureSagaState, OwnedAccountClosureSagaStep,
    };
    use crate::command::{
        AccountCloseCommand, OwnedAccountClosureFailedRecordCommand,
        OwnedAccountClosureScanCommand, OwnedAccountClosureStartCommand,
        OwnedAccountClosureSucceededRecordCommand,
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

    fn event_envelope<P>(
        correlation_id: CorrelationId,
        aggregate_type: AggregateType,
        aggregate_id: impl AggregateId,
        payload: P,
    ) -> EventEnvelope
    where
        P: EventPayload,
    {
        EventEnvelope {
            event_sequence: EventSequence::try_from(1).expect("sequence should be valid"),
            event_id: EventId::new(),
            aggregate_type: AggregateTypeOwned::from(aggregate_type),
            aggregate_id: AggregateIdValue::from(aggregate_id.value()),
            aggregate_version: AggregateVersion::try_from(1).expect("version should be valid"),
            event_name: EventNameOwned::from(payload.name()),
            payload: SerializedEventPayload::try_from(
                payload
                    .try_into_json_value()
                    .expect("payload should serialize"),
            )
            .expect("payload should be valid"),
            occurred_at: EventOccurredAt::now(),
            correlation_id,
            causation_id: CausationId::from(MessageId::new()),
            context: request_context(correlation_id),
        }
    }

    fn user_event_envelope(
        correlation_id: CorrelationId,
        user_id: UserId,
        payload: UserEventPayload,
    ) -> EventEnvelope {
        event_envelope(correlation_id, User::TYPE, user_id, payload)
    }

    fn closure_event_envelope(
        correlation_id: CorrelationId,
        closure_id: OwnedAccountClosureId,
        payload: OwnedAccountClosureEventPayload,
    ) -> EventEnvelope {
        event_envelope(
            correlation_id,
            OwnedAccountClosure::TYPE,
            closure_id,
            payload,
        )
    }

    fn saga_instance(
        correlation_id: CorrelationId,
    ) -> SagaInstance<OwnedAccountClosureSagaState, OwnedAccountClosureSagaStep> {
        SagaInstance::<OwnedAccountClosureSagaState, OwnedAccountClosureSagaStep>::new(
            SagaNameOwned::from(
                OwnedAccountClosureSaga
                    .definition()
                    .expect("valid saga definition")
                    .name(),
            ),
            correlation_id,
            EventId::new(),
        )
    }

    fn handle_event(
        saga: &OwnedAccountClosureSaga,
        instance: &mut SagaInstance<OwnedAccountClosureSagaState, OwnedAccountClosureSagaStep>,
        envelope: &EventEnvelope,
        step: Option<OwnedAccountClosureSagaStep>,
    ) -> Result<bool, OwnedAccountClosureSagaHandlerError> {
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
    fn removed_owner_starts_one_closure() {
        let correlation_id = CorrelationId::from(Uuid::now_v7());
        let user_id = UserId::new();
        let mut instance = saga_instance(correlation_id);
        handle_event(
            &OwnedAccountClosureSaga,
            &mut instance,
            &user_event_envelope(correlation_id, user_id, UserEventPayload::Removed),
            None,
        )
        .unwrap();
        assert_eq!(instance.uncommitted_commands().len(), 1);
        assert_eq!(
            instance.uncommitted_commands()[0]
                .try_to_command::<OwnedAccountClosureStartCommand>()
                .unwrap()
                .owner,
            AccountOwner::User(user_id)
        );
    }

    #[test]
    fn started_loads_first_page_and_remembers_closure_id() {
        let correlation_id = CorrelationId::from(Uuid::now_v7());
        let closure_id = OwnedAccountClosureId::new();
        let mut instance = saga_instance(correlation_id);
        handle_event(
            &OwnedAccountClosureSaga,
            &mut instance,
            &closure_event_envelope(
                correlation_id,
                closure_id,
                OwnedAccountClosureEventPayload::Started {
                    owner: AccountOwner::User(UserId::new()),
                },
            ),
            Some(OwnedAccountClosureSagaStep::Start),
        )
        .unwrap();
        assert_eq!(
            instance.state.as_ref().unwrap().owned_account_closure_id,
            closure_id
        );
        assert_eq!(
            instance.uncommitted_commands()[0]
                .try_to_command::<OwnedAccountClosureScanCommand>()
                .unwrap(),
            OwnedAccountClosureScanCommand {
                owned_account_closure_id: closure_id,
            }
        );
    }

    #[test]
    fn scan_advances_without_waiting_for_close_requests_or_results() {
        let correlation_id = CorrelationId::from(Uuid::now_v7());
        let closure_id = OwnedAccountClosureId::new();
        let account_id = AccountId::new();
        let mut instance = saga_instance(correlation_id);
        instance.state = Some(OwnedAccountClosureSagaState {
            owned_account_closure_id: closure_id,
        });
        // Deliver the page boundary before the individual request.
        handle_event(
            &OwnedAccountClosureSaga,
            &mut instance,
            &closure_event_envelope(
                correlation_id,
                closure_id,
                OwnedAccountClosureEventPayload::Scanned {
                    next_cursor: Some(account_id),
                },
            ),
            Some(OwnedAccountClosureSagaStep::Scan),
        )
        .unwrap();
        assert_eq!(
            instance.uncommitted_commands()[0]
                .try_to_command::<OwnedAccountClosureScanCommand>()
                .unwrap()
                .owned_account_closure_id,
            closure_id
        );
        instance.clear_uncommitted_commands();
        handle_event(
            &OwnedAccountClosureSaga,
            &mut instance,
            &closure_event_envelope(
                correlation_id,
                closure_id,
                OwnedAccountClosureEventPayload::Requested { account_id },
            ),
            Some(OwnedAccountClosureSagaStep::Scan),
        )
        .unwrap();
        assert_eq!(instance.uncommitted_commands().len(), 1);
        assert_eq!(
            instance.uncommitted_commands()[0]
                .try_to_command::<AccountCloseCommand>()
                .unwrap(),
            AccountCloseCommand { account_id }
        );
    }

    #[test]
    fn closed_account_dispatches_succeeded_record() {
        use banking_ledger_domain::account::{Account, AccountEventPayload};
        let correlation_id = CorrelationId::from(Uuid::now_v7());
        let closure_id = OwnedAccountClosureId::new();
        let account_id = AccountId::new();
        let mut instance = saga_instance(correlation_id);
        instance.state = Some(OwnedAccountClosureSagaState {
            owned_account_closure_id: closure_id,
        });
        handle_event(
            &OwnedAccountClosureSaga,
            &mut instance,
            &event_envelope(
                correlation_id,
                Account::TYPE,
                account_id,
                AccountEventPayload::Closed,
            ),
            Some(OwnedAccountClosureSagaStep::CloseAccount),
        )
        .unwrap();
        let command = instance.uncommitted_commands()[0]
            .try_to_command::<OwnedAccountClosureSucceededRecordCommand>()
            .unwrap();
        assert_eq!(command.account_id, account_id);
        assert_eq!(command.owned_account_closure_id, closure_id);
    }

    #[test]
    fn terminal_close_failure_records_the_account_from_the_failed_command() {
        use appletheia::application::command::{
            CommandAttemptCount, CommandFailedAt, CommandFailureEnvelope, CommandTerminalReason,
        };
        let correlation_id = CorrelationId::from(Uuid::now_v7());
        let closure_id = OwnedAccountClosureId::new();
        let account_id = AccountId::new();
        let mut instance = saga_instance(correlation_id);
        instance.state = Some(OwnedAccountClosureSagaState {
            owned_account_closure_id: closure_id,
        });
        handle_event(
            &OwnedAccountClosureSaga,
            &mut instance,
            &closure_event_envelope(
                correlation_id,
                closure_id,
                OwnedAccountClosureEventPayload::Requested { account_id },
            ),
            Some(OwnedAccountClosureSagaStep::Scan),
        )
        .unwrap();
        let command = &instance.uncommitted_commands()[0];
        let failure = CommandFailureEnvelope::new(
            command,
            command.saga_origin.clone().unwrap(),
            CommandTerminalReason::NonRetryable,
            CommandAttemptCount::first(),
            CommandFailedAt::now(),
        );
        instance.clear_uncommitted_commands();
        let definition = OwnedAccountClosureSaga.definition().unwrap();
        let SagaRoute::OnCommandFailed { step, handler, .. } = definition
            .find_command_failure_route(OwnedAccountClosureSagaStep::CloseAccount)
            .unwrap()
        else {
            panic!("expected failure route")
        };
        let mut context = SagaContext::new(&mut instance, failure.causation_id, *step);
        handler(&mut context, &failure).unwrap();
        let record = instance.uncommitted_commands()[0]
            .try_to_command::<OwnedAccountClosureFailedRecordCommand>()
            .unwrap();
        assert_eq!(record.account_id, account_id);
        assert_eq!(record.owned_account_closure_id, closure_id);
    }

    #[test]
    fn scan_completion_and_recorded_results_do_not_dispatch_completion_commands() {
        let correlation_id = CorrelationId::from(Uuid::now_v7());
        let closure_id = OwnedAccountClosureId::new();
        let account_id = AccountId::new();
        let mut instance = saga_instance(correlation_id);
        for (step, payload) in [
            (
                OwnedAccountClosureSagaStep::Scan,
                OwnedAccountClosureEventPayload::Scanned { next_cursor: None },
            ),
            (
                OwnedAccountClosureSagaStep::RecordSucceeded,
                OwnedAccountClosureEventPayload::Succeeded { account_id },
            ),
            (
                OwnedAccountClosureSagaStep::RecordFailed,
                OwnedAccountClosureEventPayload::Failed { account_id },
            ),
        ] {
            assert_eq!(
                handle_event(
                    &OwnedAccountClosureSaga,
                    &mut instance,
                    &closure_event_envelope(correlation_id, closure_id, payload),
                    Some(step)
                )
                .unwrap(),
                step == OwnedAccountClosureSagaStep::Scan
            );
        }
        assert!(instance.uncommitted_commands().is_empty());
    }
}
