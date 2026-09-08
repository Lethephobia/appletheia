use appletheia::application::saga::SagaError;
use appletheia::application::saga::{Saga, SagaDefinition, SagaDefinitionBuilder, SagaName};
use banking_iam_domain::{Organization, OrganizationEventPayload};

use crate::command::OrganizationPictureObjectDeleteCommand;

use super::{
    OrganizationOldPictureObjectDeletionSagaHandlerError,
    OrganizationOldPictureObjectDeletionSagaState, OrganizationOldPictureObjectDeletionSagaStep,
};

/// Coordinates old organization picture object deletion after picture changes.
pub struct OrganizationOldPictureObjectDeletionSaga;

impl Saga for OrganizationOldPictureObjectDeletionSaga {
    type State = OrganizationOldPictureObjectDeletionSagaState;
    type Step = OrganizationOldPictureObjectDeletionSagaStep;
    type HandlerError = OrganizationOldPictureObjectDeletionSagaHandlerError;

    fn definition(
        &self,
    ) -> Result<SagaDefinition<'_, Self::State, Self::Step, Self::HandlerError>, SagaError> {
        SagaDefinitionBuilder::<Self::State, Self::Step, Self::HandlerError>::new(SagaName::new(
            "organization_old_picture_object_deletion",
        ))
        .add_start_step(OrganizationOldPictureObjectDeletionSagaStep::DeletePictureObject)
        .on::<Organization>(OrganizationEventPayload::PICTURE_CHANGED)
        .handle(|ctx, domain_event| {
            let OrganizationEventPayload::PictureChanged { old_picture, .. } =
                domain_event.payload()
            else {
                return Err(OrganizationOldPictureObjectDeletionSagaHandlerError::UnexpectedEvent);
            };

            let state =
                OrganizationOldPictureObjectDeletionSagaState::new(domain_event.aggregate_id());
            ctx.set_state(state);
            let Some(object_name) = old_picture
                .as_ref()
                .and_then(|picture| picture.as_object_name())
                .cloned()
            else {
                return Ok(());
            };

            ctx.append_command(&OrganizationPictureObjectDeleteCommand { object_name })
                .map_err(|_| {
                    OrganizationOldPictureObjectDeletionSagaHandlerError::UnexpectedEvent
                })?;
            Ok(())
        })
        .build()
        .map_err(SagaError::from)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use appletheia::application::event::{
        AggregateIdValue, AggregateTypeOwned, EventEnvelope, EventNameOwned, EventSequence,
        SerializedEventPayload,
    };
    use appletheia::application::request_context::{
        CausationId, CorrelationId, MessageId, Principal, RequestContext,
    };
    use appletheia::application::saga::{SagaContext, SagaRoute};
    use appletheia::application::saga::{SagaInstance, SagaNameOwned};
    use appletheia::domain::{
        Aggregate, AggregateId, AggregateVersion, EventId, EventOccurredAt, EventPayload,
    };
    use banking_iam_domain::{
        OrganizationId, OrganizationPictureObjectName, OrganizationPictureRef,
    };

    #[test]
    fn old_picture_command_is_retained_and_absent_picture_does_not_enqueue() {
        let saga = OrganizationOldPictureObjectDeletionSaga;
        let definition = saga.definition().expect("valid definition");
        let aggregate_id = OrganizationId::new();
        let object_name = OrganizationPictureObjectName::new(aggregate_id);
        for old_picture in [
            Some(OrganizationPictureRef::object_name(object_name.clone())),
            None,
        ] {
            let should_enqueue = old_picture.is_some();
            let payload = OrganizationEventPayload::PictureChanged {
                picture: None,
                old_picture,
            };
            let message_id = MessageId::new();
            let correlation_id = CorrelationId::from(message_id.value());
            let input = EventEnvelope {
                event_sequence: EventSequence::try_from(1).expect("sequence"),
                event_id: EventId::new(),
                aggregate_type: AggregateTypeOwned::from(Organization::TYPE),
                aggregate_id: AggregateIdValue::from(aggregate_id.value()),
                aggregate_version: AggregateVersion::try_from(1).expect("version"),
                event_name: EventNameOwned::from(payload.name()),
                payload: SerializedEventPayload::try_from(
                    payload.into_json_value().expect("serialize"),
                )
                .expect("payload"),
                occurred_at: EventOccurredAt::now(),
                correlation_id,
                causation_id: CausationId::from(message_id),
                context: RequestContext::new(correlation_id, message_id, Principal::System)
                    .expect("context"),
            };
            let mut instance = SagaInstance::new(
                SagaNameOwned::from(definition.name()),
                correlation_id,
                input.event_id,
            );
            let Some(SagaRoute::StartsOn { step, handler, .. }) =
                definition.find_event_route(&input, None)
            else {
                panic!("picture change route");
            };
            let mut context =
                SagaContext::new(&mut instance, CausationId::from(input.event_id), *step);
            handler(&mut context, &input).expect("handle picture change");
            assert_eq!(
                instance.uncommitted_commands().len(),
                usize::from(should_enqueue)
            );
            if should_enqueue {
                let command = &instance.uncommitted_commands()[0];
                assert_eq!(command.causation_id, CausationId::from(input.event_id));
                assert_eq!(
                    command
                        .try_into_command::<OrganizationPictureObjectDeleteCommand>()
                        .expect("delete command")
                        .object_name,
                    object_name
                );
            }
        }
    }
}
