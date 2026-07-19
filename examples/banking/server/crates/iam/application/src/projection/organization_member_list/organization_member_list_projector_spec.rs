use appletheia::application::event::EventSelector;
use appletheia::application::messaging::Subscription;
use appletheia::application::projection::{ProjectorDescriptor, ProjectorName, ProjectorSpec};
use banking_iam_domain::{Organization, OrganizationEventPayload, User, UserEventPayload};

/// Projector specification for organization member lists.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct OrganizationMemberListProjectorSpec;

impl ProjectorSpec for OrganizationMemberListProjectorSpec {
    const DESCRIPTOR: ProjectorDescriptor = ProjectorDescriptor::new(
        ProjectorName::new("organization_member_list"),
        Subscription::AnyOf(&[
            EventSelector::new::<Organization>(OrganizationEventPayload::CREATED),
            EventSelector::new::<Organization>(OrganizationEventPayload::OWNERSHIP_TRANSFERRED),
            EventSelector::new::<Organization>(OrganizationEventPayload::HANDLE_CHANGED),
            EventSelector::new::<Organization>(OrganizationEventPayload::DISPLAY_NAME_CHANGED),
            EventSelector::new::<Organization>(OrganizationEventPayload::PICTURE_CHANGED),
            EventSelector::new::<Organization>(OrganizationEventPayload::REMOVED),
            EventSelector::new::<User>(UserEventPayload::REGISTERED),
            EventSelector::new::<User>(UserEventPayload::USERNAME_CHANGED),
            EventSelector::new::<User>(UserEventPayload::DISPLAY_NAME_CHANGED),
            EventSelector::new::<User>(UserEventPayload::PICTURE_CHANGED),
            EventSelector::new::<User>(UserEventPayload::ORGANIZATION_MEMBERSHIP_GRANTED),
            EventSelector::new::<User>(UserEventPayload::ORGANIZATION_MEMBERSHIP_ROLES_CHANGED),
            EventSelector::new::<User>(UserEventPayload::ORGANIZATION_MEMBERSHIP_REMOVED),
            EventSelector::new::<User>(UserEventPayload::REMOVED),
        ]),
    );
}

#[cfg(test)]
mod tests {
    use appletheia::application::event::EventSelector;
    use appletheia::application::messaging::Subscription;
    use appletheia::application::projection::ProjectorSpec;
    use banking_iam_domain::{Organization, OrganizationEventPayload, User, UserEventPayload};

    use super::OrganizationMemberListProjectorSpec;

    #[test]
    fn subscribes_to_membership_and_ownership_events() {
        let Subscription::AnyOf(selectors) =
            OrganizationMemberListProjectorSpec::DESCRIPTOR.subscription
        else {
            panic!("subscription should select explicit events");
        };
        let membership_granted =
            EventSelector::new::<User>(UserEventPayload::ORGANIZATION_MEMBERSHIP_GRANTED);
        let ownership_transferred =
            EventSelector::new::<Organization>(OrganizationEventPayload::OWNERSHIP_TRANSFERRED);

        assert!(
            selectors
                .iter()
                .any(|selector| selector.is_same_as(&membership_granted))
        );
        assert!(
            selectors
                .iter()
                .any(|selector| selector.is_same_as(&ownership_transferred))
        );
    }
}
