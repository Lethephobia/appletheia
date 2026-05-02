use appletheia::application::event::EventSelector;
use appletheia::application::messaging::Subscription;
use appletheia::application::projection::{ProjectorDescriptor, ProjectorName, ProjectorSpec};
use appletheia::domain::Aggregate;
use banking_iam_domain::{OrganizationJoinRequest, OrganizationJoinRequestEventPayload};

/// Declares the subscription for the organization join request view projector.
pub struct OrganizationJoinRequestProjectorSpec;

impl ProjectorSpec for OrganizationJoinRequestProjectorSpec {
    const DESCRIPTOR: ProjectorDescriptor = ProjectorDescriptor::new(
        ProjectorName::new("organization_join_request"),
        Subscription::AnyOf(&[
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
        ]),
    );
}
