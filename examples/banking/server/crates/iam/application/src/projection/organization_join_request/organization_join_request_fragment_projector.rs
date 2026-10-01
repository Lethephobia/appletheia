use appletheia::application::event::EventEnvelope;
use appletheia::application::projection::Projector;
use appletheia::application::read_model::MaterializationEventContext;
use banking_iam_domain::{
    OrganizationJoinRequest, OrganizationJoinRequestEventPayload, OrganizationJoinRequestStatus,
};

use crate::projection::{
    OrganizationJoinRequestFragment, OrganizationJoinRequestFragmentUpsert,
    OrganizationJoinRequestFragmentWriter,
};

use super::{
    OrganizationJoinRequestFragmentProjectorError, OrganizationJoinRequestFragmentProjectorSpec,
};

/// Projects join request events into organization join request fragments.
pub struct OrganizationJoinRequestFragmentProjector<W>
where
    W: OrganizationJoinRequestFragmentWriter,
{
    organization_join_request_fragment_writer: W,
}

impl<W> OrganizationJoinRequestFragmentProjector<W>
where
    W: OrganizationJoinRequestFragmentWriter,
{
    pub fn new(organization_join_request_fragment_writer: W) -> Self {
        Self {
            organization_join_request_fragment_writer,
        }
    }
}

impl<W> Projector for OrganizationJoinRequestFragmentProjector<W>
where
    W: OrganizationJoinRequestFragmentWriter,
{
    type Spec = OrganizationJoinRequestFragmentProjectorSpec;
    type Fragment = OrganizationJoinRequestFragment;
    type Uow = W::Uow;
    type Error = OrganizationJoinRequestFragmentProjectorError;

    async fn project(
        &self,
        uow: &mut Self::Uow,
        event_context: MaterializationEventContext,
        event: &EventEnvelope,
    ) -> Result<(), Self::Error> {
        let join_request_event = event.try_to_domain_event::<OrganizationJoinRequest>()?;
        let join_request_id = join_request_event.aggregate_id();

        match join_request_event.payload() {
            OrganizationJoinRequestEventPayload::Submitted {
                organization_id,
                requester_id,
            } => {
                self.organization_join_request_fragment_writer
                    .upsert(
                        uow,
                        event_context,
                        OrganizationJoinRequestFragmentUpsert {
                            join_request_id,
                            organization_id: *organization_id,
                            requester_user_id: *requester_id,
                            status: OrganizationJoinRequestStatus::Pending,
                        },
                    )
                    .await?;
            }
            OrganizationJoinRequestEventPayload::Approved { .. } => {
                self.organization_join_request_fragment_writer
                    .update_status(
                        uow,
                        event_context,
                        join_request_id,
                        OrganizationJoinRequestStatus::Approved,
                    )
                    .await?;
            }
            OrganizationJoinRequestEventPayload::Rejected { .. } => {
                self.organization_join_request_fragment_writer
                    .update_status(
                        uow,
                        event_context,
                        join_request_id,
                        OrganizationJoinRequestStatus::Rejected,
                    )
                    .await?;
            }
            OrganizationJoinRequestEventPayload::Canceled { .. } => {
                self.organization_join_request_fragment_writer
                    .update_status(
                        uow,
                        event_context,
                        join_request_id,
                        OrganizationJoinRequestStatus::Canceled,
                    )
                    .await?;
            }
        }

        Ok(())
    }
}
