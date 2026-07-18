use appletheia::application::event::EventEnvelope;
use appletheia::application::projection::Projector;
use banking_ledger_domain::wallet_bookmark::{WalletBookmark, WalletBookmarkEventPayload};
use banking_shared_kernel_application::read_model::ReadModelEventContext;

use super::{WalletBookmarkListProjectorError, WalletBookmarkListProjectorSpec};
use crate::read_model::{WalletBookmarkListUpsert, WalletBookmarkListWriter};

/// Projects wallet bookmark events into wallet bookmark list read models.
pub struct WalletBookmarkListProjector<W>
where
    W: WalletBookmarkListWriter,
{
    writer: W,
}

impl<W> WalletBookmarkListProjector<W>
where
    W: WalletBookmarkListWriter,
{
    pub fn new(writer: W) -> Self {
        Self { writer }
    }
}

impl<W> Projector for WalletBookmarkListProjector<W>
where
    W: WalletBookmarkListWriter,
{
    type Spec = WalletBookmarkListProjectorSpec;
    type Uow = W::Uow;
    type Error = WalletBookmarkListProjectorError;

    async fn project(&self, uow: &mut Self::Uow, event: &EventEnvelope) -> Result<(), Self::Error> {
        let event_context = ReadModelEventContext::from(event);
        let domain_event = event.try_into_domain_event::<WalletBookmark>()?;
        let wallet_bookmark_id = domain_event.aggregate_id();

        match domain_event.payload() {
            WalletBookmarkEventPayload::Registered {
                owner,
                display_name,
                description,
                token_account_owner_address,
                ..
            } => {
                self.writer
                    .upsert_wallet_bookmark(
                        uow,
                        event_context,
                        WalletBookmarkListUpsert {
                            id: wallet_bookmark_id,
                            owner: *owner,
                            display_name: display_name.clone(),
                            description: description.clone(),
                            token_account_owner_address: token_account_owner_address.clone(),
                        },
                    )
                    .await?;
            }
            WalletBookmarkEventPayload::DisplayNameChanged { display_name } => {
                self.writer
                    .update_display_name(
                        uow,
                        event_context,
                        wallet_bookmark_id,
                        display_name.clone(),
                    )
                    .await?;
            }
            WalletBookmarkEventPayload::DescriptionChanged { description } => {
                self.writer
                    .update_description(uow, event_context, wallet_bookmark_id, description.clone())
                    .await?;
            }
            WalletBookmarkEventPayload::Removed => {
                self.writer
                    .delete_wallet_bookmark(uow, event_context, wallet_bookmark_id)
                    .await?;
            }
            WalletBookmarkEventPayload::RegisterRejected { .. }
            | WalletBookmarkEventPayload::RemoveRejected { .. }
            | WalletBookmarkEventPayload::DisplayNameChangeRejected { .. }
            | WalletBookmarkEventPayload::DescriptionChangeRejected { .. } => {}
        }

        Ok(())
    }
}
