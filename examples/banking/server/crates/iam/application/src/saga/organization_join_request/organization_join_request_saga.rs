use crate::command::OrganizationMembershipCreateCommand;
use appletheia::application::command::CommandFailureEnvelope;
use appletheia::application::event::EventEnvelope;
use appletheia::application::request_context::CausationId;
use appletheia::application::saga::{Saga, SagaInstance};
use banking_iam_domain::{
    OrganizationJoinRequest, OrganizationJoinRequestEventPayload, OrganizationMembership,
    OrganizationMembershipEventPayload, OrganizationRoles,
};

use super::{
    OrganizationJoinRequestSagaError, OrganizationJoinRequestSagaSpec,
    OrganizationJoinRequestSagaState, OrganizationJoinRequestSagaStep,
};

/// Coordinates the organization join request workflow into organization membership creation.
pub struct OrganizationJoinRequestSaga;

impl Saga for OrganizationJoinRequestSaga {
    type Spec = OrganizationJoinRequestSagaSpec;
    type State = OrganizationJoinRequestSagaState;
    type Step = OrganizationJoinRequestSagaStep;
    type Error = OrganizationJoinRequestSagaError;

    fn on_event(
        &self,
        instance: &mut SagaInstance<Self::State, Self::Step>,
        event: &EventEnvelope,
        _causative_step: Option<Self::Step>,
    ) -> Result<(), Self::Error> {
        if event.is_for_aggregate::<OrganizationJoinRequest>() {
            let join_request_event = event.try_into_domain_event::<OrganizationJoinRequest>()?;
            if let OrganizationJoinRequestEventPayload::Approved {
                organization_id,
                requester_id,
            } = join_request_event.payload()
            {
                *instance.state_mut() = Some(OrganizationJoinRequestSagaState::new(
                    join_request_event.aggregate_id(),
                ));

                instance.append_command(
                    CausationId::from(event.event_id),
                    OrganizationJoinRequestSagaStep::CreateMembership,
                    &OrganizationMembershipCreateCommand {
                        organization_id: *organization_id,
                        user_id: *requester_id,
                        roles: OrganizationRoles::default(),
                    },
                )?;
            }

            return Ok(());
        } else if event.is_for_aggregate::<OrganizationMembership>() {
            let membership_event = event.try_into_domain_event::<OrganizationMembership>()?;
            if let OrganizationMembershipEventPayload::Created { .. } = membership_event.payload() {
                instance.complete();
            }
        }

        Ok(())
    }

    fn on_command_failed(
        &self,
        instance: &mut SagaInstance<Self::State, Self::Step>,
        _failure: &CommandFailureEnvelope,
        _causative_step: Self::Step,
    ) -> Result<(), Self::Error> {
        instance.complete();
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
    use appletheia::domain::{Aggregate, AggregateId, EventId, EventOccurredAt, EventPayload};
    use banking_iam_domain::{
        OrganizationId, OrganizationJoinRequest, OrganizationJoinRequestEventPayload,
        OrganizationJoinRequestId, OrganizationMembership, OrganizationMembershipEventPayload,
        OrganizationMembershipId, OrganizationRoles, User, UserId,
    };

    use crate::command::OrganizationMembershipCreateCommand;

    use super::{
        OrganizationJoinRequestSaga, OrganizationJoinRequestSagaSpec,
        OrganizationJoinRequestSagaStep,
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

    fn handle_event(
        saga: &OrganizationJoinRequestSaga,
        instance: &mut SagaInstance<
            <OrganizationJoinRequestSaga as Saga>::State,
            OrganizationJoinRequestSagaStep,
        >,
        envelope: &EventEnvelope,
        step: Option<OrganizationJoinRequestSagaStep>,
    ) -> Result<(), super::OrganizationJoinRequestSagaError> {
        saga.on_event(instance, envelope, step)
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
            SagaNameOwned::from(OrganizationJoinRequestSagaSpec::DESCRIPTOR.name),
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

        assert_eq!(instance.status, SagaStatus::InProgress);
        assert_eq!(instance.uncommitted_commands().len(), 1);
        let command: OrganizationMembershipCreateCommand = instance.uncommitted_commands()[0]
            .try_into_command()
            .expect("command should deserialize");
        assert_eq!(command.user_id, requester_id);
        assert_eq!(command.organization_id, organization_id);
        assert_eq!(command.roles, OrganizationRoles::default());
    }

    #[test]
    fn created_membership_completes_saga() {
        let saga = OrganizationJoinRequestSaga;
        let correlation_id = CorrelationId::from(Uuid::now_v7());
        let organization_id = OrganizationId::new();
        let join_request_id = OrganizationJoinRequestId::new();
        let requester_id = UserId::new();
        let mut instance = SagaInstance::<
            <OrganizationJoinRequestSaga as Saga>::State,
            OrganizationJoinRequestSagaStep,
        >::new(
            SagaNameOwned::from(OrganizationJoinRequestSagaSpec::DESCRIPTOR.name),
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
        handle_event(
            &saga,
            &mut instance,
            &membership_created_event_envelope(correlation_id),
            Some(OrganizationJoinRequestSagaStep::CreateMembership),
        )
        .expect("membership created event should be handled");

        assert_eq!(instance.status, SagaStatus::Completed);
        assert!(instance.uncommitted_commands().is_empty());
    }
}
