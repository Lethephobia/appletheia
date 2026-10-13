use crate::postgresql::unit_of_work::PgUnitOfWork;
use appletheia_application::outbox::read_model_change::{
    ReadModelChangeOutboxEnqueueError, ReadModelChangeOutboxEnqueuer, ReadModelChangeOutboxId,
};
use appletheia_application::read_model::ReadModelChangeEnvelope;
use chrono::{DateTime, Utc};

pub struct PgReadModelChangeOutboxEnqueuer;

impl PgReadModelChangeOutboxEnqueuer {
    pub fn new() -> Self {
        Self
    }
}

impl Default for PgReadModelChangeOutboxEnqueuer {
    fn default() -> Self {
        Self::new()
    }
}

impl ReadModelChangeOutboxEnqueuer for PgReadModelChangeOutboxEnqueuer {
    type Uow = PgUnitOfWork;

    async fn enqueue(
        &self,
        uow: &mut Self::Uow,
        change: &ReadModelChangeEnvelope,
    ) -> Result<(), ReadModelChangeOutboxEnqueueError> {
        sqlx::query("INSERT INTO read_model_change_outbox (id, change_id, source_event_id, changed_at, read_model_type, read_model_id) VALUES ($1, $2, $3, $4, $5, $6)")
            .bind(ReadModelChangeOutboxId::new().value())
            .bind(change.change_id.value())
            .bind(change.source_event_id.value())
            .bind(DateTime::<Utc>::from(change.changed_at))
            .bind(change.change.read_model_ref().read_model_type.value())
            .bind(change.change.read_model_ref().id.value())
            .execute(uow.transaction_mut().as_mut()).await
            .map_err(|source| ReadModelChangeOutboxEnqueueError::Persistence(Box::new(source)))?;
        Ok(())
    }
}
