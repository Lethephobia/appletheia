use appletheia::application::read_model::MaterializationEventContext;
use appletheia::domain::AggregateId;
use appletheia::infrastructure::postgresql::PgUnitOfWork;
use banking_iam_application::{
    OrganizationMembershipFragment, OrganizationMembershipFragmentKey,
    OrganizationMembershipFragmentUpsert, OrganizationMembershipFragmentWriter,
    OrganizationMembershipFragmentWriterError,
};
use banking_iam_domain::{OrganizationId, OrganizationRoles, UserId};

mod pg_organization_membership_fragment_row;

use pg_organization_membership_fragment_row::PgOrganizationMembershipFragmentRow;

/// PostgreSQL-backed organization membership fragment writer.
pub struct PgOrganizationMembershipFragmentWriter;

impl PgOrganizationMembershipFragmentWriter {
    pub fn new() -> Self {
        Self
    }

    fn roles_json(
        roles: &OrganizationRoles,
    ) -> Result<String, OrganizationMembershipFragmentWriterError> {
        serde_json::to_string(roles).map_err(|error| {
            OrganizationMembershipFragmentWriterError::Persistence(Box::new(error))
        })
    }

    fn map_row(
        row: Option<PgOrganizationMembershipFragmentRow>,
    ) -> Result<Option<OrganizationMembershipFragment>, OrganizationMembershipFragmentWriterError>
    {
        let Some(membership_row) = row else {
            return Ok(None);
        };
        OrganizationMembershipFragment::try_from(membership_row).map(Some)
    }
}

impl Default for PgOrganizationMembershipFragmentWriter {
    fn default() -> Self {
        Self::new()
    }
}

impl OrganizationMembershipFragmentWriter for PgOrganizationMembershipFragmentWriter {
    type Uow = PgUnitOfWork;

    async fn upsert(
        &self,
        uow: &mut Self::Uow,
        event_context: MaterializationEventContext,
        upsert: OrganizationMembershipFragmentUpsert,
    ) -> Result<Option<OrganizationMembershipFragment>, OrganizationMembershipFragmentWriterError>
    {
        let roles_json = Self::roles_json(&upsert.roles)?;

        let row = sqlx::query_as::<_, PgOrganizationMembershipFragmentRow>(
            r#"
            INSERT INTO organization_membership_fragments (
                organization_membership_id, user_id, organization_id, roles, updated_at,
                created_at, source_event_sequence, updated_event_sequence, source_event_id,
                updated_event_id
            )
            VALUES ($7, $1, $2, $3::jsonb, $4, $4, $5, $5, $6, $6)
            ON CONFLICT (user_id, organization_id) DO UPDATE SET
                organization_membership_id = EXCLUDED.organization_membership_id,
                roles = EXCLUDED.roles,
                updated_at = EXCLUDED.updated_at,
                updated_event_id = EXCLUDED.updated_event_id,
                updated_event_sequence = EXCLUDED.updated_event_sequence
            WHERE organization_membership_fragments.updated_event_sequence < EXCLUDED.updated_event_sequence
            RETURNING organization_membership_id, user_id, organization_id, roles, created_at,
                      source_event_id, updated_event_id
            "#,
        )
        .bind(upsert.user_id.value())
        .bind(upsert.organization_id.value())
        .bind(roles_json)
        .bind(event_context.occurred_at.value())
        .bind(event_context.event_sequence.value())
        .bind(event_context.event_id.value())
        .bind(upsert.organization_membership_id.value())
        .fetch_optional(uow.transaction_mut().as_mut())
        .await
        .map_err(|error| {
            OrganizationMembershipFragmentWriterError::Persistence(Box::new(error))
        })?;

        Self::map_row(row)
    }

    async fn update_roles(
        &self,
        uow: &mut Self::Uow,
        event_context: MaterializationEventContext,
        user_id: UserId,
        organization_id: OrganizationId,
        roles: OrganizationRoles,
    ) -> Result<Option<OrganizationMembershipFragment>, OrganizationMembershipFragmentWriterError>
    {
        let roles_json = Self::roles_json(&roles)?;

        let row = sqlx::query_as::<_, PgOrganizationMembershipFragmentRow>(
            r#"
            UPDATE organization_membership_fragments
               SET roles = $3::jsonb, updated_at = $4,
                   updated_event_sequence = $5, updated_event_id = $6
             WHERE user_id = $1
               AND organization_id = $2
               AND updated_event_sequence < $5
            RETURNING organization_membership_id, user_id, organization_id, roles, created_at,
                      source_event_id, updated_event_id
            "#,
        )
        .bind(user_id.value())
        .bind(organization_id.value())
        .bind(roles_json)
        .bind(event_context.occurred_at.value())
        .bind(event_context.event_sequence.value())
        .bind(event_context.event_id.value())
        .fetch_optional(uow.transaction_mut().as_mut())
        .await
        .map_err(|error| OrganizationMembershipFragmentWriterError::Persistence(Box::new(error)))?;

        Self::map_row(row)
    }

    async fn delete(
        &self,
        uow: &mut Self::Uow,
        event_context: MaterializationEventContext,
        user_id: UserId,
        organization_id: OrganizationId,
    ) -> Result<bool, OrganizationMembershipFragmentWriterError> {
        let result = sqlx::query(
            r#"
            DELETE FROM organization_membership_fragments
             WHERE user_id = $1
               AND organization_id = $2
               AND updated_event_sequence < $3
            "#,
        )
        .bind(user_id.value())
        .bind(organization_id.value())
        .bind(event_context.event_sequence.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|error| OrganizationMembershipFragmentWriterError::Persistence(Box::new(error)))?;

        Ok(result.rows_affected() > 0)
    }

    async fn delete_for_user(
        &self,
        uow: &mut Self::Uow,
        event_context: MaterializationEventContext,
        user_id: UserId,
    ) -> Result<Vec<OrganizationMembershipFragmentKey>, OrganizationMembershipFragmentWriterError>
    {
        let rows = sqlx::query_as::<_, (uuid::Uuid, uuid::Uuid)>(
            "DELETE FROM organization_membership_fragments WHERE user_id = $1 AND updated_event_sequence < $2 RETURNING user_id, organization_id",
        )
        .bind(user_id.value())
        .bind(event_context.event_sequence.value())
        .fetch_all(uow.transaction_mut().as_mut())
        .await
        .map_err(|error| {
            OrganizationMembershipFragmentWriterError::Persistence(Box::new(error))
        })?;

        rows.into_iter()
            .map(|(removed_user_id, removed_organization_id)| {
                Ok(OrganizationMembershipFragmentKey {
                    user_id: UserId::try_from_uuid(removed_user_id).map_err(|error| {
                        OrganizationMembershipFragmentWriterError::Persistence(Box::new(error))
                    })?,
                    organization_id: OrganizationId::try_from_uuid(removed_organization_id)
                        .map_err(|error| {
                            OrganizationMembershipFragmentWriterError::Persistence(Box::new(error))
                        })?,
                })
            })
            .collect()
    }
}
