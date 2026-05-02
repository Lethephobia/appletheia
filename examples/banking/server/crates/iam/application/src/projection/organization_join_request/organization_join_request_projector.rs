use appletheia::application::event::EventEnvelope;
use appletheia::application::projection::Projector;
use banking_iam_domain::{
    OrganizationJoinRequest, OrganizationJoinRequestEventPayload, OrganizationJoinRequestStatus,
};

use super::{OrganizationJoinRequestProjectorError, OrganizationJoinRequestProjectorSpec};
use crate::view::{OrganizationJoinRequestViewStore, OrganizationJoinRequestViewUpsert};

/// Projects organization join request events into normalized join request views.
pub struct OrganizationJoinRequestProjector<VS>
where
    VS: OrganizationJoinRequestViewStore,
{
    view_store: VS,
}

impl<VS> OrganizationJoinRequestProjector<VS>
where
    VS: OrganizationJoinRequestViewStore,
{
    pub fn new(view_store: VS) -> Self {
        Self { view_store }
    }
}

impl<VS> Projector for OrganizationJoinRequestProjector<VS>
where
    VS: OrganizationJoinRequestViewStore,
{
    type Spec = OrganizationJoinRequestProjectorSpec;
    type Uow = VS::Uow;
    type Error = OrganizationJoinRequestProjectorError;

    async fn project(&self, uow: &mut Self::Uow, event: &EventEnvelope) -> Result<(), Self::Error> {
        let domain_event = event.try_into_domain_event::<OrganizationJoinRequest>()?;
        let join_request_id = domain_event.aggregate_id();

        match domain_event.payload() {
            OrganizationJoinRequestEventPayload::Requested {
                organization_id,
                requester_id,
                ..
            } => {
                self.view_store
                    .upsert(
                        uow,
                        OrganizationJoinRequestViewUpsert {
                            id: join_request_id,
                            organization_id: *organization_id,
                            requester_id: *requester_id,
                            status: OrganizationJoinRequestStatus::Pending,
                        },
                        event.event_sequence,
                    )
                    .await?;
            }
            OrganizationJoinRequestEventPayload::Approved { .. } => {
                self.view_store
                    .update_status(
                        uow,
                        join_request_id,
                        OrganizationJoinRequestStatus::Approved,
                        event.event_sequence,
                    )
                    .await?;
            }
            OrganizationJoinRequestEventPayload::Rejected { .. } => {
                self.view_store
                    .update_status(
                        uow,
                        join_request_id,
                        OrganizationJoinRequestStatus::Rejected,
                        event.event_sequence,
                    )
                    .await?;
            }
            OrganizationJoinRequestEventPayload::Canceled { .. } => {
                self.view_store
                    .update_status(
                        uow,
                        join_request_id,
                        OrganizationJoinRequestStatus::Canceled,
                        event.event_sequence,
                    )
                    .await?;
            }
            OrganizationJoinRequestEventPayload::ApproveRejected { .. }
            | OrganizationJoinRequestEventPayload::RejectRejected { .. }
            | OrganizationJoinRequestEventPayload::CancelRejected { .. } => {}
        }

        Ok(())
    }
}
