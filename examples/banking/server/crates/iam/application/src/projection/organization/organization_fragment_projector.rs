use appletheia::application::event::EventEnvelope;
use appletheia::application::projection::Projector;
use appletheia::application::read_model::{
    MaterializationEventContext, ReadModelFragmentPartition, ReadModelPartition,
};
use banking_iam_domain::{Organization, OrganizationEventPayload, OrganizationOwner};

use crate::projection::{
    OrganizationFragment, OrganizationFragmentUpsert, OrganizationFragmentWriter,
};

use super::{OrganizationFragmentProjectorError, OrganizationFragmentProjectorSpec};

/// Projects shared organization fragments.
pub struct OrganizationFragmentProjector<W>
where
    W: OrganizationFragmentWriter,
{
    organization_fragment_writer: W,
}

impl<W> OrganizationFragmentProjector<W>
where
    W: OrganizationFragmentWriter,
{
    pub fn new(organization_fragment_writer: W) -> Self {
        Self {
            organization_fragment_writer,
        }
    }
}

impl<W> Projector for OrganizationFragmentProjector<W>
where
    W: OrganizationFragmentWriter,
{
    type Spec = OrganizationFragmentProjectorSpec;
    type Fragment = OrganizationFragment;
    type Uow = W::Uow;
    type Error = OrganizationFragmentProjectorError;

    async fn project(
        &self,
        uow: &mut Self::Uow,
        event_context: MaterializationEventContext,
        event: &EventEnvelope,
    ) -> Result<Vec<ReadModelFragmentPartition<Self::Fragment>>, Self::Error> {
        let mut invalidated_partitions = Vec::new();
        let organization_event = event.try_into_domain_event::<Organization>()?;
        let organization_id = organization_event.aggregate_id();

        match organization_event.payload() {
            OrganizationEventPayload::Created {
                owner,
                handle,
                display_name,
                description,
                website_url,
                picture,
            } => {
                let OrganizationOwner::User(owner_user_id) = owner;

                if let Some(fragment) = self
                    .organization_fragment_writer
                    .upsert_organization(
                        uow,
                        event_context,
                        OrganizationFragmentUpsert {
                            id: organization_id,
                            owner_user_id: *owner_user_id,
                            handle: handle.clone(),
                            display_name: display_name.clone(),
                            description: description.clone(),
                            website_url: website_url.clone(),
                            picture: picture.clone(),
                        },
                    )
                    .await?
                {
                    invalidated_partitions.push(ReadModelPartition::from_fragment(&fragment));
                }
            }
            OrganizationEventPayload::OwnershipTransferred { owner } => {
                let OrganizationOwner::User(owner_user_id) = owner;

                if let Some(fragment) = self
                    .organization_fragment_writer
                    .update_owner(uow, event_context, organization_id, *owner_user_id)
                    .await?
                {
                    invalidated_partitions.push(ReadModelPartition::from_fragment(&fragment));
                }
            }
            OrganizationEventPayload::HandleChanged { handle } => {
                if let Some(fragment) = self
                    .organization_fragment_writer
                    .update_handle(uow, event_context, organization_id, handle.clone())
                    .await?
                {
                    invalidated_partitions.push(ReadModelPartition::from_fragment(&fragment));
                }
            }
            OrganizationEventPayload::DisplayNameChanged { display_name } => {
                if let Some(fragment) = self
                    .organization_fragment_writer
                    .update_display_name(uow, event_context, organization_id, display_name.clone())
                    .await?
                {
                    invalidated_partitions.push(ReadModelPartition::from_fragment(&fragment));
                }
            }
            OrganizationEventPayload::DescriptionChanged { description } => {
                if let Some(fragment) = self
                    .organization_fragment_writer
                    .update_description(uow, event_context, organization_id, description.clone())
                    .await?
                {
                    invalidated_partitions.push(ReadModelPartition::from_fragment(&fragment));
                }
            }
            OrganizationEventPayload::WebsiteUrlChanged { website_url } => {
                if let Some(fragment) = self
                    .organization_fragment_writer
                    .update_website_url(uow, event_context, organization_id, website_url.clone())
                    .await?
                {
                    invalidated_partitions.push(ReadModelPartition::from_fragment(&fragment));
                }
            }
            OrganizationEventPayload::PictureChanged { picture, .. } => {
                if let Some(fragment) = self
                    .organization_fragment_writer
                    .update_picture(uow, event_context, organization_id, picture.clone())
                    .await?
                {
                    invalidated_partitions.push(ReadModelPartition::from_fragment(&fragment));
                }
            }
            OrganizationEventPayload::Removed => {
                if self
                    .organization_fragment_writer
                    .delete_organization(uow, event_context, organization_id)
                    .await?
                {
                    invalidated_partitions.push(ReadModelPartition::new(organization_id));
                }
            }
            OrganizationEventPayload::CreateRejected { .. }
            | OrganizationEventPayload::OwnershipTransferRejected { .. }
            | OrganizationEventPayload::HandleChangeRejected { .. }
            | OrganizationEventPayload::DisplayNameChangeRejected { .. }
            | OrganizationEventPayload::DescriptionChangeRejected { .. }
            | OrganizationEventPayload::WebsiteUrlChangeRejected { .. }
            | OrganizationEventPayload::PictureChangeRejected { .. }
            | OrganizationEventPayload::RemoveRejected { .. } => {}
        }

        Ok(invalidated_partitions)
    }
}
