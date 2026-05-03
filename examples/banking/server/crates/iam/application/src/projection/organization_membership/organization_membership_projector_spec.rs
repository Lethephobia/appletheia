use appletheia::application::event::EventSelector;
use appletheia::application::messaging::Subscription;
use appletheia::application::projection::{ProjectorDescriptor, ProjectorName, ProjectorSpec};
use appletheia::domain::Aggregate;
use banking_iam_domain::{OrganizationMembership, OrganizationMembershipEventPayload};

/// Declares the subscription for the membership projection projector.
pub struct OrganizationMembershipProjectorSpec;

impl ProjectorSpec for OrganizationMembershipProjectorSpec {
    const DESCRIPTOR: ProjectorDescriptor = ProjectorDescriptor::new(
        ProjectorName::new("membership"),
        Subscription::AnyOf(&[
            EventSelector::new(
                OrganizationMembership::TYPE,
                OrganizationMembershipEventPayload::CREATED,
            ),
            EventSelector::new(
                OrganizationMembership::TYPE,
                OrganizationMembershipEventPayload::ACTIVATED,
            ),
            EventSelector::new(
                OrganizationMembership::TYPE,
                OrganizationMembershipEventPayload::INACTIVATED,
            ),
            EventSelector::new(
                OrganizationMembership::TYPE,
                OrganizationMembershipEventPayload::REMOVED,
            ),
        ]),
    );
}
