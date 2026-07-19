use appletheia::application::event::EventSelector;
use appletheia::application::messaging::Subscription;
use appletheia::application::projection::{ProjectorDescriptor, ProjectorName, ProjectorSpec};
use banking_iam_domain::{
    Organization, OrganizationEventPayload, OrganizationInvitation,
    OrganizationInvitationEventPayload, User, UserEventPayload,
};

/// Projector specification for organization invitation lists.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct OrganizationInvitationListProjectorSpec;

impl ProjectorSpec for OrganizationInvitationListProjectorSpec {
    const DESCRIPTOR: ProjectorDescriptor = ProjectorDescriptor::new(
        ProjectorName::new("organization_invitation_list"),
        Subscription::AnyOf(&[
            EventSelector::new::<OrganizationInvitation>(
                OrganizationInvitationEventPayload::ISSUED,
            ),
            EventSelector::new::<OrganizationInvitation>(
                OrganizationInvitationEventPayload::ISSUE_REJECTED,
            ),
            EventSelector::new::<OrganizationInvitation>(
                OrganizationInvitationEventPayload::ACCEPTED,
            ),
            EventSelector::new::<OrganizationInvitation>(
                OrganizationInvitationEventPayload::DECLINED,
            ),
            EventSelector::new::<OrganizationInvitation>(
                OrganizationInvitationEventPayload::CANCELED,
            ),
            EventSelector::new::<User>(UserEventPayload::REGISTERED),
            EventSelector::new::<User>(UserEventPayload::USERNAME_CHANGED),
            EventSelector::new::<User>(UserEventPayload::DISPLAY_NAME_CHANGED),
            EventSelector::new::<User>(UserEventPayload::PICTURE_CHANGED),
            EventSelector::new::<User>(UserEventPayload::REMOVED),
            EventSelector::new::<Organization>(OrganizationEventPayload::CREATED),
            EventSelector::new::<Organization>(OrganizationEventPayload::HANDLE_CHANGED),
            EventSelector::new::<Organization>(OrganizationEventPayload::DISPLAY_NAME_CHANGED),
            EventSelector::new::<Organization>(OrganizationEventPayload::PICTURE_CHANGED),
            EventSelector::new::<Organization>(OrganizationEventPayload::REMOVED),
        ]),
    );
}

#[cfg(test)]
mod tests {
    use appletheia::application::event::EventSelector;
    use appletheia::application::messaging::Subscription;
    use appletheia::application::projection::ProjectorSpec;
    use banking_iam_domain::{
        Organization, OrganizationEventPayload, OrganizationInvitation,
        OrganizationInvitationEventPayload,
    };

    use super::OrganizationInvitationListProjectorSpec;

    #[test]
    fn subscribes_to_issue_rejected_for_rejected_history() {
        let Subscription::AnyOf(selectors) =
            OrganizationInvitationListProjectorSpec::DESCRIPTOR.subscription
        else {
            panic!("subscription should select explicit events");
        };
        let issue_rejected = EventSelector::new::<OrganizationInvitation>(
            OrganizationInvitationEventPayload::ISSUE_REJECTED,
        );

        assert!(
            selectors
                .iter()
                .any(|selector| selector.is_same_as(&issue_rejected))
        );
    }

    #[test]
    fn subscribes_to_organization_profile_events() {
        let Subscription::AnyOf(selectors) =
            OrganizationInvitationListProjectorSpec::DESCRIPTOR.subscription
        else {
            panic!("subscription should select explicit events");
        };
        let created = EventSelector::new::<Organization>(OrganizationEventPayload::CREATED);
        let picture_changed =
            EventSelector::new::<Organization>(OrganizationEventPayload::PICTURE_CHANGED);

        assert!(
            selectors
                .iter()
                .any(|selector| selector.is_same_as(&created))
        );
        assert!(
            selectors
                .iter()
                .any(|selector| selector.is_same_as(&picture_changed))
        );
    }
}
