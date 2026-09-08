use crate::command::OrganizationMembershipCreateCommand;
use appletheia::application::saga::SagaError;
use appletheia::application::saga::{Saga, SagaDefinition, SagaDefinitionBuilder, SagaName};
use banking_iam_domain::{OrganizationInvitation, OrganizationInvitationEventPayload};

use super::{
    OrganizationInvitationSagaHandlerError, OrganizationInvitationSagaState,
    OrganizationInvitationSagaStep,
};

/// Coordinates the organization invitation workflow into organization membership creation.
pub struct OrganizationInvitationSaga;

impl Saga for OrganizationInvitationSaga {
    type State = OrganizationInvitationSagaState;
    type Step = OrganizationInvitationSagaStep;
    type HandlerError = OrganizationInvitationSagaHandlerError;

    fn definition(
        &self,
    ) -> Result<SagaDefinition<'_, Self::State, Self::Step, Self::HandlerError>, SagaError> {
        SagaDefinitionBuilder::<Self::State, Self::Step, Self::HandlerError>::new(SagaName::new(
            "organization_invitation",
        ))
        .add_start_step(OrganizationInvitationSagaStep::CreateMembership)
        .on::<OrganizationInvitation>(OrganizationInvitationEventPayload::ACCEPTED)
        .handle(|ctx, invitation_event| {
            if let OrganizationInvitationEventPayload::Accepted {
                organization_id,
                invitee_id,
                roles,
            } = invitation_event.payload()
            {
                ctx.set_state(OrganizationInvitationSagaState::new(
                    invitation_event.aggregate_id(),
                ));

                ctx.append_command(&OrganizationMembershipCreateCommand {
                    organization_id: *organization_id,
                    user_id: *invitee_id,
                    roles: roles.clone(),
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
    use super::OrganizationInvitationSagaHandlerError;
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
        OrganizationId, OrganizationInvitation, OrganizationInvitationEventPayload,
        OrganizationInvitationId, OrganizationMembership, OrganizationMembershipEventPayload,
        OrganizationMembershipId, OrganizationRoles, User, UserId,
    };

    use crate::command::OrganizationMembershipCreateCommand;

    use super::{OrganizationInvitationSaga, OrganizationInvitationSagaStep};

    fn request_context(correlation_id: CorrelationId) -> RequestContext {
        let subject = AggregateRef::from_id::<User>(UserId::new());

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
        saga: &OrganizationInvitationSaga,
        instance: &mut SagaInstance<
            <OrganizationInvitationSaga as Saga>::State,
            OrganizationInvitationSagaStep,
        >,
        envelope: &EventEnvelope,
        step: Option<OrganizationInvitationSagaStep>,
    ) -> Result<bool, SagaRouteError<OrganizationInvitationSagaHandlerError>> {
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
    fn accepted_event_appends_membership_create_command() {
        let saga = OrganizationInvitationSaga;
        let correlation_id = CorrelationId::from(Uuid::now_v7());
        let organization_id = OrganizationId::new();
        let invitation_id = OrganizationInvitationId::new();
        let invitee_id = UserId::new();
        let roles = OrganizationRoles::default();
        let mut instance = SagaInstance::<
            <OrganizationInvitationSaga as Saga>::State,
            OrganizationInvitationSagaStep,
        >::new(
            SagaNameOwned::from(
                OrganizationInvitationSaga
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
            &invitation_accepted_event_envelope(
                correlation_id,
                organization_id,
                invitation_id,
                invitee_id,
                roles.clone(),
            ),
            None,
        )
        .expect("accepted event should be handled");

        assert!(instance.state.is_some());
        assert_eq!(instance.uncommitted_commands().len(), 1);
        let command: OrganizationMembershipCreateCommand = instance.uncommitted_commands()[0]
            .try_into_command()
            .expect("command should deserialize");
        assert_eq!(command.user_id, invitee_id);
        assert_eq!(command.organization_id, organization_id);
        assert_eq!(command.roles, roles);
    }

    #[test]
    fn created_membership_is_not_subscribed() {
        let saga = OrganizationInvitationSaga;
        let correlation_id = CorrelationId::from(Uuid::now_v7());
        let organization_id = OrganizationId::new();
        let invitation_id = OrganizationInvitationId::new();
        let invitee_id = UserId::new();
        let roles = OrganizationRoles::default();
        let mut instance = SagaInstance::<
            <OrganizationInvitationSaga as Saga>::State,
            OrganizationInvitationSagaStep,
        >::new(
            SagaNameOwned::from(
                OrganizationInvitationSaga
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
            &invitation_accepted_event_envelope(
                correlation_id,
                organization_id,
                invitation_id,
                invitee_id,
                roles,
            ),
            None,
        )
        .expect("accepted event should be handled");
        let pending = instance.uncommitted_commands().to_vec();
        let matched = handle_event(
            &saga,
            &mut instance,
            &membership_created_event_envelope(correlation_id),
            Some(OrganizationInvitationSagaStep::CreateMembership),
        )
        .expect("membership created event should be handled");

        assert!(!matched);
        assert_eq!(instance.uncommitted_commands(), pending);
    }
}
