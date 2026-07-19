use appletheia::application::event::EventEnvelope;
use appletheia::application::projection::Projector;
use banking_iam_domain::{
    Organization, OrganizationEventPayload, OrganizationInvitation,
    OrganizationInvitationEventPayload, OrganizationInvitationIssuer, User, UserEventPayload,
};
use banking_shared_kernel_application::read_model::ReadModelEventContext;

use crate::read_model::{
    OrganizationInvitationListInviteeUpsert, OrganizationInvitationListIssuer,
    OrganizationInvitationListItemStatus, OrganizationInvitationListOrganizationUpsert,
    OrganizationInvitationListUpsert, OrganizationInvitationListWriter,
};

use super::{OrganizationInvitationListProjectorError, OrganizationInvitationListProjectorSpec};

/// Projects invitation and profile events into organization invitation lists.
pub struct OrganizationInvitationListProjector<W>
where
    W: OrganizationInvitationListWriter,
{
    writer: W,
}

impl<W> OrganizationInvitationListProjector<W>
where
    W: OrganizationInvitationListWriter,
{
    pub fn new(writer: W) -> Self {
        Self { writer }
    }

    fn issuer(issuer: OrganizationInvitationIssuer) -> OrganizationInvitationListIssuer {
        match issuer {
            OrganizationInvitationIssuer::User(user_id) => {
                OrganizationInvitationListIssuer::User(user_id)
            }
            OrganizationInvitationIssuer::System => OrganizationInvitationListIssuer::System,
        }
    }
}

impl<W> Projector for OrganizationInvitationListProjector<W>
where
    W: OrganizationInvitationListWriter,
{
    type Spec = OrganizationInvitationListProjectorSpec;
    type Uow = W::Uow;
    type Error = OrganizationInvitationListProjectorError;

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
                            OrganizationInvitationListUpsert {
                                invitation_id,
                                organization_id: *organization_id,
                                invitee_user_id: *invitee_id,
                                roles: roles.clone(),
                                issuer: Self::issuer(*issuer),
                                expires_at: *expires_at,
                                status: OrganizationInvitationListItemStatus::Pending,
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
                            OrganizationInvitationListUpsert {
                                invitation_id,
                                organization_id: *organization_id,
                                invitee_user_id: *invitee_id,
                                roles: roles.clone(),
                                issuer: Self::issuer(*issuer),
                                expires_at: *expires_at,
                                status: OrganizationInvitationListItemStatus::Rejected,
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
                            OrganizationInvitationListItemStatus::Accepted,
                        )
                        .await?;
                }
                OrganizationInvitationEventPayload::Declined { .. } => {
                    self.writer
                        .update_status(
                            uow,
                            event_context,
                            invitation_id,
                            OrganizationInvitationListItemStatus::Declined,
                        )
                        .await?;
                }
                OrganizationInvitationEventPayload::Canceled { .. } => {
                    self.writer
                        .update_status(
                            uow,
                            event_context,
                            invitation_id,
                            OrganizationInvitationListItemStatus::Canceled,
                        )
                        .await?;
                }
                OrganizationInvitationEventPayload::AcceptRejected { .. }
                | OrganizationInvitationEventPayload::DeclineRejected { .. }
                | OrganizationInvitationEventPayload::CancelRejected { .. } => {}
            }

            return Ok(());
        }

        if event.is_for_aggregate::<User>() {
            let user_event = event.try_into_domain_event::<User>()?;
            let user_id = user_event.aggregate_id();

            match user_event.payload() {
                UserEventPayload::Registered { .. } => {
                    self.writer
                        .upsert_invitee(
                            uow,
                            event_context,
                            OrganizationInvitationListInviteeUpsert {
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
                        .update_invitee_username(uow, event_context, user_id, username.clone())
                        .await?;
                }
                UserEventPayload::DisplayNameChanged { display_name } => {
                    self.writer
                        .update_invitee_display_name(
                            uow,
                            event_context,
                            user_id,
                            display_name.clone(),
                        )
                        .await?;
                }
                UserEventPayload::PictureChanged { picture, .. } => {
                    self.writer
                        .update_invitee_picture(uow, event_context, user_id, picture.clone())
                        .await?;
                }
                UserEventPayload::Removed => {
                    self.writer
                        .delete_invitee_and_invitations(uow, event_context, user_id)
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
                    handle,
                    display_name,
                    picture,
                    ..
                } => {
                    self.writer
                        .upsert_organization(
                            uow,
                            event_context,
                            OrganizationInvitationListOrganizationUpsert {
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
        }

        Ok(())
    }
}
