use appletheia::domain::AggregateId;
use appletheia::infrastructure::postgresql::PgUnitOfWork;
use banking_iam_application::{
    UserOrganizationJoinRequestListItemStatus, UserOrganizationJoinRequestListOrganizationUpsert,
    UserOrganizationJoinRequestListUpsert, UserOrganizationJoinRequestListUserUpsert,
    UserOrganizationJoinRequestListWriter, UserOrganizationJoinRequestListWriterError,
};
use banking_iam_domain::{
    OrganizationDisplayName, OrganizationHandle, OrganizationId, OrganizationJoinRequestId,
    OrganizationPictureRef, UserDisplayName, UserId, UserPictureRef, Username,
};
use banking_shared_kernel_application::read_model::ReadModelEventContext;

use super::super::{
    pg_organization_picture_ref_columns::PgOrganizationPictureRefColumns,
    pg_user_picture_ref_columns::PgUserPictureRefColumns,
};

/// PostgreSQL-backed user organization join request list writer.
pub struct PgUserOrganizationJoinRequestListWriter;

impl PgUserOrganizationJoinRequestListWriter {
    pub fn new() -> Self {
        Self
    }

    fn status_name(status: UserOrganizationJoinRequestListItemStatus) -> &'static str {
        match status {
            UserOrganizationJoinRequestListItemStatus::Pending => "pending",
            UserOrganizationJoinRequestListItemStatus::Approved => "approved",
            UserOrganizationJoinRequestListItemStatus::Rejected => "rejected",
            UserOrganizationJoinRequestListItemStatus::Canceled => "canceled",
        }
    }
}

impl Default for PgUserOrganizationJoinRequestListWriter {
    fn default() -> Self {
        Self::new()
    }
}

impl UserOrganizationJoinRequestListWriter for PgUserOrganizationJoinRequestListWriter {
    type Uow = PgUnitOfWork;

    async fn upsert_join_request(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        upsert: UserOrganizationJoinRequestListUpsert,
    ) -> Result<(), UserOrganizationJoinRequestListWriterError> {
        sqlx::query(
            r#"
            INSERT INTO user_organization_join_request_list_items (
                id, requester_user_id, organization_id, status, updated_at, created_at,
                source_event_sequence, updated_event_sequence, source_event_id, updated_event_id
            )
            VALUES ($1, $2, $3, $4, $5, $5, $6, $6, $7, $7)
            ON CONFLICT (id) DO UPDATE SET
                requester_user_id = EXCLUDED.requester_user_id,
                organization_id = EXCLUDED.organization_id,
                status = EXCLUDED.status,
                updated_at = EXCLUDED.updated_at,
                updated_event_sequence = EXCLUDED.updated_event_sequence,
                updated_event_id = EXCLUDED.updated_event_id
            WHERE user_organization_join_request_list_items.updated_event_sequence < EXCLUDED.updated_event_sequence
            "#,
        )
        .bind(upsert.join_request_id.value())
        .bind(upsert.requester_user_id.value())
        .bind(upsert.organization_id.value())
        .bind(Self::status_name(upsert.status))
        .bind(event_context.occurred_at.value())
        .bind(event_context.event_sequence.value())
        .bind(event_context.event_id.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|error| UserOrganizationJoinRequestListWriterError::Persistence(Box::new(error)))?;
        Ok(())
    }

    async fn update_status(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        join_request_id: OrganizationJoinRequestId,
        status: UserOrganizationJoinRequestListItemStatus,
    ) -> Result<(), UserOrganizationJoinRequestListWriterError> {
        sqlx::query(
            r#"UPDATE user_organization_join_request_list_items
               SET status = $2, updated_at = $3, updated_event_sequence = $4, updated_event_id = $5
             WHERE id = $1 AND updated_event_sequence < $4"#,
        )
        .bind(join_request_id.value())
        .bind(Self::status_name(status))
        .bind(event_context.occurred_at.value())
        .bind(event_context.event_sequence.value())
        .bind(event_context.event_id.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|error| {
            UserOrganizationJoinRequestListWriterError::Persistence(Box::new(error))
        })?;
        Ok(())
    }

    async fn upsert_user(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        upsert: UserOrganizationJoinRequestListUserUpsert,
    ) -> Result<(), UserOrganizationJoinRequestListWriterError> {
        let (picture_type, object_name, external_url) =
            PgUserPictureRefColumns::from_picture(upsert.picture.as_ref());
        sqlx::query(
            r#"
            INSERT INTO user_organization_join_request_list_users (
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
            WHERE user_organization_join_request_list_users.updated_event_sequence < EXCLUDED.updated_event_sequence
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
        .map_err(|error| UserOrganizationJoinRequestListWriterError::Persistence(Box::new(error)))?;
        Ok(())
    }

    async fn update_user_username(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        user_id: UserId,
        username: Username,
    ) -> Result<(), UserOrganizationJoinRequestListWriterError> {
        sqlx::query(
            r#"UPDATE user_organization_join_request_list_users
               SET username = $2, updated_at = $3, updated_event_sequence = $4, updated_event_id = $5
             WHERE user_id = $1 AND updated_event_sequence < $4"#,
        )
        .bind(user_id.value())
        .bind(username.value())
        .bind(event_context.occurred_at.value())
        .bind(event_context.event_sequence.value())
        .bind(event_context.event_id.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|error| UserOrganizationJoinRequestListWriterError::Persistence(Box::new(error)))?;
        Ok(())
    }

    async fn update_user_display_name(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        user_id: UserId,
        display_name: UserDisplayName,
    ) -> Result<(), UserOrganizationJoinRequestListWriterError> {
        sqlx::query(
            r#"UPDATE user_organization_join_request_list_users
               SET display_name = $2, updated_at = $3, updated_event_sequence = $4, updated_event_id = $5
             WHERE user_id = $1 AND updated_event_sequence < $4"#,
        )
        .bind(user_id.value())
        .bind(display_name.value())
        .bind(event_context.occurred_at.value())
        .bind(event_context.event_sequence.value())
        .bind(event_context.event_id.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|error| UserOrganizationJoinRequestListWriterError::Persistence(Box::new(error)))?;
        Ok(())
    }

    async fn update_user_picture(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        user_id: UserId,
        picture: Option<UserPictureRef>,
    ) -> Result<(), UserOrganizationJoinRequestListWriterError> {
        let (picture_type, object_name, external_url) =
            PgUserPictureRefColumns::from_picture(picture.as_ref());
        sqlx::query(
            r#"UPDATE user_organization_join_request_list_users
               SET picture_type = $2, picture_object_name = $3, picture_external_url = $4,
                   updated_at = $5, updated_event_sequence = $6, updated_event_id = $7
             WHERE user_id = $1 AND updated_event_sequence < $6"#,
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
        .map_err(|error| {
            UserOrganizationJoinRequestListWriterError::Persistence(Box::new(error))
        })?;
        Ok(())
    }

    async fn upsert_organization(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        upsert: UserOrganizationJoinRequestListOrganizationUpsert,
    ) -> Result<(), UserOrganizationJoinRequestListWriterError> {
        let (picture_type, object_name, external_url) =
            PgOrganizationPictureRefColumns::from_picture(upsert.picture.as_ref());
        sqlx::query(
            r#"
            INSERT INTO user_organization_join_request_list_organizations (
                organization_id, handle, display_name, picture_type, picture_object_name,
                picture_external_url, updated_at, created_at, source_event_sequence,
                updated_event_sequence, source_event_id, updated_event_id
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $7, $8, $8, $9, $9)
            ON CONFLICT (organization_id) DO UPDATE SET
                handle = EXCLUDED.handle,
                display_name = EXCLUDED.display_name,
                picture_type = EXCLUDED.picture_type,
                picture_object_name = EXCLUDED.picture_object_name,
                picture_external_url = EXCLUDED.picture_external_url,
                updated_at = EXCLUDED.updated_at,
                updated_event_sequence = EXCLUDED.updated_event_sequence,
                updated_event_id = EXCLUDED.updated_event_id
            WHERE user_organization_join_request_list_organizations.updated_event_sequence < EXCLUDED.updated_event_sequence
            "#,
        )
        .bind(upsert.organization_id.value())
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
        .map_err(|error| UserOrganizationJoinRequestListWriterError::Persistence(Box::new(error)))?;
        Ok(())
    }

    async fn update_organization_handle(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        organization_id: OrganizationId,
        handle: OrganizationHandle,
    ) -> Result<(), UserOrganizationJoinRequestListWriterError> {
        sqlx::query(
            r#"UPDATE user_organization_join_request_list_organizations
               SET handle = $2, updated_at = $3, updated_event_sequence = $4, updated_event_id = $5
             WHERE organization_id = $1 AND updated_event_sequence < $4"#,
        )
        .bind(organization_id.value())
        .bind(handle.value())
        .bind(event_context.occurred_at.value())
        .bind(event_context.event_sequence.value())
        .bind(event_context.event_id.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|error| {
            UserOrganizationJoinRequestListWriterError::Persistence(Box::new(error))
        })?;
        Ok(())
    }

    async fn update_organization_display_name(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        organization_id: OrganizationId,
        display_name: OrganizationDisplayName,
    ) -> Result<(), UserOrganizationJoinRequestListWriterError> {
        sqlx::query(
            r#"UPDATE user_organization_join_request_list_organizations
               SET display_name = $2, updated_at = $3, updated_event_sequence = $4, updated_event_id = $5
             WHERE organization_id = $1 AND updated_event_sequence < $4"#,
        )
        .bind(organization_id.value())
        .bind(display_name.value())
        .bind(event_context.occurred_at.value())
        .bind(event_context.event_sequence.value())
        .bind(event_context.event_id.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|error| UserOrganizationJoinRequestListWriterError::Persistence(Box::new(error)))?;
        Ok(())
    }

    async fn update_organization_picture(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        organization_id: OrganizationId,
        picture: Option<OrganizationPictureRef>,
    ) -> Result<(), UserOrganizationJoinRequestListWriterError> {
        let (picture_type, object_name, external_url) =
            PgOrganizationPictureRefColumns::from_picture(picture.as_ref());
        sqlx::query(
            r#"UPDATE user_organization_join_request_list_organizations
               SET picture_type = $2, picture_object_name = $3, picture_external_url = $4,
                   updated_at = $5, updated_event_sequence = $6, updated_event_id = $7
             WHERE organization_id = $1 AND updated_event_sequence < $6"#,
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
        .map_err(|error| {
            UserOrganizationJoinRequestListWriterError::Persistence(Box::new(error))
        })?;
        Ok(())
    }

    async fn delete_organization_and_join_requests(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        organization_id: OrganizationId,
    ) -> Result<(), UserOrganizationJoinRequestListWriterError> {
        sqlx::query(
            "DELETE FROM user_organization_join_request_list_items WHERE organization_id = $1",
        )
        .bind(organization_id.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|error| {
            UserOrganizationJoinRequestListWriterError::Persistence(Box::new(error))
        })?;
        sqlx::query(
            "DELETE FROM user_organization_join_request_list_organizations WHERE organization_id = $1 AND updated_event_sequence < $2",
        )
        .bind(organization_id.value())
        .bind(event_context.event_sequence.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|error| UserOrganizationJoinRequestListWriterError::Persistence(Box::new(error)))?;
        Ok(())
    }

    async fn delete_user_and_join_requests(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        user_id: UserId,
    ) -> Result<(), UserOrganizationJoinRequestListWriterError> {
        sqlx::query(
            "DELETE FROM user_organization_join_request_list_items WHERE requester_user_id = $1",
        )
        .bind(user_id.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|error| {
            UserOrganizationJoinRequestListWriterError::Persistence(Box::new(error))
        })?;
        sqlx::query(
            "DELETE FROM user_organization_join_request_list_users WHERE user_id = $1 AND updated_event_sequence < $2",
        )
        .bind(user_id.value())
        .bind(event_context.event_sequence.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|error| UserOrganizationJoinRequestListWriterError::Persistence(Box::new(error)))?;
        Ok(())
    }
}
