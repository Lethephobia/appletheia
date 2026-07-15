use appletheia::application::unit_of_work::UnitOfWork;

use banking_iam_domain::{
    OrganizationDisplayName, OrganizationHandle, OrganizationId, OrganizationPictureRef,
    UserDisplayName, UserId, UserPictureRef, Username,
};
use banking_ledger_domain::account::{AccountId, AccountName, AccountOwner};
use banking_ledger_domain::core::CurrencyAmount;
use banking_ledger_domain::currency::{
    CurrencyId, CurrencyName, CurrencySymbol, MintAccountAddress,
};
use banking_shared_kernel_application::read_model::ReadModelEventContext;

use super::{
    OwnedAccountListAccountUpsert, OwnedAccountListCurrencyUpsert, OwnedAccountListItemStatus,
    OwnedAccountListOwnerOrganizationUpsert, OwnedAccountListOwnerUserUpsert,
    OwnedAccountListWriterError,
};

#[allow(async_fn_in_trait)]
pub trait OwnedAccountListWriter: Send + Sync {
    type Uow: UnitOfWork;

    async fn upsert_account(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        upsert: OwnedAccountListAccountUpsert,
    ) -> Result<(), OwnedAccountListWriterError>;

    async fn update_account_owner(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        id: AccountId,
        owner: AccountOwner,
    ) -> Result<(), OwnedAccountListWriterError>;

    async fn update_account_name(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        id: AccountId,
        name: AccountName,
    ) -> Result<(), OwnedAccountListWriterError>;

    async fn increase_balance(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        id: AccountId,
        amount: CurrencyAmount,
    ) -> Result<(), OwnedAccountListWriterError>;

    async fn decrease_balance(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        id: AccountId,
        amount: CurrencyAmount,
    ) -> Result<(), OwnedAccountListWriterError>;

    async fn reserve_balance(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        id: AccountId,
        amount: CurrencyAmount,
    ) -> Result<(), OwnedAccountListWriterError>;

    async fn release_reserved_balance(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        id: AccountId,
        amount: CurrencyAmount,
    ) -> Result<(), OwnedAccountListWriterError>;

    async fn commit_reserved_balance(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        id: AccountId,
        amount: CurrencyAmount,
    ) -> Result<(), OwnedAccountListWriterError>;

    async fn update_account_status(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        id: AccountId,
        status: OwnedAccountListItemStatus,
    ) -> Result<(), OwnedAccountListWriterError>;

    async fn delete_account(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        id: AccountId,
    ) -> Result<(), OwnedAccountListWriterError>;

    async fn upsert_currency(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        upsert: OwnedAccountListCurrencyUpsert,
    ) -> Result<(), OwnedAccountListWriterError>;

    async fn update_currency_symbol(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        id: CurrencyId,
        symbol: CurrencySymbol,
    ) -> Result<(), OwnedAccountListWriterError>;

    async fn update_currency_name(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        id: CurrencyId,
        name: CurrencyName,
    ) -> Result<(), OwnedAccountListWriterError>;

    async fn update_mint_account_address(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        id: CurrencyId,
        mint_account_address: MintAccountAddress,
    ) -> Result<(), OwnedAccountListWriterError>;

    async fn delete_currency(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        id: CurrencyId,
    ) -> Result<(), OwnedAccountListWriterError>;

    async fn upsert_owner_user(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        upsert: OwnedAccountListOwnerUserUpsert,
    ) -> Result<(), OwnedAccountListWriterError>;

    async fn update_owner_user_username(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        id: UserId,
        username: Username,
    ) -> Result<(), OwnedAccountListWriterError>;

    async fn update_owner_user_display_name(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        id: UserId,
        display_name: UserDisplayName,
    ) -> Result<(), OwnedAccountListWriterError>;

    async fn update_owner_user_picture(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        id: UserId,
        picture: Option<UserPictureRef>,
    ) -> Result<(), OwnedAccountListWriterError>;

    async fn delete_owner_user(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        id: UserId,
    ) -> Result<(), OwnedAccountListWriterError>;

    async fn upsert_owner_organization(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        upsert: OwnedAccountListOwnerOrganizationUpsert,
    ) -> Result<(), OwnedAccountListWriterError>;

    async fn update_owner_organization_handle(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        id: OrganizationId,
        handle: OrganizationHandle,
    ) -> Result<(), OwnedAccountListWriterError>;

    async fn update_owner_organization_display_name(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        id: OrganizationId,
        display_name: OrganizationDisplayName,
    ) -> Result<(), OwnedAccountListWriterError>;

    async fn update_owner_organization_picture(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        id: OrganizationId,
        picture: Option<OrganizationPictureRef>,
    ) -> Result<(), OwnedAccountListWriterError>;

    async fn delete_owner_organization(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        id: OrganizationId,
    ) -> Result<(), OwnedAccountListWriterError>;
}
