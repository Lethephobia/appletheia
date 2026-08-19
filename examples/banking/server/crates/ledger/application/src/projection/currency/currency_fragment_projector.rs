use appletheia::application::event::EventEnvelope;
use appletheia::application::projection::Projector;
use appletheia::application::read_model::{
    MaterializationEventContext, ReadModelFragmentPartition, ReadModelPartition,
};
use banking_ledger_domain::core::CurrencyAmount;
use banking_ledger_domain::currency::{Currency, CurrencyEventPayload};

use super::{CurrencyFragmentProjectorError, CurrencyFragmentProjectorSpec};
use crate::projection::{
    CurrencyFragment, CurrencyFragmentUpsert, CurrencyFragmentWriter, MaterializedCurrencyStatus,
};

/// Projects currency events into currency fragments.
pub struct CurrencyFragmentProjector<W>
where
    W: CurrencyFragmentWriter,
{
    currency_fragment_writer: W,
}

impl<W> CurrencyFragmentProjector<W>
where
    W: CurrencyFragmentWriter,
{
    pub fn new(currency_fragment_writer: W) -> Self {
        Self {
            currency_fragment_writer,
        }
    }
}

impl<W> Projector for CurrencyFragmentProjector<W>
where
    W: CurrencyFragmentWriter,
{
    type Spec = CurrencyFragmentProjectorSpec;
    type Fragment = CurrencyFragment;
    type Uow = W::Uow;
    type Error = CurrencyFragmentProjectorError;

    async fn project(
        &self,
        uow: &mut Self::Uow,
        event_context: MaterializationEventContext,
        event: &EventEnvelope,
    ) -> Result<Vec<ReadModelFragmentPartition<Self::Fragment>>, Self::Error> {
        let mut invalidated_partitions = Vec::new();
        let domain_event = event.try_into_domain_event::<Currency>()?;
        let currency_id = domain_event.aggregate_id();

        match domain_event.payload() {
            CurrencyEventPayload::Defined {
                owner,
                symbol,
                name,
                decimals,
                description,
                image,
                ..
            } => {
                if let Some(fragment) = self
                    .currency_fragment_writer
                    .upsert_currency(
                        uow,
                        event_context,
                        CurrencyFragmentUpsert {
                            id: currency_id,
                            owner: *owner,
                            symbol: symbol.clone(),
                            name: name.clone(),
                            decimals: *decimals,
                            description: description.clone(),
                            image: image.clone(),
                            mint_account_address: None,
                            supply: CurrencyAmount::zero(),
                            status: MaterializedCurrencyStatus::Provisioning,
                        },
                    )
                    .await?
                {
                    invalidated_partitions.push(ReadModelPartition::from_fragment(&fragment));
                }
            }
            CurrencyEventPayload::Provisioned { mint_account } => {
                if let Some(fragment) = self
                    .currency_fragment_writer
                    .provision_currency(
                        uow,
                        event_context,
                        currency_id,
                        mint_account.mint_account_address().clone(),
                    )
                    .await?
                {
                    invalidated_partitions.push(ReadModelPartition::from_fragment(&fragment));
                }
            }
            CurrencyEventPayload::OwnershipTransferred { owner } => {
                if let Some(fragment) = self
                    .currency_fragment_writer
                    .update_currency_owner(uow, event_context, currency_id, *owner)
                    .await?
                {
                    invalidated_partitions.push(ReadModelPartition::from_fragment(&fragment));
                }
            }
            CurrencyEventPayload::SymbolChanged { symbol } => {
                if let Some(fragment) = self
                    .currency_fragment_writer
                    .update_currency_symbol(uow, event_context, currency_id, symbol.clone())
                    .await?
                {
                    invalidated_partitions.push(ReadModelPartition::from_fragment(&fragment));
                }
            }
            CurrencyEventPayload::NameChanged { name } => {
                if let Some(fragment) = self
                    .currency_fragment_writer
                    .update_currency_name(uow, event_context, currency_id, name.clone())
                    .await?
                {
                    invalidated_partitions.push(ReadModelPartition::from_fragment(&fragment));
                }
            }
            CurrencyEventPayload::DescriptionChanged { description } => {
                if let Some(fragment) = self
                    .currency_fragment_writer
                    .update_currency_description(
                        uow,
                        event_context,
                        currency_id,
                        description.clone(),
                    )
                    .await?
                {
                    invalidated_partitions.push(ReadModelPartition::from_fragment(&fragment));
                }
            }
            CurrencyEventPayload::ImageChanged { image, .. } => {
                if let Some(fragment) = self
                    .currency_fragment_writer
                    .update_currency_image(uow, event_context, currency_id, image.clone())
                    .await?
                {
                    invalidated_partitions.push(ReadModelPartition::from_fragment(&fragment));
                }
            }
            CurrencyEventPayload::SupplyCommitted { amount } => {
                if let Some(fragment) = self
                    .currency_fragment_writer
                    .increase_currency_supply(uow, event_context, currency_id, *amount)
                    .await?
                {
                    invalidated_partitions.push(ReadModelPartition::from_fragment(&fragment));
                }
            }
            CurrencyEventPayload::Activated => {
                if let Some(fragment) = self
                    .currency_fragment_writer
                    .update_currency_status(
                        uow,
                        event_context,
                        currency_id,
                        MaterializedCurrencyStatus::Active,
                    )
                    .await?
                {
                    invalidated_partitions.push(ReadModelPartition::from_fragment(&fragment));
                }
            }
            CurrencyEventPayload::Deactivated => {
                if let Some(fragment) = self
                    .currency_fragment_writer
                    .update_currency_status(
                        uow,
                        event_context,
                        currency_id,
                        MaterializedCurrencyStatus::Inactive,
                    )
                    .await?
                {
                    invalidated_partitions.push(ReadModelPartition::from_fragment(&fragment));
                }
            }
            CurrencyEventPayload::Removed => {
                if self
                    .currency_fragment_writer
                    .delete_currency(uow, event_context, currency_id)
                    .await?
                {
                    invalidated_partitions.push(ReadModelPartition::new(currency_id));
                }
            }
            CurrencyEventPayload::DefineRejected { .. }
            | CurrencyEventPayload::OwnershipTransferRejected { .. }
            | CurrencyEventPayload::ProvisionRejected { .. }
            | CurrencyEventPayload::SymbolChangeRejected { .. }
            | CurrencyEventPayload::NameChangeRejected { .. }
            | CurrencyEventPayload::DescriptionChangeRejected { .. }
            | CurrencyEventPayload::ImageChangeRejected { .. }
            | CurrencyEventPayload::MintMetadataSynced
            | CurrencyEventPayload::MintMetadataSyncRejected { .. }
            | CurrencyEventPayload::SupplyReserved { .. }
            | CurrencyEventPayload::SupplyReserveRejected { .. }
            | CurrencyEventPayload::MintSupplySynced { .. }
            | CurrencyEventPayload::MintSupplySyncRejected { .. }
            | CurrencyEventPayload::SupplyCommitRejected { .. }
            | CurrencyEventPayload::SupplyReleased { .. }
            | CurrencyEventPayload::SupplyReleaseRejected { .. }
            | CurrencyEventPayload::ActivateRejected { .. }
            | CurrencyEventPayload::DeactivateRejected { .. }
            | CurrencyEventPayload::RemoveRejected { .. } => {}
        }

        Ok(invalidated_partitions)
    }
}
