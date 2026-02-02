use sqlx::FromRow;

use appletheia_application::event::{EventSequence, EventSequenceError};
use appletheia_application::projection::{
    ProjectionCheckpointStore, ProjectionCheckpointStoreError, ProjectorNameOwned,
};

use crate::postgresql::unit_of_work::PgUnitOfWork;

#[derive(Debug)]
pub struct PgProjectionCheckpointStore;

impl PgProjectionCheckpointStore {
    pub fn new() -> Self {
        Self
    }
}

impl Default for PgProjectionCheckpointStore {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, FromRow)]
struct ProjectionCheckpointRow {
    last_event_sequence: i64,
}

impl ProjectionCheckpointStore for PgProjectionCheckpointStore {
    type Uow = PgUnitOfWork;

    async fn load(
        &self,
        uow: &mut Self::Uow,
        projector_name: ProjectorNameOwned,
    ) -> Result<Option<EventSequence>, ProjectionCheckpointStoreError> {
        let transaction = uow.transaction_mut();

        let row: Option<ProjectionCheckpointRow> = sqlx::query_as(
            r#"
            SELECT last_event_sequence
              FROM projection_checkpoints
             WHERE projector_name = $1
            "#,
        )
        .bind(projector_name.value())
        .fetch_optional(transaction.as_mut())
        .await
        .map_err(|source| ProjectionCheckpointStoreError::Persistence(Box::new(source)))?;

        let Some(row) = row else {
            return Ok(None);
        };

        let seq =
            EventSequence::try_from(row.last_event_sequence).map_err(|e: EventSequenceError| {
                ProjectionCheckpointStoreError::Persistence(Box::new(e))
            })?;

        Ok(Some(seq))
    }

    async fn save(
        &self,
        uow: &mut Self::Uow,
        projector_name: ProjectorNameOwned,
        event_sequence: EventSequence,
    ) -> Result<(), ProjectionCheckpointStoreError> {
        let transaction = uow.transaction_mut();

        sqlx::query(
            r#"
            INSERT INTO projection_checkpoints (projector_name, last_event_sequence)
            VALUES ($1, $2)
            ON CONFLICT (projector_name)
            DO UPDATE SET last_event_sequence = EXCLUDED.last_event_sequence,
                          updated_at = now()
            "#,
        )
        .bind(projector_name.value())
        .bind(event_sequence.value())
        .execute(transaction.as_mut())
        .await
        .map_err(|source| ProjectionCheckpointStoreError::Persistence(Box::new(source)))?;

        Ok(())
    }

    async fn reset(
        &self,
        uow: &mut Self::Uow,
        projector_name: ProjectorNameOwned,
    ) -> Result<(), ProjectionCheckpointStoreError> {
        let transaction = uow.transaction_mut();

        sqlx::query(
            r#"
            DELETE FROM projection_checkpoints
             WHERE projector_name = $1
            "#,
        )
        .bind(projector_name.value())
        .execute(transaction.as_mut())
        .await
        .map_err(|source| ProjectionCheckpointStoreError::Persistence(Box::new(source)))?;

        Ok(())
    }
}
