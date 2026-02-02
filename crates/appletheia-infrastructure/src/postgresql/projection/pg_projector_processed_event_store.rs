use appletheia_application::projection::{
    ProjectorNameOwned, ProjectorProcessedEventStore, ProjectorProcessedEventStoreError,
};
use appletheia_domain::EventId;

use crate::postgresql::unit_of_work::PgUnitOfWork;

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

    async fn mark_processed(
        &self,
        uow: &mut Self::Uow,
        projector_name: ProjectorNameOwned,
        event_id: EventId,
    ) -> Result<bool, ProjectorProcessedEventStoreError> {
        let transaction = uow.transaction_mut();

        let projector_name_value = projector_name.value();
        let event_id_value = event_id.value();

        let done = sqlx::query(
            r#"
            INSERT INTO projector_processed_events (projector_name, event_id)
            VALUES ($1, $2)
            ON CONFLICT (projector_name, event_id) DO NOTHING
            "#,
        )
        .bind(projector_name_value)
        .bind(event_id_value)
        .execute(transaction.as_mut())
        .await
        .map_err(|source| ProjectorProcessedEventStoreError::Persistence(Box::new(source)))?;

        Ok(done.rows_affected() == 1)
    }
}
