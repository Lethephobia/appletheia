use std::collections::HashMap;

use appletheia_application::authorization::{
    AggregateRef, RelationName, Relationship, RelationshipChange, RelationshipId,
    RelationshipStore, RelationshipStoreError, RelationshipSubject,
};
use appletheia_application::event::{AggregateIdValue, AggregateTypeOwned};
use sqlx::{Postgres, QueryBuilder, Row};
use uuid::Uuid;

use crate::postgresql::PgUnitOfWork;

use super::pg_relationship_row::PgRelationshipRow;

pub struct PgRelationshipStore;

impl PgRelationshipStore {
    pub fn new() -> Self {
        Self
    }
}

impl Default for PgRelationshipStore {
    fn default() -> Self {
        Self::new()
    }
}

impl RelationshipStore for PgRelationshipStore {
    type Uow = PgUnitOfWork;

    async fn apply_changes(
        &self,
        uow: &mut PgUnitOfWork,
        changes: &[RelationshipChange],
    ) -> Result<(), RelationshipStoreError> {
        if changes.is_empty() {
            return Ok(());
        }

        const CHUNK_SIZE: usize = 1000;

        let transaction = uow.transaction_mut();

        let mut deduped: HashMap<Relationship, bool> = HashMap::new();
        for change in changes {
            let (relationship, is_upsert) = match change {
                RelationshipChange::Upsert(relationship) => (relationship, true),
                RelationshipChange::Delete(relationship) => (relationship, false),
            };

            deduped.insert(relationship.clone(), is_upsert);
        }

        let mut deletes: Vec<Relationship> = Vec::new();
        let mut upserts: Vec<Relationship> = Vec::new();
        for (relationship, is_upsert) in deduped {
            if is_upsert {
                upserts.push(relationship);
            } else {
                deletes.push(relationship);
            }
        }

        for chunk in deletes.chunks(CHUNK_SIZE) {
            let mut query = QueryBuilder::<Postgres>::new(
                r#"
                DELETE FROM relationships r
                USING (
                "#,
            );

            query.push_values(chunk, |mut b, item| {
                let (
                    subject_aggregate_type,
                    subject_aggregate_id,
                    subject_relation,
                    subject_is_wildcard,
                ) = match &item.subject {
                    RelationshipSubject::Aggregate(subject) => (
                        subject.aggregate_type.value(),
                        Some(subject.aggregate_id.value()),
                        None,
                        false,
                    ),
                    RelationshipSubject::Wildcard { aggregate_type } => {
                        (aggregate_type.value(), None, None, true)
                    }
                    RelationshipSubject::AggregateSet {
                        aggregate,
                        relation,
                    } => (
                        aggregate.aggregate_type.value(),
                        Some(aggregate.aggregate_id.value()),
                        Some(relation.value()),
                        false,
                    ),
                };

                b.push_bind(item.aggregate.aggregate_type.value())
                    .push_bind(item.aggregate.aggregate_id.value())
                    .push_bind(item.relation.value())
                    .push_bind(subject_aggregate_type)
                    .push_bind(subject_aggregate_id)
                    .push_bind(subject_relation)
                    .push_bind(subject_is_wildcard);
            });

            query.push(
                r#"
                ) AS v(
                    aggregate_type,
                    aggregate_id,
                    relation,
                    subject_aggregate_type,
                    subject_aggregate_id,
                    subject_relation,
                    subject_is_wildcard
                )
                WHERE r.aggregate_type = v.aggregate_type
                  AND r.aggregate_id = v.aggregate_id
                  AND r.relation = v.relation
                  AND r.subject_aggregate_type = v.subject_aggregate_type
                  AND r.subject_is_wildcard = v.subject_is_wildcard
                  AND r.subject_aggregate_id IS NOT DISTINCT FROM v.subject_aggregate_id
                  AND r.subject_relation IS NOT DISTINCT FROM v.subject_relation
                "#,
            );

            query
                .build()
                .execute(transaction.as_mut())
                .await
                .map_err(|e| RelationshipStoreError::Persistence(Box::new(e)))?;
        }

        for chunk in upserts.chunks(CHUNK_SIZE) {
            let mut query = QueryBuilder::<Postgres>::new(
                r#"
                INSERT INTO relationships (
                    id,
                    aggregate_type,
                    aggregate_id,
                    relation,
                    subject_aggregate_type,
                    subject_aggregate_id,
                    subject_relation,
                    subject_is_wildcard
                )
                "#,
            );

            query.push_values(chunk, |mut b, item| {
                let (
                    subject_aggregate_type,
                    subject_aggregate_id,
                    subject_relation,
                    subject_is_wildcard,
                ) = match &item.subject {
                    RelationshipSubject::Aggregate(subject) => (
                        subject.aggregate_type.value(),
                        Some(subject.aggregate_id.value()),
                        None,
                        false,
                    ),
                    RelationshipSubject::Wildcard { aggregate_type } => {
                        (aggregate_type.value(), None, None, true)
                    }
                    RelationshipSubject::AggregateSet {
                        aggregate,
                        relation,
                    } => (
                        aggregate.aggregate_type.value(),
                        Some(aggregate.aggregate_id.value()),
                        Some(relation.value()),
                        false,
                    ),
                };

                b.push_bind(RelationshipId::new().value())
                    .push_bind(item.aggregate.aggregate_type.value())
                    .push_bind(item.aggregate.aggregate_id.value())
                    .push_bind(item.relation.value())
                    .push_bind(subject_aggregate_type)
                    .push_bind(subject_aggregate_id)
                    .push_bind(subject_relation)
                    .push_bind(subject_is_wildcard);
            });

            query.push(" ON CONFLICT DO NOTHING");

            query
                .build()
                .execute(transaction.as_mut())
                .await
                .map_err(|e| RelationshipStoreError::Persistence(Box::new(e)))?;
        }

        Ok(())
    }

    async fn read_subjects_by_aggregate(
        &self,
        uow: &mut PgUnitOfWork,
        aggregate: &AggregateRef,
        relation: &RelationName,
    ) -> Result<Vec<RelationshipSubject>, RelationshipStoreError> {
        let transaction = uow.transaction_mut();
        let rows: Vec<PgRelationshipRow> = sqlx::query_as(
            r#"
            SELECT
                id,
                aggregate_type,
                aggregate_id,
                relation,
                subject_aggregate_type,
                subject_aggregate_id,
                subject_relation,
                subject_is_wildcard
            FROM relationships
            WHERE aggregate_type = $1
              AND aggregate_id = $2
              AND relation = $3
            "#,
        )
        .bind(aggregate.aggregate_type.value())
        .bind(aggregate.aggregate_id.value())
        .bind(relation.value())
        .fetch_all(transaction.as_mut())
        .await
        .map_err(|e| RelationshipStoreError::Persistence(Box::new(e)))?;

        let mut out: Vec<RelationshipSubject> = Vec::with_capacity(rows.len());

        for row in rows {
            let relationship = row
                .try_into_relationship()
                .map_err(|e| RelationshipStoreError::MappingFailed(Box::new(e)))?;
            out.push(relationship.subject);
        }

        Ok(out)
    }

    async fn read_aggregates_by_subject(
        &self,
        uow: &mut PgUnitOfWork,
        subject: &RelationshipSubject,
        aggregate_type: &AggregateTypeOwned,
        relation: &RelationName,
    ) -> Result<Vec<AggregateRef>, RelationshipStoreError> {
        let mut query = QueryBuilder::<Postgres>::new(
            r#"
            SELECT DISTINCT aggregate_type, aggregate_id
            FROM relationships
            WHERE relation =
            "#,
        );
        query.push_bind(relation.value());

        match subject {
            RelationshipSubject::Aggregate(subject) => {
                query.push(" AND subject_is_wildcard = false");
                query.push(" AND subject_relation IS NULL");
                query.push(" AND subject_aggregate_type = ");
                query.push_bind(subject.aggregate_type.value());
                query.push(" AND subject_aggregate_id = ");
                query.push_bind(subject.aggregate_id.value());
            }
            RelationshipSubject::Wildcard { aggregate_type } => {
                query.push(" AND subject_is_wildcard = true");
                query.push(" AND subject_aggregate_type = ");
                query.push_bind(aggregate_type.value());
            }
            RelationshipSubject::AggregateSet {
                aggregate,
                relation,
            } => {
                query.push(" AND subject_is_wildcard = false");
                query.push(" AND subject_relation = ");
                query.push_bind(relation.value());
                query.push(" AND subject_aggregate_type = ");
                query.push_bind(aggregate.aggregate_type.value());
                query.push(" AND subject_aggregate_id = ");
                query.push_bind(aggregate.aggregate_id.value());
            }
        }

        query.push(" AND aggregate_type = ");
        query.push_bind(aggregate_type.value());

        let transaction = uow.transaction_mut();
        let rows = query
            .build()
            .fetch_all(transaction.as_mut())
            .await
            .map_err(|e| RelationshipStoreError::Persistence(Box::new(e)))?;

        let mut out = Vec::with_capacity(rows.len());
        for row in rows {
            let aggregate_type: String = row
                .try_get("aggregate_type")
                .map_err(|e| RelationshipStoreError::MappingFailed(Box::new(e)))?;
            let aggregate_type: AggregateTypeOwned = aggregate_type
                .parse()
                .map_err(|e| RelationshipStoreError::MappingFailed(Box::new(e)))?;
            let aggregate_id: Uuid = row
                .try_get("aggregate_id")
                .map_err(|e| RelationshipStoreError::MappingFailed(Box::new(e)))?;
            out.push(AggregateRef {
                aggregate_type,
                aggregate_id: AggregateIdValue::from(aggregate_id),
            });
        }

        Ok(out)
    }
}
