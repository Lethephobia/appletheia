use appletheia::application::event::EventSelector;
use appletheia::application::messaging::Subscription;
use appletheia::application::projection::{ProjectorDescriptor, ProjectorName, ProjectorSpec};
use appletheia::domain::Aggregate;
use banking_iam_domain::{
    Organization, OrganizationEventPayload, OrganizationJoinRequest,
    OrganizationJoinRequestEventPayload,
};

/// Declares the subscription for the organization join request requester relationship projector.
pub struct OrganizationJoinRequestRequesterRelationshipProjectorSpec;

impl ProjectorSpec for OrganizationJoinRequestRequesterRelationshipProjectorSpec {
    const DESCRIPTOR: ProjectorDescriptor = ProjectorDescriptor::new(
        ProjectorName::new("organization_join_request_requester_relationship"),
        Subscription::Only(&[
            EventSelector::new(
                OrganizationJoinRequest::TYPE,
                OrganizationJoinRequestEventPayload::REQUESTED,
            ),
            EventSelector::new(
                OrganizationJoinRequest::TYPE,
                OrganizationJoinRequestEventPayload::APPROVED,
            ),
            EventSelector::new(
                OrganizationJoinRequest::TYPE,
                OrganizationJoinRequestEventPayload::REJECTED,
            ),
            EventSelector::new(
                OrganizationJoinRequest::TYPE,
                OrganizationJoinRequestEventPayload::CANCELED,
            ),
            EventSelector::new(Organization::TYPE, OrganizationEventPayload::REMOVED),
        ]),
    );
}
