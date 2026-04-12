use appletheia::application::event::EventSelector;
use appletheia::application::saga::{SagaDescriptor, SagaName, SagaPredecessor, SagaSpec};
use appletheia::domain::Aggregate;
use banking_iam_domain::{OrganizationJoinRequest, OrganizationJoinRequestEventPayload};

/// Declares the descriptor and state for the organization join request saga.
pub struct OrganizationJoinRequestApprovedSagaSpec;

impl SagaSpec for OrganizationJoinRequestApprovedSagaSpec {
    const DESCRIPTOR: SagaDescriptor = SagaDescriptor::new(
        SagaName::new("organization_join_request_approved"),
        EventSelector::new(
            OrganizationJoinRequest::TYPE,
            OrganizationJoinRequestEventPayload::APPROVED,
        ),
        SagaPredecessor::None,
    );
}
