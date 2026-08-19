use appletheia::application::read_model::MaterializationEventContext;
use appletheia::domain::AggregateId;
use appletheia::infrastructure::postgresql::PgUnitOfWork;
use banking_iam_application::{
    OrganizationInvitationFragment, OrganizationInvitationFragmentUpsert,
    OrganizationInvitationFragmentWriter, OrganizationInvitationFragmentWriterError,
};
use banking_iam_domain::{
    OrganizationInvitationId, OrganizationInvitationIssuer, OrganizationInvitationStatus,
    OrganizationRoles,
};
use uuid::Uuid;

mod pg_organization_invitation_fragment_row;

use pg_organization_invitation_fragment_row::PgOrganizationInvitationFragmentRow;

/// PostgreSQL-backed organization invitation fragment writer.
pub struct PgOrganizationInvitationFragmentWriter;

impl PgOrganizationInvitationFragmentWriter {
    pub fn new() -> Self {
        Self
    }

    fn status_name(status: OrganizationInvitationStatus) -> &'static str {
        match status {
            OrganizationInvitationStatus::Pending => "pending",
            OrganizationInvitationStatus::Accepted => "accepted",
            OrganizationInvitationStatus::Declined => "declined",
            OrganizationInvitationStatus::Canceled => "canceled",
            OrganizationInvitationStatus::Rejected => "rejected",
        }
    }

    fn issuer_columns(issuer: OrganizationInvitationIssuer) -> (&'static str, Option<Uuid>) {
        match issuer {
            OrganizationInvitationIssuer::User(user_id) => ("user", Some(user_id.value())),
            OrganizationInvitationIssuer::System => ("system", None),
        }
    }

    fn roles_json(
        roles: &OrganizationRoles,
    ) -> Result<String, OrganizationInvitationFragmentWriterError> {
        serde_json::to_string(roles).map_err(|error| {
            OrganizationInvitationFragmentWriterError::Persistence(Box::new(error))
        })
    }

    fn map_row(
        row: Option<PgOrganizationInvitationFragmentRow>,
    ) -> Result<Option<OrganizationInvitationFragment>, OrganizationInvitationFragmentWriterError>
    {
        let Some(invitation_row) = row else {
            return Ok(None);
        };
        OrganizationInvitationFragment::try_from(invitation_row).map(Some)
    }
}

impl Default for PgOrganizationInvitationFragmentWriter {
    fn default() -> Self {
        Self::new()
    }
}

impl OrganizationInvitationFragmentWriter for PgOrganizationInvitationFragmentWriter {
    type Uow = PgUnitOfWork;

    async fn upsert(
        &self,
        uow: &mut Self::Uow,
        event_context: MaterializationEventContext,
        upsert: OrganizationInvitationFragmentUpsert,
    ) -> Result<Option<OrganizationInvitationFragment>, OrganizationInvitationFragmentWriterError>
    {
        let roles_json = Self::roles_json(&upsert.roles)?;
        let (issuer_type, issuer_user_id) = Self::issuer_columns(upsert.issuer);

        let row = sqlx::query_as::<_, PgOrganizationInvitationFragmentRow>(
            r#"
            INSERT INTO organization_invitation_fragments (
                id, organization_id, invitee_user_id, roles, issuer_type, issuer_user_id,
                expires_at, status, updated_at, created_at, source_event_sequence,
                updated_event_sequence, source_event_id, updated_event_id
            )
            VALUES ($1, $2, $3, $4::jsonb, $5, $6, $7, $8, $9, $9, $10, $10, $11, $11)
            ON CONFLICT (id) DO UPDATE SET
                organization_id = EXCLUDED.organization_id,
                invitee_user_id = EXCLUDED.invitee_user_id,
                roles = EXCLUDED.roles,
                issuer_type = EXCLUDED.issuer_type,
                issuer_user_id = EXCLUDED.issuer_user_id,
                expires_at = EXCLUDED.expires_at,
                status = EXCLUDED.status,
                updated_at = EXCLUDED.updated_at,
                updated_event_sequence = EXCLUDED.updated_event_sequence,
                updated_event_id = EXCLUDED.updated_event_id
            WHERE organization_invitation_fragments.updated_event_sequence < EXCLUDED.updated_event_sequence
            RETURNING id AS invitation_id, organization_id, invitee_user_id,
                      roles::text AS roles, issuer_type, issuer_user_id, expires_at,
                      status, created_at, source_event_id, updated_event_id
            "#,
        )
        .bind(upsert.invitation_id.value())
        .bind(upsert.organization_id.value())
        .bind(upsert.invitee_user_id.value())
        .bind(roles_json)
        .bind(issuer_type)
        .bind(issuer_user_id)
        .bind(upsert.expires_at.value())
        .bind(Self::status_name(upsert.status))
        .bind(event_context.occurred_at.value())
        .bind(event_context.event_sequence.value())
        .bind(event_context.event_id.value())
        .fetch_optional(uow.transaction_mut().as_mut())
        .await
        .map_err(|error| {
            OrganizationInvitationFragmentWriterError::Persistence(Box::new(error))
        })?;

        Self::map_row(row)
    }

    async fn update_status(
        &self,
        uow: &mut Self::Uow,
        event_context: MaterializationEventContext,
        invitation_id: OrganizationInvitationId,
        status: OrganizationInvitationStatus,
    ) -> Result<Option<OrganizationInvitationFragment>, OrganizationInvitationFragmentWriterError>
    {
        let row = sqlx::query_as::<_, PgOrganizationInvitationFragmentRow>(
            r#"
            UPDATE organization_invitation_fragments
               SET status = $2, updated_at = $3, updated_event_sequence = $4,
                   updated_event_id = $5
             WHERE id = $1 AND updated_event_sequence < $4
            RETURNING id AS invitation_id, organization_id, invitee_user_id,
                      roles::text AS roles, issuer_type, issuer_user_id, expires_at,
                      status, created_at, source_event_id, updated_event_id
            "#,
        )
        .bind(invitation_id.value())
        .bind(Self::status_name(status))
        .bind(event_context.occurred_at.value())
        .bind(event_context.event_sequence.value())
        .bind(event_context.event_id.value())
        .fetch_optional(uow.transaction_mut().as_mut())
        .await
        .map_err(|error| OrganizationInvitationFragmentWriterError::Persistence(Box::new(error)))?;

        Self::map_row(row)
    }
}
