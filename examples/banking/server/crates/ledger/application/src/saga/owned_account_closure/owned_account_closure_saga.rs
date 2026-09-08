use super::{
    OwnedAccountClosureSagaHandlerError, OwnedAccountClosureSagaState, OwnedAccountClosureSagaStep,
};
use crate::command::{
    AccountCloseCommand, OwnedAccountClosureAccountCloseRecordCommand,
    OwnedAccountClosureCompleteCommand, OwnedAccountClosureFailCommand,
    OwnedAccountClosurePageLoadCommand, OwnedAccountClosureRequestCommand,
};
use appletheia::application::command::Command;
use appletheia::application::saga::SagaError;
use appletheia::application::saga::{
    Saga, SagaContext, SagaDefinition, SagaDefinitionBuilder, SagaName,
};
use appletheia::domain::Event;
use banking_iam_domain::{Organization, OrganizationEventPayload, User, UserEventPayload};
use banking_ledger_domain::account::{Account, AccountEventPayload, AccountOwner};
use banking_ledger_domain::owned_account_closure::OwnedAccountClosureId;
use banking_ledger_domain::owned_account_closure::{
    OwnedAccountClosure, OwnedAccountClosureEventPayload, OwnedAccountClosureFailureReason,
};

/// Coordinates closing every account owned by a removed owner.
pub struct OwnedAccountClosureSaga;

impl OwnedAccountClosureSaga {
    const PAGE_SIZE: u32 = 100;

    fn process_page(
        ctx: &mut SagaContext<'_, OwnedAccountClosureSagaState, OwnedAccountClosureSagaStep>,
        closure_event: &Event<OwnedAccountClosureId, OwnedAccountClosureEventPayload>,
    ) -> Result<(), OwnedAccountClosureSagaHandlerError> {
        if let OwnedAccountClosureEventPayload::PageLoaded {
            account_ids,
            next_cursor,
        } = closure_event.payload()
        {
            let state = ctx.state_required_mut()?;
            state.set_loaded_page(account_ids.clone(), *next_cursor);
            for account_id in account_ids {
                ctx.append_command(&AccountCloseCommand {
                    account_id: *account_id,
                })?;
            }
            Self::append_next_step(ctx)?;
        }
        Ok(())
    }

    fn append_next_step(
        ctx: &mut SagaContext<'_, OwnedAccountClosureSagaState, OwnedAccountClosureSagaStep>,
    ) -> Result<(), OwnedAccountClosureSagaHandlerError> {
        let state = ctx.state_required_mut()?;
        if state.has_pending_accounts() {
            return Ok(());
        }

        let owned_account_closure_id = state
            .owned_account_closure_id
            .ok_or(OwnedAccountClosureSagaHandlerError::MissingOwnedAccountClosureId)?;

        if state.has_next_page() {
            let cursor = state.next_cursor;
            ctx.append_command(&OwnedAccountClosurePageLoadCommand {
                owned_account_closure_id,
                cursor,
                page_size: Self::PAGE_SIZE,
            })?;
        } else if state.has_rejections() {
            ctx.append_command(&OwnedAccountClosureFailCommand {
                owned_account_closure_id,
                reason: OwnedAccountClosureFailureReason::AccountCloseRejected,
            })?;
        } else {
            ctx.append_command(&OwnedAccountClosureCompleteCommand {
                owned_account_closure_id,
            })?;
        }

        Ok(())
    }
}

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
        .add_start_step(OwnedAccountClosureSagaStep::Request)
        .on::<User>(UserEventPayload::REMOVED)
        .handle(|ctx, event| {
            let owner = AccountOwner::User(event.aggregate_id());
            ctx.set_state(OwnedAccountClosureSagaState::new(owner));
            ctx.append_command(&OwnedAccountClosureRequestCommand { owner })?;
            Ok(())
        })
        .add_start_step(OwnedAccountClosureSagaStep::Request)
        .on::<Organization>(OrganizationEventPayload::REMOVED)
        .handle(|ctx, event| {
            let owner = AccountOwner::Organization(event.aggregate_id());
            ctx.set_state(OwnedAccountClosureSagaState::new(owner));
            ctx.append_command(&OwnedAccountClosureRequestCommand { owner })?;
            Ok(())
        })
        .add_step(OwnedAccountClosureSagaStep::Advance)
        .on::<OwnedAccountClosure>(
            OwnedAccountClosureSagaStep::Request,
            OwnedAccountClosureEventPayload::REQUESTED,
        )
        .handle(|ctx, closure_event| {
            let closure_id = closure_event.aggregate_id();
            let state = ctx.state_required_mut()?;
            state.owned_account_closure_id = Some(closure_id);
            ctx.append_command(&OwnedAccountClosurePageLoadCommand {
                owned_account_closure_id: closure_id,
                cursor: None,
                page_size: Self::PAGE_SIZE,
            })?;
            Ok(())
        })
        .add_step(OwnedAccountClosureSagaStep::ProcessPage)
        .on::<OwnedAccountClosure>(
            OwnedAccountClosureSagaStep::Advance,
            OwnedAccountClosureEventPayload::PAGE_LOADED,
        )
        .handle(Self::process_page)
        .add_step(OwnedAccountClosureSagaStep::ProcessPage)
        .on::<OwnedAccountClosure>(
            OwnedAccountClosureSagaStep::ProcessPage,
            OwnedAccountClosureEventPayload::PAGE_LOADED,
        )
        .handle(Self::process_page)
        .add_step(OwnedAccountClosureSagaStep::Advance)
        .on::<OwnedAccountClosure>(
            OwnedAccountClosureSagaStep::RecordClosedAccount,
            OwnedAccountClosureEventPayload::ACCOUNT_CLOSE_RECORDED,
        )
        .handle(|ctx, closure_event| {
            if let OwnedAccountClosureEventPayload::AccountCloseRecorded { account_id } =
                closure_event.payload()
            {
                let state = ctx.state_required_mut()?;
                state.closed_account_count = state.closed_account_count.saturating_add(1);
                state.remove_pending_account(*account_id);
                Self::append_next_step(ctx)?;
            }
            Ok(())
        })
        .add_step(OwnedAccountClosureSagaStep::Advance)
        .on::<OwnedAccountClosure>(
            OwnedAccountClosureSagaStep::RecordRejectedAccountClose,
            OwnedAccountClosureEventPayload::ACCOUNT_CLOSE_REJECTION_RECORDED,
        )
        .handle(|ctx, closure_event| {
            if let OwnedAccountClosureEventPayload::AccountCloseRejectionRecorded {
                account_id,
                ..
            } = closure_event.payload()
            {
                let state = ctx.state_required_mut()?;
                state.rejected_account_count = state.rejected_account_count.saturating_add(1);
                state.remove_pending_account(*account_id);
                Self::append_next_step(ctx)?;
            }
            Ok(())
        })
        .add_step(OwnedAccountClosureSagaStep::RecordClosedAccount)
        .on::<Account>(
            OwnedAccountClosureSagaStep::ProcessPage,
            AccountEventPayload::CLOSED,
        )
        .handle(|ctx, event| {
            let owned_account_closure_id = ctx
                .state_required()?
                .owned_account_closure_id
                .ok_or(OwnedAccountClosureSagaHandlerError::MissingOwnedAccountClosureId)?;
            ctx.append_command(&OwnedAccountClosureAccountCloseRecordCommand {
                owned_account_closure_id,
                account_id: event.aggregate_id(),
            })?;
            Ok(())
        })
        .add_failure_step(OwnedAccountClosureSagaStep::Fail)
        .on(OwnedAccountClosureSagaStep::Advance)
        .handle(|ctx, failure| {
            if failure.command_name.value() != OwnedAccountClosurePageLoadCommand::NAME.value() {
                return Ok(());
            }

            let owned_account_closure_id = ctx
                .state_required()?
                .owned_account_closure_id
                .ok_or(OwnedAccountClosureSagaHandlerError::MissingOwnedAccountClosureId)?;
            ctx.append_command(&OwnedAccountClosureFailCommand {
                owned_account_closure_id,
                reason: OwnedAccountClosureFailureReason::PageLoadRejected,
            })?;
            Ok(())
        })
        .add_failure_step(OwnedAccountClosureSagaStep::Fail)
        .on(OwnedAccountClosureSagaStep::ProcessPage)
        .handle(|ctx, failure| {
            let reason = if failure.command_name.value() == AccountCloseCommand::NAME.value() {
                OwnedAccountClosureFailureReason::AccountCloseRejected
            } else if failure.command_name.value()
                == OwnedAccountClosurePageLoadCommand::NAME.value()
            {
                OwnedAccountClosureFailureReason::PageLoadRejected
            } else {
                return Ok(());
            };

            let owned_account_closure_id = ctx
                .state_required()?
                .owned_account_closure_id
                .ok_or(OwnedAccountClosureSagaHandlerError::MissingOwnedAccountClosureId)?;
            ctx.append_command(&OwnedAccountClosureFailCommand {
                owned_account_closure_id,
                reason,
            })?;
            Ok(())
        })
        .add_failure_step(OwnedAccountClosureSagaStep::Fail)
        .on(OwnedAccountClosureSagaStep::RecordClosedAccount)
        .handle(|ctx, _failure| {
            let owned_account_closure_id = ctx
                .state_required()?
                .owned_account_closure_id
                .ok_or(OwnedAccountClosureSagaHandlerError::MissingOwnedAccountClosureId)?;
            ctx.append_command(&OwnedAccountClosureFailCommand {
                owned_account_closure_id,
                reason: OwnedAccountClosureFailureReason::AccountCloseRecordRejected,
            })?;
            Ok(())
        })
        .add_failure_step(OwnedAccountClosureSagaStep::Fail)
        .on(OwnedAccountClosureSagaStep::RecordRejectedAccountClose)
        .handle(|ctx, _failure| {
            let owned_account_closure_id = ctx
                .state_required()?
                .owned_account_closure_id
                .ok_or(OwnedAccountClosureSagaHandlerError::MissingOwnedAccountClosureId)?;
            ctx.append_command(&OwnedAccountClosureFailCommand {
                owned_account_closure_id,
                reason: OwnedAccountClosureFailureReason::AccountCloseRejectionRecordRejected,
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
    use appletheia::application::authorization::AggregateRef;
    use appletheia::application::saga::SagaRouteError;
    use appletheia::application::saga::{SagaContext, SagaRoute};
    use appletheia::domain::AggregateVersion;
    use uuid::Uuid;

    use appletheia::application::event::{
        AggregateIdValue, AggregateTypeOwned, EventEnvelope, EventNameOwned, EventSequence,
        SerializedEventPayload,
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
        AccountCloseCommand, OwnedAccountClosureCompleteCommand,
        OwnedAccountClosurePageLoadCommand, OwnedAccountClosureRequestCommand,
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
                payload.into_json_value().expect("payload should serialize"),
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
    ) -> Result<bool, SagaRouteError<OwnedAccountClosureSagaHandlerError>> {
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
    fn user_removed_appends_closure_request_command() {
        let saga = OwnedAccountClosureSaga;
        let correlation_id = CorrelationId::from(Uuid::now_v7());
        let user_id = UserId::new();
        let owner = AccountOwner::User(user_id);
        let mut instance = saga_instance(correlation_id);

        handle_event(
            &saga,
            &mut instance,
            &user_event_envelope(correlation_id, user_id, UserEventPayload::Removed),
            None,
        )
        .expect("saga should succeed");

        assert_eq!(instance.uncommitted_commands().len(), 1);
        let command = instance.uncommitted_commands()[0]
            .try_into_command::<OwnedAccountClosureRequestCommand>()
            .expect("command should deserialize");
        assert_eq!(command, OwnedAccountClosureRequestCommand { owner });
    }

    #[test]
    fn requested_appends_first_page_load_command() {
        let saga = OwnedAccountClosureSaga;
        let correlation_id = CorrelationId::from(Uuid::now_v7());
        let closure_id = OwnedAccountClosureId::new();
        let owner = AccountOwner::User(UserId::new());
        let mut instance = saga_instance(correlation_id);
        instance.state = Some(OwnedAccountClosureSagaState::new(owner));

        handle_event(
            &saga,
            &mut instance,
            &closure_event_envelope(
                correlation_id,
                closure_id,
                OwnedAccountClosureEventPayload::Requested { owner },
            ),
            Some(OwnedAccountClosureSagaStep::Request),
        )
        .expect("saga should succeed");

        let command = instance.uncommitted_commands()[0]
            .try_into_command::<OwnedAccountClosurePageLoadCommand>()
            .expect("command should deserialize");
        assert_eq!(
            command,
            OwnedAccountClosurePageLoadCommand {
                owned_account_closure_id: closure_id,
                cursor: None,
                page_size: OwnedAccountClosureSaga::PAGE_SIZE,
            }
        );
    }

    #[test]
    fn loaded_page_appends_account_close_commands_and_waits_for_results() {
        let saga = OwnedAccountClosureSaga;
        let correlation_id = CorrelationId::from(Uuid::now_v7());
        let closure_id = OwnedAccountClosureId::new();
        let account_id = AccountId::new();
        let next_cursor = AccountId::new();
        let mut state = OwnedAccountClosureSagaState::new(AccountOwner::User(UserId::new()));
        state.owned_account_closure_id = Some(closure_id);
        let mut instance = saga_instance(correlation_id);
        instance.state = Some(state);

        handle_event(
            &saga,
            &mut instance,
            &closure_event_envelope(
                correlation_id,
                closure_id,
                OwnedAccountClosureEventPayload::PageLoaded {
                    account_ids: vec![account_id],
                    next_cursor: Some(next_cursor),
                },
            ),
            Some(OwnedAccountClosureSagaStep::Advance),
        )
        .expect("saga should succeed");

        assert_eq!(instance.uncommitted_commands().len(), 1);
        let command = instance.uncommitted_commands()[0]
            .try_into_command::<AccountCloseCommand>()
            .expect("command should deserialize");
        assert_eq!(command, AccountCloseCommand { account_id });
        assert_eq!(
            instance
                .state
                .as_ref()
                .map(|state| state.pending_account_ids.as_slice()),
            Some([account_id].as_slice())
        );
    }

    #[test]
    fn empty_last_page_appends_complete_command_without_a_completion_route() {
        let saga = OwnedAccountClosureSaga;
        let correlation_id = CorrelationId::from(Uuid::now_v7());
        let closure_id = OwnedAccountClosureId::new();
        let mut state = OwnedAccountClosureSagaState::new(AccountOwner::User(UserId::new()));
        state.owned_account_closure_id = Some(closure_id);
        let mut instance = saga_instance(correlation_id);
        instance.state = Some(state);

        handle_event(
            &saga,
            &mut instance,
            &closure_event_envelope(
                correlation_id,
                closure_id,
                OwnedAccountClosureEventPayload::PageLoaded {
                    account_ids: Vec::new(),
                    next_cursor: None,
                },
            ),
            Some(OwnedAccountClosureSagaStep::Advance),
        )
        .expect("saga should request completion");

        let complete_command = instance.uncommitted_commands()[0]
            .try_into_command::<OwnedAccountClosureCompleteCommand>()
            .expect("command should deserialize");
        assert_eq!(
            complete_command,
            OwnedAccountClosureCompleteCommand {
                owned_account_closure_id: closure_id,
            }
        );

        instance.clear_uncommitted_commands();
        handle_event(
            &saga,
            &mut instance,
            &closure_event_envelope(
                correlation_id,
                closure_id,
                OwnedAccountClosureEventPayload::Completed {
                    closed_account_count: 0,
                },
            ),
            Some(OwnedAccountClosureSagaStep::ProcessPage),
        )
        .expect("unsubscribed completion event should be ignored");

        assert!(instance.uncommitted_commands().is_empty());
    }

    #[test]
    fn later_page_input_is_not_blocked_by_a_previous_completion_command() {
        let saga = OwnedAccountClosureSaga;
        let correlation_id = CorrelationId::from(Uuid::now_v7());
        let closure_id = OwnedAccountClosureId::new();
        let mut state = OwnedAccountClosureSagaState::new(AccountOwner::User(UserId::new()));
        state.owned_account_closure_id = Some(closure_id);
        let mut instance = saga_instance(correlation_id);
        instance.state = Some(state);

        handle_event(
            &saga,
            &mut instance,
            &closure_event_envelope(
                correlation_id,
                closure_id,
                OwnedAccountClosureEventPayload::PageLoaded {
                    account_ids: Vec::new(),
                    next_cursor: None,
                },
            ),
            Some(OwnedAccountClosureSagaStep::Advance),
        )
        .expect("saga should request completion");

        assert!(instance.state.is_some());
        handle_event(
            &saga,
            &mut instance,
            &closure_event_envelope(
                correlation_id,
                closure_id,
                OwnedAccountClosureEventPayload::PageLoaded {
                    account_ids: Vec::new(),
                    next_cursor: None,
                },
            ),
            Some(OwnedAccountClosureSagaStep::Advance),
        )
        .expect("later input is still processable");
        assert_eq!(instance.uncommitted_commands().len(), 2);
    }

    #[test]
    fn empty_page_continuation_keeps_process_page_step() {
        let saga = OwnedAccountClosureSaga;
        let correlation_id = CorrelationId::from(Uuid::now_v7());
        let closure_id = OwnedAccountClosureId::new();
        let mut instance = saga_instance(correlation_id);
        let mut state = OwnedAccountClosureSagaState::new(AccountOwner::User(UserId::new()));
        state.owned_account_closure_id = Some(closure_id);
        instance.state = Some(state);
        let first_page = closure_event_envelope(
            correlation_id,
            closure_id,
            OwnedAccountClosureEventPayload::PageLoaded {
                account_ids: Vec::new(),
                next_cursor: Some(AccountId::new()),
            },
        );
        handle_event(
            &saga,
            &mut instance,
            &first_page,
            Some(OwnedAccountClosureSagaStep::Advance),
        )
        .unwrap();
        let dispatched_step = instance.uncommitted_commands()[0]
            .saga_origin
            .as_ref()
            .unwrap()
            .step
            .try_into_step::<OwnedAccountClosureSagaStep>()
            .unwrap();
        assert_eq!(dispatched_step, OwnedAccountClosureSagaStep::ProcessPage);
        instance.uncommitted_commands()[0]
            .try_into_command::<OwnedAccountClosurePageLoadCommand>()
            .unwrap();
        instance.clear_uncommitted_commands();
        let last_page = closure_event_envelope(
            correlation_id,
            closure_id,
            OwnedAccountClosureEventPayload::PageLoaded {
                account_ids: Vec::new(),
                next_cursor: None,
            },
        );
        assert!(handle_event(&saga, &mut instance, &last_page, Some(dispatched_step)).unwrap());
        instance.uncommitted_commands()[0]
            .try_into_command::<OwnedAccountClosureCompleteCommand>()
            .unwrap();
    }

    #[test]
    fn shared_steps_preserve_command_specific_failure_behavior() {
        use crate::command::OwnedAccountClosureFailCommand;
        use appletheia::application::command::{
            CommandAttemptCount, CommandEnvelope, CommandFailedAt, CommandFailureEnvelope,
            CommandOptions, CommandTerminalReason,
        };
        use appletheia::application::saga::{SagaCommandOrigin, SerializedSagaStep};
        use banking_ledger_domain::owned_account_closure::OwnedAccountClosureFailureReason;

        let saga = OwnedAccountClosureSaga;
        let definition = saga.definition().unwrap();
        let correlation_id = CorrelationId::from(Uuid::now_v7());
        let closure_id = OwnedAccountClosureId::new();
        let causation_id = CausationId::from(MessageId::new());
        let commands = [
            (
                CommandEnvelope::new(
                    &OwnedAccountClosurePageLoadCommand {
                        owned_account_closure_id: closure_id,
                        cursor: None,
                        page_size: 100,
                    },
                    correlation_id,
                    causation_id,
                    CommandOptions::default(),
                )
                .unwrap(),
                Some(OwnedAccountClosureFailureReason::PageLoadRejected),
            ),
            (
                CommandEnvelope::new(
                    &AccountCloseCommand {
                        account_id: AccountId::new(),
                    },
                    correlation_id,
                    causation_id,
                    CommandOptions::default(),
                )
                .unwrap(),
                Some(OwnedAccountClosureFailureReason::AccountCloseRejected),
            ),
            (
                CommandEnvelope::new(
                    &OwnedAccountClosureCompleteCommand {
                        owned_account_closure_id: closure_id,
                    },
                    correlation_id,
                    causation_id,
                    CommandOptions::default(),
                )
                .unwrap(),
                None,
            ),
            (
                CommandEnvelope::new(
                    &OwnedAccountClosureFailCommand {
                        owned_account_closure_id: closure_id,
                        reason: OwnedAccountClosureFailureReason::AccountCloseRejected,
                    },
                    correlation_id,
                    causation_id,
                    CommandOptions::default(),
                )
                .unwrap(),
                None,
            ),
        ];
        for step in [
            OwnedAccountClosureSagaStep::Advance,
            OwnedAccountClosureSagaStep::ProcessPage,
        ] {
            for (command, expected_reason) in &commands {
                if step == OwnedAccountClosureSagaStep::Advance
                    && command.try_into_command::<AccountCloseCommand>().is_ok()
                {
                    continue;
                }
                let mut instance = saga_instance(correlation_id);
                let mut state =
                    OwnedAccountClosureSagaState::new(AccountOwner::User(UserId::new()));
                state.owned_account_closure_id = Some(closure_id);
                instance.state = Some(state);
                let origin = SagaCommandOrigin {
                    saga_name: instance.saga_name.clone(),
                    saga_instance_id: instance.saga_instance_id,
                    step: SerializedSagaStep::new(step).unwrap(),
                };
                let failure = CommandFailureEnvelope::new(
                    command,
                    origin,
                    CommandTerminalReason::NonRetryable,
                    CommandAttemptCount::first(),
                    CommandFailedAt::now(),
                );
                let Some(SagaRoute::OnCommandFailed {
                    step: route_step,
                    handler,
                    ..
                }) = definition.find_command_failure_route(step)
                else {
                    panic!("failure route");
                };
                let mut context = SagaContext::new(
                    &mut instance,
                    CausationId::from(failure.failure_id),
                    *route_step,
                );
                handler(&mut context, &failure).unwrap();
                if let Some(reason) = expected_reason {
                    assert_eq!(instance.uncommitted_commands().len(), 1);
                    let compensation = instance.uncommitted_commands()[0]
                        .try_into_command::<OwnedAccountClosureFailCommand>()
                        .unwrap();
                    assert_eq!(&compensation.reason, reason);
                    assert_eq!(
                        instance.uncommitted_commands()[0]
                            .saga_origin
                            .as_ref()
                            .unwrap()
                            .step
                            .try_into_step::<OwnedAccountClosureSagaStep>()
                            .unwrap(),
                        OwnedAccountClosureSagaStep::Fail
                    );
                } else {
                    assert!(instance.uncommitted_commands().is_empty());
                }
            }
        }
    }
}
