use appletheia::application::event::EventEnvelope;
use appletheia::application::projection::Projector;
use banking_iam_domain::{
    Organization, OrganizationEventPayload, OrganizationOwner, User, UserEventPayload,
};
use banking_shared_kernel_application::read_model::ReadModelEventContext;

use crate::read_model::{
    OrganizationManagementInfoOwnerUpsert, OrganizationManagementInfoUpsert,
    OrganizationManagementInfoWriter,
};

use super::{OrganizationManagementInfoProjectorError, OrganizationManagementInfoProjectorSpec};

/// Projects organization and owner-user events into organization-management information.
pub struct OrganizationManagementInfoProjector<W>
where
    W: OrganizationManagementInfoWriter,
{
    writer: W,
}

impl<W> OrganizationManagementInfoProjector<W>
where
    W: OrganizationManagementInfoWriter,
{
    pub fn new(writer: W) -> Self {
        Self { writer }
    }
}

impl<W> Projector for OrganizationManagementInfoProjector<W>
where
    W: OrganizationManagementInfoWriter,
{
    type Spec = OrganizationManagementInfoProjectorSpec;
    type Uow = W::Uow;
    type Error = OrganizationManagementInfoProjectorError;

    async fn project(&self, uow: &mut Self::Uow, event: &EventEnvelope) -> Result<(), Self::Error> {
        let event_context = ReadModelEventContext::from(event);

        if event.is_for_aggregate::<User>() {
            let user_event = event.try_into_domain_event::<User>()?;
            let user_id = user_event.aggregate_id();

            match user_event.payload() {
                UserEventPayload::Registered { .. } => {
                    self.writer
                        .upsert_owner(
                            uow,
                            event_context,
                            OrganizationManagementInfoOwnerUpsert {
                                user_id,
                                username: None,
                                display_name: None,
                                picture: None,
                            },
                        )
                        .await?;
                }
                UserEventPayload::UsernameChanged { username } => {
                    self.writer
                        .update_owner_username(uow, event_context, user_id, username.clone())
                        .await?;
                }
                UserEventPayload::DisplayNameChanged { display_name } => {
                    self.writer
                        .update_owner_display_name(
                            uow,
                            event_context,
                            user_id,
                            display_name.clone(),
                        )
                        .await?;
                }
                UserEventPayload::PictureChanged { picture, .. } => {
                    self.writer
                        .update_owner_picture(uow, event_context, user_id, picture.clone())
                        .await?;
                }
                UserEventPayload::Removed => {
                    self.writer
                        .delete_owner(uow, event_context, user_id)
                        .await?;
                }
                UserEventPayload::IdentityLinked { .. }
                | UserEventPayload::IdentityLinkRejected { .. }
                | UserEventPayload::IdentityEmailChanged { .. }
                | UserEventPayload::IdentityEmailChangeRejected { .. }
                | UserEventPayload::UsernameChangeRejected { .. }
                | UserEventPayload::DisplayNameChangeRejected { .. }
                | UserEventPayload::BioChanged { .. }
                | UserEventPayload::BioChangeRejected { .. }
                | UserEventPayload::PictureChangeRejected { .. }
                | UserEventPayload::OrganizationMembershipGranted { .. }
                | UserEventPayload::OrganizationMembershipGrantRejected { .. }
                | UserEventPayload::OrganizationMembershipRolesChanged { .. }
                | UserEventPayload::OrganizationMembershipRolesChangeRejected { .. }
                | UserEventPayload::OrganizationMembershipRemoved { .. }
                | UserEventPayload::OrganizationMembershipRemoveRejected { .. }
                | UserEventPayload::Activated
                | UserEventPayload::ActivateRejected { .. }
                | UserEventPayload::Deactivated
                | UserEventPayload::DeactivateRejected { .. }
                | UserEventPayload::RemoveRejected { .. } => {}
            }

            return Ok(());
        }

        if event.is_for_aggregate::<Organization>() {
            let organization_event = event.try_into_domain_event::<Organization>()?;
            let organization_id = organization_event.aggregate_id();

            match organization_event.payload() {
                OrganizationEventPayload::Created {
                    owner,
                    handle,
                    display_name,
                    description,
                    website_url,
                    picture,
                } => {
                    let OrganizationOwner::User(owner_user_id) = owner;

                    self.writer
                        .upsert_organization(
                            uow,
                            event_context,
                            OrganizationManagementInfoUpsert {
                                id: organization_id,
                                owner_user_id: *owner_user_id,
                                handle: handle.clone(),
                                display_name: display_name.clone(),
                                description: description.clone(),
                                website_url: website_url.clone(),
                                picture: picture.clone(),
                            },
                        )
                        .await?;
                }
                OrganizationEventPayload::OwnershipTransferred { owner } => {
                    let OrganizationOwner::User(owner_user_id) = owner;

                    self.writer
                        .update_owner(uow, event_context, organization_id, *owner_user_id)
                        .await?;
                }
                OrganizationEventPayload::HandleChanged { handle } => {
                    self.writer
                        .update_handle(uow, event_context, organization_id, handle.clone())
                        .await?;
                }
                OrganizationEventPayload::DisplayNameChanged { display_name } => {
                    self.writer
                        .update_display_name(
                            uow,
                            event_context,
                            organization_id,
                            display_name.clone(),
                        )
                        .await?;
                }
                OrganizationEventPayload::DescriptionChanged { description } => {
                    self.writer
                        .update_description(
                            uow,
                            event_context,
                            organization_id,
                            description.clone(),
                        )
                        .await?;
                }
                OrganizationEventPayload::WebsiteUrlChanged { website_url } => {
                    self.writer
                        .update_website_url(
                            uow,
                            event_context,
                            organization_id,
                            website_url.clone(),
                        )
                        .await?;
                }
                OrganizationEventPayload::PictureChanged { picture, .. } => {
                    self.writer
                        .update_picture(uow, event_context, organization_id, picture.clone())
                        .await?;
                }
                OrganizationEventPayload::Removed => {
                    self.writer
                        .delete_organization(uow, event_context, organization_id)
                        .await?;
                }
                OrganizationEventPayload::CreateRejected { .. }
                | OrganizationEventPayload::OwnershipTransferRejected { .. }
                | OrganizationEventPayload::HandleChangeRejected { .. }
                | OrganizationEventPayload::DisplayNameChangeRejected { .. }
                | OrganizationEventPayload::DescriptionChangeRejected { .. }
                | OrganizationEventPayload::WebsiteUrlChangeRejected { .. }
                | OrganizationEventPayload::PictureChangeRejected { .. }
                | OrganizationEventPayload::RemoveRejected { .. } => {}
            }
        }

        Ok(())
    }
}
