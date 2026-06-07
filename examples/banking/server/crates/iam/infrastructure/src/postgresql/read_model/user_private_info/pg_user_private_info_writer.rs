use appletheia::domain::AggregateId;
use appletheia::infrastructure::postgresql::PgUnitOfWork;
use banking_iam_application::{
    ReadModelEventContext, UserPrivateInfoIdentityUpsert,
    UserPrivateInfoOrganizationMembershipUpsert, UserPrivateInfoOrganizationUpsert,
    UserPrivateInfoStatus, UserPrivateInfoUserUpsert, UserPrivateInfoWriter,
    UserPrivateInfoWriterError,
};
use banking_iam_domain::{
    OrganizationDisplayName, OrganizationHandle, OrganizationId, OrganizationPictureRef,
    OrganizationRoles, UserBio, UserDisplayName, UserId, UserIdentityProvider, UserIdentitySubject,
    UserPictureRef, Username, core::Email,
};

use super::super::pg_organization_picture_ref_columns::PgOrganizationPictureRefColumns;
use super::super::pg_user_picture_ref_columns::PgUserPictureRefColumns;

/// PostgreSQL-backed user-private information writer.
pub struct PgUserPrivateInfoWriter;

impl PgUserPrivateInfoWriter {
    pub fn new() -> Self {
        Self
    }

    fn status_name(status: UserPrivateInfoStatus) -> &'static str {
        match status {
            UserPrivateInfoStatus::Active => "active",
            UserPrivateInfoStatus::Inactive => "inactive",
        }
    }

    fn roles_json(roles: &OrganizationRoles) -> Result<String, UserPrivateInfoWriterError> {
        serde_json::to_string(roles)
            .map_err(|e| UserPrivateInfoWriterError::Persistence(Box::new(e)))
    }
}

impl Default for PgUserPrivateInfoWriter {
    fn default() -> Self {
        Self::new()
    }
}

impl UserPrivateInfoWriter for PgUserPrivateInfoWriter {
    type Uow = PgUnitOfWork;

    async fn upsert_user(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        upsert: UserPrivateInfoUserUpsert,
    ) -> Result<(), UserPrivateInfoWriterError> {
        let (picture_type, object_name, external_url) =
            PgUserPictureRefColumns::from_picture(upsert.picture.as_ref());

        sqlx::query(
            r#"
            INSERT INTO user_private_infos (
                id, username, display_name, bio, picture_type, picture_object_name,
                picture_external_url, status, updated_at, created_at, source_event_sequence, updated_event_sequence, source_event_id, updated_event_id
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $11, $12, $12)
            ON CONFLICT (id) DO UPDATE SET
                username = EXCLUDED.username,
                display_name = EXCLUDED.display_name,
                bio = EXCLUDED.bio,
                picture_type = EXCLUDED.picture_type,
                picture_object_name = EXCLUDED.picture_object_name,
                picture_external_url = EXCLUDED.picture_external_url,
                status = EXCLUDED.status,
                updated_at = EXCLUDED.updated_at,
                updated_event_id = EXCLUDED.updated_event_id,
                updated_event_sequence = EXCLUDED.updated_event_sequence
            WHERE user_private_infos.updated_event_sequence < EXCLUDED.updated_event_sequence
            "#,
        )
        .bind(upsert.id.value())
        .bind(upsert.username.as_ref().map(Username::value))
        .bind(upsert.display_name.as_ref().map(UserDisplayName::value))
        .bind(upsert.bio.as_ref().map(UserBio::value))
        .bind(picture_type)
        .bind(object_name)
        .bind(external_url)
        .bind(Self::status_name(upsert.status))
        .bind(event_context.occurred_at.value())
        .bind(event_context.occurred_at.value())
        .bind(event_context.event_sequence.value())
        .bind(event_context.event_id.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|e| UserPrivateInfoWriterError::Persistence(Box::new(e)))?;

        Ok(())
    }

    async fn upsert_identity(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        upsert: UserPrivateInfoIdentityUpsert,
    ) -> Result<(), UserPrivateInfoWriterError> {
        sqlx::query(
            r#"
            INSERT INTO user_private_info_identities (
                user_id, provider, subject, email, updated_at, created_at, source_event_sequence, updated_event_sequence, source_event_id, updated_event_id
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $7, $8, $8)
            ON CONFLICT (user_id, provider, subject) DO UPDATE SET
                email = EXCLUDED.email,
                updated_at = EXCLUDED.updated_at,
                updated_event_id = EXCLUDED.updated_event_id,
                updated_event_sequence = EXCLUDED.updated_event_sequence
            WHERE user_private_info_identities.updated_event_sequence < EXCLUDED.updated_event_sequence
            "#,
        )
        .bind(upsert.user_id.value())
        .bind(upsert.provider.value())
        .bind(upsert.subject.value())
        .bind(upsert.email.as_ref().map(Email::value))
        .bind(event_context.occurred_at.value())
        .bind(event_context.occurred_at.value())
        .bind(event_context.event_sequence.value())
        .bind(event_context.event_id.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|e| UserPrivateInfoWriterError::Persistence(Box::new(e)))?;

        Ok(())
    }

    async fn update_identity_email(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        user_id: UserId,
        provider: UserIdentityProvider,
        subject: UserIdentitySubject,
        email: Option<Email>,
    ) -> Result<(), UserPrivateInfoWriterError> {
        sqlx::query(
            r#"
            UPDATE user_private_info_identities
               SET email = $4, updated_at = $5,
                   updated_event_sequence = $6,
                   updated_event_id = $7
             WHERE user_id = $1
               AND provider = $2
               AND subject = $3
               AND updated_event_sequence < $6
            "#,
        )
        .bind(user_id.value())
        .bind(provider.value())
        .bind(subject.value())
        .bind(email.as_ref().map(Email::value))
        .bind(event_context.occurred_at.value())
        .bind(event_context.event_sequence.value())
        .bind(event_context.event_id.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|e| UserPrivateInfoWriterError::Persistence(Box::new(e)))?;

        Ok(())
    }

    async fn upsert_organization_membership(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        upsert: UserPrivateInfoOrganizationMembershipUpsert,
    ) -> Result<(), UserPrivateInfoWriterError> {
        let roles_json = Self::roles_json(&upsert.roles)?;

        sqlx::query(
            r#"
            INSERT INTO user_private_info_organization_memberships (
                user_id, organization_id, roles, updated_at, created_at,
                source_event_sequence, updated_event_sequence, source_event_id, updated_event_id
            )
            VALUES ($1, $2, $3::jsonb, $4, $5, $6, $6, $7, $7)
            ON CONFLICT (user_id, organization_id) DO UPDATE SET
                roles = EXCLUDED.roles,
                updated_at = EXCLUDED.updated_at,
                updated_event_id = EXCLUDED.updated_event_id,
                updated_event_sequence = EXCLUDED.updated_event_sequence
            WHERE user_private_info_organization_memberships.updated_event_sequence < EXCLUDED.updated_event_sequence
            "#,
        )
        .bind(upsert.user_id.value())
        .bind(upsert.organization_id.value())
        .bind(roles_json)
        .bind(event_context.occurred_at.value())
        .bind(event_context.occurred_at.value())
        .bind(event_context.event_sequence.value())
        .bind(event_context.event_id.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|e| UserPrivateInfoWriterError::Persistence(Box::new(e)))?;

        Ok(())
    }

    async fn update_organization_membership_roles(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        user_id: UserId,
        organization_id: OrganizationId,
        roles: OrganizationRoles,
    ) -> Result<(), UserPrivateInfoWriterError> {
        let roles_json = Self::roles_json(&roles)?;

        sqlx::query(
            r#"
            UPDATE user_private_info_organization_memberships
               SET roles = $3::jsonb,
                   updated_at = $4,
                   updated_event_sequence = $5,
                   updated_event_id = $6
             WHERE user_id = $1
               AND organization_id = $2
               AND updated_event_sequence < $5
            "#,
        )
        .bind(user_id.value())
        .bind(organization_id.value())
        .bind(roles_json)
        .bind(event_context.occurred_at.value())
        .bind(event_context.event_sequence.value())
        .bind(event_context.event_id.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|e| UserPrivateInfoWriterError::Persistence(Box::new(e)))?;

        Ok(())
    }

    async fn delete_organization_membership(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        user_id: UserId,
        organization_id: OrganizationId,
    ) -> Result<(), UserPrivateInfoWriterError> {
        sqlx::query(
            r#"
            DELETE FROM user_private_info_organization_memberships
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
        .map_err(|e| UserPrivateInfoWriterError::Persistence(Box::new(e)))?;

        Ok(())
    }

    async fn update_username(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        id: UserId,
        username: Username,
    ) -> Result<(), UserPrivateInfoWriterError> {
        sqlx::query(
            r#"
            UPDATE user_private_infos
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
        .map_err(|e| UserPrivateInfoWriterError::Persistence(Box::new(e)))?;

        Ok(())
    }

    async fn update_display_name(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        id: UserId,
        display_name: UserDisplayName,
    ) -> Result<(), UserPrivateInfoWriterError> {
        sqlx::query(
            r#"
            UPDATE user_private_infos
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
        .map_err(|e| UserPrivateInfoWriterError::Persistence(Box::new(e)))?;

        Ok(())
    }

    async fn update_bio(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        id: UserId,
        bio: Option<UserBio>,
    ) -> Result<(), UserPrivateInfoWriterError> {
        sqlx::query(
            r#"
            UPDATE user_private_infos
               SET bio = $2, updated_at = $3,
                   updated_event_sequence = $4,
                   updated_event_id = $5
             WHERE id = $1 AND updated_event_sequence < $4
            "#,
        )
        .bind(id.value())
        .bind(bio.as_ref().map(UserBio::value))
        .bind(event_context.occurred_at.value())
        .bind(event_context.event_sequence.value())
        .bind(event_context.event_id.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|e| UserPrivateInfoWriterError::Persistence(Box::new(e)))?;

        Ok(())
    }

    async fn update_picture(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        id: UserId,
        picture: Option<UserPictureRef>,
    ) -> Result<(), UserPrivateInfoWriterError> {
        let (picture_type, object_name, external_url) =
            PgUserPictureRefColumns::from_picture(picture.as_ref());

        sqlx::query(
            r#"
            UPDATE user_private_infos
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
        .map_err(|e| UserPrivateInfoWriterError::Persistence(Box::new(e)))?;

        Ok(())
    }

    async fn update_status(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        id: UserId,
        status: UserPrivateInfoStatus,
    ) -> Result<(), UserPrivateInfoWriterError> {
        sqlx::query(
            r#"
            UPDATE user_private_infos
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
        .map_err(|e| UserPrivateInfoWriterError::Persistence(Box::new(e)))?;

        Ok(())
    }

    async fn delete_user(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        id: UserId,
    ) -> Result<(), UserPrivateInfoWriterError> {
        sqlx::query(
            r#"
            DELETE FROM user_private_info_organization_memberships
             WHERE user_id = $1 AND updated_event_sequence < $2
            "#,
        )
        .bind(id.value())
        .bind(event_context.event_sequence.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|e| UserPrivateInfoWriterError::Persistence(Box::new(e)))?;

        sqlx::query(
            r#"
            DELETE FROM user_private_info_identities
             WHERE user_id = $1 AND updated_event_sequence < $2
            "#,
        )
        .bind(id.value())
        .bind(event_context.event_sequence.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|e| UserPrivateInfoWriterError::Persistence(Box::new(e)))?;

        sqlx::query(
            r#"
            DELETE FROM user_private_infos
             WHERE id = $1 AND updated_event_sequence < $2
            "#,
        )
        .bind(id.value())
        .bind(event_context.event_sequence.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|e| UserPrivateInfoWriterError::Persistence(Box::new(e)))?;

        Ok(())
    }

    async fn upsert_organization(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        upsert: UserPrivateInfoOrganizationUpsert,
    ) -> Result<(), UserPrivateInfoWriterError> {
        let (picture_type, object_name, external_url) =
            PgOrganizationPictureRefColumns::from_picture(upsert.picture.as_ref());

        sqlx::query(
            r#"
            INSERT INTO user_private_info_organizations (
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
            WHERE user_private_info_organizations.updated_event_sequence < EXCLUDED.updated_event_sequence
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
        .map_err(|e| UserPrivateInfoWriterError::Persistence(Box::new(e)))?;

        Ok(())
    }

    async fn update_organization_handle(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        id: OrganizationId,
        handle: OrganizationHandle,
    ) -> Result<(), UserPrivateInfoWriterError> {
        sqlx::query(
            r#"
            UPDATE user_private_info_organizations
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
        .map_err(|e| UserPrivateInfoWriterError::Persistence(Box::new(e)))?;

        Ok(())
    }

    async fn update_organization_display_name(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        id: OrganizationId,
        display_name: OrganizationDisplayName,
    ) -> Result<(), UserPrivateInfoWriterError> {
        sqlx::query(
            r#"
            UPDATE user_private_info_organizations
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
        .map_err(|e| UserPrivateInfoWriterError::Persistence(Box::new(e)))?;

        Ok(())
    }

    async fn update_organization_picture(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        id: OrganizationId,
        picture: Option<OrganizationPictureRef>,
    ) -> Result<(), UserPrivateInfoWriterError> {
        let (picture_type, object_name, external_url) =
            PgOrganizationPictureRefColumns::from_picture(picture.as_ref());

        sqlx::query(
            r#"
            UPDATE user_private_info_organizations
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
        .map_err(|e| UserPrivateInfoWriterError::Persistence(Box::new(e)))?;

        Ok(())
    }

    async fn delete_organization(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        id: OrganizationId,
    ) -> Result<(), UserPrivateInfoWriterError> {
        sqlx::query(
            r#"
            DELETE FROM user_private_info_organization_memberships
             WHERE organization_id = $1 AND updated_event_sequence < $2
            "#,
        )
        .bind(id.value())
        .bind(event_context.event_sequence.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|e| UserPrivateInfoWriterError::Persistence(Box::new(e)))?;

        sqlx::query(
            r#"
            DELETE FROM user_private_info_organizations
             WHERE id = $1 AND updated_event_sequence < $2
            "#,
        )
        .bind(id.value())
        .bind(event_context.event_sequence.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|e| UserPrivateInfoWriterError::Persistence(Box::new(e)))?;

        Ok(())
    }
}
