use appletheia::application::event::EventEnvelope;
use appletheia::application::projection::Projector;
use appletheia::application::read_model::MaterializationEventContext;
use banking_ledger_domain::currency::{Currency, CurrencyEventPayload, CurrencyStatus};
use banking_ledger_domain::token_binding::{TokenBinding, TokenBindingEventPayload};

use super::{
    CurrencyFragment, CurrencyFragmentProjectorError, CurrencyFragmentProjectorSpec,
    CurrencyFragmentUpsert, CurrencyFragmentWriter, CurrencyTokenBindingFragment,
};

pub struct CurrencyFragmentProjector<W>
where
    W: CurrencyFragmentWriter,
{
    writer: W,
}

impl<W> CurrencyFragmentProjector<W>
where
    W: CurrencyFragmentWriter,
{
    pub fn new(writer: W) -> Self {
        Self { writer }
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
    ) -> Result<(), Self::Error> {
        if event.is_for_aggregate::<Currency>() {
            let currency_event = event.try_to_domain_event::<Currency>()?;
            let currency_id = currency_event.aggregate_id();
            match currency_event.payload() {
                CurrencyEventPayload::Defined {
                    currency_registrar_id,
                    code,
                    decimals,
                    description,
                } => {
                    self.writer
                        .upsert_currency(
                            uow,
                            event_context,
                            CurrencyFragmentUpsert {
                                id: currency_id,
                                currency_registrar_id: *currency_registrar_id,
                                code: code.clone(),
                                decimals: *decimals,
                                description: description.clone(),
                                status: CurrencyStatus::Defined,
                            },
                        )
                        .await?;
                }
                CurrencyEventPayload::DescriptionChanged { description } => {
                    self.writer
                        .update_currency_description(
                            uow,
                            event_context,
                            currency_id,
                            description.clone(),
                        )
                        .await?;
                }
                CurrencyEventPayload::Activated => {
                    self.writer
                        .update_currency_status(
                            uow,
                            event_context,
                            currency_id,
                            CurrencyStatus::Active,
                        )
                        .await?;
                }
                CurrencyEventPayload::Deactivated => {
                    self.writer
                        .update_currency_status(
                            uow,
                            event_context,
                            currency_id,
                            CurrencyStatus::Inactive,
                        )
                        .await?;
                }
            }
        } else if event.is_for_aggregate::<TokenBinding>() {
            let token_binding_event = event.try_to_domain_event::<TokenBinding>()?;
            match token_binding_event.payload() {
                TokenBindingEventPayload::Defined {
                    currency_id,
                    chain_network,
                    token_address,
                    deposit_enabled,
                    withdrawal_enabled,
                } => {
                    self.writer
                        .define_token_binding(
                            uow,
                            event_context,
                            *currency_id,
                            CurrencyTokenBindingFragment {
                                id: token_binding_event.aggregate_id(),
                                chain_network: *chain_network,
                                token_address: *token_address,
                                deposit_enabled: *deposit_enabled,
                                withdrawal_enabled: *withdrawal_enabled,
                            },
                        )
                        .await?;
                }
                TokenBindingEventPayload::DepositEnabledChanged { enabled } => {
                    self.writer
                        .update_token_binding_deposit_enabled(
                            uow,
                            event_context,
                            token_binding_event.aggregate_id(),
                            *enabled,
                        )
                        .await?;
                }
                TokenBindingEventPayload::WithdrawalEnabledChanged { enabled } => {
                    self.writer
                        .update_token_binding_withdrawal_enabled(
                            uow,
                            event_context,
                            token_binding_event.aggregate_id(),
                            *enabled,
                        )
                        .await?;
                }
                TokenBindingEventPayload::Removed => {
                    self.writer
                        .remove_token_binding(
                            uow,
                            event_context,
                            token_binding_event.aggregate_id(),
                        )
                        .await?;
                }
            }
        }

        Ok(())
    }
}
