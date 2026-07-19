use appletheia::application::event::EventSelector;
use appletheia::application::messaging::Subscription;
use appletheia::application::projection::{ProjectorDescriptor, ProjectorName, ProjectorSpec};
use banking_iam_domain::{
    Organization, OrganizationEventPayload, OrganizationInvitation,
    OrganizationInvitationEventPayload, User, UserEventPayload,
};

/// Projector specification for user organization invitation lists.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct UserOrganizationInvitationListProjectorSpec;

impl ProjectorSpec for UserOrganizationInvitationListProjectorSpec {
    const DESCRIPTOR: ProjectorDescriptor = ProjectorDescriptor::new(
        ProjectorName::new("user_organization_invitation_list"),
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
            EventSelector::new::<Organization>(OrganizationEventPayload::CREATED),
            EventSelector::new::<Organization>(OrganizationEventPayload::HANDLE_CHANGED),
            EventSelector::new::<Organization>(OrganizationEventPayload::DISPLAY_NAME_CHANGED),
            EventSelector::new::<Organization>(OrganizationEventPayload::PICTURE_CHANGED),
            EventSelector::new::<Organization>(OrganizationEventPayload::REMOVED),
            EventSelector::new::<User>(UserEventPayload::REGISTERED),
            EventSelector::new::<User>(UserEventPayload::USERNAME_CHANGED),
            EventSelector::new::<User>(UserEventPayload::DISPLAY_NAME_CHANGED),
            EventSelector::new::<User>(UserEventPayload::PICTURE_CHANGED),
            EventSelector::new::<User>(UserEventPayload::REMOVED),
        ]),
    );
}

#[cfg(test)]
mod tests {
    use appletheia::application::event::EventSelector;
    use appletheia::application::messaging::Subscription;
    use appletheia::application::projection::ProjectorSpec;
    use banking_iam_domain::{User, UserEventPayload};

    use super::UserOrganizationInvitationListProjectorSpec;

    #[test]
    fn subscribes_to_user_profile_events() {
        let Subscription::AnyOf(selectors) =
            UserOrganizationInvitationListProjectorSpec::DESCRIPTOR.subscription
        else {
            panic!("subscription should select explicit events");
        };
        let registered = EventSelector::new::<User>(UserEventPayload::REGISTERED);
        let picture_changed = EventSelector::new::<User>(UserEventPayload::PICTURE_CHANGED);

        assert!(
            selectors
                .iter()
                .any(|selector| selector.is_same_as(&registered))
        );
        assert!(
            selectors
                .iter()
                .any(|selector| selector.is_same_as(&picture_changed))
        );
    }
}
