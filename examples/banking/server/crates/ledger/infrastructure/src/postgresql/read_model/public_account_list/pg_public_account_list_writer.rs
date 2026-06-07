use appletheia::domain::AggregateId;
use appletheia::infrastructure::postgresql::PgUnitOfWork;
use banking_iam_domain::{
    OrganizationDisplayName, OrganizationHandle, OrganizationId, OrganizationPictureRef,
    UserDisplayName, UserId, UserPictureRef, Username,
};
use banking_ledger_application::{
    PublicAccountListAccountUpsert, PublicAccountListCurrencyUpsert, PublicAccountListItemStatus,
    PublicAccountListOwnerOrganizationUpsert, PublicAccountListOwnerUserUpsert,
    PublicAccountListWriter, PublicAccountListWriterError, ReadModelEventContext,
};
use banking_ledger_domain::account::{AccountId, AccountOwner};
use banking_ledger_domain::currency::{CurrencyId, CurrencyName, CurrencySymbol};

use super::super::pg_organization_picture_ref_columns::PgOrganizationPictureRefColumns;
use super::super::pg_user_picture_ref_columns::PgUserPictureRefColumns;

/// PostgreSQL-backed public account list writer.
pub struct PgPublicAccountListWriter;

impl PgPublicAccountListWriter {
    pub fn new() -> Self {
        Self
    }

    fn owner_parts(owner: AccountOwner) -> (&'static str, uuid::Uuid) {
        match owner {
            AccountOwner::User(user_id) => ("user", user_id.value()),
            AccountOwner::Organization(organization_id) => {
                ("organization", organization_id.value())
            }
        }
    }

    fn status_name(status: PublicAccountListItemStatus) -> &'static str {
        match status {
            PublicAccountListItemStatus::Active => "active",
            PublicAccountListItemStatus::Frozen => "frozen",
        }
    }
}

impl Default for PgPublicAccountListWriter {
    fn default() -> Self {
        Self::new()
    }
}

impl PublicAccountListWriter for PgPublicAccountListWriter {
    type Uow = PgUnitOfWork;

    async fn upsert_account(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        upsert: PublicAccountListAccountUpsert,
    ) -> Result<(), PublicAccountListWriterError> {
        let (owner_type, owner_id) = Self::owner_parts(upsert.owner);

        sqlx::query(
            r#"
            INSERT INTO public_account_list_items (
                id, owner_type, owner_id, currency_id, status, updated_at, created_at, source_event_sequence, updated_event_sequence, source_event_id, updated_event_id
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $8, $9, $9)
            ON CONFLICT (id) DO UPDATE SET
                owner_type = EXCLUDED.owner_type,
                owner_id = EXCLUDED.owner_id,
                currency_id = EXCLUDED.currency_id,
                status = EXCLUDED.status,
                updated_at = EXCLUDED.updated_at,
                updated_event_id = EXCLUDED.updated_event_id,
                updated_event_sequence = EXCLUDED.updated_event_sequence
            WHERE public_account_list_items.updated_event_sequence < EXCLUDED.updated_event_sequence
            "#,
        )
        .bind(upsert.id.value())
        .bind(owner_type)
        .bind(owner_id)
        .bind(upsert.currency_id.value())
        .bind(Self::status_name(upsert.status))
        .bind(event_context.occurred_at.value())
        .bind(event_context.occurred_at.value())
        .bind(event_context.event_sequence.value())
        .bind(event_context.event_id.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|e| PublicAccountListWriterError::Persistence(Box::new(e)))?;
        Ok(())
    }

    async fn update_account_owner(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        id: AccountId,
        owner: AccountOwner,
    ) -> Result<(), PublicAccountListWriterError> {
        let (owner_type, owner_id) = Self::owner_parts(owner);

        sqlx::query(
            r#"
            UPDATE public_account_list_items
               SET owner_type = $2, owner_id = $3, updated_at = $4,
                   updated_event_sequence = $5,
                   updated_event_id = $6
             WHERE id = $1 AND updated_event_sequence < $5
            "#,
        )
        .bind(id.value())
        .bind(owner_type)
        .bind(owner_id)
        .bind(event_context.occurred_at.value())
        .bind(event_context.event_sequence.value())
        .bind(event_context.event_id.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|e| PublicAccountListWriterError::Persistence(Box::new(e)))?;
        Ok(())
    }

    async fn update_account_status(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        id: AccountId,
        status: PublicAccountListItemStatus,
    ) -> Result<(), PublicAccountListWriterError> {
        sqlx::query(
            r#"
            UPDATE public_account_list_items
               SET status = $2, updated_at = $3,
                   updated_event_sequence = $4,
                   updated_event_id = $5
             WHERE id = $1 AND updated_event_sequence < $4
            "#,
        )
        .bind(id.value())
        .bind(Self::status_name(status))
        .bind(event_context.occurred_at.value())
        .bind(event_context.event_sequence.value())
        .bind(event_context.event_id.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|e| PublicAccountListWriterError::Persistence(Box::new(e)))?;
        Ok(())
    }

    async fn delete_account(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        id: AccountId,
    ) -> Result<(), PublicAccountListWriterError> {
        sqlx::query(
            r#"
            DELETE FROM public_account_list_items
             WHERE id = $1 AND updated_event_sequence < $2
            "#,
        )
        .bind(id.value())
        .bind(event_context.event_sequence.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|e| PublicAccountListWriterError::Persistence(Box::new(e)))?;
        Ok(())
    }

    async fn upsert_currency(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        upsert: PublicAccountListCurrencyUpsert,
    ) -> Result<(), PublicAccountListWriterError> {
        sqlx::query(
            r#"
            INSERT INTO public_account_list_item_currencies (
                id, symbol, name, decimals, updated_at, created_at, source_event_sequence, updated_event_sequence, source_event_id, updated_event_id
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $7, $8, $8)
            ON CONFLICT (id) DO UPDATE SET
                symbol = EXCLUDED.symbol,
                name = EXCLUDED.name,
                decimals = EXCLUDED.decimals,
                updated_at = EXCLUDED.updated_at,
                updated_event_id = EXCLUDED.updated_event_id,
                updated_event_sequence = EXCLUDED.updated_event_sequence
            WHERE public_account_list_item_currencies.updated_event_sequence < EXCLUDED.updated_event_sequence
            "#,
        )
        .bind(upsert.id.value())
        .bind(upsert.symbol.value())
        .bind(upsert.name.value())
        .bind(i16::from(upsert.decimals.value()))
        .bind(event_context.occurred_at.value())
        .bind(event_context.occurred_at.value())
        .bind(event_context.event_sequence.value())
        .bind(event_context.event_id.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|e| PublicAccountListWriterError::Persistence(Box::new(e)))?;
        Ok(())
    }

    async fn update_currency_symbol(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        id: CurrencyId,
        symbol: CurrencySymbol,
    ) -> Result<(), PublicAccountListWriterError> {
        sqlx::query(
            r#"
            UPDATE public_account_list_item_currencies
               SET symbol = $2, updated_at = $3,
                   updated_event_sequence = $4,
                   updated_event_id = $5
             WHERE id = $1 AND updated_event_sequence < $4
            "#,
        )
        .bind(id.value())
        .bind(symbol.value())
        .bind(event_context.occurred_at.value())
        .bind(event_context.event_sequence.value())
        .bind(event_context.event_id.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|e| PublicAccountListWriterError::Persistence(Box::new(e)))?;
        Ok(())
    }

    async fn update_currency_name(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        id: CurrencyId,
        name: CurrencyName,
    ) -> Result<(), PublicAccountListWriterError> {
        sqlx::query(
            r#"
            UPDATE public_account_list_item_currencies
               SET name = $2, updated_at = $3,
                   updated_event_sequence = $4,
                   updated_event_id = $5
             WHERE id = $1 AND updated_event_sequence < $4
            "#,
        )
        .bind(id.value())
        .bind(name.value())
        .bind(event_context.occurred_at.value())
        .bind(event_context.event_sequence.value())
        .bind(event_context.event_id.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|e| PublicAccountListWriterError::Persistence(Box::new(e)))?;
        Ok(())
    }

    async fn delete_currency(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        id: CurrencyId,
    ) -> Result<(), PublicAccountListWriterError> {
        sqlx::query(
            r#"
            DELETE FROM public_account_list_items
             WHERE currency_id = $1 AND updated_event_sequence < $2
            "#,
        )
        .bind(id.value())
        .bind(event_context.event_sequence.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|e| PublicAccountListWriterError::Persistence(Box::new(e)))?;

        sqlx::query(
            r#"
            DELETE FROM public_account_list_item_currencies
             WHERE id = $1 AND updated_event_sequence < $2
            "#,
        )
        .bind(id.value())
        .bind(event_context.event_sequence.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|e| PublicAccountListWriterError::Persistence(Box::new(e)))?;
        Ok(())
    }

    async fn upsert_owner_user(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        upsert: PublicAccountListOwnerUserUpsert,
    ) -> Result<(), PublicAccountListWriterError> {
        let (picture_type, object_name, external_url) =
            PgUserPictureRefColumns::from_picture(upsert.picture.as_ref());

        sqlx::query(
            r#"
            INSERT INTO public_account_list_item_owner_users (
                id, username, display_name, picture_type, picture_object_name,
                picture_external_url, updated_at, created_at, source_event_sequence, updated_event_sequence, source_event_id, updated_event_id
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $9, $10, $10)
            ON CONFLICT (id) DO UPDATE SET
                username = EXCLUDED.username,
                display_name = EXCLUDED.display_name,
                picture_type = EXCLUDED.picture_type,
                picture_object_name = EXCLUDED.picture_object_name,
                picture_external_url = EXCLUDED.picture_external_url,
                updated_at = EXCLUDED.updated_at,
                updated_event_id = EXCLUDED.updated_event_id,
                updated_event_sequence = EXCLUDED.updated_event_sequence
            WHERE public_account_list_item_owner_users.updated_event_sequence < EXCLUDED.updated_event_sequence
            "#,
        )
        .bind(upsert.id.value())
        .bind(upsert.username.as_ref().map(Username::value))
        .bind(upsert.display_name.as_ref().map(UserDisplayName::value))
        .bind(picture_type)
        .bind(object_name)
        .bind(external_url)
        .bind(event_context.occurred_at.value())
        .bind(event_context.occurred_at.value())
        .bind(event_context.event_sequence.value())
        .bind(event_context.event_id.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|e| PublicAccountListWriterError::Persistence(Box::new(e)))?;
        Ok(())
    }

    async fn update_owner_user_username(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        id: UserId,
        username: Username,
    ) -> Result<(), PublicAccountListWriterError> {
        sqlx::query(
            r#"
            UPDATE public_account_list_item_owner_users
               SET username = $2, updated_at = $3,
                   updated_event_sequence = $4,
                   updated_event_id = $5
             WHERE id = $1 AND updated_event_sequence < $4
            "#,
        )
        .bind(id.value())
        .bind(username.value())
        .bind(event_context.occurred_at.value())
        .bind(event_context.event_sequence.value())
        .bind(event_context.event_id.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|e| PublicAccountListWriterError::Persistence(Box::new(e)))?;
        Ok(())
    }

    async fn update_owner_user_display_name(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        id: UserId,
        display_name: UserDisplayName,
    ) -> Result<(), PublicAccountListWriterError> {
        sqlx::query(
            r#"
            UPDATE public_account_list_item_owner_users
               SET display_name = $2, updated_at = $3,
                   updated_event_sequence = $4,
                   updated_event_id = $5
             WHERE id = $1 AND updated_event_sequence < $4
            "#,
        )
        .bind(id.value())
        .bind(display_name.value())
        .bind(event_context.occurred_at.value())
        .bind(event_context.event_sequence.value())
        .bind(event_context.event_id.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|e| PublicAccountListWriterError::Persistence(Box::new(e)))?;
        Ok(())
    }

    async fn update_owner_user_picture(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        id: UserId,
        picture: Option<UserPictureRef>,
    ) -> Result<(), PublicAccountListWriterError> {
        let (picture_type, object_name, external_url) =
            PgUserPictureRefColumns::from_picture(picture.as_ref());

        sqlx::query(
            r#"
            UPDATE public_account_list_item_owner_users
               SET picture_type = $2,
                   picture_object_name = $3,
                   picture_external_url = $4,
                   updated_at = $5,
                   updated_event_sequence = $6,
                   updated_event_id = $7
             WHERE id = $1 AND updated_event_sequence < $6
            "#,
        )
        .bind(id.value())
        .bind(picture_type)
        .bind(object_name)
        .bind(external_url)
        .bind(event_context.occurred_at.value())
        .bind(event_context.event_sequence.value())
        .bind(event_context.event_id.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|e| PublicAccountListWriterError::Persistence(Box::new(e)))?;
        Ok(())
    }

    async fn delete_owner_user(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        id: UserId,
    ) -> Result<(), PublicAccountListWriterError> {
        sqlx::query(
            r#"
            DELETE FROM public_account_list_items
             WHERE owner_type = 'user'
               AND owner_id = $1
               AND updated_event_sequence < $2
            "#,
        )
        .bind(id.value())
        .bind(event_context.event_sequence.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|e| PublicAccountListWriterError::Persistence(Box::new(e)))?;

        sqlx::query(
            r#"
            DELETE FROM public_account_list_item_owner_users
             WHERE id = $1 AND updated_event_sequence < $2
            "#,
        )
        .bind(id.value())
        .bind(event_context.event_sequence.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|e| PublicAccountListWriterError::Persistence(Box::new(e)))?;
        Ok(())
    }

    async fn upsert_owner_organization(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        upsert: PublicAccountListOwnerOrganizationUpsert,
    ) -> Result<(), PublicAccountListWriterError> {
        let (picture_type, object_name, external_url) =
            PgOrganizationPictureRefColumns::from_picture(upsert.picture.as_ref());

        sqlx::query(
            r#"
            INSERT INTO public_account_list_item_owner_organizations (
                id, handle, display_name, picture_type, picture_object_name, picture_external_url,
                updated_at, created_at, source_event_sequence, updated_event_sequence, source_event_id, updated_event_id
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $9, $10, $10)
            ON CONFLICT (id) DO UPDATE SET
                handle = EXCLUDED.handle,
                display_name = EXCLUDED.display_name,
                picture_type = EXCLUDED.picture_type,
                picture_object_name = EXCLUDED.picture_object_name,
                picture_external_url = EXCLUDED.picture_external_url,
                updated_at = EXCLUDED.updated_at,
                updated_event_id = EXCLUDED.updated_event_id,
                updated_event_sequence = EXCLUDED.updated_event_sequence
            WHERE public_account_list_item_owner_organizations.updated_event_sequence < EXCLUDED.updated_event_sequence
            "#,
        )
        .bind(upsert.id.value())
        .bind(upsert.handle.value())
        .bind(upsert.display_name.value())
        .bind(picture_type)
        .bind(object_name)
        .bind(external_url)
        .bind(event_context.occurred_at.value())
        .bind(event_context.occurred_at.value())
        .bind(event_context.event_sequence.value())
        .bind(event_context.event_id.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|e| PublicAccountListWriterError::Persistence(Box::new(e)))?;
        Ok(())
    }

    async fn update_owner_organization_handle(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        id: OrganizationId,
        handle: OrganizationHandle,
    ) -> Result<(), PublicAccountListWriterError> {
        sqlx::query(
            r#"
            UPDATE public_account_list_item_owner_organizations
               SET handle = $2, updated_at = $3,
                   updated_event_sequence = $4,
                   updated_event_id = $5
             WHERE id = $1 AND updated_event_sequence < $4
            "#,
        )
        .bind(id.value())
        .bind(handle.value())
        .bind(event_context.occurred_at.value())
        .bind(event_context.event_sequence.value())
        .bind(event_context.event_id.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|e| PublicAccountListWriterError::Persistence(Box::new(e)))?;
        Ok(())
    }

    async fn update_owner_organization_display_name(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        id: OrganizationId,
        display_name: OrganizationDisplayName,
    ) -> Result<(), PublicAccountListWriterError> {
        sqlx::query(
            r#"
            UPDATE public_account_list_item_owner_organizations
               SET display_name = $2, updated_at = $3,
                   updated_event_sequence = $4,
                   updated_event_id = $5
             WHERE id = $1 AND updated_event_sequence < $4
            "#,
        )
        .bind(id.value())
        .bind(display_name.value())
        .bind(event_context.occurred_at.value())
        .bind(event_context.event_sequence.value())
        .bind(event_context.event_id.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|e| PublicAccountListWriterError::Persistence(Box::new(e)))?;
        Ok(())
    }

    async fn update_owner_organization_picture(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        id: OrganizationId,
        picture: Option<OrganizationPictureRef>,
    ) -> Result<(), PublicAccountListWriterError> {
        let (picture_type, object_name, external_url) =
            PgOrganizationPictureRefColumns::from_picture(picture.as_ref());

        sqlx::query(
            r#"
            UPDATE public_account_list_item_owner_organizations
               SET picture_type = $2,
                   picture_object_name = $3,
                   picture_external_url = $4,
                   updated_at = $5,
                   updated_event_sequence = $6,
                   updated_event_id = $7
             WHERE id = $1 AND updated_event_sequence < $6
            "#,
        )
        .bind(id.value())
        .bind(picture_type)
        .bind(object_name)
        .bind(external_url)
        .bind(event_context.occurred_at.value())
        .bind(event_context.event_sequence.value())
        .bind(event_context.event_id.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|e| PublicAccountListWriterError::Persistence(Box::new(e)))?;
        Ok(())
    }

    async fn delete_owner_organization(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        id: OrganizationId,
    ) -> Result<(), PublicAccountListWriterError> {
        sqlx::query(
            r#"
            DELETE FROM public_account_list_items
             WHERE owner_type = 'organization'
               AND owner_id = $1
               AND updated_event_sequence < $2
            "#,
        )
        .bind(id.value())
        .bind(event_context.event_sequence.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|e| PublicAccountListWriterError::Persistence(Box::new(e)))?;

        sqlx::query(
            r#"
            DELETE FROM public_account_list_item_owner_organizations
             WHERE id = $1 AND updated_event_sequence < $2
            "#,
        )
        .bind(id.value())
        .bind(event_context.event_sequence.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|e| PublicAccountListWriterError::Persistence(Box::new(e)))?;
        Ok(())
    }
}
