use appletheia::application::event::EventEnvelope;
use appletheia::application::saga::{Saga, SagaInstance, SagaSpec};
use appletheia::domain::Aggregate;
use banking_iam_domain::{
    OrganizationInvitation, OrganizationInvitationEventPayload, OrganizationMembership,
    OrganizationMembershipEventPayload,
};

use crate::command::OrganizationMembershipCreateCommand;

use super::{
    OrganizationInvitationSagaError, OrganizationInvitationSagaSpec,
    OrganizationInvitationSagaState,
};

/// Coordinates organization invitation workflow into organization membership creation.
pub struct OrganizationInvitationSaga;

impl Saga for OrganizationInvitationSaga {
    type Spec = OrganizationInvitationSagaSpec;
    type Error = OrganizationInvitationSagaError;

    fn on_event(
        &self,
        instance: &mut SagaInstance<<Self::Spec as SagaSpec>::State>,
        event: &EventEnvelope,
    ) -> Result<(), Self::Error> {
        if event.aggregate_type.value() == OrganizationInvitation::TYPE.value() {
            let invitation_event = event.try_into_domain_event::<OrganizationInvitation>()?;
            if let OrganizationInvitationEventPayload::Accepted {
                organization_id,
                invitee_id,
            } = invitation_event.payload()
            {
                let state = instance
                    .state_mut()
                    .get_or_insert_with(OrganizationInvitationSagaState::default);
                state.organization_invitation_id = Some(invitation_event.aggregate_id());
                state.organization_id = Some(*organization_id);
                state.invitee_id = Some(*invitee_id);

                instance.append_command(
                    event,
                    &OrganizationMembershipCreateCommand {
                        organization_id: *organization_id,
                        user_id: *invitee_id,
                    },
                )?;
            }

            return Ok(());
        }

        if event.aggregate_type.value() == OrganizationMembership::TYPE.value() {
            let membership_event = event.try_into_domain_event::<OrganizationMembership>()?;
            if let OrganizationMembershipEventPayload::Created {
                organization_id,
                user_id,
                ..
            } = membership_event.payload()
            {
                let Some(state) = instance.state.as_ref() else {
                    return Ok(());
                };

                if state.organization_id == Some(*organization_id)
                    && state.invitee_id == Some(*user_id)
                {
                    instance.succeed();
                }
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use appletheia::application::event::{
        AggregateIdValue, AggregateTypeOwned, EventEnvelope, EventNameOwned, EventSequence,
        SerializedEventPayload,
    };
    use appletheia::application::request_context::{
        ActorRef, CausationId, CorrelationId, MessageId, Principal, RequestContext,
    };
    use appletheia::application::saga::{Saga, SagaInstance, SagaNameOwned, SagaSpec, SagaStatus};
    use appletheia::domain::{Aggregate, AggregateId, EventId, EventOccurredAt, EventPayload};
    use banking_iam_domain::{
        OrganizationId, OrganizationInvitation, OrganizationInvitationEventPayload,
        OrganizationInvitationId, OrganizationMembership, OrganizationMembershipEventPayload,
        OrganizationMembershipId, User, UserId,
    };

    use crate::command::OrganizationMembershipCreateCommand;

    use super::{OrganizationInvitationSaga, OrganizationInvitationSagaSpec};

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

    fn invitation_accepted_event_envelope(
        correlation_id: CorrelationId,
        organization_id: OrganizationId,
        invitation_id: OrganizationInvitationId,
        invitee_id: UserId,
    ) -> EventEnvelope {
        let payload = OrganizationInvitationEventPayload::Accepted {
            organization_id,
            invitee_id,
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

    fn membership_created_event_envelope(
        correlation_id: CorrelationId,
        organization_id: OrganizationId,
        user_id: UserId,
    ) -> EventEnvelope {
        let membership_id = OrganizationMembershipId::new();
        let payload = OrganizationMembershipEventPayload::Created {
            id: membership_id,
            organization_id,
            user_id,
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
        let correlation_id = CorrelationId::from(uuid::Uuid::now_v7());
        let organization_id = OrganizationId::new();
        let invitation_id = OrganizationInvitationId::new();
        let invitee_id = UserId::new();
        let mut instance = SagaInstance::<<OrganizationInvitationSagaSpec as SagaSpec>::State>::new(
            SagaNameOwned::from(OrganizationInvitationSagaSpec::DESCRIPTOR.name),
            correlation_id,
        );

        saga.on_event(
            &mut instance,
            &invitation_accepted_event_envelope(
                correlation_id,
                organization_id,
                invitation_id,
                invitee_id,
            ),
        )
        .expect("accepted event should be handled");

        assert_eq!(instance.status, SagaStatus::InProgress);
        assert_eq!(instance.uncommitted_commands().len(), 1);
        let command: OrganizationMembershipCreateCommand = instance.uncommitted_commands()[0]
            .try_into_command()
            .expect("command should deserialize");
        assert_eq!(command.organization_id, organization_id);
        assert_eq!(command.user_id, invitee_id);
    }

    #[test]
    fn created_membership_completes_saga() {
        let saga = OrganizationInvitationSaga;
        let correlation_id = CorrelationId::from(uuid::Uuid::now_v7());
        let organization_id = OrganizationId::new();
        let invitation_id = OrganizationInvitationId::new();
        let invitee_id = UserId::new();
        let mut instance = SagaInstance::<<OrganizationInvitationSagaSpec as SagaSpec>::State>::new(
            SagaNameOwned::from(OrganizationInvitationSagaSpec::DESCRIPTOR.name),
            correlation_id,
        );

        saga.on_event(
            &mut instance,
            &invitation_accepted_event_envelope(
                correlation_id,
                organization_id,
                invitation_id,
                invitee_id,
            ),
        )
        .expect("accepted event should be handled");
        saga.on_event(
            &mut instance,
            &membership_created_event_envelope(correlation_id, organization_id, invitee_id),
        )
        .expect("membership created event should be handled");

        assert_eq!(instance.status, SagaStatus::Succeeded);
        assert!(instance.uncommitted_commands().is_empty());
    }
}
