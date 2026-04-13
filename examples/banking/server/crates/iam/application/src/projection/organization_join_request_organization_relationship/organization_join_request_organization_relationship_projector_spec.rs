use appletheia::application::event::EventSelector;
use appletheia::application::messaging::Subscription;
use appletheia::application::projection::{ProjectorDescriptor, ProjectorName, ProjectorSpec};
use appletheia::domain::Aggregate;
use banking_iam_domain::{OrganizationJoinRequest, OrganizationJoinRequestEventPayload};

/// Declares the subscription for the organization join request organization relationship projector.
pub struct OrganizationJoinRequestOrganizationRelationshipProjectorSpec;

impl ProjectorSpec for OrganizationJoinRequestOrganizationRelationshipProjectorSpec {
    const DESCRIPTOR: ProjectorDescriptor = ProjectorDescriptor::new(
        ProjectorName::new("organization_join_request_organization_relationship"),
        Subscription::AnyOf(&[EventSelector::new(
            OrganizationJoinRequest::TYPE,
            OrganizationJoinRequestEventPayload::REQUESTED,
        )]),
    );
}
