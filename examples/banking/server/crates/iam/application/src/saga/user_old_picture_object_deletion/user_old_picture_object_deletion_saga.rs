use appletheia::application::saga::SagaError;
use appletheia::application::saga::{Saga, SagaDefinition, SagaDefinitionBuilder, SagaName};
use banking_iam_domain::{User, UserEventPayload};

use crate::command::UserPictureObjectDeleteCommand;

use super::{
    UserOldPictureObjectDeletionSagaHandlerError, UserOldPictureObjectDeletionSagaState,
    UserOldPictureObjectDeletionSagaStep,
};

/// Coordinates old user picture object deletion after picture changes.
pub struct UserOldPictureObjectDeletionSaga;

impl Saga for UserOldPictureObjectDeletionSaga {
    type State = UserOldPictureObjectDeletionSagaState;
    type Step = UserOldPictureObjectDeletionSagaStep;
    type HandlerError = UserOldPictureObjectDeletionSagaHandlerError;

    fn definition(
        &self,
    ) -> Result<SagaDefinition<'_, Self::State, Self::Step, Self::HandlerError>, SagaError> {
        SagaDefinitionBuilder::<Self::State, Self::Step, Self::HandlerError>::new(SagaName::new(
            "user_old_picture_object_deletion",
        ))
        .add_start_step(UserOldPictureObjectDeletionSagaStep::DeletePictureObject)
        .on::<User>(UserEventPayload::PICTURE_CHANGED)
        .handle(|ctx, domain_event| {
            let UserEventPayload::PictureChanged { old_picture, .. } = domain_event.payload()
            else {
                return Err(UserOldPictureObjectDeletionSagaHandlerError::UnexpectedEvent);
            };

            let state = UserOldPictureObjectDeletionSagaState::new(domain_event.aggregate_id());
            ctx.set_state(state);
            let Some(object_name) = old_picture
                .as_ref()
                .and_then(|picture| picture.as_object_name())
                .cloned()
            else {
                return Ok(());
            };

            ctx.append_command(&UserPictureObjectDeleteCommand { object_name })
                .map_err(|_| UserOldPictureObjectDeletionSagaHandlerError::UnexpectedEvent)?;
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
    use banking_iam_domain::{UserId, UserPictureObjectName, UserPictureRef};

    #[test]
    fn old_picture_command_is_retained_and_absent_picture_does_not_enqueue() {
        let saga = UserOldPictureObjectDeletionSaga;
        let definition = saga.definition().expect("valid definition");
        let aggregate_id = UserId::new();
        let object_name = UserPictureObjectName::new(aggregate_id);
        for old_picture in [Some(UserPictureRef::object_name(object_name.clone())), None] {
            let should_enqueue = old_picture.is_some();
            let payload = UserEventPayload::PictureChanged {
                picture: None,
                old_picture,
            };
            let message_id = MessageId::new();
            let correlation_id = CorrelationId::from(message_id.value());
            let input = EventEnvelope {
                event_sequence: EventSequence::try_from(1).expect("sequence"),
                event_id: EventId::new(),
                aggregate_type: AggregateTypeOwned::from(User::TYPE),
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
                        .try_into_command::<UserPictureObjectDeleteCommand>()
                        .expect("delete command")
                        .object_name,
                    object_name
                );
            }
        }
    }
}
