use appletheia::application::event::EventEnvelope;
use appletheia::application::projection::Projector;
use banking_iam_domain::{Organization, OrganizationEventPayload};
use banking_shared_kernel_application::read_model::ReadModelEventContext;

use crate::read_model::{OrganizationInternalInfoUpsert, OrganizationInternalInfoWriter};

use super::{OrganizationInternalInfoProjectorError, OrganizationInternalInfoProjectorSpec};

/// Projects organization events into organization-internal information read models.
pub struct OrganizationInternalInfoProjector<W>
where
    W: OrganizationInternalInfoWriter,
{
    writer: W,
}

impl<W> OrganizationInternalInfoProjector<W>
where
    W: OrganizationInternalInfoWriter,
{
    pub fn new(writer: W) -> Self {
        Self { writer }
    }
}

impl<W> Projector for OrganizationInternalInfoProjector<W>
where
    W: OrganizationInternalInfoWriter,
{
    type Spec = OrganizationInternalInfoProjectorSpec;
    type Uow = W::Uow;
    type Error = OrganizationInternalInfoProjectorError;

    async fn project(&self, uow: &mut Self::Uow, event: &EventEnvelope) -> Result<(), Self::Error> {
        let event_context = ReadModelEventContext::from(event);
        let domain_event = event.try_into_domain_event::<Organization>()?;
        let organization_id = domain_event.aggregate_id();

        match domain_event.payload() {
            OrganizationEventPayload::Created {
                handle,
                display_name,
                description,
                website_url,
                picture,
                ..
            } => {
                self.writer
                    .upsert_organization(
                        uow,
                        event_context,
                        OrganizationInternalInfoUpsert {
                            id: organization_id,
                            handle: handle.clone(),
                            display_name: display_name.clone(),
                            description: description.clone(),
                            website_url: website_url.clone(),
                            picture: picture.clone(),
                        },
                    )
                    .await?;
            }
            OrganizationEventPayload::HandleChanged { handle } => {
                self.writer
                    .update_handle(uow, event_context, organization_id, handle.clone())
                    .await?;
            }
            OrganizationEventPayload::DisplayNameChanged { display_name } => {
                self.writer
                    .update_display_name(uow, event_context, organization_id, display_name.clone())
                    .await?;
            }
            OrganizationEventPayload::DescriptionChanged { description } => {
                self.writer
                    .update_description(uow, event_context, organization_id, description.clone())
                    .await?;
            }
            OrganizationEventPayload::WebsiteUrlChanged { website_url } => {
                self.writer
                    .update_website_url(uow, event_context, organization_id, website_url.clone())
                    .await?;
            }
            OrganizationEventPayload::PictureChanged { picture, .. } => {
                self.writer
                    .update_picture(uow, event_context, organization_id, picture.clone())
                    .await?;
            }
            OrganizationEventPayload::Removed => {
                self.writer
                    .delete_organization(uow, event_context, organization_id)
                    .await?;
            }
            OrganizationEventPayload::CreateRejected { .. }
            | OrganizationEventPayload::OwnershipTransferred { .. }
            | OrganizationEventPayload::OwnershipTransferRejected { .. }
            | OrganizationEventPayload::HandleChangeRejected { .. }
            | OrganizationEventPayload::DisplayNameChangeRejected { .. }
            | OrganizationEventPayload::DescriptionChangeRejected { .. }
            | OrganizationEventPayload::WebsiteUrlChangeRejected { .. }
            | OrganizationEventPayload::PictureChangeRejected { .. }
            | OrganizationEventPayload::RemoveRejected { .. } => {}
        }

        Ok(())
    }
}
