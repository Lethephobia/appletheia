use appletheia::application::event::EventSelector;
use appletheia::application::messaging::Subscription;
use appletheia::application::saga::{SagaDescriptor, SagaName, SagaSpec, SagaStartEvents};
use banking_iam_domain::{
    OrganizationJoinRequest, OrganizationJoinRequestEventPayload, User, UserEventPayload,
};

use super::OrganizationJoinRequestSagaState;

/// Declares the descriptor and state for the organization join request saga.
pub struct OrganizationJoinRequestSagaSpec;

impl SagaSpec for OrganizationJoinRequestSagaSpec {
    type State = OrganizationJoinRequestSagaState;

    const DESCRIPTOR: SagaDescriptor = SagaDescriptor::new(
        SagaName::new("organization_join_request"),
        SagaStartEvents::new(&[EventSelector::new::<OrganizationJoinRequest>(
            OrganizationJoinRequestEventPayload::APPROVED,
        )]),
        Subscription::AnyOf(&[
            EventSelector::new::<OrganizationJoinRequest>(
                OrganizationJoinRequestEventPayload::APPROVED,
            ),
            EventSelector::new::<User>(UserEventPayload::ORGANIZATION_MEMBERSHIP_GRANTED),
            EventSelector::new::<User>(UserEventPayload::ORGANIZATION_MEMBERSHIP_GRANT_REJECTED),
        ]),
    );
}
