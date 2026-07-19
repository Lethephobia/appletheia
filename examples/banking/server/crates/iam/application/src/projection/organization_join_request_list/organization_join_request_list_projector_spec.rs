use appletheia::application::event::EventSelector;
use appletheia::application::messaging::Subscription;
use appletheia::application::projection::{ProjectorDescriptor, ProjectorName, ProjectorSpec};
use banking_iam_domain::{
    Organization, OrganizationEventPayload, OrganizationJoinRequest,
    OrganizationJoinRequestEventPayload, User, UserEventPayload,
};

/// Projector specification for organization join request lists.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct OrganizationJoinRequestListProjectorSpec;

impl ProjectorSpec for OrganizationJoinRequestListProjectorSpec {
    const DESCRIPTOR: ProjectorDescriptor = ProjectorDescriptor::new(
        ProjectorName::new("organization_join_request_list"),
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
        Organization, OrganizationEventPayload, OrganizationJoinRequest,
        OrganizationJoinRequestEventPayload,
    };

    use super::OrganizationJoinRequestListProjectorSpec;

    #[test]
    fn excludes_submit_rejected_without_aggregate_state() {
        let Subscription::AnyOf(selectors) =
            OrganizationJoinRequestListProjectorSpec::DESCRIPTOR.subscription
        else {
            panic!("subscription should select explicit events");
        };
        let submitted = EventSelector::new::<OrganizationJoinRequest>(
            OrganizationJoinRequestEventPayload::SUBMITTED,
        );
        let submit_rejected = EventSelector::new::<OrganizationJoinRequest>(
            OrganizationJoinRequestEventPayload::SUBMIT_REJECTED,
        );

        assert!(
            selectors
                .iter()
                .any(|selector| selector.is_same_as(&submitted))
        );
        assert!(
            selectors
                .iter()
                .all(|selector| !selector.is_same_as(&submit_rejected))
        );
    }

    #[test]
    fn subscribes_to_organization_profile_events() {
        let Subscription::AnyOf(selectors) =
            OrganizationJoinRequestListProjectorSpec::DESCRIPTOR.subscription
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
