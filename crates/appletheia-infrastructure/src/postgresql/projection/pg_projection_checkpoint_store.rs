use appletheia_application::event::{EventSequence, EventSequenceError};
use appletheia_application::projection::{
    ProjectionCheckpointId, ProjectionCheckpointStore, ProjectionCheckpointStoreError,
    ProjectorNameOwned,
};

use crate::postgresql::unit_of_work::PgUnitOfWork;

use super::pg_projection_checkpoint_row::PgProjectionCheckpointRow;

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

impl ProjectionCheckpointStore for PgProjectionCheckpointStore {
    type Uow = PgUnitOfWork;

    async fn load(
        &self,
        uow: &mut Self::Uow,
        projector_name: ProjectorNameOwned,
    ) -> Result<Option<EventSequence>, ProjectionCheckpointStoreError> {
        let transaction = uow.transaction_mut();

        let row: Option<PgProjectionCheckpointRow> = sqlx::query_as(
            r#"
            SELECT
              id,
              projector_name,
              last_event_sequence,
              updated_at
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

        let id_value = ProjectionCheckpointId::new().value();

        sqlx::query(
            r#"
            INSERT INTO projection_checkpoints (id, projector_name, last_event_sequence)
            VALUES ($1, $2, $3)
            ON CONFLICT (projector_name)
            DO UPDATE SET last_event_sequence = GREATEST(
                              projection_checkpoints.last_event_sequence,
                              EXCLUDED.last_event_sequence
                          ),
                          updated_at = now()
            "#,
        )
        .bind(id_value)
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
