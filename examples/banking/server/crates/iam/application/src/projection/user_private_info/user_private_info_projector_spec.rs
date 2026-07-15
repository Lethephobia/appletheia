use appletheia::application::event::EventSelector;
use appletheia::application::messaging::Subscription;
use appletheia::application::projection::{ProjectorDescriptor, ProjectorName, ProjectorSpec};
use banking_iam_domain::{Organization, OrganizationEventPayload, User, UserEventPayload};

/// Projector specification for user-private information read models.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct UserPrivateInfoProjectorSpec;

impl ProjectorSpec for UserPrivateInfoProjectorSpec {
    const DESCRIPTOR: ProjectorDescriptor = ProjectorDescriptor::new(
        ProjectorName::new("user_private_info"),
        Subscription::AnyOf(&[
            EventSelector::new::<User>(UserEventPayload::REGISTERED),
            EventSelector::new::<User>(UserEventPayload::IDENTITY_LINKED),
            EventSelector::new::<User>(UserEventPayload::IDENTITY_EMAIL_CHANGED),
            EventSelector::new::<User>(UserEventPayload::USERNAME_CHANGED),
            EventSelector::new::<User>(UserEventPayload::DISPLAY_NAME_CHANGED),
            EventSelector::new::<User>(UserEventPayload::BIO_CHANGED),
            EventSelector::new::<User>(UserEventPayload::PICTURE_CHANGED),
            EventSelector::new::<User>(UserEventPayload::ACTIVATED),
            EventSelector::new::<User>(UserEventPayload::DEACTIVATED),
            EventSelector::new::<User>(UserEventPayload::REMOVED),
            EventSelector::new::<User>(UserEventPayload::ORGANIZATION_MEMBERSHIP_GRANTED),
            EventSelector::new::<User>(UserEventPayload::ORGANIZATION_MEMBERSHIP_ROLES_CHANGED),
            EventSelector::new::<User>(UserEventPayload::ORGANIZATION_MEMBERSHIP_REMOVED),
            EventSelector::new::<Organization>(OrganizationEventPayload::CREATED),
            EventSelector::new::<Organization>(OrganizationEventPayload::HANDLE_CHANGED),
            EventSelector::new::<Organization>(OrganizationEventPayload::DISPLAY_NAME_CHANGED),
            EventSelector::new::<Organization>(OrganizationEventPayload::PICTURE_CHANGED),
            EventSelector::new::<Organization>(OrganizationEventPayload::REMOVED),
        ]),
    );
}
