use appletheia::application::command::CommandOptions;
use appletheia::application::event::EventEnvelope;
use appletheia::application::saga::{Saga, SagaInstance, SagaSpec};
use appletheia::domain::Aggregate;
use banking_iam_domain::{
    OrganizationJoinRequest, OrganizationJoinRequestEventPayload, OrganizationMembership,
    OrganizationMembershipEventPayload,
};

use crate::command::OrganizationMembershipCreateCommand;

use super::{
    OrganizationJoinRequestSagaError, OrganizationJoinRequestSagaSpec,
    OrganizationJoinRequestSagaState,
};

/// Coordinates organization join request workflow into organization membership creation.
pub struct OrganizationJoinRequestSaga;

impl Saga for OrganizationJoinRequestSaga {
    type Spec = OrganizationJoinRequestSagaSpec;
    type Error = OrganizationJoinRequestSagaError;

    fn on_event(
        &self,
        instance: &mut SagaInstance<<Self::Spec as SagaSpec>::State>,
        event: &EventEnvelope,
    ) -> Result<(), Self::Error> {
        if event.aggregate_type.value() == OrganizationJoinRequest::TYPE.value() {
            let join_request_event = event.try_into_domain_event::<OrganizationJoinRequest>()?;
            if let OrganizationJoinRequestEventPayload::Approved {
                organization_id,
                requester_id,
            } = join_request_event.payload()
            {
                let state = instance
                    .state_mut()
                    .get_or_insert_with(OrganizationJoinRequestSagaState::default);
                state.organization_join_request_id = Some(join_request_event.aggregate_id());

                instance.append_command(
                    event,
                    &OrganizationMembershipCreateCommand {
                        organization_id: *organization_id,
                        user_id: *requester_id,
                    },
                    CommandOptions::default(),
                )?;
            }

            return Ok(());
        }

        if event.aggregate_type.value() == OrganizationMembership::TYPE.value() {
            let membership_event = event.try_into_domain_event::<OrganizationMembership>()?;
            if let OrganizationMembershipEventPayload::Created { .. } = membership_event.payload() {
                let Some(_) = instance.state.as_ref() else {
                    return Ok(());
                };

                instance.succeed();
            }
        }

        Ok(())
    }
}
