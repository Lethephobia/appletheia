use appletheia::application::unit_of_work::UnitOfWork;

use crate::read_model::ReadModelEventContext;
use banking_iam_domain::{
    OrganizationDisplayName, OrganizationHandle, OrganizationId, OrganizationPictureRef,
    UserDisplayName, UserId, UserPictureRef, Username,
};
use banking_ledger_domain::core::CurrencyAmount;
use banking_ledger_domain::currency::{
    CurrencyDescription, CurrencyId, CurrencyImageRef, CurrencyName, CurrencyOwner, CurrencySymbol,
};

use super::{
    CurrencyListCurrencyUpsert, CurrencyListItemStatus, CurrencyListOwnerOrganizationUpsert,
    CurrencyListOwnerUserUpsert, CurrencyListWriterError,
};

#[allow(async_fn_in_trait)]
pub trait CurrencyListWriter: Send + Sync {
    type Uow: UnitOfWork;

    async fn upsert_currency(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        upsert: CurrencyListCurrencyUpsert,
    ) -> Result<(), CurrencyListWriterError>;

    async fn update_currency_owner(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        id: CurrencyId,
        owner: CurrencyOwner,
    ) -> Result<(), CurrencyListWriterError>;

    async fn update_currency_symbol(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        id: CurrencyId,
        symbol: CurrencySymbol,
    ) -> Result<(), CurrencyListWriterError>;

    async fn update_currency_name(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        id: CurrencyId,
        name: CurrencyName,
    ) -> Result<(), CurrencyListWriterError>;

    async fn update_currency_description(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        id: CurrencyId,
        description: Option<CurrencyDescription>,
    ) -> Result<(), CurrencyListWriterError>;

    async fn update_currency_image(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        id: CurrencyId,
        image: Option<CurrencyImageRef>,
    ) -> Result<(), CurrencyListWriterError>;

    async fn increase_currency_supply(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        id: CurrencyId,
        amount: CurrencyAmount,
    ) -> Result<(), CurrencyListWriterError>;

    async fn decrease_currency_supply(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        id: CurrencyId,
        amount: CurrencyAmount,
    ) -> Result<(), CurrencyListWriterError>;

    async fn update_currency_status(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        id: CurrencyId,
        status: CurrencyListItemStatus,
    ) -> Result<(), CurrencyListWriterError>;

    async fn delete_currency(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        id: CurrencyId,
    ) -> Result<(), CurrencyListWriterError>;

    async fn upsert_owner_user(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        upsert: CurrencyListOwnerUserUpsert,
    ) -> Result<(), CurrencyListWriterError>;

    async fn update_owner_user_username(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        id: UserId,
        username: Username,
    ) -> Result<(), CurrencyListWriterError>;

    async fn update_owner_user_display_name(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        id: UserId,
        display_name: UserDisplayName,
    ) -> Result<(), CurrencyListWriterError>;

    async fn update_owner_user_picture(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        id: UserId,
        picture: Option<UserPictureRef>,
    ) -> Result<(), CurrencyListWriterError>;

    async fn delete_owner_user(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        id: UserId,
    ) -> Result<(), CurrencyListWriterError>;

    async fn upsert_owner_organization(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        upsert: CurrencyListOwnerOrganizationUpsert,
    ) -> Result<(), CurrencyListWriterError>;

    async fn update_owner_organization_handle(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        id: OrganizationId,
        handle: OrganizationHandle,
    ) -> Result<(), CurrencyListWriterError>;

    async fn update_owner_organization_display_name(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        id: OrganizationId,
        display_name: OrganizationDisplayName,
    ) -> Result<(), CurrencyListWriterError>;

    async fn update_owner_organization_picture(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        id: OrganizationId,
        picture: Option<OrganizationPictureRef>,
    ) -> Result<(), CurrencyListWriterError>;

    async fn delete_owner_organization(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        id: OrganizationId,
    ) -> Result<(), CurrencyListWriterError>;
}
