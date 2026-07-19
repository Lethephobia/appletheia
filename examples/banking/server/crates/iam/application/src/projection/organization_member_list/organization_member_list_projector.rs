use appletheia::application::event::EventEnvelope;
use appletheia::application::projection::Projector;
use banking_iam_domain::{
    Organization, OrganizationEventPayload, OrganizationOwner, User, UserEventPayload, UserId,
};
use banking_shared_kernel_application::read_model::ReadModelEventContext;

use crate::read_model::{
    OrganizationMemberListMemberUpsert, OrganizationMemberListMembershipUpsert,
    OrganizationMemberListOrganizationUpsert, OrganizationMemberListWriter,
};

use super::{OrganizationMemberListProjectorError, OrganizationMemberListProjectorSpec};

/// Projects organization and user events into organization member lists.
pub struct OrganizationMemberListProjector<W>
where
    W: OrganizationMemberListWriter,
{
    writer: W,
}

impl<W> OrganizationMemberListProjector<W>
where
    W: OrganizationMemberListWriter,
{
    pub fn new(writer: W) -> Self {
        Self { writer }
    }

    fn owner_user_id(owner: &OrganizationOwner) -> UserId {
        match owner {
            OrganizationOwner::User(user_id) => *user_id,
        }
    }
}

impl<W> Projector for OrganizationMemberListProjector<W>
where
    W: OrganizationMemberListWriter,
{
    type Spec = OrganizationMemberListProjectorSpec;
    type Uow = W::Uow;
    type Error = OrganizationMemberListProjectorError;

    async fn project(&self, uow: &mut Self::Uow, event: &EventEnvelope) -> Result<(), Self::Error> {
        let event_context = ReadModelEventContext::from(event);

        if event.is_for_aggregate::<Organization>() {
            let organization_event = event.try_into_domain_event::<Organization>()?;
            let organization_id = organization_event.aggregate_id();

            match organization_event.payload() {
                OrganizationEventPayload::Created {
                    owner,
                    handle,
                    display_name,
                    picture,
                    ..
                } => {
                    self.writer
                        .upsert_organization(
                            uow,
                            event_context,
                            OrganizationMemberListOrganizationUpsert {
                                organization_id,
                                owner_user_id: Self::owner_user_id(owner),
                                handle: handle.clone(),
                                display_name: display_name.clone(),
                                picture: picture.clone(),
                            },
                        )
                        .await?;
                }
                OrganizationEventPayload::OwnershipTransferred { owner } => {
                    self.writer
                        .update_organization_owner(
                            uow,
                            event_context,
                            organization_id,
                            Self::owner_user_id(owner),
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
                        .delete_organization_and_memberships(uow, event_context, organization_id)
                        .await?;
                }
                OrganizationEventPayload::CreateRejected { .. }
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

            return Ok(());
        }

        if event.is_for_aggregate::<User>() {
            let user_event = event.try_into_domain_event::<User>()?;
            let user_id = user_event.aggregate_id();

            match user_event.payload() {
                UserEventPayload::Registered { .. } => {
                    self.writer
                        .upsert_member(
                            uow,
                            event_context,
                            OrganizationMemberListMemberUpsert {
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
                        .update_member_username(uow, event_context, user_id, username.clone())
                        .await?;
                }
                UserEventPayload::DisplayNameChanged { display_name } => {
                    self.writer
                        .update_member_display_name(
                            uow,
                            event_context,
                            user_id,
                            display_name.clone(),
                        )
                        .await?;
                }
                UserEventPayload::PictureChanged { picture, .. } => {
                    self.writer
                        .update_member_picture(uow, event_context, user_id, picture.clone())
                        .await?;
                }
                UserEventPayload::OrganizationMembershipGranted {
                    organization_id,
                    roles,
                } => {
                    self.writer
                        .upsert_membership(
                            uow,
                            event_context,
                            OrganizationMemberListMembershipUpsert {
                                organization_id: *organization_id,
                                user_id,
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
                        .update_membership_roles(
                            uow,
                            event_context,
                            *organization_id,
                            user_id,
                            roles.clone(),
                        )
                        .await?;
                }
                UserEventPayload::OrganizationMembershipRemoved { organization_id } => {
                    self.writer
                        .delete_membership(uow, event_context, *organization_id, user_id)
                        .await?;
                }
                UserEventPayload::Removed => {
                    self.writer
                        .delete_member_and_memberships(uow, event_context, user_id)
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
                | UserEventPayload::OrganizationMembershipGrantRejected { .. }
                | UserEventPayload::OrganizationMembershipRolesChangeRejected { .. }
                | UserEventPayload::OrganizationMembershipRemoveRejected { .. }
                | UserEventPayload::Activated
                | UserEventPayload::ActivateRejected { .. }
                | UserEventPayload::Deactivated
                | UserEventPayload::DeactivateRejected { .. }
                | UserEventPayload::RemoveRejected { .. } => {}
            }
        }

        Ok(())
    }
}
