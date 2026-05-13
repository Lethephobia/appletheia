use appletheia::application::event::EventSequence;
use appletheia::application::unit_of_work::UnitOfWork;
use appletheia::domain::EventOccurredAt;
use banking_iam_domain::{
    OrganizationDisplayName, OrganizationHandle, OrganizationId, OrganizationPictureRef,
    UserDisplayName, UserId, UserPictureRef, Username,
};
use banking_ledger_domain::account::{AccountId, AccountName, AccountOwner};
use banking_ledger_domain::core::CurrencyAmount;
use banking_ledger_domain::currency::{CurrencyId, CurrencyName, CurrencySymbol};

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
        upsert: OwnedAccountListAccountUpsert,
    ) -> Result<(), OwnedAccountListWriterError>;

    async fn update_account_owner(
        &self,
        uow: &mut Self::Uow,
        id: AccountId,
        owner: AccountOwner,
        event_sequence: EventSequence,
        occurred_at: EventOccurredAt,
    ) -> Result<(), OwnedAccountListWriterError>;

    async fn update_account_name(
        &self,
        uow: &mut Self::Uow,
        id: AccountId,
        name: AccountName,
        event_sequence: EventSequence,
        occurred_at: EventOccurredAt,
    ) -> Result<(), OwnedAccountListWriterError>;

    async fn increase_balance(
        &self,
        uow: &mut Self::Uow,
        id: AccountId,
        amount: CurrencyAmount,
        event_sequence: EventSequence,
        occurred_at: EventOccurredAt,
    ) -> Result<(), OwnedAccountListWriterError>;

    async fn decrease_balance(
        &self,
        uow: &mut Self::Uow,
        id: AccountId,
        amount: CurrencyAmount,
        event_sequence: EventSequence,
        occurred_at: EventOccurredAt,
    ) -> Result<(), OwnedAccountListWriterError>;

    async fn reserve_balance(
        &self,
        uow: &mut Self::Uow,
        id: AccountId,
        amount: CurrencyAmount,
        event_sequence: EventSequence,
        occurred_at: EventOccurredAt,
    ) -> Result<(), OwnedAccountListWriterError>;

    async fn release_reserved_balance(
        &self,
        uow: &mut Self::Uow,
        id: AccountId,
        amount: CurrencyAmount,
        event_sequence: EventSequence,
        occurred_at: EventOccurredAt,
    ) -> Result<(), OwnedAccountListWriterError>;

    async fn commit_reserved_balance(
        &self,
        uow: &mut Self::Uow,
        id: AccountId,
        amount: CurrencyAmount,
        event_sequence: EventSequence,
        occurred_at: EventOccurredAt,
    ) -> Result<(), OwnedAccountListWriterError>;

    async fn update_account_status(
        &self,
        uow: &mut Self::Uow,
        id: AccountId,
        status: OwnedAccountListItemStatus,
        event_sequence: EventSequence,
        occurred_at: EventOccurredAt,
    ) -> Result<(), OwnedAccountListWriterError>;

    async fn delete_account(
        &self,
        uow: &mut Self::Uow,
        id: AccountId,
        event_sequence: EventSequence,
        occurred_at: EventOccurredAt,
    ) -> Result<(), OwnedAccountListWriterError>;

    async fn upsert_currency(
        &self,
        uow: &mut Self::Uow,
        upsert: OwnedAccountListCurrencyUpsert,
    ) -> Result<(), OwnedAccountListWriterError>;

    async fn update_currency_symbol(
        &self,
        uow: &mut Self::Uow,
        id: CurrencyId,
        symbol: CurrencySymbol,
        event_sequence: EventSequence,
        occurred_at: EventOccurredAt,
    ) -> Result<(), OwnedAccountListWriterError>;

    async fn update_currency_name(
        &self,
        uow: &mut Self::Uow,
        id: CurrencyId,
        name: CurrencyName,
        event_sequence: EventSequence,
        occurred_at: EventOccurredAt,
    ) -> Result<(), OwnedAccountListWriterError>;

    async fn delete_currency(
        &self,
        uow: &mut Self::Uow,
        id: CurrencyId,
        event_sequence: EventSequence,
        occurred_at: EventOccurredAt,
    ) -> Result<(), OwnedAccountListWriterError>;

    async fn upsert_owner_user(
        &self,
        uow: &mut Self::Uow,
        upsert: OwnedAccountListOwnerUserUpsert,
    ) -> Result<(), OwnedAccountListWriterError>;

    async fn update_owner_user_username(
        &self,
        uow: &mut Self::Uow,
        id: UserId,
        username: Username,
        event_sequence: EventSequence,
        occurred_at: EventOccurredAt,
    ) -> Result<(), OwnedAccountListWriterError>;

    async fn update_owner_user_display_name(
        &self,
        uow: &mut Self::Uow,
        id: UserId,
        display_name: UserDisplayName,
        event_sequence: EventSequence,
        occurred_at: EventOccurredAt,
    ) -> Result<(), OwnedAccountListWriterError>;

    async fn update_owner_user_picture(
        &self,
        uow: &mut Self::Uow,
        id: UserId,
        picture: Option<UserPictureRef>,
        event_sequence: EventSequence,
        occurred_at: EventOccurredAt,
    ) -> Result<(), OwnedAccountListWriterError>;

    async fn delete_owner_user(
        &self,
        uow: &mut Self::Uow,
        id: UserId,
        event_sequence: EventSequence,
    ) -> Result<(), OwnedAccountListWriterError>;

    async fn upsert_owner_organization(
        &self,
        uow: &mut Self::Uow,
        upsert: OwnedAccountListOwnerOrganizationUpsert,
    ) -> Result<(), OwnedAccountListWriterError>;

    async fn update_owner_organization_handle(
        &self,
        uow: &mut Self::Uow,
        id: OrganizationId,
        handle: OrganizationHandle,
        event_sequence: EventSequence,
        occurred_at: EventOccurredAt,
    ) -> Result<(), OwnedAccountListWriterError>;

    async fn update_owner_organization_display_name(
        &self,
        uow: &mut Self::Uow,
        id: OrganizationId,
        display_name: OrganizationDisplayName,
        event_sequence: EventSequence,
        occurred_at: EventOccurredAt,
    ) -> Result<(), OwnedAccountListWriterError>;

    async fn update_owner_organization_picture(
        &self,
        uow: &mut Self::Uow,
        id: OrganizationId,
        picture: Option<OrganizationPictureRef>,
        event_sequence: EventSequence,
        occurred_at: EventOccurredAt,
    ) -> Result<(), OwnedAccountListWriterError>;

    async fn delete_owner_organization(
        &self,
        uow: &mut Self::Uow,
        id: OrganizationId,
        event_sequence: EventSequence,
    ) -> Result<(), OwnedAccountListWriterError>;
}
