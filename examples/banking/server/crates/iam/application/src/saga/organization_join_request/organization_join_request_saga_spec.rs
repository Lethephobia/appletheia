use appletheia::application::event::EventSelector;
use appletheia::application::messaging::Subscription;
use appletheia::application::saga::{SagaDescriptor, SagaName, SagaSpec};
use appletheia::domain::Aggregate;
use banking_iam_domain::{
    OrganizationJoinRequest, OrganizationJoinRequestEventPayload, OrganizationMembership,
    OrganizationMembershipEventPayload,
};

use super::OrganizationJoinRequestSagaState;

/// Declares the descriptor and state for the organization join request saga.
pub struct OrganizationJoinRequestSagaSpec;

impl SagaSpec for OrganizationJoinRequestSagaSpec {
    type State = OrganizationJoinRequestSagaState;

    const DESCRIPTOR: SagaDescriptor = SagaDescriptor::new(
        SagaName::new("organization_join_request"),
        Subscription::Only(&[
            EventSelector::new(
                OrganizationJoinRequest::TYPE,
                OrganizationJoinRequestEventPayload::APPROVED,
            ),
            EventSelector::new(
                OrganizationMembership::TYPE,
                OrganizationMembershipEventPayload::CREATED,
            ),
        ]),
    );
}
