use super::{
    OwnedAccountClosureSagaError, OwnedAccountClosureSagaSpec, OwnedAccountClosureSagaState,
    OwnedAccountClosureSagaStep,
};
use crate::command::{
    AccountCloseCommand, OwnedAccountClosureAccountCloseRecordCommand,
    OwnedAccountClosureCompleteCommand, OwnedAccountClosureFailCommand,
    OwnedAccountClosurePageLoadCommand, OwnedAccountClosureRequestCommand,
};
use appletheia::application::command::CommandFailureEnvelope;
use appletheia::application::event::EventEnvelope;
use appletheia::application::request_context::CausationId;
use appletheia::application::saga::{Saga, SagaInstance, SagaSpec};
use banking_iam_domain::{Organization, OrganizationEventPayload, User, UserEventPayload};
use banking_ledger_domain::account::{Account, AccountEventPayload, AccountOwner};
use banking_ledger_domain::owned_account_closure::{
    OwnedAccountClosure, OwnedAccountClosureEventPayload, OwnedAccountClosureFailureReason,
};

/// Coordinates closing every account owned by a removed owner.
pub struct OwnedAccountClosureSaga;

impl OwnedAccountClosureSaga {
    const PAGE_SIZE: u32 = 100;

    fn append_next_step(
        instance: &mut SagaInstance<OwnedAccountClosureSagaState, OwnedAccountClosureSagaStep>,
        event: &EventEnvelope,
    ) -> Result<(), OwnedAccountClosureSagaError> {
        let state = instance.state_required_mut()?;
        if state.has_pending_accounts() {
            return Ok(());
        }

        let owned_account_closure_id = state
            .owned_account_closure_id
            .ok_or(OwnedAccountClosureSagaError::MissingOwnedAccountClosureId)?;

        if state.has_next_page() {
            let cursor = state.next_cursor;
            instance.append_command(
                CausationId::from(event.event_id),
                OwnedAccountClosureSagaStep::LoadPage,
                &OwnedAccountClosurePageLoadCommand {
                    owned_account_closure_id,
                    cursor,
                    page_size: Self::PAGE_SIZE,
                },
            )?;
        } else if state.has_rejections() {
            instance.append_command(
                CausationId::from(event.event_id),
                OwnedAccountClosureSagaStep::Fail,
                &OwnedAccountClosureFailCommand {
                    owned_account_closure_id,
                    reason: OwnedAccountClosureFailureReason::AccountCloseRejected,
                },
            )?;
        } else {
            instance.append_command(
                CausationId::from(event.event_id),
                OwnedAccountClosureSagaStep::Complete,
                &OwnedAccountClosureCompleteCommand {
                    owned_account_closure_id,
                },
            )?;
        }

        Ok(())
    }
}

impl Saga for OwnedAccountClosureSaga {
    type Spec = OwnedAccountClosureSagaSpec;
    type Step = OwnedAccountClosureSagaStep;
    type Error = OwnedAccountClosureSagaError;

    fn on_event(
        &self,
        instance: &mut SagaInstance<<Self::Spec as SagaSpec>::State, Self::Step>,
        event: &EventEnvelope,
        _causative_step: Option<Self::Step>,
    ) -> Result<(), Self::Error> {
        let envelope = event;
        if envelope.is_for_aggregate::<User>() {
            let user_event = envelope.try_into_domain_event::<User>()?;
            if let UserEventPayload::Removed = user_event.payload() {
                let owner = AccountOwner::User(user_event.aggregate_id());
                *instance.state_mut() = Some(OwnedAccountClosureSagaState::new(owner));
                instance.append_command(
                    CausationId::from(envelope.event_id),
                    OwnedAccountClosureSagaStep::Request,
                    &OwnedAccountClosureRequestCommand { owner },
                )?;
            }

            return Ok(());
        } else if envelope.is_for_aggregate::<Organization>() {
            let organization_event = envelope.try_into_domain_event::<Organization>()?;
            if let OrganizationEventPayload::Removed = organization_event.payload() {
                let owner = AccountOwner::Organization(organization_event.aggregate_id());
                *instance.state_mut() = Some(OwnedAccountClosureSagaState::new(owner));
                instance.append_command(
                    CausationId::from(envelope.event_id),
                    OwnedAccountClosureSagaStep::Request,
                    &OwnedAccountClosureRequestCommand { owner },
                )?;
            }

            return Ok(());
        } else if envelope.is_for_aggregate::<OwnedAccountClosure>() {
            let closure_event = envelope.try_into_domain_event::<OwnedAccountClosure>()?;
            match closure_event.payload() {
                OwnedAccountClosureEventPayload::Requested { .. } => {
                    let closure_id = closure_event.aggregate_id();
                    let state = instance.state_required_mut()?;
                    state.owned_account_closure_id = Some(closure_id);
                    instance.append_command(
                        CausationId::from(envelope.event_id),
                        OwnedAccountClosureSagaStep::LoadPage,
                        &OwnedAccountClosurePageLoadCommand {
                            owned_account_closure_id: closure_id,
                            cursor: None,
                            page_size: Self::PAGE_SIZE,
                        },
                    )?;
                }
                OwnedAccountClosureEventPayload::PageLoaded {
                    account_ids,
                    next_cursor,
                } => {
                    let state = instance.state_required_mut()?;
                    state.set_loaded_page(account_ids.clone(), *next_cursor);
                    for account_id in account_ids {
                        instance.append_command(
                            CausationId::from(envelope.event_id),
                            OwnedAccountClosureSagaStep::CloseAccount,
                            &AccountCloseCommand {
                                account_id: *account_id,
                            },
                        )?;
                    }
                    Self::append_next_step(instance, envelope)?;
                }
                OwnedAccountClosureEventPayload::AccountCloseRecorded { account_id } => {
                    let state = instance.state_required_mut()?;
                    state.closed_account_count = state.closed_account_count.saturating_add(1);
                    state.remove_pending_account(*account_id);
                    Self::append_next_step(instance, envelope)?;
                }
                OwnedAccountClosureEventPayload::AccountCloseRejectionRecorded {
                    account_id,
                    ..
                } => {
                    let state = instance.state_required_mut()?;
                    state.rejected_account_count = state.rejected_account_count.saturating_add(1);
                    state.remove_pending_account(*account_id);
                    Self::append_next_step(instance, envelope)?;
                }
                OwnedAccountClosureEventPayload::Completed { .. } => {
                    instance.succeed();
                }
                OwnedAccountClosureEventPayload::Failed { .. } => {
                    instance.fail();
                }
            }

            return Ok(());
        } else if envelope.is_for_aggregate::<Account>() {
            let account_event = envelope.try_into_domain_event::<Account>()?;
            let owned_account_closure_id = instance
                .state_required_mut()?
                .owned_account_closure_id
                .ok_or(OwnedAccountClosureSagaError::MissingOwnedAccountClosureId)?;

            if account_event.payload() == &AccountEventPayload::Closed {
                instance.append_command(
                    CausationId::from(envelope.event_id),
                    OwnedAccountClosureSagaStep::RecordClosedAccount,
                    &OwnedAccountClosureAccountCloseRecordCommand {
                        owned_account_closure_id,
                        account_id: account_event.aggregate_id(),
                    },
                )?;
            }
        }

        Ok(())
    }

    fn on_command_failed(
        &self,
        instance: &mut SagaInstance<<Self::Spec as SagaSpec>::State, Self::Step>,
        failure: &CommandFailureEnvelope,
        causative_step: Self::Step,
    ) -> Result<(), Self::Error> {
        let reason = match causative_step {
            OwnedAccountClosureSagaStep::Request => {
                instance.fail();
                return Ok(());
            }
            OwnedAccountClosureSagaStep::LoadPage => {
                OwnedAccountClosureFailureReason::PageLoadRejected
            }
            OwnedAccountClosureSagaStep::CloseAccount => {
                OwnedAccountClosureFailureReason::AccountCloseRejected
            }
            OwnedAccountClosureSagaStep::RecordClosedAccount => {
                OwnedAccountClosureFailureReason::AccountCloseRecordRejected
            }
            OwnedAccountClosureSagaStep::RecordRejectedAccountClose => {
                OwnedAccountClosureFailureReason::AccountCloseRejectionRecordRejected
            }
            OwnedAccountClosureSagaStep::Complete | OwnedAccountClosureSagaStep::Fail => {
                instance.fail();
                return Ok(());
            }
        };
        let owned_account_closure_id = instance
            .state_required_mut()?
            .owned_account_closure_id
            .ok_or(OwnedAccountClosureSagaError::MissingOwnedAccountClosureId)?;
        instance.append_command(
            CausationId::from(failure.failure_id),
            OwnedAccountClosureSagaStep::Fail,
            &OwnedAccountClosureFailCommand {
                owned_account_closure_id,
                reason,
            },
        )?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use uuid::Uuid;

    use appletheia::application::event::{
        AggregateIdValue, AggregateTypeOwned, EventEnvelope, EventNameOwned, EventSequence,
        SerializedEventPayload,
    };
    use appletheia::application::request_context::{
        CausationId, CorrelationId, MessageId, Principal, RequestContext,
    };
    use appletheia::application::saga::{Saga, SagaInstance, SagaNameOwned, SagaSpec, SagaStatus};
    use appletheia::domain::{
        Aggregate, AggregateId, AggregateType, EventId, EventOccurredAt, EventPayload,
    };
    use banking_iam_domain::{User, UserEventPayload, UserId};
    use banking_ledger_domain::account::{AccountId, AccountOwner};
    use banking_ledger_domain::owned_account_closure::{
        OwnedAccountClosure, OwnedAccountClosureEventPayload, OwnedAccountClosureId,
    };

    use super::{
        OwnedAccountClosureSaga, OwnedAccountClosureSagaSpec, OwnedAccountClosureSagaState,
        OwnedAccountClosureSagaStep,
    };
    use crate::command::{
        AccountCloseCommand, OwnedAccountClosureCompleteCommand,
        OwnedAccountClosurePageLoadCommand, OwnedAccountClosureRequestCommand,
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
            SagaNameOwned::from(OwnedAccountClosureSagaSpec::DESCRIPTOR.name),
            correlation_id,
            EventId::new(),
        )
    }

    fn handle_event(
        saga: &OwnedAccountClosureSaga,
        instance: &mut SagaInstance<OwnedAccountClosureSagaState, OwnedAccountClosureSagaStep>,
        envelope: &EventEnvelope,
        step: Option<OwnedAccountClosureSagaStep>,
    ) -> Result<(), super::OwnedAccountClosureSagaError> {
        saga.on_event(instance, envelope, step)
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
            Some(OwnedAccountClosureSagaStep::LoadPage),
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
    fn empty_last_page_appends_complete_command_and_completed_event_succeeds() {
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
            Some(OwnedAccountClosureSagaStep::LoadPage),
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
            Some(OwnedAccountClosureSagaStep::Complete),
        )
        .expect("completed event should succeed saga");

        assert_eq!(instance.status, SagaStatus::Succeeded);
    }

    #[test]
    fn empty_last_page_keeps_non_terminal_status_until_completed_event() {
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
            Some(OwnedAccountClosureSagaStep::LoadPage),
        )
        .expect("saga should request completion");

        assert_eq!(instance.status, SagaStatus::InProgress);
    }
}
