use appletheia::application::read_model::MaterializationEventContext;
use appletheia::application::unit_of_work::UnitOfWork;
use banking_ledger_domain::currency::{CurrencyDescription, CurrencyId, CurrencyStatus};
use banking_ledger_domain::token_binding::TokenBindingId;

use super::{
    CurrencyFragment, CurrencyFragmentUpsert, CurrencyFragmentWriterError,
    CurrencyTokenBindingFragment,
};

#[allow(async_fn_in_trait)]
pub trait CurrencyFragmentWriter: Send + Sync {
    type Uow: UnitOfWork;

    async fn upsert_currency(
        &self,
        uow: &mut Self::Uow,
        event_context: MaterializationEventContext,
        upsert: CurrencyFragmentUpsert,
    ) -> Result<Option<CurrencyFragment>, CurrencyFragmentWriterError>;

    async fn update_currency_status(
        &self,
        uow: &mut Self::Uow,
        event_context: MaterializationEventContext,
        id: CurrencyId,
        status: CurrencyStatus,
    ) -> Result<Option<CurrencyFragment>, CurrencyFragmentWriterError>;

    async fn update_currency_description(
        &self,
        uow: &mut Self::Uow,
        event_context: MaterializationEventContext,
        id: CurrencyId,
        description: Option<CurrencyDescription>,
    ) -> Result<Option<CurrencyFragment>, CurrencyFragmentWriterError>;

    async fn define_token_binding(
        &self,
        uow: &mut Self::Uow,
        event_context: MaterializationEventContext,
        currency_id: CurrencyId,
        token_binding: CurrencyTokenBindingFragment,
    ) -> Result<Option<CurrencyFragment>, CurrencyFragmentWriterError>;

    async fn update_token_binding_deposit_enabled(
        &self,
        uow: &mut Self::Uow,
        event_context: MaterializationEventContext,
        token_binding_id: TokenBindingId,
        enabled: bool,
    ) -> Result<Option<CurrencyFragment>, CurrencyFragmentWriterError>;

    async fn update_token_binding_withdrawal_enabled(
        &self,
        uow: &mut Self::Uow,
        event_context: MaterializationEventContext,
        token_binding_id: TokenBindingId,
        enabled: bool,
    ) -> Result<Option<CurrencyFragment>, CurrencyFragmentWriterError>;

    async fn remove_token_binding(
        &self,
        uow: &mut Self::Uow,
        event_context: MaterializationEventContext,
        token_binding_id: TokenBindingId,
    ) -> Result<Option<CurrencyFragment>, CurrencyFragmentWriterError>;
}
