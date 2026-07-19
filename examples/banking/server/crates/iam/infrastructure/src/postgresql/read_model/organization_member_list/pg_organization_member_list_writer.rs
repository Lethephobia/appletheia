use appletheia::domain::AggregateId;
use appletheia::infrastructure::postgresql::PgUnitOfWork;
use banking_iam_application::{
    OrganizationMemberListMemberUpsert, OrganizationMemberListMembershipUpsert,
    OrganizationMemberListOrganizationUpsert, OrganizationMemberListWriter,
    OrganizationMemberListWriterError,
};
use banking_iam_domain::{
    OrganizationDisplayName, OrganizationHandle, OrganizationId, OrganizationPictureRef,
    OrganizationRoles, UserDisplayName, UserId, UserPictureRef, Username,
};
use banking_shared_kernel_application::read_model::ReadModelEventContext;

use super::super::{
    pg_organization_picture_ref_columns::PgOrganizationPictureRefColumns,
    pg_user_picture_ref_columns::PgUserPictureRefColumns,
};

/// PostgreSQL-backed organization member list writer.
pub struct PgOrganizationMemberListWriter;

impl PgOrganizationMemberListWriter {
    pub fn new() -> Self {
        Self
    }

    fn roles_json(roles: &OrganizationRoles) -> Result<String, OrganizationMemberListWriterError> {
        serde_json::to_string(roles)
            .map_err(|error| OrganizationMemberListWriterError::Persistence(Box::new(error)))
    }
}

impl Default for PgOrganizationMemberListWriter {
    fn default() -> Self {
        Self::new()
    }
}

impl OrganizationMemberListWriter for PgOrganizationMemberListWriter {
    type Uow = PgUnitOfWork;

    async fn upsert_organization(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        upsert: OrganizationMemberListOrganizationUpsert,
    ) -> Result<(), OrganizationMemberListWriterError> {
        let (picture_type, object_name, external_url) =
            PgOrganizationPictureRefColumns::from_picture(upsert.picture.as_ref());
        sqlx::query(
            r#"
            INSERT INTO organization_member_list_organizations (
                organization_id, owner_user_id, owner_since, owner_source_event_id,
                owner_updated_event_id, handle, display_name, picture_type,
                picture_object_name, picture_external_url, updated_at, created_at,
                source_event_sequence, updated_event_sequence, source_event_id,
                updated_event_id
            )
            VALUES ($1, $2, $8, $10, $10, $3, $4, $5, $6, $7, $8, $8, $9, $9, $10, $10)
            ON CONFLICT (organization_id) DO UPDATE SET
                owner_user_id = EXCLUDED.owner_user_id,
                owner_since = EXCLUDED.owner_since,
                owner_source_event_id = EXCLUDED.owner_source_event_id,
                owner_updated_event_id = EXCLUDED.owner_updated_event_id,
                handle = EXCLUDED.handle,
                display_name = EXCLUDED.display_name,
                picture_type = EXCLUDED.picture_type,
                picture_object_name = EXCLUDED.picture_object_name,
                picture_external_url = EXCLUDED.picture_external_url,
                updated_at = EXCLUDED.updated_at,
                updated_event_sequence = EXCLUDED.updated_event_sequence,
                updated_event_id = EXCLUDED.updated_event_id
            WHERE organization_member_list_organizations.updated_event_sequence < EXCLUDED.updated_event_sequence
            "#,
        )
        .bind(upsert.organization_id.value())
        .bind(upsert.owner_user_id.value())
        .bind(upsert.handle.value())
        .bind(upsert.display_name.value())
        .bind(picture_type)
        .bind(object_name)
        .bind(external_url)
        .bind(event_context.occurred_at.value())
        .bind(event_context.event_sequence.value())
        .bind(event_context.event_id.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|error| OrganizationMemberListWriterError::Persistence(Box::new(error)))?;
        Ok(())
    }

    async fn update_organization_owner(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        organization_id: OrganizationId,
        owner_user_id: UserId,
    ) -> Result<(), OrganizationMemberListWriterError> {
        sqlx::query(
            r#"
            UPDATE organization_member_list_organizations
               SET owner_user_id = $2, owner_since = $3, owner_source_event_id = $5,
                   owner_updated_event_id = $5, updated_at = $3,
                   updated_event_sequence = $4, updated_event_id = $5
             WHERE organization_id = $1 AND updated_event_sequence < $4
            "#,
        )
        .bind(organization_id.value())
        .bind(owner_user_id.value())
        .bind(event_context.occurred_at.value())
        .bind(event_context.event_sequence.value())
        .bind(event_context.event_id.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|error| OrganizationMemberListWriterError::Persistence(Box::new(error)))?;
        Ok(())
    }

    async fn update_organization_handle(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        organization_id: OrganizationId,
        handle: OrganizationHandle,
    ) -> Result<(), OrganizationMemberListWriterError> {
        sqlx::query(
            r#"
            UPDATE organization_member_list_organizations
               SET handle = $2, updated_at = $3, updated_event_sequence = $4,
                   updated_event_id = $5
             WHERE organization_id = $1 AND updated_event_sequence < $4
            "#,
        )
        .bind(organization_id.value())
        .bind(handle.value())
        .bind(event_context.occurred_at.value())
        .bind(event_context.event_sequence.value())
        .bind(event_context.event_id.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|error| OrganizationMemberListWriterError::Persistence(Box::new(error)))?;
        Ok(())
    }

    async fn update_organization_display_name(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        organization_id: OrganizationId,
        display_name: OrganizationDisplayName,
    ) -> Result<(), OrganizationMemberListWriterError> {
        sqlx::query(
            r#"
            UPDATE organization_member_list_organizations
               SET display_name = $2, updated_at = $3, updated_event_sequence = $4,
                   updated_event_id = $5
             WHERE organization_id = $1 AND updated_event_sequence < $4
            "#,
        )
        .bind(organization_id.value())
        .bind(display_name.value())
        .bind(event_context.occurred_at.value())
        .bind(event_context.event_sequence.value())
        .bind(event_context.event_id.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|error| OrganizationMemberListWriterError::Persistence(Box::new(error)))?;
        Ok(())
    }

    async fn update_organization_picture(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        organization_id: OrganizationId,
        picture: Option<OrganizationPictureRef>,
    ) -> Result<(), OrganizationMemberListWriterError> {
        let (picture_type, object_name, external_url) =
            PgOrganizationPictureRefColumns::from_picture(picture.as_ref());
        sqlx::query(
            r#"
            UPDATE organization_member_list_organizations
               SET picture_type = $2, picture_object_name = $3, picture_external_url = $4,
                   updated_at = $5, updated_event_sequence = $6, updated_event_id = $7
             WHERE organization_id = $1 AND updated_event_sequence < $6
            "#,
        )
        .bind(organization_id.value())
        .bind(picture_type)
        .bind(object_name)
        .bind(external_url)
        .bind(event_context.occurred_at.value())
        .bind(event_context.event_sequence.value())
        .bind(event_context.event_id.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|error| OrganizationMemberListWriterError::Persistence(Box::new(error)))?;
        Ok(())
    }

    async fn delete_organization_and_memberships(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        organization_id: OrganizationId,
    ) -> Result<(), OrganizationMemberListWriterError> {
        sqlx::query("DELETE FROM organization_member_list_memberships WHERE organization_id = $1")
            .bind(organization_id.value())
            .execute(uow.transaction_mut().as_mut())
            .await
            .map_err(|error| OrganizationMemberListWriterError::Persistence(Box::new(error)))?;
        sqlx::query(
            r#"
            DELETE FROM organization_member_list_organizations
             WHERE organization_id = $1 AND updated_event_sequence < $2
            "#,
        )
        .bind(organization_id.value())
        .bind(event_context.event_sequence.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|error| OrganizationMemberListWriterError::Persistence(Box::new(error)))?;
        Ok(())
    }

    async fn upsert_member(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        upsert: OrganizationMemberListMemberUpsert,
    ) -> Result<(), OrganizationMemberListWriterError> {
        let (picture_type, object_name, external_url) =
            PgUserPictureRefColumns::from_picture(upsert.picture.as_ref());
        sqlx::query(
            r#"
            INSERT INTO organization_member_list_users (
                user_id, username, display_name, picture_type, picture_object_name,
                picture_external_url, updated_at, created_at, source_event_sequence,
                updated_event_sequence, source_event_id, updated_event_id
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $7, $8, $8, $9, $9)
            ON CONFLICT (user_id) DO UPDATE SET
                username = EXCLUDED.username,
                display_name = EXCLUDED.display_name,
                picture_type = EXCLUDED.picture_type,
                picture_object_name = EXCLUDED.picture_object_name,
                picture_external_url = EXCLUDED.picture_external_url,
                updated_at = EXCLUDED.updated_at,
                updated_event_sequence = EXCLUDED.updated_event_sequence,
                updated_event_id = EXCLUDED.updated_event_id
            WHERE organization_member_list_users.updated_event_sequence < EXCLUDED.updated_event_sequence
            "#,
        )
        .bind(upsert.user_id.value())
        .bind(upsert.username.as_ref().map(Username::value))
        .bind(upsert.display_name.as_ref().map(UserDisplayName::value))
        .bind(picture_type)
        .bind(object_name)
        .bind(external_url)
        .bind(event_context.occurred_at.value())
        .bind(event_context.event_sequence.value())
        .bind(event_context.event_id.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|error| OrganizationMemberListWriterError::Persistence(Box::new(error)))?;
        Ok(())
    }

    async fn update_member_username(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        user_id: UserId,
        username: Username,
    ) -> Result<(), OrganizationMemberListWriterError> {
        sqlx::query(
            r#"
            UPDATE organization_member_list_users
               SET username = $2, updated_at = $3, updated_event_sequence = $4,
                   updated_event_id = $5
             WHERE user_id = $1 AND updated_event_sequence < $4
            "#,
        )
        .bind(user_id.value())
        .bind(username.value())
        .bind(event_context.occurred_at.value())
        .bind(event_context.event_sequence.value())
        .bind(event_context.event_id.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|error| OrganizationMemberListWriterError::Persistence(Box::new(error)))?;
        Ok(())
    }

    async fn update_member_display_name(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        user_id: UserId,
        display_name: UserDisplayName,
    ) -> Result<(), OrganizationMemberListWriterError> {
        sqlx::query(
            r#"
            UPDATE organization_member_list_users
               SET display_name = $2, updated_at = $3, updated_event_sequence = $4,
                   updated_event_id = $5
             WHERE user_id = $1 AND updated_event_sequence < $4
            "#,
        )
        .bind(user_id.value())
        .bind(display_name.value())
        .bind(event_context.occurred_at.value())
        .bind(event_context.event_sequence.value())
        .bind(event_context.event_id.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|error| OrganizationMemberListWriterError::Persistence(Box::new(error)))?;
        Ok(())
    }

    async fn update_member_picture(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        user_id: UserId,
        picture: Option<UserPictureRef>,
    ) -> Result<(), OrganizationMemberListWriterError> {
        let (picture_type, object_name, external_url) =
            PgUserPictureRefColumns::from_picture(picture.as_ref());
        sqlx::query(
            r#"
            UPDATE organization_member_list_users
               SET picture_type = $2, picture_object_name = $3, picture_external_url = $4,
                   updated_at = $5, updated_event_sequence = $6, updated_event_id = $7
             WHERE user_id = $1 AND updated_event_sequence < $6
            "#,
        )
        .bind(user_id.value())
        .bind(picture_type)
        .bind(object_name)
        .bind(external_url)
        .bind(event_context.occurred_at.value())
        .bind(event_context.event_sequence.value())
        .bind(event_context.event_id.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|error| OrganizationMemberListWriterError::Persistence(Box::new(error)))?;
        Ok(())
    }

    async fn delete_member_and_memberships(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        user_id: UserId,
    ) -> Result<(), OrganizationMemberListWriterError> {
        sqlx::query(
            r#"
            DELETE FROM organization_member_list_memberships
             WHERE user_id = $1 AND updated_event_sequence < $2
            "#,
        )
        .bind(user_id.value())
        .bind(event_context.event_sequence.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|error| OrganizationMemberListWriterError::Persistence(Box::new(error)))?;
        sqlx::query(
            r#"
            DELETE FROM organization_member_list_users
             WHERE user_id = $1 AND updated_event_sequence < $2
            "#,
        )
        .bind(user_id.value())
        .bind(event_context.event_sequence.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|error| OrganizationMemberListWriterError::Persistence(Box::new(error)))?;
        Ok(())
    }

    async fn upsert_membership(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        upsert: OrganizationMemberListMembershipUpsert,
    ) -> Result<(), OrganizationMemberListWriterError> {
        let roles_json = Self::roles_json(&upsert.roles)?;
        sqlx::query(
            r#"
            INSERT INTO organization_member_list_memberships (
                organization_id, user_id, roles, joined_at, updated_at,
                source_event_sequence, updated_event_sequence, source_event_id, updated_event_id
            )
            VALUES ($1, $2, $3::jsonb, $4, $4, $5, $5, $6, $6)
            ON CONFLICT (organization_id, user_id) DO UPDATE SET
                roles = EXCLUDED.roles,
                joined_at = EXCLUDED.joined_at,
                updated_at = EXCLUDED.updated_at,
                source_event_sequence = EXCLUDED.source_event_sequence,
                updated_event_sequence = EXCLUDED.updated_event_sequence,
                source_event_id = EXCLUDED.source_event_id,
                updated_event_id = EXCLUDED.updated_event_id
            WHERE organization_member_list_memberships.updated_event_sequence < EXCLUDED.updated_event_sequence
            "#,
        )
        .bind(upsert.organization_id.value())
        .bind(upsert.user_id.value())
        .bind(roles_json)
        .bind(event_context.occurred_at.value())
        .bind(event_context.event_sequence.value())
        .bind(event_context.event_id.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|error| OrganizationMemberListWriterError::Persistence(Box::new(error)))?;
        Ok(())
    }

    async fn update_membership_roles(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        organization_id: OrganizationId,
        user_id: UserId,
        roles: OrganizationRoles,
    ) -> Result<(), OrganizationMemberListWriterError> {
        let roles_json = Self::roles_json(&roles)?;
        sqlx::query(
            r#"
            UPDATE organization_member_list_memberships
               SET roles = $3::jsonb, updated_at = $4, updated_event_sequence = $5,
                   updated_event_id = $6
             WHERE organization_id = $1 AND user_id = $2 AND updated_event_sequence < $5
            "#,
        )
        .bind(organization_id.value())
        .bind(user_id.value())
        .bind(roles_json)
        .bind(event_context.occurred_at.value())
        .bind(event_context.event_sequence.value())
        .bind(event_context.event_id.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|error| OrganizationMemberListWriterError::Persistence(Box::new(error)))?;
        Ok(())
    }

    async fn delete_membership(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        organization_id: OrganizationId,
        user_id: UserId,
    ) -> Result<(), OrganizationMemberListWriterError> {
        sqlx::query(
            r#"
            DELETE FROM organization_member_list_memberships
             WHERE organization_id = $1 AND user_id = $2 AND updated_event_sequence < $3
            "#,
        )
        .bind(organization_id.value())
        .bind(user_id.value())
        .bind(event_context.event_sequence.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|error| OrganizationMemberListWriterError::Persistence(Box::new(error)))?;
        Ok(())
    }
}
