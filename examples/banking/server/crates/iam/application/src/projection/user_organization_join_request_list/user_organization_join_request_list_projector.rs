use appletheia::application::event::EventEnvelope;
use appletheia::application::projection::Projector;
use banking_iam_domain::{
    Organization, OrganizationEventPayload, OrganizationJoinRequest,
    OrganizationJoinRequestEventPayload, User, UserEventPayload,
};
use banking_shared_kernel_application::read_model::ReadModelEventContext;

use crate::read_model::{
    UserOrganizationJoinRequestListItemStatus, UserOrganizationJoinRequestListOrganizationUpsert,
    UserOrganizationJoinRequestListUpsert, UserOrganizationJoinRequestListUserUpsert,
    UserOrganizationJoinRequestListWriter,
};

use super::{
    UserOrganizationJoinRequestListProjectorError, UserOrganizationJoinRequestListProjectorSpec,
};

/// Projects join request and organization events into user organization join request lists.
pub struct UserOrganizationJoinRequestListProjector<W>
where
    W: UserOrganizationJoinRequestListWriter,
{
    writer: W,
}

impl<W> UserOrganizationJoinRequestListProjector<W>
where
    W: UserOrganizationJoinRequestListWriter,
{
    pub fn new(writer: W) -> Self {
        Self { writer }
    }
}

impl<W> Projector for UserOrganizationJoinRequestListProjector<W>
where
    W: UserOrganizationJoinRequestListWriter,
{
    type Spec = UserOrganizationJoinRequestListProjectorSpec;
    type Uow = W::Uow;
    type Error = UserOrganizationJoinRequestListProjectorError;

    async fn project(&self, uow: &mut Self::Uow, event: &EventEnvelope) -> Result<(), Self::Error> {
        let event_context = ReadModelEventContext::from(event);

        if event.is_for_aggregate::<OrganizationJoinRequest>() {
            let join_request_event = event.try_into_domain_event::<OrganizationJoinRequest>()?;
            let join_request_id = join_request_event.aggregate_id();

            match join_request_event.payload() {
                OrganizationJoinRequestEventPayload::Submitted {
                    organization_id,
                    requester_id,
                } => {
                    self.writer
                        .upsert_join_request(
                            uow,
                            event_context,
                            UserOrganizationJoinRequestListUpsert {
                                join_request_id,
                                requester_user_id: *requester_id,
                                organization_id: *organization_id,
                                status: UserOrganizationJoinRequestListItemStatus::Pending,
                            },
                        )
                        .await?;
                }
                OrganizationJoinRequestEventPayload::Approved { .. } => {
                    self.writer
                        .update_status(
                            uow,
                            event_context,
                            join_request_id,
                            UserOrganizationJoinRequestListItemStatus::Approved,
                        )
                        .await?;
                }
                OrganizationJoinRequestEventPayload::Rejected { .. } => {
                    self.writer
                        .update_status(
                            uow,
                            event_context,
                            join_request_id,
                            UserOrganizationJoinRequestListItemStatus::Rejected,
                        )
                        .await?;
                }
                OrganizationJoinRequestEventPayload::Canceled { .. } => {
                    self.writer
                        .update_status(
                            uow,
                            event_context,
                            join_request_id,
                            UserOrganizationJoinRequestListItemStatus::Canceled,
                        )
                        .await?;
                }
                OrganizationJoinRequestEventPayload::SubmitRejected { .. }
                | OrganizationJoinRequestEventPayload::ApproveRejected { .. }
                | OrganizationJoinRequestEventPayload::RejectRejected { .. }
                | OrganizationJoinRequestEventPayload::CancelRejected { .. } => {}
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
                            UserOrganizationJoinRequestListOrganizationUpsert {
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
                        .delete_organization_and_join_requests(uow, event_context, organization_id)
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
                            UserOrganizationJoinRequestListUserUpsert {
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
                        .delete_user_and_join_requests(uow, event_context, user_id)
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
