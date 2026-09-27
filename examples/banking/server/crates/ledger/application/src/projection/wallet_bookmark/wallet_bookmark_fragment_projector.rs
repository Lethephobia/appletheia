use appletheia::application::event::EventEnvelope;
use appletheia::application::projection::Projector;
use appletheia::application::read_model::MaterializationEventContext;
use banking_ledger_domain::wallet_bookmark::{WalletBookmark, WalletBookmarkEventPayload};

use super::{WalletBookmarkFragmentProjectorError, WalletBookmarkFragmentProjectorSpec};
use crate::projection::{
    WalletBookmarkFragment, WalletBookmarkFragmentUpsert, WalletBookmarkFragmentWriter,
};

/// Projects wallet bookmark events into wallet bookmark fragments.
pub struct WalletBookmarkFragmentProjector<W>
where
    W: WalletBookmarkFragmentWriter,
{
    wallet_bookmark_fragment_writer: W,
}

impl<W> WalletBookmarkFragmentProjector<W>
where
    W: WalletBookmarkFragmentWriter,
{
    pub fn new(wallet_bookmark_fragment_writer: W) -> Self {
        Self {
            wallet_bookmark_fragment_writer,
        }
    }
}

impl<W> Projector for WalletBookmarkFragmentProjector<W>
where
    W: WalletBookmarkFragmentWriter,
{
    type Spec = WalletBookmarkFragmentProjectorSpec;
    type Fragment = WalletBookmarkFragment;
    type Uow = W::Uow;
    type Error = WalletBookmarkFragmentProjectorError;

    async fn project(
        &self,
        uow: &mut Self::Uow,
        event_context: MaterializationEventContext,
        event: &EventEnvelope,
    ) -> Result<(), Self::Error> {
        let domain_event = event.try_to_domain_event::<WalletBookmark>()?;
        let wallet_bookmark_id = domain_event.aggregate_id();

        match domain_event.payload() {
            WalletBookmarkEventPayload::Registered {
                owner,
                display_name,
                description,
                token_owner_address,
                ..
            } => {
                self.wallet_bookmark_fragment_writer
                    .upsert_wallet_bookmark(
                        uow,
                        event_context,
                        WalletBookmarkFragmentUpsert {
                            id: wallet_bookmark_id,
                            owner: *owner,
                            display_name: display_name.clone(),
                            description: description.clone(),
                            token_owner_address: *token_owner_address,
                        },
                    )
                    .await?;
            }
            WalletBookmarkEventPayload::DisplayNameChanged { display_name } => {
                self.wallet_bookmark_fragment_writer
                    .update_display_name(
                        uow,
                        event_context,
                        wallet_bookmark_id,
                        display_name.clone(),
                    )
                    .await?;
            }
            WalletBookmarkEventPayload::DescriptionChanged { description } => {
                self.wallet_bookmark_fragment_writer
                    .update_description(uow, event_context, wallet_bookmark_id, description.clone())
                    .await?;
            }
            WalletBookmarkEventPayload::Removed => {
                self.wallet_bookmark_fragment_writer
                    .delete_wallet_bookmark(uow, event_context, wallet_bookmark_id)
                    .await?;
            }
        }

        Ok(())
    }
}
