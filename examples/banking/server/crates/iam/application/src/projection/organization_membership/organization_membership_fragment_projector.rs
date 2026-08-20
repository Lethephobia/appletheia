use appletheia::application::event::EventEnvelope;
use appletheia::application::projection::Projector;
use appletheia::application::read_model::{
    MaterializationEventContext, ReadModelFragmentPartition, ReadModelPartition,
};
use banking_iam_domain::{
    OrganizationMembership, OrganizationMembershipEventPayload, User, UserEventPayload,
};

use crate::projection::{
    OrganizationMembershipFragment, OrganizationMembershipFragmentKey,
    OrganizationMembershipFragmentUpsert, OrganizationMembershipFragmentWriter,
};

use super::{
    OrganizationMembershipFragmentProjectorError, OrganizationMembershipFragmentProjectorSpec,
};

/// Projects organization membership events into organization membership fragments.
pub struct OrganizationMembershipFragmentProjector<W>
where
    W: OrganizationMembershipFragmentWriter,
{
    organization_membership_fragment_writer: W,
}

impl<W> OrganizationMembershipFragmentProjector<W>
where
    W: OrganizationMembershipFragmentWriter,
{
    pub fn new(organization_membership_fragment_writer: W) -> Self {
        Self {
            organization_membership_fragment_writer,
        }
    }
}

impl<W> Projector for OrganizationMembershipFragmentProjector<W>
where
    W: OrganizationMembershipFragmentWriter,
{
    type Spec = OrganizationMembershipFragmentProjectorSpec;
    type Fragment = OrganizationMembershipFragment;
    type Uow = W::Uow;
    type Error = OrganizationMembershipFragmentProjectorError;

    async fn project(
        &self,
        uow: &mut Self::Uow,
        event_context: MaterializationEventContext,
        event: &EventEnvelope,
    ) -> Result<Vec<ReadModelFragmentPartition<Self::Fragment>>, Self::Error> {
        let mut invalidated_partitions = Vec::new();

        if event.is_for_aggregate::<OrganizationMembership>() {
            let membership_event = event.try_into_domain_event::<OrganizationMembership>()?;

            match membership_event.payload() {
                OrganizationMembershipEventPayload::Created {
                    organization_id,
                    user_id,
                    roles,
                } => {
                    if let Some(fragment) = self
                        .organization_membership_fragment_writer
                        .upsert(
                            uow,
                            event_context,
                            OrganizationMembershipFragmentUpsert {
                                user_id: *user_id,
                                organization_id: *organization_id,
                                roles: roles.clone(),
                            },
                        )
                        .await?
                    {
                        invalidated_partitions.push(ReadModelPartition::from_fragment(&fragment));
                    }
                }
                OrganizationMembershipEventPayload::RolesChanged {
                    organization_id,
                    user_id,
                    roles,
                } => {
                    if let Some(fragment) = self
                        .organization_membership_fragment_writer
                        .update_roles(
                            uow,
                            event_context,
                            *user_id,
                            *organization_id,
                            roles.clone(),
                        )
                        .await?
                    {
                        invalidated_partitions.push(ReadModelPartition::from_fragment(&fragment));
                    }
                }
                OrganizationMembershipEventPayload::Removed {
                    organization_id,
                    user_id,
                } => {
                    if self
                        .organization_membership_fragment_writer
                        .delete(uow, event_context, *user_id, *organization_id)
                        .await?
                    {
                        let key = OrganizationMembershipFragmentKey {
                            user_id: *user_id,
                            organization_id: *organization_id,
                        };
                        invalidated_partitions.push(ReadModelPartition::new(key));
                    }
                }
                OrganizationMembershipEventPayload::CreateRejected { .. }
                | OrganizationMembershipEventPayload::RolesChangeRejected { .. }
                | OrganizationMembershipEventPayload::RemoveRejected { .. } => {}
            }

            return Ok(invalidated_partitions);
        }

        let user_event = event.try_into_domain_event::<User>()?;
        let user_id = user_event.aggregate_id();

        match user_event.payload() {
            UserEventPayload::Removed => {
                let removed_keys = self
                    .organization_membership_fragment_writer
                    .delete_for_user(uow, event_context, user_id)
                    .await?;
                for key in removed_keys {
                    invalidated_partitions.push(ReadModelPartition::new(key));
                }
            }
            UserEventPayload::Registered { .. }
            | UserEventPayload::IdentityLinked { .. }
            | UserEventPayload::IdentityLinkRejected { .. }
            | UserEventPayload::IdentityEmailChanged { .. }
            | UserEventPayload::IdentityEmailChangeRejected { .. }
            | UserEventPayload::UsernameChanged { .. }
            | UserEventPayload::UsernameChangeRejected { .. }
            | UserEventPayload::DisplayNameChanged { .. }
            | UserEventPayload::DisplayNameChangeRejected { .. }
            | UserEventPayload::BioChanged { .. }
            | UserEventPayload::BioChangeRejected { .. }
            | UserEventPayload::PictureChanged { .. }
            | UserEventPayload::PictureChangeRejected { .. }
            | UserEventPayload::Activated
            | UserEventPayload::ActivateRejected { .. }
            | UserEventPayload::Deactivated
            | UserEventPayload::DeactivateRejected { .. }
            | UserEventPayload::RemoveRejected { .. } => {}
        }

        Ok(invalidated_partitions)
    }
}
