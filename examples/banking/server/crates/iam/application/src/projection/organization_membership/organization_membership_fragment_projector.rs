use appletheia::application::event::EventEnvelope;
use appletheia::application::projection::Projector;
use appletheia::application::read_model::{
    MaterializationEventContext, ReadModelFragment, ReadModelInvalidatedPartitions,
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
    ) -> Result<
        ReadModelInvalidatedPartitions<<Self::Fragment as ReadModelFragment>::Key>,
        Self::Error,
    > {
        let mut invalidated_partitions = ReadModelInvalidatedPartitions::new();

        if event.is_for_aggregate::<OrganizationMembership>() {
            let membership_event = event.try_to_domain_event::<OrganizationMembership>()?;
            let organization_membership_id = membership_event.aggregate_id();

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
                                organization_membership_id,
                                user_id: *user_id,
                                organization_id: *organization_id,
                                roles: roles.clone(),
                            },
                        )
                        .await?
                    {
                        invalidated_partitions.insert(fragment.key());
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
                        invalidated_partitions.insert(fragment.key());
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
                        invalidated_partitions.insert(key);
                    }
                }
            }

            return Ok(invalidated_partitions);
        }

        let user_event = event.try_to_domain_event::<User>()?;
        let user_id = user_event.aggregate_id();

        match user_event.payload() {
            UserEventPayload::Removed => {
                let removed_keys = self
                    .organization_membership_fragment_writer
                    .delete_for_user(uow, event_context, user_id)
                    .await?;
                for key in removed_keys {
                    invalidated_partitions.insert(key);
                }
            }
            UserEventPayload::Registered { .. }
            | UserEventPayload::IdentityLinked { .. }
            | UserEventPayload::IdentityEmailChanged { .. }
            | UserEventPayload::UsernameChanged { .. }
            | UserEventPayload::DisplayNameChanged { .. }
            | UserEventPayload::BioChanged { .. }
            | UserEventPayload::PictureChanged { .. }
            | UserEventPayload::Activated
            | UserEventPayload::Deactivated => {}
        }

        Ok(invalidated_partitions)
    }
}
