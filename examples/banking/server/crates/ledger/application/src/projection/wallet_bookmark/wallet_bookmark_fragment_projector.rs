use appletheia::application::event::EventEnvelope;
use appletheia::application::projection::Projector;
use appletheia::application::read_model::{
    MaterializationEventContext, ReadModelFragmentPartition, ReadModelPartition,
};
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
    ) -> Result<Vec<ReadModelFragmentPartition<Self::Fragment>>, Self::Error> {
        let mut invalidated_partitions = Vec::new();
        let domain_event = event.try_into_domain_event::<WalletBookmark>()?;
        let wallet_bookmark_id = domain_event.aggregate_id();

        match domain_event.payload() {
            WalletBookmarkEventPayload::Registered {
                owner,
                display_name,
                description,
                token_owner_address,
                ..
            } => {
                if let Some(fragment) = self
                    .wallet_bookmark_fragment_writer
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
                    .await?
                {
                    invalidated_partitions.push(ReadModelPartition::from_fragment(&fragment));
                }
            }
            WalletBookmarkEventPayload::DisplayNameChanged { display_name } => {
                if let Some(fragment) = self
                    .wallet_bookmark_fragment_writer
                    .update_display_name(
                        uow,
                        event_context,
                        wallet_bookmark_id,
                        display_name.clone(),
                    )
                    .await?
                {
                    invalidated_partitions.push(ReadModelPartition::from_fragment(&fragment));
                }
            }
            WalletBookmarkEventPayload::DescriptionChanged { description } => {
                if let Some(fragment) = self
                    .wallet_bookmark_fragment_writer
                    .update_description(uow, event_context, wallet_bookmark_id, description.clone())
                    .await?
                {
                    invalidated_partitions.push(ReadModelPartition::from_fragment(&fragment));
                }
            }
            WalletBookmarkEventPayload::Removed => {
                if self
                    .wallet_bookmark_fragment_writer
                    .delete_wallet_bookmark(uow, event_context, wallet_bookmark_id)
                    .await?
                {
                    invalidated_partitions.push(ReadModelPartition::new(wallet_bookmark_id));
                }
            }
            WalletBookmarkEventPayload::RemoveRejected { .. }
            | WalletBookmarkEventPayload::DisplayNameChangeRejected { .. }
            | WalletBookmarkEventPayload::DescriptionChangeRejected { .. } => {}
        }

        Ok(invalidated_partitions)
    }
}
