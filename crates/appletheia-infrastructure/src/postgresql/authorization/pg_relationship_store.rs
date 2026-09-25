use appletheia_application::aggregate::{AggregateIdValue, AggregateRef, AggregateTypeOwned};
use appletheia_application::authorization::{
    RelationRefOwned, Relationship, RelationshipId, RelationshipStore, RelationshipStoreError,
    RelationshipSubject,
};
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

    async fn replace(
        &self,
        uow: &mut PgUnitOfWork,
        source: &AggregateRef,
        relationships: &[Relationship],
    ) -> Result<(), RelationshipStoreError> {
        let transaction = uow.transaction_mut();
        sqlx::query("DELETE FROM relationships WHERE source_aggregate_type = $1 AND source_aggregate_id = $2")
            .bind(source.aggregate_type.value())
            .bind(source.aggregate_id.value())
            .execute(transaction.as_mut()).await
            .map_err(|error| RelationshipStoreError::Persistence(Box::new(error)))?;

        if relationships.is_empty() {
            return Ok(());
        }

        let mut query = QueryBuilder::<Postgres>::new(
            r#"
            INSERT INTO relationships (
                id,
                source_aggregate_type,
                source_aggregate_id,
                target_aggregate_type,
                target_aggregate_id,
                relation,
                subject_aggregate_type,
                subject_aggregate_id,
                subject_relation,
                subject_is_wildcard
            )
            "#,
        );

        query.push_values(relationships, |mut b, item| {
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
                    Some(relation.relation_name.value()),
                    false,
                ),
            };

            b.push_bind(RelationshipId::new().value())
                .push_bind(source.aggregate_type.value())
                .push_bind(source.aggregate_id.value())
                .push_bind(item.target.aggregate_type.value())
                .push_bind(item.target.aggregate_id.value())
                .push_bind(item.relation.relation_name.value())
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

        Ok(())
    }

    async fn read_targets_by_subject(
        &self,
        uow: &mut PgUnitOfWork,
        subject: &RelationshipSubject,
        relation: &RelationRefOwned,
    ) -> Result<Vec<AggregateRef>, RelationshipStoreError> {
        let mut query = QueryBuilder::<Postgres>::new(
            r#"
            SELECT DISTINCT target_aggregate_type, target_aggregate_id
            FROM relationships
            WHERE relation =
            "#,
        );
        query.push_bind(relation.relation_name.value());

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
                query.push_bind(relation.relation_name.value());
                query.push(" AND subject_aggregate_type = ");
                query.push_bind(aggregate.aggregate_type.value());
                query.push(" AND subject_aggregate_id = ");
                query.push_bind(aggregate.aggregate_id.value());
            }
        }

        query.push(" AND target_aggregate_type = ");
        query.push_bind(relation.aggregate_type.value());

        let transaction = uow.transaction_mut();
        let rows = query
            .build()
            .fetch_all(transaction.as_mut())
            .await
            .map_err(|e| RelationshipStoreError::Persistence(Box::new(e)))?;

        let mut out = Vec::with_capacity(rows.len());
        for row in rows {
            let target_aggregate_type_string: String = row
                .try_get("target_aggregate_type")
                .map_err(|e| RelationshipStoreError::MappingFailed(Box::new(e)))?;
            let target_aggregate_type: AggregateTypeOwned = target_aggregate_type_string
                .parse()
                .map_err(|e| RelationshipStoreError::MappingFailed(Box::new(e)))?;
            let target_aggregate_id: Uuid = row
                .try_get("target_aggregate_id")
                .map_err(|e| RelationshipStoreError::MappingFailed(Box::new(e)))?;
            out.push(AggregateRef {
                aggregate_type: target_aggregate_type,
                aggregate_id: AggregateIdValue::from(target_aggregate_id),
            });
        }

        Ok(out)
    }

    async fn read_subjects_by_target(
        &self,
        uow: &mut PgUnitOfWork,
        target: &AggregateRef,
        relation: &RelationRefOwned,
        subject_aggregate_type: Option<&AggregateTypeOwned>,
    ) -> Result<Vec<RelationshipSubject>, RelationshipStoreError> {
        let transaction = uow.transaction_mut();
        let mut query = QueryBuilder::<Postgres>::new(
            r#"
            SELECT
                id,
                source_aggregate_type,
                source_aggregate_id,
                target_aggregate_type,
                target_aggregate_id,
                relation,
                subject_aggregate_type,
                subject_aggregate_id,
                subject_relation,
                subject_is_wildcard
            FROM relationships
            WHERE target_aggregate_type =
            "#,
        );
        query.push_bind(target.aggregate_type.value());
        query.push(" AND target_aggregate_id = ");
        query.push_bind(target.aggregate_id.value());
        query.push(" AND relation = ");
        query.push_bind(relation.relation_name.value());

        if let Some(subject_aggregate_type) = subject_aggregate_type {
            query.push(" AND subject_aggregate_type = ");
            query.push_bind(subject_aggregate_type.value());
        }

        let rows: Vec<PgRelationshipRow> = query
            .build_query_as()
            .fetch_all(transaction.as_mut())
            .await
            .map_err(|e| RelationshipStoreError::Persistence(Box::new(e)))?;

        let mut out: Vec<RelationshipSubject> = Vec::with_capacity(rows.len());

        for row in rows {
            let relationship = row
                .try_into_relationship()
                .map_err(|e| RelationshipStoreError::MappingFailed(Box::new(e)))?;
            if !out.contains(&relationship.subject) {
                out.push(relationship.subject);
            }
        }

        Ok(out)
    }
}

#[cfg(test)]
mod tests {
    use appletheia_application::aggregate::AggregateRef;
    use appletheia_application::authorization::{
        RelationRefOwned, Relationship, RelationshipStore, RelationshipSubject,
    };
    use appletheia_application::unit_of_work::{UnitOfWork, UnitOfWorkFactory};
    use sqlx::PgPool;
    use uuid::Uuid;

    use super::PgRelationshipStore;
    use crate::postgresql::PgUnitOfWorkFactory;

    fn aggregate(kind: &str) -> AggregateRef {
        AggregateRef {
            aggregate_type: kind.parse().unwrap(),
            aggregate_id: Uuid::now_v7().into(),
        }
    }

    fn member(target: &AggregateRef, user: &AggregateRef) -> Relationship {
        Relationship {
            target: target.clone(),
            relation: RelationRefOwned {
                aggregate_type: target.aggregate_type.clone(),
                relation_name: "member".parse().unwrap(),
            },
            subject: RelationshipSubject::Aggregate(user.clone()),
        }
    }

    async fn seed(pool: &PgPool, source: &AggregateRef) {
        sqlx::query("INSERT INTO events (id, aggregate_type, aggregate_id, aggregate_version, event_name, payload, occurred_at, correlation_id, causation_id) VALUES ($1, $2, $3, 1, 'created', '{}', now(), $4, $5)")
            .bind(Uuid::now_v7()).bind(source.aggregate_type.value()).bind(source.aggregate_id.value()).bind(Uuid::now_v7()).bind(Uuid::now_v7())
            .execute(pool).await.unwrap();
    }

    #[sqlx::test(migrations = "migrations/postgresql")]
    #[ignore = "requires PostgreSQL"]
    async fn replacement_preserves_other_sources_and_rolls_back(pool: PgPool) {
        let factory = PgUnitOfWorkFactory::new(pool.clone());
        let store = PgRelationshipStore;
        let first = aggregate("membership");
        let second = aggregate("membership");
        let target = aggregate("organization");
        let user = aggregate("user");
        let tuple = member(&target, &user);
        seed(&pool, &first).await;
        seed(&pool, &second).await;
        let mut uow = factory.begin().await.unwrap();
        store
            .replace(&mut uow, &first, std::slice::from_ref(&tuple))
            .await
            .unwrap();
        store
            .replace(&mut uow, &second, std::slice::from_ref(&tuple))
            .await
            .unwrap();
        assert_eq!(
            store
                .read_subjects_by_target(&mut uow, &target, &tuple.relation, None)
                .await
                .unwrap(),
            vec![tuple.subject.clone()]
        );
        assert_eq!(
            store
                .read_targets_by_subject(&mut uow, &tuple.subject, &tuple.relation)
                .await
                .unwrap(),
            vec![target.clone()]
        );
        uow.commit().await.unwrap();

        let mut uow = factory.begin().await.unwrap();
        store.replace(&mut uow, &first, &[]).await.unwrap();
        assert_eq!(
            store
                .read_subjects_by_target(&mut uow, &target, &tuple.relation, None)
                .await
                .unwrap()
                .len(),
            1
        );
        uow.commit().await.unwrap();

        let mut uow = factory.begin().await.unwrap();
        store.replace(&mut uow, &second, &[]).await.unwrap();
        assert!(
            store
                .read_subjects_by_target(&mut uow, &target, &tuple.relation, None)
                .await
                .unwrap()
                .is_empty()
        );
        uow.rollback().await.unwrap();

        let mut uow = factory.begin().await.unwrap();
        assert_eq!(
            store
                .read_subjects_by_target(&mut uow, &target, &tuple.relation, None)
                .await
                .unwrap()
                .len(),
            1
        );
        uow.rollback().await.unwrap();

        // A replacement followed by a failed event write must not commit its deletion.
        let mut uow = factory.begin().await.unwrap();
        store.replace(&mut uow, &second, &[]).await.unwrap();
        assert!(
            sqlx::query("INSERT INTO events SELECT * FROM events WHERE aggregate_id = $1")
                .bind(second.aggregate_id.value())
                .execute(uow.transaction_mut().as_mut())
                .await
                .is_err()
        );
        uow.rollback().await.unwrap();
        let mut uow = factory.begin().await.unwrap();
        assert_eq!(
            store
                .read_subjects_by_target(&mut uow, &target, &tuple.relation, None)
                .await
                .unwrap()
                .len(),
            1
        );
        uow.rollback().await.unwrap();
    }
}
