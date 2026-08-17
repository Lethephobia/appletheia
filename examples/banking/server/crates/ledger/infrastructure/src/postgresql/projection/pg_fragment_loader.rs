use appletheia::application::read_model::ReadModelObservation;
use appletheia::domain::{AggregateId, EventId, EventOccurredAt};
use appletheia::infrastructure::postgresql::PgUnitOfWork;
use banking_iam_application::{MaterializedUserStatus, OrganizationFragment, UserFragment};
use banking_iam_domain::{
    OrganizationDescription, OrganizationDisplayName, OrganizationHandle, OrganizationId,
    OrganizationWebsiteUrl, UserBio, UserDisplayName, UserId, Username,
};
use banking_ledger_application::{AccountFragment, CurrencyFragment, FragmentOwner};
use banking_ledger_domain::account::AccountId;
use banking_ledger_domain::currency::CurrencyId;
use sqlx::types::chrono::{DateTime, Utc};
use uuid::Uuid;

use super::account_fragment::PgAccountFragmentRow;
use super::currency_fragment::PgCurrencyFragmentRow;
use crate::postgresql::read_model::{PgOrganizationPictureRefColumns, PgUserPictureRefColumns};

type LoadError = Box<dyn std::error::Error + Send + Sync>;

#[derive(Debug, sqlx::FromRow)]
struct PgUserFragmentDependencyRow {
    id: Uuid,
    username: Option<String>,
    display_name: Option<String>,
    bio: Option<String>,
    picture_type: Option<String>,
    picture_object_name: Option<String>,
    picture_external_url: Option<String>,
    status: String,
    created_at: DateTime<Utc>,
    source_event_id: Uuid,
    updated_event_id: Uuid,
}

#[derive(Debug, sqlx::FromRow)]
struct PgOrganizationFragmentDependencyRow {
    id: Uuid,
    owner_user_id: Uuid,
    owner_since: DateTime<Utc>,
    owner_source_event_id: Uuid,
    owner_updated_event_id: Uuid,
    handle: String,
    display_name: String,
    description: Option<String>,
    website_url: Option<String>,
    picture_type: Option<String>,
    picture_object_name: Option<String>,
    picture_external_url: Option<String>,
    created_at: DateTime<Utc>,
    source_event_id: Uuid,
    updated_event_id: Uuid,
}

/// Materializes complete Ledger fragment dependency graphs from normalized tables.
pub struct PgFragmentLoader;

impl PgFragmentLoader {
    pub async fn load_owner(
        uow: &mut PgUnitOfWork,
        owner_type: &str,
        owner_id: Uuid,
    ) -> Result<FragmentOwner, LoadError> {
        match owner_type {
            "user" => {
                let user_id = UserId::try_from_uuid(owner_id).map_err(boxed)?;
                Self::load_user(uow, user_id)
                    .await
                    .map(|user| FragmentOwner::User(Box::new(user)))
            }
            "organization" => {
                let organization_id = OrganizationId::try_from_uuid(owner_id).map_err(boxed)?;
                Self::load_organization(uow, organization_id)
                    .await
                    .map(|organization| FragmentOwner::Organization(Box::new(organization)))
            }
            _ => Err(message("unknown fragment owner type")),
        }
    }

    pub async fn load_currency(
        uow: &mut PgUnitOfWork,
        currency_id: CurrencyId,
    ) -> Result<CurrencyFragment, LoadError> {
        let row = sqlx::query_as::<_, PgCurrencyFragmentRow>(
            r#"
            SELECT
                id, owner_type, owner_id, symbol, name, decimals,
                description, image_type, image_object_name, image_external_url,
                mint_account_address, supply::text AS supply, status, created_at,
                source_event_id, updated_event_id
            FROM currency_fragments
            WHERE id = $1
            "#,
        )
        .bind(currency_id.value())
        .fetch_optional(uow.transaction_mut().as_mut())
        .await
        .map_err(boxed)?
        .ok_or_else(|| message("currency fragment dependency was not found"))?;
        let owner = Self::load_owner(uow, &row.owner_type, row.owner_id).await?;

        row.try_into_fragment(owner).map_err(boxed)
    }

    pub async fn load_account(
        uow: &mut PgUnitOfWork,
        account_id: AccountId,
    ) -> Result<AccountFragment, LoadError> {
        let row = sqlx::query_as::<_, PgAccountFragmentRow>(
            r#"
            SELECT
                id, owner_type, owner_id, name, currency_id,
                balance::text AS balance, reserved_balance::text AS reserved_balance,
                status, created_at, source_event_id, updated_event_id
            FROM account_fragments
            WHERE id = $1
            "#,
        )
        .bind(account_id.value())
        .fetch_optional(uow.transaction_mut().as_mut())
        .await
        .map_err(boxed)?
        .ok_or_else(|| message("account fragment dependency was not found"))?;
        let owner = Self::load_owner(uow, &row.owner_type, row.owner_id).await?;
        let currency_id = CurrencyId::try_from_uuid(row.currency_id).map_err(boxed)?;
        let currency = Self::load_currency(uow, currency_id).await?;

        row.try_into_fragment(owner, currency).map_err(boxed)
    }

    async fn load_user(uow: &mut PgUnitOfWork, user_id: UserId) -> Result<UserFragment, LoadError> {
        let row = sqlx::query_as::<_, PgUserFragmentDependencyRow>(
            r#"
            SELECT
                id, username, display_name, bio, picture_type,
                picture_object_name, picture_external_url, status, created_at,
                source_event_id, updated_event_id
            FROM user_fragments
            WHERE id = $1
            "#,
        )
        .bind(user_id.value())
        .fetch_optional(uow.transaction_mut().as_mut())
        .await
        .map_err(boxed)?
        .ok_or_else(|| message("user fragment dependency was not found"))?;
        let status = match row.status.as_str() {
            "active" => MaterializedUserStatus::Active,
            "inactive" => MaterializedUserStatus::Inactive,
            _ => return Err(message("unknown user fragment status")),
        };

        Ok(UserFragment {
            id: UserId::try_from_uuid(row.id).map_err(boxed)?,
            username: row
                .username
                .map(Username::try_from)
                .transpose()
                .map_err(boxed)?,
            display_name: row
                .display_name
                .map(UserDisplayName::try_from)
                .transpose()
                .map_err(boxed)?,
            bio: row.bio.map(UserBio::try_from).transpose().map_err(boxed)?,
            picture: PgUserPictureRefColumns {
                picture_type: row.picture_type,
                object_name: row.picture_object_name,
                external_url: row.picture_external_url,
            }
            .into_picture()
            .map_err(boxed)?,
            status,
            created_at: EventOccurredAt::from(row.created_at),
            observation: observation(row.source_event_id, row.updated_event_id)?,
        })
    }

    async fn load_organization(
        uow: &mut PgUnitOfWork,
        organization_id: OrganizationId,
    ) -> Result<OrganizationFragment, LoadError> {
        let row = sqlx::query_as::<_, PgOrganizationFragmentDependencyRow>(
            r#"
            SELECT
                id, owner_user_id, owner_since,
                owner_source_event_id, owner_updated_event_id, handle, display_name,
                description, website_url, picture_type, picture_object_name,
                picture_external_url, created_at, source_event_id, updated_event_id
            FROM organization_fragments
            WHERE id = $1
            "#,
        )
        .bind(organization_id.value())
        .fetch_optional(uow.transaction_mut().as_mut())
        .await
        .map_err(boxed)?
        .ok_or_else(|| message("organization fragment dependency was not found"))?;
        let owner_user_id = UserId::try_from_uuid(row.owner_user_id).map_err(boxed)?;
        let owner = Self::load_user(uow, owner_user_id).await?;

        Ok(OrganizationFragment {
            id: OrganizationId::try_from_uuid(row.id).map_err(boxed)?,
            owner,
            owner_since: EventOccurredAt::from(row.owner_since),
            owner_observation: observation(row.owner_source_event_id, row.owner_updated_event_id)?,
            handle: OrganizationHandle::try_from(row.handle).map_err(boxed)?,
            display_name: OrganizationDisplayName::try_from(row.display_name).map_err(boxed)?,
            description: row
                .description
                .map(OrganizationDescription::try_from)
                .transpose()
                .map_err(boxed)?,
            website_url: row
                .website_url
                .map(OrganizationWebsiteUrl::try_from)
                .transpose()
                .map_err(boxed)?,
            picture: PgOrganizationPictureRefColumns {
                picture_type: row.picture_type,
                object_name: row.picture_object_name,
                external_url: row.picture_external_url,
            }
            .into_picture()
            .map_err(boxed)?,
            created_at: EventOccurredAt::from(row.created_at),
            observation: observation(row.source_event_id, row.updated_event_id)?,
        })
    }
}

fn observation(
    source_event_id: Uuid,
    updated_event_id: Uuid,
) -> Result<ReadModelObservation, LoadError> {
    Ok(ReadModelObservation::new(
        EventId::try_from(source_event_id).map_err(boxed)?,
        EventId::try_from(updated_event_id).map_err(boxed)?,
    ))
}

fn boxed(error: impl std::error::Error + Send + Sync + 'static) -> LoadError {
    Box::new(error)
}

fn message(value: &'static str) -> LoadError {
    Box::new(std::io::Error::other(value))
}
