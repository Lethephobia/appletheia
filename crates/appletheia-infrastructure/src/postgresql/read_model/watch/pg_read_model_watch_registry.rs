use appletheia_application::read_model::{
    ReadModelRef,
    watch::{
        ReadModelWatchEndpointId, ReadModelWatchEndpointLeaseExpiresAt, ReadModelWatchGeneration,
        ReadModelWatchId, ReadModelWatchRegistry, ReadModelWatchRegistryError,
    },
};
use chrono::Utc;
use sqlx::{Postgres, QueryBuilder};

use crate::postgresql::unit_of_work::PgUnitOfWork;

use super::{
    pg_read_model_watch_endpoint_row::PgReadModelWatchEndpointRow,
    pg_read_model_watch_registry_error::PgReadModelWatchRegistryError,
    pg_read_model_watch_row::PgReadModelWatchRow,
};

#[derive(Default)]
pub struct PgReadModelWatchRegistry;

impl PgReadModelWatchRegistry {
    // PostgreSQL accepts at most 65,535 parameters; each target binds four values.
    const MAX_TARGETS_PER_INSERT: usize = u16::MAX as usize / 4;

    // Two parameters per target, plus the watch ID and generation.
    const MAX_TARGETS_PER_CONTAINS: usize = (u16::MAX as usize - 2) / 2;

    pub fn new() -> Self {
        Self
    }

    async fn lock_endpoint_key_shared(
        &self,
        uow: &mut PgUnitOfWork,
        endpoint_id: ReadModelWatchEndpointId,
    ) -> Result<(), PgReadModelWatchRegistryError> {
        let endpoint = sqlx::query_as::<_, PgReadModelWatchEndpointRow>(
            r#"
            SELECT endpoint_id, lease_until
            FROM read_model_watch_endpoints
            WHERE endpoint_id = $1
            FOR KEY SHARE
            "#,
        )
        .bind(endpoint_id.value())
        .fetch_optional(uow.transaction_mut().as_mut())
        .await?
        .ok_or(PgReadModelWatchRegistryError::EndpointInactive)?;
        let now = Utc::now();
        if ReadModelWatchEndpointLeaseExpiresAt::from(endpoint.lease_until).is_expired_at(now) {
            return Err(PgReadModelWatchRegistryError::EndpointInactive);
        }
        Ok(())
    }

    async fn read_watch_for_update(
        &self,
        uow: &mut PgUnitOfWork,
        endpoint_id: ReadModelWatchEndpointId,
        watch_id: ReadModelWatchId,
    ) -> Result<PgReadModelWatchRow, PgReadModelWatchRegistryError> {
        self.lock_endpoint_key_shared(uow, endpoint_id).await?;
        let row = sqlx::query_as::<_, PgReadModelWatchRow>(
            r#"
            SELECT watch_id, endpoint_id, generation
            FROM read_model_watches
            WHERE watch_id = $1
            FOR UPDATE
            "#,
        )
        .bind(watch_id.value())
        .fetch_optional(uow.transaction_mut().as_mut())
        .await?
        .ok_or(PgReadModelWatchRegistryError::WatchNotFound)?;
        if row.endpoint_id != endpoint_id.value() {
            return Err(PgReadModelWatchRegistryError::EndpointMismatch);
        }
        let endpoint = sqlx::query_as::<_, PgReadModelWatchEndpointRow>(
            r#"
            SELECT endpoint_id, lease_until
            FROM read_model_watch_endpoints
            WHERE endpoint_id = $1
            "#,
        )
        .bind(endpoint_id.value())
        .fetch_optional(uow.transaction_mut().as_mut())
        .await?
        .ok_or(PgReadModelWatchRegistryError::EndpointInactive)?;
        let now = Utc::now();
        if ReadModelWatchEndpointLeaseExpiresAt::from(endpoint.lease_until).is_expired_at(now) {
            return Err(PgReadModelWatchRegistryError::EndpointInactive);
        }
        Ok(row)
    }
}

impl ReadModelWatchRegistry for PgReadModelWatchRegistry {
    type Uow = PgUnitOfWork;

    async fn register_watch(
        &self,
        uow: &mut Self::Uow,
        endpoint_id: ReadModelWatchEndpointId,
        watch_id: ReadModelWatchId,
    ) -> Result<(), ReadModelWatchRegistryError> {
        self.lock_endpoint_key_shared(uow, endpoint_id)
            .await
            .map_err(|source| ReadModelWatchRegistryError::Backend(Box::new(source)))?;
        sqlx::query(
            r#"
            INSERT INTO read_model_watches (watch_id, endpoint_id, generation)
            VALUES ($1, $2, 0)
            "#,
        )
        .bind(watch_id.value())
        .bind(endpoint_id.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|source| ReadModelWatchRegistryError::Backend(Box::new(source)))?;
        Ok(())
    }

    async fn remove_watch(
        &self,
        uow: &mut Self::Uow,
        endpoint_id: ReadModelWatchEndpointId,
        watch_id: ReadModelWatchId,
    ) -> Result<(), ReadModelWatchRegistryError> {
        let row = sqlx::query_as::<_, PgReadModelWatchRow>(
            r#"
            SELECT watch_id, endpoint_id, generation
            FROM read_model_watches
            WHERE watch_id = $1
            FOR UPDATE
            "#,
        )
        .bind(watch_id.value())
        .fetch_optional(uow.transaction_mut().as_mut())
        .await
        .map_err(|source| ReadModelWatchRegistryError::Backend(Box::new(source)))?;
        let Some(watch) = row else {
            return Ok(());
        };
        if watch.endpoint_id != endpoint_id.value() {
            return Err(ReadModelWatchRegistryError::Backend(Box::new(
                PgReadModelWatchRegistryError::EndpointMismatch,
            )));
        }
        sqlx::query("DELETE FROM read_model_watches WHERE watch_id = $1")
            .bind(watch.watch_id)
            .execute(uow.transaction_mut().as_mut())
            .await
            .map_err(|source| ReadModelWatchRegistryError::Backend(Box::new(source)))?;
        Ok(())
    }

    async fn begin_generation(
        &self,
        uow: &mut Self::Uow,
        endpoint_id: ReadModelWatchEndpointId,
        watch_id: ReadModelWatchId,
    ) -> Result<ReadModelWatchGeneration, ReadModelWatchRegistryError> {
        let row = self
            .read_watch_for_update(uow, endpoint_id, watch_id)
            .await
            .map_err(|source| ReadModelWatchRegistryError::Backend(Box::new(source)))?;
        let generation = ReadModelWatchGeneration::try_from(row.generation)
            .and_then(ReadModelWatchGeneration::try_next)
            .map_err(|source| ReadModelWatchRegistryError::Backend(Box::new(source)))?;
        sqlx::query("UPDATE read_model_watches SET generation = $2 WHERE watch_id = $1")
            .bind(watch_id.value())
            .bind(generation.value())
            .execute(uow.transaction_mut().as_mut())
            .await
            .map_err(|source| ReadModelWatchRegistryError::Backend(Box::new(source)))?;
        Ok(generation)
    }

    async fn register_targets(
        &self,
        uow: &mut Self::Uow,
        endpoint_id: ReadModelWatchEndpointId,
        watch_id: ReadModelWatchId,
        generation: ReadModelWatchGeneration,
        targets: &[ReadModelRef],
    ) -> Result<(), ReadModelWatchRegistryError> {
        let watch = self
            .read_watch_for_update(uow, endpoint_id, watch_id)
            .await
            .map_err(|source| ReadModelWatchRegistryError::Backend(Box::new(source)))?;
        if watch.generation != generation.value() {
            return Err(ReadModelWatchRegistryError::Backend(Box::new(
                PgReadModelWatchRegistryError::StaleGeneration,
            )));
        }
        for chunk in targets.chunks(Self::MAX_TARGETS_PER_INSERT) {
            let mut query_builder = QueryBuilder::<Postgres>::new(
                "INSERT INTO read_model_watch_targets \
                 (watch_id, generation, read_model_type, read_model_id) ",
            );
            query_builder.push_values(chunk, |mut builder, target| {
                builder
                    .push_bind(watch_id.value())
                    .push_bind(generation.value())
                    .push_bind(target.read_model_type.value())
                    .push_bind(target.id.value());
            });
            query_builder.push(" ON CONFLICT DO NOTHING");
            query_builder
                .build()
                .execute(uow.transaction_mut().as_mut())
                .await
                .map_err(|source| ReadModelWatchRegistryError::Backend(Box::new(source)))?;
        }
        Ok(())
    }

    async fn contains_targets(
        &self,
        uow: &mut Self::Uow,
        endpoint_id: ReadModelWatchEndpointId,
        watch_id: ReadModelWatchId,
        generation: ReadModelWatchGeneration,
        targets: &[ReadModelRef],
    ) -> Result<bool, ReadModelWatchRegistryError> {
        let watch = self
            .read_watch_for_update(uow, endpoint_id, watch_id)
            .await
            .map_err(|source| ReadModelWatchRegistryError::Backend(Box::new(source)))?;
        if watch.generation != generation.value() {
            return Err(ReadModelWatchRegistryError::Backend(Box::new(
                PgReadModelWatchRegistryError::StaleGeneration,
            )));
        }
        for chunk in targets.chunks(Self::MAX_TARGETS_PER_CONTAINS) {
            let mut query_builder = QueryBuilder::<Postgres>::new(
                "WITH requested(read_model_type, read_model_id) AS (",
            );
            query_builder.push_values(chunk, |mut builder, target| {
                builder
                    .push_bind(target.read_model_type.value())
                    .push_bind(target.id.value());
            });
            query_builder.push(
                r#")
                SELECT NOT EXISTS (
                    SELECT 1
                    FROM requested
                    LEFT JOIN read_model_watch_targets AS target
                      ON target.watch_id = "#,
            );
            query_builder
                .push_bind(watch_id.value())
                .push(" AND target.generation = ")
                .push_bind(generation.value())
                .push(
                    r#"
                     AND target.read_model_type = requested.read_model_type
                     AND target.read_model_id = requested.read_model_id
                    WHERE target.watch_id IS NULL
                )
                "#,
                );
            let contains_targets: bool = query_builder
                .build_query_scalar()
                .fetch_one(uow.transaction_mut().as_mut())
                .await
                .map_err(|source| ReadModelWatchRegistryError::Backend(Box::new(source)))?;
            if !contains_targets {
                return Ok(false);
            }
        }
        Ok(true)
    }

    async fn delete_previous_generation_targets(
        &self,
        uow: &mut Self::Uow,
        endpoint_id: ReadModelWatchEndpointId,
        watch_id: ReadModelWatchId,
        generation: ReadModelWatchGeneration,
    ) -> Result<(), ReadModelWatchRegistryError> {
        let watch = self
            .read_watch_for_update(uow, endpoint_id, watch_id)
            .await
            .map_err(|source| ReadModelWatchRegistryError::Backend(Box::new(source)))?;
        if watch.generation != generation.value() {
            return Err(ReadModelWatchRegistryError::Backend(Box::new(
                PgReadModelWatchRegistryError::StaleGeneration,
            )));
        }
        sqlx::query(
            r#"
            DELETE
            FROM read_model_watch_targets
            WHERE watch_id = $1
            AND generation < $2
            "#,
        )
        .bind(watch_id.value())
        .bind(generation.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|source| ReadModelWatchRegistryError::Backend(Box::new(source)))?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::postgresql::unit_of_work::PgUnitOfWorkFactory;
    use appletheia_application::{
        read_model::{ReadModelId, ReadModelTypeOwned},
        unit_of_work::{UnitOfWork, UnitOfWorkFactory},
    };
    use sqlx::PgPool;
    use std::slice;

    async fn register(pool: &PgPool) -> (ReadModelWatchEndpointId, ReadModelWatchId) {
        let endpoint_id = ReadModelWatchEndpointId::new();
        let watch_id = ReadModelWatchId::new();
        sqlx::query(
            r#"
            INSERT INTO read_model_watch_endpoints (endpoint_id, lease_until)
            VALUES ($1, clock_timestamp() + interval '1 hour')
            "#,
        )
        .bind(endpoint_id.value())
        .execute(pool)
        .await
        .unwrap();
        let factory = PgUnitOfWorkFactory::new(pool.clone());
        let mut uow = factory.begin().await.unwrap();
        PgReadModelWatchRegistry::new()
            .register_watch(&mut uow, endpoint_id, watch_id)
            .await
            .unwrap();
        uow.commit().await.unwrap();
        (endpoint_id, watch_id)
    }

    fn target(id: &str) -> ReadModelRef {
        ReadModelRef {
            read_model_type: ReadModelTypeOwned::try_from("user").unwrap(),
            id: ReadModelId::from(id),
        }
    }

    #[sqlx::test(migrations = "migrations/postgresql")]
    #[ignore = "requires PostgreSQL"]
    async fn generations_preserve_targets_and_obey_transaction_boundaries(pool: PgPool) {
        let (endpoint, watch) = register(&pool).await;
        let registry = PgReadModelWatchRegistry::new();
        let factory = PgUnitOfWorkFactory::new(pool.clone());
        let a = target("a");
        let b = target("b");
        let mut first = factory.begin().await.unwrap();
        let g1 = registry
            .begin_generation(&mut first, endpoint, watch)
            .await
            .unwrap();
        registry
            .register_targets(&mut first, endpoint, watch, g1, &[a.clone(), a.clone()])
            .await
            .unwrap();
        first.commit().await.unwrap();
        let count: i64 = sqlx::query_scalar("SELECT count(*) FROM read_model_watch_targets")
            .fetch_one(&pool)
            .await
            .unwrap();
        assert_eq!(count, 1);

        let mut second = factory.begin().await.unwrap();
        let g2 = registry
            .begin_generation(&mut second, endpoint, watch)
            .await
            .unwrap();
        assert_eq!(g2.value(), g1.value() + 1);
        registry
            .register_targets(&mut second, endpoint, watch, g2, slice::from_ref(&b))
            .await
            .unwrap();
        assert!(
            registry
                .contains_targets(&mut second, endpoint, watch, g2, &[b.clone(), b.clone()])
                .await
                .unwrap()
        );
        assert!(
            !registry
                .contains_targets(&mut second, endpoint, watch, g2, slice::from_ref(&a))
                .await
                .unwrap()
        );
        assert!(
            registry
                .contains_targets(&mut second, endpoint, watch, g2, &[])
                .await
                .unwrap()
        );
        second.commit().await.unwrap();

        let mut stale = factory.begin().await.unwrap();
        assert!(
            registry
                .register_targets(&mut stale, endpoint, watch, g1, &[])
                .await
                .is_err()
        );
        assert!(
            registry
                .delete_previous_generation_targets(&mut stale, endpoint, watch, g1)
                .await
                .is_err()
        );
        stale.rollback().await.unwrap();

        let mut removal = factory.begin().await.unwrap();
        registry
            .delete_previous_generation_targets(&mut removal, endpoint, watch, g2)
            .await
            .unwrap();
        removal.rollback().await.unwrap();
        let retained: i64 = sqlx::query_scalar("SELECT count(*) FROM read_model_watch_targets")
            .fetch_one(&pool)
            .await
            .unwrap();
        assert_eq!(retained, 2);

        let mut completion = factory.begin().await.unwrap();
        registry
            .delete_previous_generation_targets(&mut completion, endpoint, watch, g2)
            .await
            .unwrap();
        registry
            .delete_previous_generation_targets(&mut completion, endpoint, watch, g2)
            .await
            .unwrap();
        registry
            .register_targets(&mut completion, endpoint, watch, g2, &[a])
            .await
            .unwrap();
        completion.commit().await.unwrap();
        let old: i64 = sqlx::query_scalar(
            "SELECT count(*) FROM read_model_watch_targets WHERE generation = $1",
        )
        .bind(g1.value())
        .fetch_one(&pool)
        .await
        .unwrap();
        assert_eq!(old, 0);
        let mut deletion = factory.begin().await.unwrap();
        registry
            .remove_watch(&mut deletion, endpoint, watch)
            .await
            .unwrap();
        registry
            .remove_watch(&mut deletion, endpoint, watch)
            .await
            .unwrap();
        deletion.commit().await.unwrap();
        let remaining: i64 = sqlx::query_scalar("SELECT count(*) FROM read_model_watch_targets")
            .fetch_one(&pool)
            .await
            .unwrap();
        assert_eq!(remaining, 0);
    }

    #[sqlx::test(migrations = "migrations/postgresql")]
    #[ignore = "requires PostgreSQL"]
    async fn ownership_and_expiration_are_checked_even_for_empty_targets(pool: PgPool) {
        let (endpoint, watch) = register(&pool).await;
        let (other_endpoint, other_watch) = register(&pool).await;
        let factory = PgUnitOfWorkFactory::new(pool.clone());
        let registry = PgReadModelWatchRegistry::new();
        let mut uow = factory.begin().await.unwrap();
        assert!(
            registry
                .remove_watch(&mut uow, other_endpoint, watch)
                .await
                .is_err()
        );
        assert!(
            registry
                .begin_generation(&mut uow, other_endpoint, watch)
                .await
                .is_err()
        );
        assert!(
            registry
                .contains_targets(
                    &mut uow,
                    endpoint,
                    watch,
                    ReadModelWatchGeneration::new(),
                    &[]
                )
                .await
                .unwrap()
        );
        uow.commit().await.unwrap();

        sqlx::query(
            r#"
            UPDATE read_model_watch_endpoints
            SET lease_until = clock_timestamp() - interval '1 second'
            WHERE endpoint_id = $1
            "#,
        )
        .bind(other_endpoint.value())
        .execute(&pool)
        .await
        .unwrap();
        let mut inactive = factory.begin().await.unwrap();
        assert!(
            registry
                .register_watch(&mut inactive, other_endpoint, ReadModelWatchId::new(),)
                .await
                .is_err()
        );
        assert!(
            registry
                .contains_targets(
                    &mut inactive,
                    other_endpoint,
                    other_watch,
                    ReadModelWatchGeneration::new(),
                    &[]
                )
                .await
                .is_err()
        );
        inactive.rollback().await.unwrap();
    }

    #[sqlx::test(migrations = "migrations/postgresql")]
    #[ignore = "requires PostgreSQL"]
    async fn generation_allocation_serializes_and_rollback_does_not_advance(pool: PgPool) {
        let (endpoint, watch) = register(&pool).await;
        let factory = PgUnitOfWorkFactory::new(pool.clone());
        let registry = PgReadModelWatchRegistry::new();
        let mut first = factory.begin().await.unwrap();
        let g1 = registry
            .begin_generation(&mut first, endpoint, watch)
            .await
            .unwrap();
        let mut second = factory.begin().await.unwrap();
        sqlx::query("SET LOCAL lock_timeout = '5s'")
            .execute(second.transaction_mut().as_mut())
            .await
            .unwrap();
        {
            let waiting = registry.begin_generation(&mut second, endpoint, watch);
            tokio::pin!(waiting);
            assert!(
                tokio::time::timeout(std::time::Duration::from_millis(100), &mut waiting)
                    .await
                    .is_err()
            );
            first.commit().await.unwrap();
            let g2 = waiting.await.unwrap();
            assert_eq!(g2.value(), g1.value() + 1);
        }
        second.rollback().await.unwrap();
        let mut retry = factory.begin().await.unwrap();
        assert_eq!(
            registry
                .begin_generation(&mut retry, endpoint, watch)
                .await
                .unwrap()
                .value(),
            g1.value() + 1
        );
        retry.commit().await.unwrap();
    }

    #[sqlx::test(migrations = "migrations/postgresql")]
    #[ignore = "requires PostgreSQL"]
    async fn endpoint_lock_allows_lease_extension_but_blocks_deletion(pool: PgPool) {
        let (endpoint, _) = register(&pool).await;
        let factory = PgUnitOfWorkFactory::new(pool);
        let registry = PgReadModelWatchRegistry::new();
        let mut holding = factory.begin().await.unwrap();
        registry
            .lock_endpoint_key_shared(&mut holding, endpoint)
            .await
            .unwrap();

        let mut extension = factory.begin().await.unwrap();
        sqlx::query("SET LOCAL lock_timeout = '1s'")
            .execute(extension.transaction_mut().as_mut())
            .await
            .unwrap();
        sqlx::query(
            r#"
            UPDATE read_model_watch_endpoints
            SET lease_until = lease_until + interval '1 hour'
            WHERE endpoint_id = $1
            "#,
        )
        .bind(endpoint.value())
        .execute(extension.transaction_mut().as_mut())
        .await
        .unwrap();
        extension.commit().await.unwrap();

        let mut deletion = factory.begin().await.unwrap();
        sqlx::query("SET LOCAL lock_timeout = '5s'")
            .execute(deletion.transaction_mut().as_mut())
            .await
            .unwrap();
        {
            let waiting =
                sqlx::query("DELETE FROM read_model_watch_endpoints WHERE endpoint_id = $1")
                    .bind(endpoint.value())
                    .execute(deletion.transaction_mut().as_mut());
            tokio::pin!(waiting);
            assert!(
                tokio::time::timeout(std::time::Duration::from_millis(100), &mut waiting)
                    .await
                    .is_err()
            );
            holding.commit().await.unwrap();
            assert_eq!(waiting.await.unwrap().rows_affected(), 1);
        }
        deletion.commit().await.unwrap();
    }

    #[sqlx::test(migrations = "migrations/postgresql")]
    #[ignore = "requires PostgreSQL"]
    async fn large_target_registration_is_atomic_across_batches(pool: PgPool) {
        let (endpoint, watch) = register(&pool).await;
        let factory = PgUnitOfWorkFactory::new(pool.clone());
        let registry = PgReadModelWatchRegistry::new();
        let targets: Vec<_> = (0..33_000).map(|id| target(&id.to_string())).collect();
        let mut initial = factory.begin().await.unwrap();
        let generation = registry
            .begin_generation(&mut initial, endpoint, watch)
            .await
            .unwrap();
        initial.commit().await.unwrap();

        let mut rolled_back = factory.begin().await.unwrap();
        registry
            .register_targets(&mut rolled_back, endpoint, watch, generation, &targets)
            .await
            .unwrap();
        assert!(
            registry
                .contains_targets(&mut rolled_back, endpoint, watch, generation, &targets)
                .await
                .unwrap()
        );
        rolled_back.rollback().await.unwrap();
        let count: i64 = sqlx::query_scalar("SELECT count(*) FROM read_model_watch_targets")
            .fetch_one(&pool)
            .await
            .unwrap();
        assert_eq!(count, 0);

        let mut committed = factory.begin().await.unwrap();
        registry
            .register_targets(&mut committed, endpoint, watch, generation, &[])
            .await
            .unwrap();
        registry
            .register_targets(&mut committed, endpoint, watch, generation, &targets)
            .await
            .unwrap();
        registry
            .register_targets(&mut committed, endpoint, watch, generation, &targets)
            .await
            .unwrap();
        committed.commit().await.unwrap();
        let persisted_count: i64 =
            sqlx::query_scalar("SELECT count(*) FROM read_model_watch_targets")
                .fetch_one(&pool)
                .await
                .unwrap();
        assert_eq!(persisted_count, 33_000);

        let mut verification = factory.begin().await.unwrap();
        let mut missing_last = targets.clone();
        missing_last.push(target("missing"));
        assert!(
            !registry
                .contains_targets(
                    &mut verification,
                    endpoint,
                    watch,
                    generation,
                    &missing_last
                )
                .await
                .unwrap()
        );
        verification.commit().await.unwrap();
    }
}
