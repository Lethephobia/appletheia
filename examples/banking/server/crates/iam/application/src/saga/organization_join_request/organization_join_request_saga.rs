use crate::command::OrganizationMembershipCreateCommand;
use appletheia::application::saga::SagaError;
use appletheia::application::saga::{Saga, SagaDefinition, SagaDefinitionBuilder, SagaName};
use banking_iam_domain::{
    OrganizationJoinRequest, OrganizationJoinRequestEventPayload, OrganizationRoles,
};

use super::{
    OrganizationJoinRequestSagaHandlerError, OrganizationJoinRequestSagaState,
    OrganizationJoinRequestSagaStep,
};

/// Coordinates the organization join request workflow into organization membership creation.
pub struct OrganizationJoinRequestSaga;

impl Saga for OrganizationJoinRequestSaga {
    type State = OrganizationJoinRequestSagaState;
    type Step = OrganizationJoinRequestSagaStep;
    type HandlerError = OrganizationJoinRequestSagaHandlerError;

    fn definition(
        &self,
    ) -> Result<SagaDefinition<'_, Self::State, Self::Step, Self::HandlerError>, SagaError> {
        SagaDefinitionBuilder::<Self::State, Self::Step, Self::HandlerError>::new(SagaName::new(
            "organization_join_request",
        ))
        .add_start_step(OrganizationJoinRequestSagaStep::CreateMembership)
        .on::<OrganizationJoinRequest>(OrganizationJoinRequestEventPayload::APPROVED)
        .handle(|ctx, join_request_event| {
            if let OrganizationJoinRequestEventPayload::Approved {
                organization_id,
                requester_id,
            } = join_request_event.payload()
            {
                ctx.set_state(OrganizationJoinRequestSagaState::new(
                    join_request_event.aggregate_id(),
                ));

                ctx.append_command(&OrganizationMembershipCreateCommand {
                    organization_id: *organization_id,
                    user_id: *requester_id,
                    roles: OrganizationRoles::default(),
                })?;
            }
            Ok(())
        })
        .build()
        .map_err(SagaError::from)
    }
}

#[cfg(test)]
mod tests {
    use super::OrganizationJoinRequestSagaHandlerError;
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
    use appletheia::domain::{Aggregate, AggregateId, EventId, EventOccurredAt, EventPayload};
    use banking_iam_domain::{
        OrganizationId, OrganizationJoinRequest, OrganizationJoinRequestEventPayload,
        OrganizationJoinRequestId, OrganizationMembership, OrganizationMembershipEventPayload,
        OrganizationMembershipId, OrganizationRoles, User, UserId,
    };

    use crate::command::OrganizationMembershipCreateCommand;

    use super::{OrganizationJoinRequestSaga, OrganizationJoinRequestSagaStep};

    fn request_context(correlation_id: CorrelationId) -> RequestContext {
        let subject = AggregateRef::from_id::<User>(UserId::new());

        RequestContext::new(
            correlation_id,
            MessageId::new(),
            Principal::Authenticated { subject },
        )
        .expect("request context should be valid")
    }

    fn join_request_approved_event_envelope(
        correlation_id: CorrelationId,
        organization_id: OrganizationId,
        join_request_id: OrganizationJoinRequestId,
        requester_id: UserId,
    ) -> EventEnvelope {
        let payload = OrganizationJoinRequestEventPayload::Approved {
            organization_id,
            requester_id,
        };

        EventEnvelope {
            event_sequence: EventSequence::try_from(1).expect("sequence should be valid"),
            event_id: EventId::new(),
            aggregate_type: AggregateTypeOwned::from(OrganizationJoinRequest::TYPE),
            aggregate_id: AggregateIdValue::from(join_request_id.value()),
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

    fn membership_created_event_envelope(correlation_id: CorrelationId) -> EventEnvelope {
        let membership_id = OrganizationMembershipId::new();
        let payload = OrganizationMembershipEventPayload::Created {
            organization_id: OrganizationId::new(),
            user_id: UserId::new(),
            roles: OrganizationRoles::default(),
        };

        EventEnvelope {
            event_sequence: EventSequence::try_from(2).expect("sequence should be valid"),
            event_id: EventId::new(),
            aggregate_type: AggregateTypeOwned::from(OrganizationMembership::TYPE),
            aggregate_id: AggregateIdValue::from(membership_id.value()),
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

    fn handle_event(
        saga: &OrganizationJoinRequestSaga,
        instance: &mut SagaInstance<
            <OrganizationJoinRequestSaga as Saga>::State,
            OrganizationJoinRequestSagaStep,
        >,
        envelope: &EventEnvelope,
        step: Option<OrganizationJoinRequestSagaStep>,
    ) -> Result<bool, SagaRouteError<OrganizationJoinRequestSagaHandlerError>> {
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
    fn approved_event_appends_membership_create_command() {
        let saga = OrganizationJoinRequestSaga;
        let correlation_id = CorrelationId::from(Uuid::now_v7());
        let organization_id = OrganizationId::new();
        let join_request_id = OrganizationJoinRequestId::new();
        let requester_id = UserId::new();
        let mut instance = SagaInstance::<
            <OrganizationJoinRequestSaga as Saga>::State,
            OrganizationJoinRequestSagaStep,
        >::new(
            SagaNameOwned::from(
                OrganizationJoinRequestSaga
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
            &join_request_approved_event_envelope(
                correlation_id,
                organization_id,
                join_request_id,
                requester_id,
            ),
            None,
        )
        .expect("approved event should be handled");

        assert!(instance.state.is_some());
        assert_eq!(instance.uncommitted_commands().len(), 1);
        let command: OrganizationMembershipCreateCommand = instance.uncommitted_commands()[0]
            .try_into_command()
            .expect("command should deserialize");
        assert_eq!(command.user_id, requester_id);
        assert_eq!(command.organization_id, organization_id);
        assert_eq!(command.roles, OrganizationRoles::default());
    }

    #[test]
    fn created_membership_is_not_subscribed() {
        let saga = OrganizationJoinRequestSaga;
        let correlation_id = CorrelationId::from(Uuid::now_v7());
        let organization_id = OrganizationId::new();
        let join_request_id = OrganizationJoinRequestId::new();
        let requester_id = UserId::new();
        let mut instance = SagaInstance::<
            <OrganizationJoinRequestSaga as Saga>::State,
            OrganizationJoinRequestSagaStep,
        >::new(
            SagaNameOwned::from(
                OrganizationJoinRequestSaga
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
            &join_request_approved_event_envelope(
                correlation_id,
                organization_id,
                join_request_id,
                requester_id,
            ),
            None,
        )
        .expect("approved event should be handled");
        let pending = instance.uncommitted_commands().to_vec();
        let matched = handle_event(
            &saga,
            &mut instance,
            &membership_created_event_envelope(correlation_id),
            Some(OrganizationJoinRequestSagaStep::CreateMembership),
        )
        .expect("membership created event should be handled");

        assert!(!matched);
        assert_eq!(instance.uncommitted_commands(), pending);
    }
}
