use appletheia::application::event::EventEnvelope;
use appletheia::application::projection::Projector;
use banking_iam_domain::{
    Organization, OrganizationEventPayload, OrganizationInvitation,
    OrganizationInvitationEventPayload, OrganizationInvitationIssuer, User, UserEventPayload,
};
use banking_shared_kernel_application::read_model::ReadModelEventContext;

use crate::read_model::{
    UserOrganizationInvitationListIssuer, UserOrganizationInvitationListItemStatus,
    UserOrganizationInvitationListOrganizationUpsert, UserOrganizationInvitationListUpsert,
    UserOrganizationInvitationListUserUpsert, UserOrganizationInvitationListWriter,
};

use super::{
    UserOrganizationInvitationListProjectorError, UserOrganizationInvitationListProjectorSpec,
};

/// Projects invitation and organization events into user organization invitation lists.
pub struct UserOrganizationInvitationListProjector<W>
where
    W: UserOrganizationInvitationListWriter,
{
    writer: W,
}

impl<W> UserOrganizationInvitationListProjector<W>
where
    W: UserOrganizationInvitationListWriter,
{
    pub fn new(writer: W) -> Self {
        Self { writer }
    }

    fn issuer(issuer: OrganizationInvitationIssuer) -> UserOrganizationInvitationListIssuer {
        match issuer {
            OrganizationInvitationIssuer::User(user_id) => {
                UserOrganizationInvitationListIssuer::User(user_id)
            }
            OrganizationInvitationIssuer::System => UserOrganizationInvitationListIssuer::System,
        }
    }
}

impl<W> Projector for UserOrganizationInvitationListProjector<W>
where
    W: UserOrganizationInvitationListWriter,
{
    type Spec = UserOrganizationInvitationListProjectorSpec;
    type Uow = W::Uow;
    type Error = UserOrganizationInvitationListProjectorError;

    async fn project(&self, uow: &mut Self::Uow, event: &EventEnvelope) -> Result<(), Self::Error> {
        let event_context = ReadModelEventContext::from(event);

        if event.is_for_aggregate::<OrganizationInvitation>() {
            let invitation_event = event.try_into_domain_event::<OrganizationInvitation>()?;
            let invitation_id = invitation_event.aggregate_id();

            match invitation_event.payload() {
                OrganizationInvitationEventPayload::Issued {
                    organization_id,
                    invitee_id,
                    roles,
                    issuer,
                    expires_at,
                } => {
                    self.writer
                        .upsert_invitation(
                            uow,
                            event_context,
                            UserOrganizationInvitationListUpsert {
                                invitation_id,
                                invitee_user_id: *invitee_id,
                                organization_id: *organization_id,
                                roles: roles.clone(),
                                issuer: Self::issuer(*issuer),
                                expires_at: *expires_at,
                                status: UserOrganizationInvitationListItemStatus::Pending,
                            },
                        )
                        .await?;
                }
                OrganizationInvitationEventPayload::IssueRejected {
                    organization_id,
                    invitee_id,
                    roles,
                    issuer,
                    expires_at,
                    ..
                } => {
                    self.writer
                        .upsert_invitation(
                            uow,
                            event_context,
                            UserOrganizationInvitationListUpsert {
                                invitation_id,
                                invitee_user_id: *invitee_id,
                                organization_id: *organization_id,
                                roles: roles.clone(),
                                issuer: Self::issuer(*issuer),
                                expires_at: *expires_at,
                                status: UserOrganizationInvitationListItemStatus::Rejected,
                            },
                        )
                        .await?;
                }
                OrganizationInvitationEventPayload::Accepted { .. } => {
                    self.writer
                        .update_status(
                            uow,
                            event_context,
                            invitation_id,
                            UserOrganizationInvitationListItemStatus::Accepted,
                        )
                        .await?;
                }
                OrganizationInvitationEventPayload::Declined { .. } => {
                    self.writer
                        .update_status(
                            uow,
                            event_context,
                            invitation_id,
                            UserOrganizationInvitationListItemStatus::Declined,
                        )
                        .await?;
                }
                OrganizationInvitationEventPayload::Canceled { .. } => {
                    self.writer
                        .update_status(
                            uow,
                            event_context,
                            invitation_id,
                            UserOrganizationInvitationListItemStatus::Canceled,
                        )
                        .await?;
                }
                OrganizationInvitationEventPayload::AcceptRejected { .. }
                | OrganizationInvitationEventPayload::DeclineRejected { .. }
                | OrganizationInvitationEventPayload::CancelRejected { .. } => {}
            }

            return Ok(());
        }

        if event.is_for_aggregate::<Organization>() {
            let organization_event = event.try_into_domain_event::<Organization>()?;
            let organization_id = organization_event.aggregate_id();

            match organization_event.payload() {
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
                            UserOrganizationInvitationListOrganizationUpsert {
                                organization_id,
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
                        .delete_organization_and_invitations(uow, event_context, organization_id)
                        .await?;
                }
                OrganizationEventPayload::CreateRejected { .. }
                | OrganizationEventPayload::OwnershipTransferred { .. }
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
                        .upsert_user(
                            uow,
                            event_context,
                            UserOrganizationInvitationListUserUpsert {
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
                        .update_user_username(uow, event_context, user_id, username.clone())
                        .await?;
                }
                UserEventPayload::DisplayNameChanged { display_name } => {
                    self.writer
                        .update_user_display_name(uow, event_context, user_id, display_name.clone())
                        .await?;
                }
                UserEventPayload::PictureChanged { picture, .. } => {
                    self.writer
                        .update_user_picture(uow, event_context, user_id, picture.clone())
                        .await?;
                }
                UserEventPayload::Removed => {
                    self.writer
                        .delete_user_and_invitations(uow, event_context, user_id)
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
        }

        Ok(())
    }
}
