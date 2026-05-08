use appletheia_application::repository::{ReferenceIndexStore, ReferenceIndexStoreError};
use appletheia_domain::aggregate::{AggregateId, AggregateType, ReferenceEntries};
use sqlx::{Postgres, QueryBuilder};
use uuid::Uuid;

use crate::postgresql::unit_of_work::PgUnitOfWork;

pub struct PgReferenceIndexStore;

impl PgReferenceIndexStore {
    pub fn new() -> Self {
        Self
    }
}

impl Default for PgReferenceIndexStore {
    fn default() -> Self {
        Self::new()
    }
}

impl ReferenceIndexStore for PgReferenceIndexStore {
    type Uow = PgUnitOfWork;

    async fn replace<I>(
        &self,
        uow: &mut Self::Uow,
        aggregate_type: AggregateType,
        source_aggregate_id: I,
        reference_entries: &ReferenceEntries,
    ) -> Result<(), ReferenceIndexStoreError>
    where
        I: AggregateId,
    {
        struct FlatEntry {
            namespace: appletheia_domain::aggregate::ReferenceKey,
            target_aggregate_id: Uuid,
        }

        let source_aggregate_type_value = aggregate_type.to_string();
        let source_aggregate_id_value = source_aggregate_id.value();
        let flattened_entries = reference_entries
            .iter()
            .flat_map(|(namespace, values)| {
                values.iter().map(move |target_aggregate_id| FlatEntry {
                    namespace: *namespace,
                    target_aggregate_id,
                })
            })
            .collect::<Vec<_>>();
        let transaction = uow.transaction_mut();

        sqlx::query(
            r#"
            DELETE FROM aggregate_reference_indexes
            WHERE source_aggregate_type = $1 AND source_aggregate_id = $2
            "#,
        )
        .bind(&source_aggregate_type_value)
        .bind(source_aggregate_id_value)
        .execute(transaction.as_mut())
        .await
        .map_err(|error| ReferenceIndexStoreError::Persistence(Box::new(error)))?;

        if flattened_entries.is_empty() {
            return Ok(());
        }

        let mut query_builder = QueryBuilder::<Postgres>::new(
            r#"
            INSERT INTO aggregate_reference_indexes (
                id, source_aggregate_type, source_aggregate_id, namespace, target_aggregate_id
            )
            "#,
        );
        query_builder.push_values(flattened_entries.iter(), |mut builder, entry| {
            builder
                .push_bind(Uuid::now_v7())
                .push_bind(&source_aggregate_type_value)
                .push_bind(source_aggregate_id_value)
                .push_bind(entry.namespace.value())
                .push_bind(entry.target_aggregate_id);
        });
        query_builder.push(" ON CONFLICT DO NOTHING");

        query_builder
            .build()
            .execute(transaction.as_mut())
            .await
            .map_err(|error| ReferenceIndexStoreError::Persistence(Box::new(error)))?;

        Ok(())
    }
}
