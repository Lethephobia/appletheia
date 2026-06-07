use appletheia::application::event::EventEnvelope;
use appletheia::application::projection::Projector;
use banking_iam_domain::{Organization, OrganizationEventPayload, User, UserEventPayload};

use crate::read_model::{
    ReadModelEventContext, UserPrivateInfoIdentityUpsert,
    UserPrivateInfoOrganizationMembershipUpsert, UserPrivateInfoOrganizationUpsert,
    UserPrivateInfoStatus, UserPrivateInfoUserUpsert, UserPrivateInfoWriter,
};

use super::{UserPrivateInfoProjectorError, UserPrivateInfoProjectorSpec};

/// Projects user events into the private information read model.
pub struct UserPrivateInfoProjector<W>
where
    W: UserPrivateInfoWriter,
{
    writer: W,
}

impl<W> UserPrivateInfoProjector<W>
where
    W: UserPrivateInfoWriter,
{
    pub fn new(writer: W) -> Self {
        Self { writer }
    }
}

impl<W> Projector for UserPrivateInfoProjector<W>
where
    W: UserPrivateInfoWriter,
{
    type Spec = UserPrivateInfoProjectorSpec;
    type Uow = W::Uow;
    type Error = UserPrivateInfoProjectorError;

    async fn project(&self, uow: &mut Self::Uow, event: &EventEnvelope) -> Result<(), Self::Error> {
        let event_context = ReadModelEventContext::from(event);

        if event.is_for_aggregate::<User>() {
            let domain_event = event.try_into_domain_event::<User>()?;
            let user_id = domain_event.aggregate_id();

            match domain_event.payload() {
                UserEventPayload::Registered {
                    initial_identity, ..
                } => {
                    self.writer
                        .upsert_user(
                            uow,
                            event_context,
                            UserPrivateInfoUserUpsert {
                                id: user_id,
                                username: None,
                                display_name: None,
                                bio: None,
                                picture: None,
                                status: UserPrivateInfoStatus::Active,
                            },
                        )
                        .await?;

                    if let Some(identity) = initial_identity {
                        self.writer
                            .upsert_identity(
                                uow,
                                event_context,
                                UserPrivateInfoIdentityUpsert {
                                    user_id,
                                    provider: identity.provider().clone(),
                                    subject: identity.subject().clone(),
                                    email: identity.email().cloned(),
                                },
                            )
                            .await?;
                    }
                }
                UserEventPayload::IdentityLinked { identity } => {
                    self.writer
                        .upsert_identity(
                            uow,
                            event_context,
                            UserPrivateInfoIdentityUpsert {
                                user_id,
                                provider: identity.provider().clone(),
                                subject: identity.subject().clone(),
                                email: identity.email().cloned(),
                            },
                        )
                        .await?;
                }
                UserEventPayload::IdentityEmailChanged {
                    provider,
                    subject,
                    email,
                } => {
                    self.writer
                        .update_identity_email(
                            uow,
                            event_context,
                            user_id,
                            provider.clone(),
                            subject.clone(),
                            email.clone(),
                        )
                        .await?;
                }
                UserEventPayload::OrganizationMembershipGranted {
                    organization_id,
                    roles,
                } => {
                    self.writer
                        .upsert_organization_membership(
                            uow,
                            event_context,
                            UserPrivateInfoOrganizationMembershipUpsert {
                                user_id,
                                organization_id: *organization_id,
                                roles: roles.clone(),
                            },
                        )
                        .await?;
                }
                UserEventPayload::OrganizationMembershipRolesChanged {
                    organization_id,
                    roles,
                } => {
                    self.writer
                        .update_organization_membership_roles(
                            uow,
                            event_context,
                            user_id,
                            *organization_id,
                            roles.clone(),
                        )
                        .await?;
                }
                UserEventPayload::OrganizationMembershipRemoved { organization_id } => {
                    self.writer
                        .delete_organization_membership(
                            uow,
                            event_context,
                            user_id,
                            *organization_id,
                        )
                        .await?;
                }
                UserEventPayload::UsernameChanged { username } => {
                    self.writer
                        .update_username(uow, event_context, user_id, username.clone())
                        .await?;
                }
                UserEventPayload::DisplayNameChanged { display_name } => {
                    self.writer
                        .update_display_name(uow, event_context, user_id, display_name.clone())
                        .await?;
                }
                UserEventPayload::BioChanged { bio } => {
                    self.writer
                        .update_bio(uow, event_context, user_id, bio.clone())
                        .await?;
                }
                UserEventPayload::PictureChanged { picture, .. } => {
                    self.writer
                        .update_picture(uow, event_context, user_id, picture.clone())
                        .await?;
                }
                UserEventPayload::Activated => {
                    self.writer
                        .update_status(uow, event_context, user_id, UserPrivateInfoStatus::Active)
                        .await?;
                }
                UserEventPayload::Deactivated => {
                    self.writer
                        .update_status(uow, event_context, user_id, UserPrivateInfoStatus::Inactive)
                        .await?;
                }
                UserEventPayload::Removed => {
                    self.writer.delete_user(uow, event_context, user_id).await?;
                }
                UserEventPayload::IdentityLinkRejected { .. }
                | UserEventPayload::IdentityEmailChangeRejected { .. }
                | UserEventPayload::UsernameChangeRejected { .. }
                | UserEventPayload::DisplayNameChangeRejected { .. }
                | UserEventPayload::BioChangeRejected { .. }
                | UserEventPayload::PictureChangeRejected { .. }
                | UserEventPayload::ActivateRejected { .. }
                | UserEventPayload::DeactivateRejected { .. }
                | UserEventPayload::RemoveRejected { .. }
                | UserEventPayload::OrganizationMembershipGrantRejected { .. }
                | UserEventPayload::OrganizationMembershipRolesChangeRejected { .. }
                | UserEventPayload::OrganizationMembershipRemoveRejected { .. } => {}
            }

            return Ok(());
        }

        if event.is_for_aggregate::<Organization>() {
            let domain_event = event.try_into_domain_event::<Organization>()?;
            let organization_id = domain_event.aggregate_id();

            match domain_event.payload() {
                OrganizationEventPayload::Created {
                    handle,
                    display_name,
                    picture,
                    ..
                } => {
                    self.writer
                        .upsert_organization(
                            uow,
                            event_context,
                            UserPrivateInfoOrganizationUpsert {
                                id: organization_id,
                                handle: handle.clone(),
                                display_name: display_name.clone(),
                                picture: picture.clone(),
                            },
                        )
                        .await?;
                }
                OrganizationEventPayload::HandleChanged { handle } => {
                    self.writer
                        .update_organization_handle(
                            uow,
                            event_context,
                            organization_id,
                            handle.clone(),
                        )
                        .await?;
                }
                OrganizationEventPayload::DisplayNameChanged { display_name } => {
                    self.writer
                        .update_organization_display_name(
                            uow,
                            event_context,
                            organization_id,
                            display_name.clone(),
                        )
                        .await?;
                }
                OrganizationEventPayload::PictureChanged { picture, .. } => {
                    self.writer
                        .update_organization_picture(
                            uow,
                            event_context,
                            organization_id,
                            picture.clone(),
                        )
                        .await?;
                }
                OrganizationEventPayload::Removed => {
                    self.writer
                        .delete_organization(uow, event_context, organization_id)
                        .await?;
                }
                OrganizationEventPayload::OwnershipTransferred { .. }
                | OrganizationEventPayload::OwnershipTransferRejected { .. }
                | OrganizationEventPayload::HandleChangeRejected { .. }
                | OrganizationEventPayload::DisplayNameChangeRejected { .. }
                | OrganizationEventPayload::DescriptionChanged { .. }
                | OrganizationEventPayload::DescriptionChangeRejected { .. }
                | OrganizationEventPayload::WebsiteUrlChanged { .. }
                | OrganizationEventPayload::WebsiteUrlChangeRejected { .. }
                | OrganizationEventPayload::PictureChangeRejected { .. }
                | OrganizationEventPayload::RemoveRejected { .. } => {}
            }
        }

        Ok(())
    }
}
