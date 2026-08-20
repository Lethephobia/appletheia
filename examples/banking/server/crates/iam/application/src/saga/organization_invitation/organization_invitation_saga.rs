use crate::command::OrganizationMembershipCreateCommand;
use appletheia::application::event::EventEnvelope;
use appletheia::application::saga::{Saga, SagaInstance, SagaSpec};
use banking_iam_domain::{
    OrganizationInvitation, OrganizationInvitationEventPayload, OrganizationMembership,
    OrganizationMembershipCreateRejectionReason, OrganizationMembershipEventPayload,
};

use super::{
    OrganizationInvitationSagaError, OrganizationInvitationSagaSpec,
    OrganizationInvitationSagaState, OrganizationInvitationSagaStatus,
};

/// Coordinates the organization invitation workflow into organization membership creation.
pub struct OrganizationInvitationSaga;

impl Saga for OrganizationInvitationSaga {
    type Spec = OrganizationInvitationSagaSpec;
    type Error = OrganizationInvitationSagaError;

    fn on_event(
        &self,
        instance: &mut SagaInstance<<Self::Spec as SagaSpec>::State>,
        event: &EventEnvelope,
    ) -> Result<(), Self::Error> {
        if event.is_for_aggregate::<OrganizationInvitation>() {
            let invitation_event = event.try_into_domain_event::<OrganizationInvitation>()?;
            if let OrganizationInvitationEventPayload::Accepted {
                organization_id,
                invitee_id,
                roles,
            } = invitation_event.payload()
            {
                *instance.state_mut() = Some(OrganizationInvitationSagaState::new(
                    invitation_event.aggregate_id(),
                ));

                instance.append_command(
                    event,
                    &OrganizationMembershipCreateCommand {
                        organization_id: *organization_id,
                        user_id: *invitee_id,
                        roles: roles.clone(),
                    },
                )?;
            }

            return Ok(());
        } else if event.is_for_aggregate::<OrganizationMembership>() {
            let membership_event = event.try_into_domain_event::<OrganizationMembership>()?;
            match membership_event.payload() {
                OrganizationMembershipEventPayload::Created { .. } => {
                    instance.state_required_mut()?.status =
                        OrganizationInvitationSagaStatus::MembershipCreated;
                    instance.succeed();
                }
                OrganizationMembershipEventPayload::CreateRejected { reason, .. } => {
                    // A duplicate invitation delivery finds the pair already
                    // taken; that is a successful outcome for this workflow,
                    // not a failure to compensate.
                    if *reason == OrganizationMembershipCreateRejectionReason::AlreadyMember {
                        instance.state_required_mut()?.status =
                            OrganizationInvitationSagaStatus::AlreadyMember;
                        instance.succeed();
                    } else {
                        instance.state_required_mut()?.status =
                            OrganizationInvitationSagaStatus::Failed;
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
        OrganizationId, OrganizationInvitation, OrganizationInvitationEventPayload,
        OrganizationInvitationId, OrganizationMembership,
        OrganizationMembershipCreateRejectionReason, OrganizationMembershipEventPayload,
        OrganizationMembershipId, OrganizationRoles, User, UserId,
    };

    use crate::command::OrganizationMembershipCreateCommand;

    use super::{OrganizationInvitationSaga, OrganizationInvitationSagaSpec};

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

    fn invitation_accepted_event_envelope(
        correlation_id: CorrelationId,
        organization_id: OrganizationId,
        invitation_id: OrganizationInvitationId,
        invitee_id: UserId,
        roles: OrganizationRoles,
    ) -> EventEnvelope {
        let payload = OrganizationInvitationEventPayload::Accepted {
            organization_id,
            invitee_id,
            roles,
        };

        EventEnvelope {
            event_sequence: EventSequence::try_from(1).expect("sequence should be valid"),
            event_id: EventId::new(),
            aggregate_type: AggregateTypeOwned::from(OrganizationInvitation::TYPE),
            aggregate_id: AggregateIdValue::from(invitation_id.value()),
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

    fn membership_create_rejected_event_envelope(
        correlation_id: CorrelationId,
        reason: OrganizationMembershipCreateRejectionReason,
    ) -> EventEnvelope {
        let membership_id = OrganizationMembershipId::new();
        let payload = OrganizationMembershipEventPayload::CreateRejected {
            organization_id: OrganizationId::new(),
            user_id: UserId::new(),
            roles: OrganizationRoles::default(),
            reason,
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

    #[test]
    fn accepted_event_appends_membership_create_command() {
        let saga = OrganizationInvitationSaga;
        let correlation_id = CorrelationId::from(Uuid::now_v7());
        let organization_id = OrganizationId::new();
        let invitation_id = OrganizationInvitationId::new();
        let invitee_id = UserId::new();
        let roles = OrganizationRoles::default();
        let mut instance = SagaInstance::<<OrganizationInvitationSagaSpec as SagaSpec>::State>::new(
            SagaNameOwned::from(OrganizationInvitationSagaSpec::DESCRIPTOR.name),
            correlation_id,
            EventId::new(),
        );

        saga.on_event(
            &mut instance,
            &invitation_accepted_event_envelope(
                correlation_id,
                organization_id,
                invitation_id,
                invitee_id,
                roles.clone(),
            ),
        )
        .expect("accepted event should be handled");

        assert_eq!(instance.status, SagaStatus::InProgress);
        assert_eq!(instance.uncommitted_commands().len(), 1);
        let command: OrganizationMembershipCreateCommand = instance.uncommitted_commands()[0]
            .try_into_command()
            .expect("command should deserialize");
        assert_eq!(command.user_id, invitee_id);
        assert_eq!(command.organization_id, organization_id);
        assert_eq!(command.roles, roles);
    }

    #[test]
    fn created_membership_completes_saga() {
        let saga = OrganizationInvitationSaga;
        let correlation_id = CorrelationId::from(Uuid::now_v7());
        let organization_id = OrganizationId::new();
        let invitation_id = OrganizationInvitationId::new();
        let invitee_id = UserId::new();
        let roles = OrganizationRoles::default();
        let mut instance = SagaInstance::<<OrganizationInvitationSagaSpec as SagaSpec>::State>::new(
            SagaNameOwned::from(OrganizationInvitationSagaSpec::DESCRIPTOR.name),
            correlation_id,
            EventId::new(),
        );

        saga.on_event(
            &mut instance,
            &invitation_accepted_event_envelope(
                correlation_id,
                organization_id,
                invitation_id,
                invitee_id,
                roles,
            ),
        )
        .expect("accepted event should be handled");
        saga.on_event(
            &mut instance,
            &membership_created_event_envelope(correlation_id),
        )
        .expect("membership created event should be handled");

        assert_eq!(instance.status, SagaStatus::Succeeded);
        assert!(instance.uncommitted_commands().is_empty());
    }

    #[test]
    fn create_rejected_membership_fails_saga() {
        let saga = OrganizationInvitationSaga;
        let correlation_id = CorrelationId::from(Uuid::now_v7());
        let organization_id = OrganizationId::new();
        let invitation_id = OrganizationInvitationId::new();
        let invitee_id = UserId::new();
        let roles = OrganizationRoles::default();
        let mut instance = SagaInstance::<<OrganizationInvitationSagaSpec as SagaSpec>::State>::new(
            SagaNameOwned::from(OrganizationInvitationSagaSpec::DESCRIPTOR.name),
            correlation_id,
            EventId::new(),
        );

        saga.on_event(
            &mut instance,
            &invitation_accepted_event_envelope(
                correlation_id,
                organization_id,
                invitation_id,
                invitee_id,
                roles,
            ),
        )
        .expect("accepted event should be handled");
        saga.on_event(
            &mut instance,
            &membership_create_rejected_event_envelope(
                correlation_id,
                OrganizationMembershipCreateRejectionReason::OrganizationRemoved,
            ),
        )
        .expect("membership create rejected event should be handled");

        assert_eq!(instance.status, SagaStatus::Failed);
        assert!(instance.uncommitted_commands().is_empty());
    }

    #[test]
    fn already_member_create_rejection_completes_saga() {
        let saga = OrganizationInvitationSaga;
        let correlation_id = CorrelationId::from(Uuid::now_v7());
        let organization_id = OrganizationId::new();
        let invitation_id = OrganizationInvitationId::new();
        let invitee_id = UserId::new();
        let roles = OrganizationRoles::default();
        let mut instance = SagaInstance::<<OrganizationInvitationSagaSpec as SagaSpec>::State>::new(
            SagaNameOwned::from(OrganizationInvitationSagaSpec::DESCRIPTOR.name),
            correlation_id,
            EventId::new(),
        );

        saga.on_event(
            &mut instance,
            &invitation_accepted_event_envelope(
                correlation_id,
                organization_id,
                invitation_id,
                invitee_id,
                roles,
            ),
        )
        .expect("accepted event should be handled");
        saga.on_event(
            &mut instance,
            &membership_create_rejected_event_envelope(
                correlation_id,
                OrganizationMembershipCreateRejectionReason::AlreadyMember,
            ),
        )
        .expect("membership create rejected event should be handled");

        assert_eq!(instance.status, SagaStatus::Succeeded);
        assert!(instance.uncommitted_commands().is_empty());
    }
}
