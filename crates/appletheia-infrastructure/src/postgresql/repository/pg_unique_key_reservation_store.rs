use std::collections::HashSet;

use appletheia_application::repository::{
    UniqueKeyReservationStore, UniqueKeyReservationStoreError,
};
use appletheia_domain::aggregate::{AggregateId, AggregateType, UniqueEntries, UniqueValue};
use sqlx::{Postgres, QueryBuilder};
use uuid::Uuid;

use crate::postgresql::unit_of_work::PgUnitOfWork;

pub struct PgUniqueKeyReservationStore;

impl PgUniqueKeyReservationStore {
    pub fn new() -> Self {
        Self
    }
}

impl Default for PgUniqueKeyReservationStore {
    fn default() -> Self {
        Self::new()
    }
}

impl UniqueKeyReservationStore for PgUniqueKeyReservationStore {
    type Uow = PgUnitOfWork;

    async fn replace<I>(
        &self,
        uow: &mut Self::Uow,
        aggregate_type: AggregateType,
        owner_aggregate_id: I,
        unique_entries: &UniqueEntries,
    ) -> Result<(), UniqueKeyReservationStoreError>
    where
        I: AggregateId,
    {
        struct FlatEntry<'a> {
            namespace: appletheia_domain::aggregate::UniqueKey,
            value: &'a UniqueValue,
            normalized_value: String,
        }

        let aggregate_type_value = aggregate_type.to_string();
        let owner_aggregate_id_value = owner_aggregate_id.value();
        let flattened_entries = unique_entries
            .iter()
            .flat_map(|(namespace, values)| {
                values.iter().map(move |value| FlatEntry {
                    namespace: *namespace,
                    value,
                    normalized_value: value.normalized_key(),
                })
            })
            .collect::<Vec<_>>();
        let transaction = uow.transaction_mut();

        sqlx::query(
            r#"
            DELETE FROM unique_key_reservations
            WHERE aggregate_type = $1 AND owner_aggregate_id = $2
            "#,
        )
        .bind(&aggregate_type_value)
        .bind(owner_aggregate_id_value)
        .execute(transaction.as_mut())
        .await
        .map_err(|error| UniqueKeyReservationStoreError::Persistence(Box::new(error)))?;

        if flattened_entries.is_empty() {
            return Ok(());
        }

        let mut query_builder = QueryBuilder::<Postgres>::new(
            r#"
            INSERT INTO unique_key_reservations (
                id, aggregate_type, owner_aggregate_id, namespace, normalized_value
            )
            "#,
        );
        query_builder.push_values(flattened_entries.iter(), |mut builder, entry| {
            builder
                .push_bind(Uuid::now_v7())
                .push_bind(&aggregate_type_value)
                .push_bind(owner_aggregate_id_value)
                .push_bind(entry.namespace.value())
                .push_bind(&entry.normalized_value);
        });
        query_builder.push(
            r#"
            ON CONFLICT DO NOTHING
            RETURNING namespace, normalized_value
            "#,
        );

        let inserted_rows = query_builder
            .build_query_as::<(String, String)>()
            .fetch_all(transaction.as_mut())
            .await
            .map_err(|error| UniqueKeyReservationStoreError::Persistence(Box::new(error)))?;

        if inserted_rows.len() == flattened_entries.len() {
            return Ok(());
        }

        let inserted_keys = inserted_rows.into_iter().collect::<HashSet<_>>();
        let conflicting_entry = flattened_entries
            .iter()
            .find(|entry| {
                !inserted_keys.contains(&(
                    entry.namespace.value().to_owned(),
                    entry.normalized_value.clone(),
                ))
            })
            .expect("missing inserted entry should identify a conflict");

        Err(UniqueKeyReservationStoreError::conflict(
            aggregate_type,
            conflicting_entry.namespace,
            conflicting_entry.value,
        ))
    }
}
