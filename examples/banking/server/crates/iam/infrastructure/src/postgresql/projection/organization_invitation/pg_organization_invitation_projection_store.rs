use appletheia::application::event::EventSequence;
use appletheia::domain::AggregateId;
use appletheia::domain::EventOccurredAt;
use appletheia::infrastructure::postgresql::PgUnitOfWork;
use banking_iam_application::{
    OrganizationInvitationProjectionStore, OrganizationInvitationProjectionStoreError,
    OrganizationInvitationProjectionUpsert,
};
use banking_iam_domain::{
    OrganizationInvitationId, OrganizationInvitationIssuer, OrganizationInvitationStatus,
};

/// PostgreSQL-backed organization invitation projection store.
pub struct PgOrganizationInvitationProjectionStore;

impl PgOrganizationInvitationProjectionStore {
    pub fn new() -> Self {
        Self
    }

    fn issuer_parts(issuer: OrganizationInvitationIssuer) -> (&'static str, Option<uuid::Uuid>) {
        match issuer {
            OrganizationInvitationIssuer::User(user_id) => ("user", Some(user_id.value())),
            OrganizationInvitationIssuer::System => ("system", None),
        }
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
}

impl Default for PgOrganizationInvitationProjectionStore {
    fn default() -> Self {
        Self::new()
    }
}

impl OrganizationInvitationProjectionStore for PgOrganizationInvitationProjectionStore {
    type Uow = PgUnitOfWork;

    async fn upsert(
        &self,
        uow: &mut Self::Uow,
        input: OrganizationInvitationProjectionUpsert,
        event_sequence: EventSequence,
        occurred_at: EventOccurredAt,
    ) -> Result<(), OrganizationInvitationProjectionStoreError> {
        let (issuer_type, issuer_id) = Self::issuer_parts(input.issuer);

        sqlx::query(
            r#"
            INSERT INTO organization_invitations (
                id,
                organization_id,
                invitee_id,
                issuer_type,
                issuer_id,
                expires_at,
                status,
                created_at, updated_at, updated_event_sequence
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            ON CONFLICT (id) DO UPDATE SET
                organization_id = EXCLUDED.organization_id,
                invitee_id = EXCLUDED.invitee_id,
                issuer_type = EXCLUDED.issuer_type,
                issuer_id = EXCLUDED.issuer_id,
                expires_at = EXCLUDED.expires_at,
                status = EXCLUDED.status,
                updated_at = EXCLUDED.updated_at,
                updated_event_sequence = EXCLUDED.updated_event_sequence
            WHERE organization_invitations.updated_event_sequence < EXCLUDED.updated_event_sequence
            "#,
        )
        .bind(input.id.value())
        .bind(input.organization_id.value())
        .bind(input.invitee_id.value())
        .bind(issuer_type)
        .bind(issuer_id)
        .bind(input.expires_at.value())
        .bind(Self::status_name(input.status))
        .bind(occurred_at.value())
        .bind(occurred_at.value())
        .bind(event_sequence.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|e| OrganizationInvitationProjectionStoreError::Persistence(Box::new(e)))?;

        Ok(())
    }

    async fn update_status(
        &self,
        uow: &mut Self::Uow,
        id: OrganizationInvitationId,
        status: OrganizationInvitationStatus,
        event_sequence: EventSequence,
        occurred_at: EventOccurredAt,
    ) -> Result<(), OrganizationInvitationProjectionStoreError> {
        sqlx::query(
            r#"
            UPDATE organization_invitations
               SET status = $2,
                   updated_at = $3,
                   updated_event_sequence = $4
             WHERE id = $1
               AND updated_event_sequence < $4
            "#,
        )
        .bind(id.value())
        .bind(Self::status_name(status))
        .bind(occurred_at.value())
        .bind(event_sequence.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|e| OrganizationInvitationProjectionStoreError::Persistence(Box::new(e)))?;

        Ok(())
    }
}
