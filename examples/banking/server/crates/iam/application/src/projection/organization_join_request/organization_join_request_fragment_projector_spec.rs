use appletheia::application::event::EventSelector;
use appletheia::application::messaging::Subscription;
use appletheia::application::projection::{ProjectorDescriptor, ProjectorName, ProjectorSpec};
use banking_iam_domain::{OrganizationJoinRequest, OrganizationJoinRequestEventPayload};

/// Projector specification for organization join request fragments.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct OrganizationJoinRequestFragmentProjectorSpec;

impl ProjectorSpec for OrganizationJoinRequestFragmentProjectorSpec {
    const DESCRIPTOR: ProjectorDescriptor = ProjectorDescriptor::new(
        ProjectorName::new("organization_join_request_fragment"),
        Subscription::AnyOf(&[
            EventSelector::new::<OrganizationJoinRequest>(
                OrganizationJoinRequestEventPayload::SUBMITTED,
            ),
            EventSelector::new::<OrganizationJoinRequest>(
                OrganizationJoinRequestEventPayload::APPROVED,
            ),
            EventSelector::new::<OrganizationJoinRequest>(
                OrganizationJoinRequestEventPayload::REJECTED,
            ),
            EventSelector::new::<OrganizationJoinRequest>(
                OrganizationJoinRequestEventPayload::CANCELED,
            ),
        ]),
    );
}
