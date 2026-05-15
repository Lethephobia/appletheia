use appletheia::application::event::EventSequence;
use appletheia::application::unit_of_work::UnitOfWork;
use appletheia::domain::EventId;
use appletheia::domain::EventOccurredAt;
use banking_iam_domain::{
    OrganizationDisplayName, OrganizationHandle, OrganizationId, OrganizationPictureRef,
    UserDisplayName, UserId, UserPictureRef, Username,
};
use banking_ledger_domain::core::CurrencyAmount;
use banking_ledger_domain::currency::{CurrencyId, CurrencyName, CurrencyOwner, CurrencySymbol};

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
        upsert: CurrencyListCurrencyUpsert,
    ) -> Result<(), CurrencyListWriterError>;

    async fn update_currency_owner(
        &self,
        uow: &mut Self::Uow,
        id: CurrencyId,
        owner: CurrencyOwner,
        event_id: EventId,
        event_sequence: EventSequence,
        occurred_at: EventOccurredAt,
    ) -> Result<(), CurrencyListWriterError>;

    async fn update_currency_symbol(
        &self,
        uow: &mut Self::Uow,
        id: CurrencyId,
        symbol: CurrencySymbol,
        event_id: EventId,
        event_sequence: EventSequence,
        occurred_at: EventOccurredAt,
    ) -> Result<(), CurrencyListWriterError>;

    async fn update_currency_name(
        &self,
        uow: &mut Self::Uow,
        id: CurrencyId,
        name: CurrencyName,
        event_id: EventId,
        event_sequence: EventSequence,
        occurred_at: EventOccurredAt,
    ) -> Result<(), CurrencyListWriterError>;

    async fn increase_currency_supply(
        &self,
        uow: &mut Self::Uow,
        id: CurrencyId,
        amount: CurrencyAmount,
        event_id: EventId,
        event_sequence: EventSequence,
        occurred_at: EventOccurredAt,
    ) -> Result<(), CurrencyListWriterError>;

    async fn decrease_currency_supply(
        &self,
        uow: &mut Self::Uow,
        id: CurrencyId,
        amount: CurrencyAmount,
        event_id: EventId,
        event_sequence: EventSequence,
        occurred_at: EventOccurredAt,
    ) -> Result<(), CurrencyListWriterError>;

    async fn update_currency_status(
        &self,
        uow: &mut Self::Uow,
        id: CurrencyId,
        status: CurrencyListItemStatus,
        event_id: EventId,
        event_sequence: EventSequence,
        occurred_at: EventOccurredAt,
    ) -> Result<(), CurrencyListWriterError>;

    async fn delete_currency(
        &self,
        uow: &mut Self::Uow,
        id: CurrencyId,
        event_id: EventId,
        event_sequence: EventSequence,
        occurred_at: EventOccurredAt,
    ) -> Result<(), CurrencyListWriterError>;

    async fn upsert_owner_user(
        &self,
        uow: &mut Self::Uow,
        upsert: CurrencyListOwnerUserUpsert,
    ) -> Result<(), CurrencyListWriterError>;

    async fn update_owner_user_username(
        &self,
        uow: &mut Self::Uow,
        id: UserId,
        username: Username,
        event_id: EventId,
        event_sequence: EventSequence,
        occurred_at: EventOccurredAt,
    ) -> Result<(), CurrencyListWriterError>;

    async fn update_owner_user_display_name(
        &self,
        uow: &mut Self::Uow,
        id: UserId,
        display_name: UserDisplayName,
        event_id: EventId,
        event_sequence: EventSequence,
        occurred_at: EventOccurredAt,
    ) -> Result<(), CurrencyListWriterError>;

    async fn update_owner_user_picture(
        &self,
        uow: &mut Self::Uow,
        id: UserId,
        picture: Option<UserPictureRef>,
        event_id: EventId,
        event_sequence: EventSequence,
        occurred_at: EventOccurredAt,
    ) -> Result<(), CurrencyListWriterError>;

    async fn delete_owner_user(
        &self,
        uow: &mut Self::Uow,
        id: UserId,
        event_id: EventId,
        event_sequence: EventSequence,
        occurred_at: EventOccurredAt,
    ) -> Result<(), CurrencyListWriterError>;

    async fn upsert_owner_organization(
        &self,
        uow: &mut Self::Uow,
        upsert: CurrencyListOwnerOrganizationUpsert,
    ) -> Result<(), CurrencyListWriterError>;

    async fn update_owner_organization_handle(
        &self,
        uow: &mut Self::Uow,
        id: OrganizationId,
        handle: OrganizationHandle,
        event_id: EventId,
        event_sequence: EventSequence,
        occurred_at: EventOccurredAt,
    ) -> Result<(), CurrencyListWriterError>;

    async fn update_owner_organization_display_name(
        &self,
        uow: &mut Self::Uow,
        id: OrganizationId,
        display_name: OrganizationDisplayName,
        event_id: EventId,
        event_sequence: EventSequence,
        occurred_at: EventOccurredAt,
    ) -> Result<(), CurrencyListWriterError>;

    async fn update_owner_organization_picture(
        &self,
        uow: &mut Self::Uow,
        id: OrganizationId,
        picture: Option<OrganizationPictureRef>,
        event_id: EventId,
        event_sequence: EventSequence,
        occurred_at: EventOccurredAt,
    ) -> Result<(), CurrencyListWriterError>;

    async fn delete_owner_organization(
        &self,
        uow: &mut Self::Uow,
        id: OrganizationId,
        event_id: EventId,
        event_sequence: EventSequence,
        occurred_at: EventOccurredAt,
    ) -> Result<(), CurrencyListWriterError>;
}
