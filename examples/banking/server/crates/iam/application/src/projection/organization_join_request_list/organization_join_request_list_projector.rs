use appletheia::application::event::EventEnvelope;
use appletheia::application::projection::Projector;
use banking_iam_domain::{
    Organization, OrganizationEventPayload, OrganizationJoinRequest,
    OrganizationJoinRequestEventPayload, User, UserEventPayload,
};
use banking_shared_kernel_application::read_model::ReadModelEventContext;

use crate::read_model::{
    OrganizationJoinRequestListItemStatus, OrganizationJoinRequestListOrganizationUpsert,
    OrganizationJoinRequestListRequesterUpsert, OrganizationJoinRequestListUpsert,
    OrganizationJoinRequestListWriter,
};

use super::{OrganizationJoinRequestListProjectorError, OrganizationJoinRequestListProjectorSpec};

/// Projects join request and profile events into organization join request lists.
pub struct OrganizationJoinRequestListProjector<W>
where
    W: OrganizationJoinRequestListWriter,
{
    writer: W,
}

impl<W> OrganizationJoinRequestListProjector<W>
where
    W: OrganizationJoinRequestListWriter,
{
    pub fn new(writer: W) -> Self {
        Self { writer }
    }
}

impl<W> Projector for OrganizationJoinRequestListProjector<W>
where
    W: OrganizationJoinRequestListWriter,
{
    type Spec = OrganizationJoinRequestListProjectorSpec;
    type Uow = W::Uow;
    type Error = OrganizationJoinRequestListProjectorError;

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
                            OrganizationJoinRequestListUpsert {
                                join_request_id,
                                organization_id: *organization_id,
                                requester_user_id: *requester_id,
                                status: OrganizationJoinRequestListItemStatus::Pending,
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
                            OrganizationJoinRequestListItemStatus::Approved,
                        )
                        .await?;
                }
                OrganizationJoinRequestEventPayload::Rejected { .. } => {
                    self.writer
                        .update_status(
                            uow,
                            event_context,
                            join_request_id,
                            OrganizationJoinRequestListItemStatus::Rejected,
                        )
                        .await?;
                }
                OrganizationJoinRequestEventPayload::Canceled { .. } => {
                    self.writer
                        .update_status(
                            uow,
                            event_context,
                            join_request_id,
                            OrganizationJoinRequestListItemStatus::Canceled,
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

        if event.is_for_aggregate::<User>() {
            let user_event = event.try_into_domain_event::<User>()?;
            let user_id = user_event.aggregate_id();

            match user_event.payload() {
                UserEventPayload::Registered { .. } => {
                    self.writer
                        .upsert_requester(
                            uow,
                            event_context,
                            OrganizationJoinRequestListRequesterUpsert {
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
                        .update_requester_username(uow, event_context, user_id, username.clone())
                        .await?;
                }
                UserEventPayload::DisplayNameChanged { display_name } => {
                    self.writer
                        .update_requester_display_name(
                            uow,
                            event_context,
                            user_id,
                            display_name.clone(),
                        )
                        .await?;
                }
                UserEventPayload::PictureChanged { picture, .. } => {
                    self.writer
                        .update_requester_picture(uow, event_context, user_id, picture.clone())
                        .await?;
                }
                UserEventPayload::Removed => {
                    self.writer
                        .delete_requester_and_join_requests(uow, event_context, user_id)
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
                            OrganizationJoinRequestListOrganizationUpsert {
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
        }

        Ok(())
    }
}
