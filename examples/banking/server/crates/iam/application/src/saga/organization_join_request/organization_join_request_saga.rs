use crate::command::UserOrganizationMembershipGrantCommand;
use appletheia::application::event::EventEnvelope;
use appletheia::application::saga::{Saga, SagaInstance, SagaSpec};
use banking_iam_domain::{
    OrganizationJoinRequest, OrganizationJoinRequestEventPayload,
    OrganizationMembershipGrantRejectionReason, OrganizationRoles, User, UserEventPayload,
};

use super::{
    OrganizationJoinRequestSagaError, OrganizationJoinRequestSagaSpec,
    OrganizationJoinRequestSagaState, OrganizationJoinRequestSagaStatus,
};

/// Coordinates organization join request workflow into organization membership grant.
pub struct OrganizationJoinRequestSaga;

impl Saga for OrganizationJoinRequestSaga {
    type Spec = OrganizationJoinRequestSagaSpec;
    type Error = OrganizationJoinRequestSagaError;

    fn on_event(
        &self,
        instance: &mut SagaInstance<<Self::Spec as SagaSpec>::State>,
        event: &EventEnvelope,
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
                    event,
                    &UserOrganizationMembershipGrantCommand {
                        user_id: *requester_id,
                        organization_id: *organization_id,
                        roles: OrganizationRoles::default(),
                    },
                )?;
            }

            return Ok(());
        } else if event.is_for_aggregate::<User>() {
            let user_event = event.try_into_domain_event::<User>()?;
            match user_event.payload() {
                UserEventPayload::OrganizationMembershipGranted { .. } => {
                    instance.state_required_mut()?.status =
                        OrganizationJoinRequestSagaStatus::MembershipGranted;
                    instance.succeed();
                }
                UserEventPayload::OrganizationMembershipGrantRejected { reason, .. } => {
                    if *reason == OrganizationMembershipGrantRejectionReason::AlreadyMember {
                        instance.state_required_mut()?.status =
                            OrganizationJoinRequestSagaStatus::AlreadyMember;
                        instance.succeed();
                    } else {
                        instance.state_required_mut()?.status =
                            OrganizationJoinRequestSagaStatus::Failed;
                        instance.fail();
                    }
                }
                _ => {}
            }
        }

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
        OrganizationJoinRequestId, OrganizationMembershipGrantRejectionReason, OrganizationRoles,
        User, UserEventPayload, UserId,
    };

    use crate::command::UserOrganizationMembershipGrantCommand;

    use super::{OrganizationJoinRequestSaga, OrganizationJoinRequestSagaSpec};

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

    fn user_membership_granted_event_envelope(correlation_id: CorrelationId) -> EventEnvelope {
        let user_id = UserId::new();
        let payload = UserEventPayload::OrganizationMembershipGranted {
            organization_id: OrganizationId::new(),
            roles: OrganizationRoles::default(),
        };

        EventEnvelope {
            event_sequence: EventSequence::try_from(2).expect("sequence should be valid"),
            event_id: EventId::new(),
            aggregate_type: AggregateTypeOwned::from(User::TYPE),
            aggregate_id: AggregateIdValue::from(user_id.value()),
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

    fn user_membership_grant_rejected_event_envelope(
        correlation_id: CorrelationId,
    ) -> EventEnvelope {
        let user_id = UserId::new();
        let payload = UserEventPayload::OrganizationMembershipGrantRejected {
            organization_id: OrganizationId::new(),
            roles: OrganizationRoles::default(),
            reason: OrganizationMembershipGrantRejectionReason::OrganizationRemoved,
        };

        EventEnvelope {
            event_sequence: EventSequence::try_from(2).expect("sequence should be valid"),
            event_id: EventId::new(),
            aggregate_type: AggregateTypeOwned::from(User::TYPE),
            aggregate_id: AggregateIdValue::from(user_id.value()),
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
    fn approved_event_appends_user_membership_grant_command() {
        let saga = OrganizationJoinRequestSaga;
        let correlation_id = CorrelationId::from(Uuid::now_v7());
        let organization_id = OrganizationId::new();
        let join_request_id = OrganizationJoinRequestId::new();
        let requester_id = UserId::new();
        let mut instance =
            SagaInstance::<<OrganizationJoinRequestSagaSpec as SagaSpec>::State>::new(
                SagaNameOwned::from(OrganizationJoinRequestSagaSpec::DESCRIPTOR.name),
                correlation_id,
                EventId::new(),
            );

        saga.on_event(
            &mut instance,
            &join_request_approved_event_envelope(
                correlation_id,
                organization_id,
                join_request_id,
                requester_id,
            ),
        )
        .expect("approved event should be handled");

        assert_eq!(instance.status, SagaStatus::InProgress);
        assert_eq!(instance.uncommitted_commands().len(), 1);
        let command: UserOrganizationMembershipGrantCommand = instance.uncommitted_commands()[0]
            .try_into_command()
            .expect("command should deserialize");
        assert_eq!(command.user_id, requester_id);
        assert_eq!(command.organization_id, organization_id);
        assert_eq!(command.roles, OrganizationRoles::default());
    }

    #[test]
    fn granted_user_membership_completes_saga() {
        let saga = OrganizationJoinRequestSaga;
        let correlation_id = CorrelationId::from(Uuid::now_v7());
        let organization_id = OrganizationId::new();
        let join_request_id = OrganizationJoinRequestId::new();
        let requester_id = UserId::new();
        let mut instance =
            SagaInstance::<<OrganizationJoinRequestSagaSpec as SagaSpec>::State>::new(
                SagaNameOwned::from(OrganizationJoinRequestSagaSpec::DESCRIPTOR.name),
                correlation_id,
                EventId::new(),
            );

        saga.on_event(
            &mut instance,
            &join_request_approved_event_envelope(
                correlation_id,
                organization_id,
                join_request_id,
                requester_id,
            ),
        )
        .expect("approved event should be handled");
        saga.on_event(
            &mut instance,
            &user_membership_granted_event_envelope(correlation_id),
        )
        .expect("membership granted event should be handled");

        assert_eq!(instance.status, SagaStatus::Succeeded);
        assert!(instance.uncommitted_commands().is_empty());
    }

    #[test]
    fn grant_rejected_user_membership_fails_saga() {
        let saga = OrganizationJoinRequestSaga;
        let correlation_id = CorrelationId::from(Uuid::now_v7());
        let organization_id = OrganizationId::new();
        let join_request_id = OrganizationJoinRequestId::new();
        let requester_id = UserId::new();
        let mut instance =
            SagaInstance::<<OrganizationJoinRequestSagaSpec as SagaSpec>::State>::new(
                SagaNameOwned::from(OrganizationJoinRequestSagaSpec::DESCRIPTOR.name),
                correlation_id,
                EventId::new(),
            );

        saga.on_event(
            &mut instance,
            &join_request_approved_event_envelope(
                correlation_id,
                organization_id,
                join_request_id,
                requester_id,
            ),
        )
        .expect("approved event should be handled");
        saga.on_event(
            &mut instance,
            &user_membership_grant_rejected_event_envelope(correlation_id),
        )
        .expect("membership grant rejected event should be handled");

        assert_eq!(instance.status, SagaStatus::Failed);
        assert!(instance.uncommitted_commands().is_empty());
    }
}
