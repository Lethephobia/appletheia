use appletheia::application::saga::{
    Saga, SagaDefinition, SagaDefinitionBuilder, SagaError, SagaName, SagaRouteBuilder,
};
use banking_iam_domain::team_membership::{TeamMembership, TeamMembershipEventPayload};
use banking_iam_domain::team_membership_cleanup::{
    TeamMembershipCleanup, TeamMembershipCleanupEventPayload,
};
use banking_iam_domain::{OrganizationMembership, OrganizationMembershipEventPayload};

use super::{
    TeamMembershipCleanupSagaHandlerError, TeamMembershipCleanupSagaState,
    TeamMembershipCleanupSagaStep,
};
use crate::command::{
    TeamMembershipCleanupMembershipFailedRecordCommand,
    TeamMembershipCleanupMembershipSucceededRecordCommand, TeamMembershipCleanupScanCommand,
    TeamMembershipCleanupStartCommand, TeamMembershipRemoveCommand,
};

pub struct TeamMembershipCleanupSaga;

impl Saga for TeamMembershipCleanupSaga {
    type State = TeamMembershipCleanupSagaState;
    type Step = TeamMembershipCleanupSagaStep;
    type HandlerError = TeamMembershipCleanupSagaHandlerError;

    fn definition(
        &self,
    ) -> Result<SagaDefinition<'_, Self::State, Self::Step, Self::HandlerError>, SagaError> {
        SagaDefinitionBuilder::<Self::State, Self::Step, Self::HandlerError>::new(SagaName::new(
            "team_membership_cleanup",
        ))
        .add_route(
            SagaRouteBuilder::new(TeamMembershipCleanupSagaStep::Start)
                .on::<OrganizationMembership>(OrganizationMembershipEventPayload::REMOVED)
                .handle(|ctx, event| {
                    ctx.append_command(&TeamMembershipCleanupStartCommand {
                        organization_membership_id: event.aggregate_id(),
                    })?;
                    Ok(())
                }),
        )
        .add_route(
            SagaRouteBuilder::new(TeamMembershipCleanupSagaStep::ProcessPage)
                .on::<TeamMembershipCleanup>(TeamMembershipCleanupEventPayload::STARTED)
                .caused_by(TeamMembershipCleanupSagaStep::Start)
                .handle(|ctx, event| {
                    let team_membership_cleanup_id = event.aggregate_id();
                    ctx.set_state(TeamMembershipCleanupSagaState {
                        team_membership_cleanup_id,
                    });
                    ctx.append_command(&TeamMembershipCleanupScanCommand {
                        team_membership_cleanup_id,
                    })?;
                    Ok(())
                }),
        )
        .add_route(
            SagaRouteBuilder::new(TeamMembershipCleanupSagaStep::ProcessPage)
                .on::<TeamMembershipCleanup>(TeamMembershipCleanupEventPayload::SCANNED)
                .caused_by(TeamMembershipCleanupSagaStep::ProcessPage)
                .handle(|ctx, event| {
                    if let TeamMembershipCleanupEventPayload::Scanned {
                        team_membership_ids,
                        next_cursor,
                    } = event.payload()
                    {
                        for team_membership_id in team_membership_ids {
                            ctx.append_command(&TeamMembershipRemoveCommand {
                                team_membership_id: *team_membership_id,
                            })?;
                        }
                        if next_cursor.is_some() {
                            ctx.append_command(&TeamMembershipCleanupScanCommand {
                                team_membership_cleanup_id: event.aggregate_id(),
                            })?;
                        }
                    }
                    Ok(())
                }),
        )
        .add_route(
            SagaRouteBuilder::new(TeamMembershipCleanupSagaStep::RecordMembershipSucceeded)
                .on::<TeamMembership>(TeamMembershipEventPayload::REMOVED)
                .caused_by(TeamMembershipCleanupSagaStep::ProcessPage)
                .handle(|ctx, event| {
                    let state: &TeamMembershipCleanupSagaState = ctx.state_required()?;
                    let team_membership_cleanup_id = state.team_membership_cleanup_id;
                    ctx.append_command(&TeamMembershipCleanupMembershipSucceededRecordCommand {
                        team_membership_cleanup_id,
                        team_membership_id: event.aggregate_id(),
                    })?;
                    Ok(())
                }),
        )
        .add_route(
            SagaRouteBuilder::new(TeamMembershipCleanupSagaStep::RecordMembershipFailed)
                .on_command_failed::<TeamMembershipRemoveCommand>(
                    TeamMembershipCleanupSagaStep::ProcessPage,
                )
                .handle(|ctx, command| {
                    let state: &TeamMembershipCleanupSagaState = ctx.state_required()?;
                    let team_membership_cleanup_id = state.team_membership_cleanup_id;
                    ctx.append_command(&TeamMembershipCleanupMembershipFailedRecordCommand {
                        team_membership_cleanup_id,
                        team_membership_id: command.team_membership_id,
                    })?;
                    Ok(())
                }),
        )
        .build()
        .map_err(SagaError::from)
    }
}

#[cfg(test)]
mod tests {
    use super::TeamMembershipCleanupSagaHandlerError;
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
    use banking_iam_domain::team_membership_cleanup::{
        TeamMembershipCleanup, TeamMembershipCleanupEventPayload, TeamMembershipCleanupId,
    };
    use banking_iam_domain::{
        OrganizationId, OrganizationMembership, OrganizationMembershipEventPayload,
        OrganizationMembershipId, TeamMembershipId,
    };
    use banking_iam_domain::{User, UserId};

    use super::{
        TeamMembershipCleanupSaga, TeamMembershipCleanupSagaState, TeamMembershipCleanupSagaStep,
    };
    use crate::command::{
        TeamMembershipCleanupMembershipFailedRecordCommand,
        TeamMembershipCleanupMembershipSucceededRecordCommand, TeamMembershipCleanupScanCommand,
        TeamMembershipCleanupStartCommand, TeamMembershipRemoveCommand,
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

    fn cleanup_event_envelope(
        correlation_id: CorrelationId,
        cleanup_id: TeamMembershipCleanupId,
        payload: TeamMembershipCleanupEventPayload,
    ) -> EventEnvelope {
        event_envelope(
            correlation_id,
            TeamMembershipCleanup::TYPE,
            cleanup_id,
            payload,
        )
    }

    fn saga_instance(
        correlation_id: CorrelationId,
    ) -> SagaInstance<TeamMembershipCleanupSagaState, TeamMembershipCleanupSagaStep> {
        SagaInstance::<TeamMembershipCleanupSagaState, TeamMembershipCleanupSagaStep>::new(
            SagaNameOwned::from(
                TeamMembershipCleanupSaga
                    .definition()
                    .expect("valid saga definition")
                    .name(),
            ),
            correlation_id,
            EventId::new(),
        )
    }

    fn handle_event(
        saga: &TeamMembershipCleanupSaga,
        instance: &mut SagaInstance<TeamMembershipCleanupSagaState, TeamMembershipCleanupSagaStep>,
        envelope: &EventEnvelope,
        step: Option<TeamMembershipCleanupSagaStep>,
    ) -> Result<bool, TeamMembershipCleanupSagaHandlerError> {
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
    fn removed_membership_starts_cleanup_for_that_membership() {
        let correlation_id = CorrelationId::from(Uuid::now_v7());
        let membership_id = OrganizationMembershipId::new();
        let mut instance = saga_instance(correlation_id);
        let envelope = event_envelope(
            correlation_id,
            OrganizationMembership::TYPE,
            membership_id,
            OrganizationMembershipEventPayload::Removed {
                organization_id: OrganizationId::new(),
                user_id: UserId::new(),
            },
        );
        handle_event(&TeamMembershipCleanupSaga, &mut instance, &envelope, None).unwrap();
        assert_eq!(instance.uncommitted_commands().len(), 1);
        assert_eq!(
            instance.uncommitted_commands()[0]
                .try_to_command::<TeamMembershipCleanupStartCommand>()
                .unwrap()
                .organization_membership_id,
            membership_id
        );
    }

    #[test]
    fn started_loads_first_page_and_remembers_cleanup_id() {
        let correlation_id = CorrelationId::from(Uuid::now_v7());
        let cleanup_id = TeamMembershipCleanupId::new();
        let mut instance = saga_instance(correlation_id);
        handle_event(
            &TeamMembershipCleanupSaga,
            &mut instance,
            &cleanup_event_envelope(
                correlation_id,
                cleanup_id,
                TeamMembershipCleanupEventPayload::Started {
                    organization_membership_id: OrganizationMembershipId::new(),
                },
            ),
            Some(TeamMembershipCleanupSagaStep::Start),
        )
        .unwrap();
        assert_eq!(
            instance.state.as_ref().unwrap().team_membership_cleanup_id,
            cleanup_id
        );
        assert_eq!(
            instance.uncommitted_commands()[0]
                .try_to_command::<TeamMembershipCleanupScanCommand>()
                .unwrap(),
            TeamMembershipCleanupScanCommand {
                team_membership_cleanup_id: cleanup_id,
            }
        );
    }

    #[test]
    fn scanned_page_dispatches_all_removals_and_next_scan_without_waiting_for_results() {
        let correlation_id = CorrelationId::from(Uuid::now_v7());
        let cleanup_id = TeamMembershipCleanupId::new();
        let team_membership_id = TeamMembershipId::new();
        let mut instance = saga_instance(correlation_id);
        instance.state = Some(TeamMembershipCleanupSagaState {
            team_membership_cleanup_id: cleanup_id,
        });
        let second_team_membership_id = TeamMembershipId::new();
        handle_event(
            &TeamMembershipCleanupSaga,
            &mut instance,
            &cleanup_event_envelope(
                correlation_id,
                cleanup_id,
                TeamMembershipCleanupEventPayload::Scanned {
                    team_membership_ids: vec![team_membership_id, second_team_membership_id],
                    next_cursor: Some(second_team_membership_id),
                },
            ),
            Some(TeamMembershipCleanupSagaStep::ProcessPage),
        )
        .unwrap();
        assert_eq!(instance.uncommitted_commands().len(), 3);
        for (command, expected_id) in instance.uncommitted_commands()[..2]
            .iter()
            .zip([team_membership_id, second_team_membership_id])
        {
            assert_eq!(
                command
                    .try_to_command::<TeamMembershipRemoveCommand>()
                    .unwrap()
                    .team_membership_id,
                expected_id
            );
        }
        assert_eq!(
            instance.uncommitted_commands()[2]
                .try_to_command::<TeamMembershipCleanupScanCommand>()
                .unwrap()
                .team_membership_cleanup_id,
            cleanup_id
        );
    }

    #[test]
    fn removed_membership_dispatches_succeeded_record() {
        use banking_iam_domain::{TeamId, TeamMembership, TeamMembershipEventPayload};
        let correlation_id = CorrelationId::from(Uuid::now_v7());
        let cleanup_id = TeamMembershipCleanupId::new();
        let team_membership_id = TeamMembershipId::new();
        let mut instance = saga_instance(correlation_id);
        instance.state = Some(TeamMembershipCleanupSagaState {
            team_membership_cleanup_id: cleanup_id,
        });
        handle_event(
            &TeamMembershipCleanupSaga,
            &mut instance,
            &event_envelope(
                correlation_id,
                TeamMembership::TYPE,
                team_membership_id,
                TeamMembershipEventPayload::Removed {
                    team_id: TeamId::new(),
                    organization_membership_id: OrganizationMembershipId::new(),
                },
            ),
            Some(TeamMembershipCleanupSagaStep::ProcessPage),
        )
        .unwrap();
        let command = instance.uncommitted_commands()[0]
            .try_to_command::<TeamMembershipCleanupMembershipSucceededRecordCommand>()
            .unwrap();
        assert_eq!(command.team_membership_id, team_membership_id);
        assert_eq!(command.team_membership_cleanup_id, cleanup_id);
    }

    #[test]
    fn terminal_remove_failure_records_the_membership_from_the_failed_command() {
        use appletheia::application::command::{
            CommandAttemptCount, CommandFailedAt, CommandFailureEnvelope, CommandTerminalReason,
        };
        let correlation_id = CorrelationId::from(Uuid::now_v7());
        let cleanup_id = TeamMembershipCleanupId::new();
        let team_membership_id = TeamMembershipId::new();
        let mut instance = saga_instance(correlation_id);
        instance.state = Some(TeamMembershipCleanupSagaState {
            team_membership_cleanup_id: cleanup_id,
        });
        handle_event(
            &TeamMembershipCleanupSaga,
            &mut instance,
            &cleanup_event_envelope(
                correlation_id,
                cleanup_id,
                TeamMembershipCleanupEventPayload::Scanned {
                    team_membership_ids: vec![team_membership_id],
                    next_cursor: None,
                },
            ),
            Some(TeamMembershipCleanupSagaStep::ProcessPage),
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
        let definition = TeamMembershipCleanupSaga.definition().unwrap();
        let SagaRoute::OnCommandFailed { step, handler, .. } = definition
            .find_command_failure_route(
                TeamMembershipCleanupSagaStep::ProcessPage,
                &failure.command_name,
            )
            .unwrap()
        else {
            panic!("expected failure route")
        };
        let mut context = SagaContext::new(&mut instance, failure.causation_id, *step);
        handler(&mut context, &failure).unwrap();
        let record = instance.uncommitted_commands()[0]
            .try_to_command::<TeamMembershipCleanupMembershipFailedRecordCommand>()
            .unwrap();
        assert_eq!(record.team_membership_id, team_membership_id);
        assert_eq!(record.team_membership_cleanup_id, cleanup_id);
    }

    #[test]
    fn terminal_scan_failure_does_not_count_as_membership_failure() {
        use appletheia::application::command::{
            CommandAttemptCount, CommandFailedAt, CommandFailureEnvelope, CommandTerminalReason,
        };
        let correlation_id = CorrelationId::from(Uuid::now_v7());
        let cleanup_id = TeamMembershipCleanupId::new();
        let team_membership_id = TeamMembershipId::new();
        let mut instance = saga_instance(correlation_id);
        instance.state = Some(TeamMembershipCleanupSagaState {
            team_membership_cleanup_id: cleanup_id,
        });
        handle_event(
            &TeamMembershipCleanupSaga,
            &mut instance,
            &cleanup_event_envelope(
                correlation_id,
                cleanup_id,
                TeamMembershipCleanupEventPayload::Scanned {
                    team_membership_ids: vec![team_membership_id],
                    next_cursor: Some(team_membership_id),
                },
            ),
            Some(TeamMembershipCleanupSagaStep::ProcessPage),
        )
        .unwrap();
        let command = &instance.uncommitted_commands()[1];
        let failure = CommandFailureEnvelope::new(
            command,
            command.saga_origin.clone().unwrap(),
            CommandTerminalReason::NonRetryable,
            CommandAttemptCount::first(),
            CommandFailedAt::now(),
        );
        instance.clear_uncommitted_commands();
        let definition = TeamMembershipCleanupSaga.definition().unwrap();
        assert!(
            definition
                .find_command_failure_route(
                    TeamMembershipCleanupSagaStep::ProcessPage,
                    &failure.command_name
                )
                .is_none()
        );
        assert!(instance.uncommitted_commands().is_empty());
    }

    #[test]
    fn scan_completion_and_recorded_results_do_not_dispatch_completion_commands() {
        let correlation_id = CorrelationId::from(Uuid::now_v7());
        let cleanup_id = TeamMembershipCleanupId::new();
        let team_membership_id = TeamMembershipId::new();
        let mut instance = saga_instance(correlation_id);
        for (step, payload) in [
            (
                TeamMembershipCleanupSagaStep::ProcessPage,
                TeamMembershipCleanupEventPayload::Scanned {
                    team_membership_ids: vec![],
                    next_cursor: None,
                },
            ),
            (
                TeamMembershipCleanupSagaStep::RecordMembershipSucceeded,
                TeamMembershipCleanupEventPayload::MembershipSucceeded { team_membership_id },
            ),
            (
                TeamMembershipCleanupSagaStep::RecordMembershipFailed,
                TeamMembershipCleanupEventPayload::MembershipFailed { team_membership_id },
            ),
        ] {
            assert_eq!(
                handle_event(
                    &TeamMembershipCleanupSaga,
                    &mut instance,
                    &cleanup_event_envelope(correlation_id, cleanup_id, payload),
                    Some(step)
                )
                .unwrap(),
                step == TeamMembershipCleanupSagaStep::ProcessPage
            );
        }
        assert!(instance.uncommitted_commands().is_empty());
    }
}
