use appletheia::application::repository::EventSaveHook;
use appletheia::domain::Event;
use banking_iam_domain::{
    OrganizationJoinRequest, OrganizationJoinRequestEventPayload, OrganizationJoinRequestId,
};

use crate::authorization::{
    OrganizationJoinRequestRelationshipUpdater, OrganizationJoinRequestRelationshipUpdaterError,
};

pub struct OrganizationJoinRequestEventSaveHook<OJRRU>
where
    OJRRU: OrganizationJoinRequestRelationshipUpdater,
{
    organization_join_request_relationship_updater: OJRRU,
}

impl<OJRRU> OrganizationJoinRequestEventSaveHook<OJRRU>
where
    OJRRU: OrganizationJoinRequestRelationshipUpdater,
{
    pub fn new(organization_join_request_relationship_updater: OJRRU) -> Self {
        Self {
            organization_join_request_relationship_updater,
        }
    }
}

impl<OJRRU> EventSaveHook<OrganizationJoinRequest> for OrganizationJoinRequestEventSaveHook<OJRRU>
where
    OJRRU: OrganizationJoinRequestRelationshipUpdater,
{
    type Uow = OJRRU::Uow;
    type Error = OrganizationJoinRequestRelationshipUpdaterError;

    async fn after_event_saved(
        &self,
        uow: &mut Self::Uow,
        event: &Event<OrganizationJoinRequestId, OrganizationJoinRequestEventPayload>,
    ) -> Result<(), Self::Error> {
        if let OrganizationJoinRequestEventPayload::Submitted {
            organization_id,
            requester_id,
            ..
        } = event.payload()
        {
            self.organization_join_request_relationship_updater
                .upsert_organization(uow, event.aggregate_id(), *organization_id)
                .await?;
            self.organization_join_request_relationship_updater
                .upsert_requester(uow, event.aggregate_id(), *requester_id)
                .await?;
        }

        Ok(())
    }
}
