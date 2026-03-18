use appletheia_application::projection::{
    ProjectorNameOwned, ProjectorProcessedEventId, ProjectorProcessedEventStore,
    ProjectorProcessedEventStoreError,
};
use appletheia_domain::EventId;

use crate::postgresql::unit_of_work::PgUnitOfWork;

use super::pg_projector_processed_event_row::PgProjectorProcessedEventRow;

#[derive(Debug)]
pub struct PgProjectorProcessedEventStore;

impl PgProjectorProcessedEventStore {
    pub fn new() -> Self {
        Self
    }
}

impl Default for PgProjectorProcessedEventStore {
    fn default() -> Self {
        Self::new()
    }
}

impl ProjectorProcessedEventStore for PgProjectorProcessedEventStore {
    type Uow = PgUnitOfWork;

    async fn is_processed(
        &self,
        uow: &mut Self::Uow,
        projector_name: ProjectorNameOwned,
        event_id: EventId,
    ) -> Result<bool, ProjectorProcessedEventStoreError> {
        let transaction = uow.transaction_mut();

        let projector_name_value = projector_name.value();
        let event_id_value = event_id.value();

        let row: Option<PgProjectorProcessedEventRow> = sqlx::query_as(
            r#"
            SELECT
              id,
              projector_name,
              event_id,
              processed_at
              FROM projector_processed_events
             WHERE projector_name = $1
               AND event_id = $2
            "#,
        )
        .bind(projector_name_value)
        .bind(event_id_value)
        .fetch_optional(transaction.as_mut())
        .await
        .map_err(|source| ProjectorProcessedEventStoreError::Persistence(Box::new(source)))?;

        Ok(row.is_some())
    }

    async fn mark_processed(
        &self,
        uow: &mut Self::Uow,
        projector_name: ProjectorNameOwned,
        event_id: EventId,
    ) -> Result<bool, ProjectorProcessedEventStoreError> {
        let transaction = uow.transaction_mut();

        let projector_name_value = projector_name.value();
        let event_id_value = event_id.value();
        let id_value = ProjectorProcessedEventId::new().value();

        let done = sqlx::query(
            r#"
            INSERT INTO projector_processed_events (id, projector_name, event_id)
            VALUES ($1, $2, $3)
            ON CONFLICT (projector_name, event_id) DO NOTHING
            "#,
        )
        .bind(id_value)
        .bind(projector_name_value)
        .bind(event_id_value)
        .execute(transaction.as_mut())
        .await
        .map_err(|source| ProjectorProcessedEventStoreError::Persistence(Box::new(source)))?;

        Ok(done.rows_affected() == 1)
    }

    async fn reset(
        &self,
        uow: &mut Self::Uow,
        projector_name: ProjectorNameOwned,
    ) -> Result<(), ProjectorProcessedEventStoreError> {
        let transaction = uow.transaction_mut();

        sqlx::query(
            r#"
            DELETE FROM projector_processed_events
             WHERE projector_name = $1
            "#,
        )
        .bind(projector_name.value())
        .execute(transaction.as_mut())
        .await
        .map_err(|source| ProjectorProcessedEventStoreError::Persistence(Box::new(source)))?;

        Ok(())
    }
}
