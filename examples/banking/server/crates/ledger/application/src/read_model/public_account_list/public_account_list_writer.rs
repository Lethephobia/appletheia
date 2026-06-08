use appletheia::application::unit_of_work::UnitOfWork;

use banking_iam_domain::{
    OrganizationDisplayName, OrganizationHandle, OrganizationId, OrganizationPictureRef,
    UserDisplayName, UserId, UserPictureRef, Username,
};
use banking_ledger_domain::account::{AccountId, AccountOwner};
use banking_ledger_domain::currency::{CurrencyId, CurrencyName, CurrencySymbol};
use banking_shared_kernel_application::read_model::ReadModelEventContext;

use super::{
    PublicAccountListAccountUpsert, PublicAccountListCurrencyUpsert, PublicAccountListItemStatus,
    PublicAccountListOwnerOrganizationUpsert, PublicAccountListOwnerUserUpsert,
    PublicAccountListWriterError,
};

#[allow(async_fn_in_trait)]
pub trait PublicAccountListWriter: Send + Sync {
    type Uow: UnitOfWork;

    async fn upsert_account(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        upsert: PublicAccountListAccountUpsert,
    ) -> Result<(), PublicAccountListWriterError>;

    async fn update_account_owner(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        id: AccountId,
        owner: AccountOwner,
    ) -> Result<(), PublicAccountListWriterError>;

    async fn update_account_status(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        id: AccountId,
        status: PublicAccountListItemStatus,
    ) -> Result<(), PublicAccountListWriterError>;

    async fn delete_account(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        id: AccountId,
    ) -> Result<(), PublicAccountListWriterError>;

    async fn upsert_currency(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        upsert: PublicAccountListCurrencyUpsert,
    ) -> Result<(), PublicAccountListWriterError>;

    async fn update_currency_symbol(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        id: CurrencyId,
        symbol: CurrencySymbol,
    ) -> Result<(), PublicAccountListWriterError>;

    async fn update_currency_name(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        id: CurrencyId,
        name: CurrencyName,
    ) -> Result<(), PublicAccountListWriterError>;

    async fn delete_currency(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        id: CurrencyId,
    ) -> Result<(), PublicAccountListWriterError>;

    async fn upsert_owner_user(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        upsert: PublicAccountListOwnerUserUpsert,
    ) -> Result<(), PublicAccountListWriterError>;

    async fn update_owner_user_username(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        id: UserId,
        username: Username,
    ) -> Result<(), PublicAccountListWriterError>;

    async fn update_owner_user_display_name(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        id: UserId,
        display_name: UserDisplayName,
    ) -> Result<(), PublicAccountListWriterError>;

    async fn update_owner_user_picture(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        id: UserId,
        picture: Option<UserPictureRef>,
    ) -> Result<(), PublicAccountListWriterError>;

    async fn delete_owner_user(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        id: UserId,
    ) -> Result<(), PublicAccountListWriterError>;

    async fn upsert_owner_organization(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        upsert: PublicAccountListOwnerOrganizationUpsert,
    ) -> Result<(), PublicAccountListWriterError>;

    async fn update_owner_organization_handle(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        id: OrganizationId,
        handle: OrganizationHandle,
    ) -> Result<(), PublicAccountListWriterError>;

    async fn update_owner_organization_display_name(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        id: OrganizationId,
        display_name: OrganizationDisplayName,
    ) -> Result<(), PublicAccountListWriterError>;

    async fn update_owner_organization_picture(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        id: OrganizationId,
        picture: Option<OrganizationPictureRef>,
    ) -> Result<(), PublicAccountListWriterError>;

    async fn delete_owner_organization(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        id: OrganizationId,
    ) -> Result<(), PublicAccountListWriterError>;
}
