use appletheia_application::read_model::watch::{
    ReadModelWatchEndpointId, ReadModelWatchEndpointLeaseExpiresAt, ReadModelWatchEndpointRegistry,
    ReadModelWatchEndpointRegistryError,
};
use chrono::Utc;

use crate::postgresql::unit_of_work::PgUnitOfWork;

use super::{
    pg_read_model_watch_endpoint_registry_error::PgReadModelWatchEndpointRegistryError,
    pg_read_model_watch_endpoint_row::PgReadModelWatchEndpointRow,
};

#[derive(Default)]
pub struct PgReadModelWatchEndpointRegistry;

impl PgReadModelWatchEndpointRegistry {
    pub fn new() -> Self {
        Self
    }
}

impl ReadModelWatchEndpointRegistry for PgReadModelWatchEndpointRegistry {
    type Uow = PgUnitOfWork;

    async fn register_endpoint(
        &self,
        uow: &mut Self::Uow,
        endpoint_id: ReadModelWatchEndpointId,
        lease_until: ReadModelWatchEndpointLeaseExpiresAt,
    ) -> Result<(), ReadModelWatchEndpointRegistryError> {
        sqlx::query(
            r#"
            INSERT INTO read_model_watch_endpoints (endpoint_id, lease_until)
            VALUES ($1, $2)
            "#,
        )
        .bind(endpoint_id.value())
        .bind(lease_until.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|source| ReadModelWatchEndpointRegistryError::Backend(Box::new(source)))?;
        Ok(())
    }

    async fn extend_endpoint_lease(
        &self,
        uow: &mut Self::Uow,
        endpoint_id: ReadModelWatchEndpointId,
        lease_until: ReadModelWatchEndpointLeaseExpiresAt,
    ) -> Result<(), ReadModelWatchEndpointRegistryError> {
        let endpoint = sqlx::query_as::<_, PgReadModelWatchEndpointRow>(
            r#"
            SELECT endpoint_id, lease_until
            FROM read_model_watch_endpoints
            WHERE endpoint_id = $1
            FOR NO KEY UPDATE
            "#,
        )
        .bind(endpoint_id.value())
        .fetch_optional(uow.transaction_mut().as_mut())
        .await
        .map_err(|source| ReadModelWatchEndpointRegistryError::Backend(Box::new(source)))?
        .ok_or_else(|| {
            ReadModelWatchEndpointRegistryError::Backend(Box::new(
                PgReadModelWatchEndpointRegistryError::EndpointInactive,
            ))
        })?;
        let now = Utc::now();
        if ReadModelWatchEndpointLeaseExpiresAt::from(endpoint.lease_until).is_expired_at(now) {
            return Err(ReadModelWatchEndpointRegistryError::Backend(Box::new(
                PgReadModelWatchEndpointRegistryError::EndpointInactive,
            )));
        }
        sqlx::query(
            r#"
            UPDATE read_model_watch_endpoints
            SET lease_until = GREATEST(lease_until, $2)
            WHERE endpoint_id = $1
            "#,
        )
        .bind(endpoint_id.value())
        .bind(lease_until.value())
        .execute(uow.transaction_mut().as_mut())
        .await
        .map_err(|source| ReadModelWatchEndpointRegistryError::Backend(Box::new(source)))?;
        Ok(())
    }

    async fn remove_endpoint(
        &self,
        uow: &mut Self::Uow,
        endpoint_id: ReadModelWatchEndpointId,
    ) -> Result<(), ReadModelWatchEndpointRegistryError> {
        sqlx::query("DELETE FROM read_model_watch_endpoints WHERE endpoint_id = $1")
            .bind(endpoint_id.value())
            .execute(uow.transaction_mut().as_mut())
            .await
            .map_err(|source| ReadModelWatchEndpointRegistryError::Backend(Box::new(source)))?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::postgresql::{
        read_model::PgReadModelWatchRegistry, unit_of_work::PgUnitOfWorkFactory,
    };
    use appletheia_application::{
        read_model::{
            ReadModelId, ReadModelRef, ReadModelTypeOwned,
            watch::{ReadModelWatchId, ReadModelWatchRegistry},
        },
        unit_of_work::{UnitOfWork, UnitOfWorkFactory},
    };
    use chrono::{DateTime, Duration};
    use sqlx::PgPool;

    #[sqlx::test(migrations = "migrations/postgresql")]
    #[ignore = "requires PostgreSQL"]
    async fn registration_and_removal_obey_transaction_boundaries(pool: PgPool) {
        let registry = PgReadModelWatchEndpointRegistry::new();
        let factory = PgUnitOfWorkFactory::new(pool.clone());
        let endpoint = ReadModelWatchEndpointId::new();
        let lease = ReadModelWatchEndpointLeaseExpiresAt::from(Utc::now() + Duration::hours(1));
        let mut rolled_back = factory.begin().await.unwrap();
        registry
            .register_endpoint(&mut rolled_back, endpoint, lease)
            .await
            .unwrap();
        rolled_back.rollback().await.unwrap();

        let mut registration = factory.begin().await.unwrap();
        registry
            .register_endpoint(&mut registration, endpoint, lease)
            .await
            .unwrap();
        let watch_registry = PgReadModelWatchRegistry::new();
        let watch = ReadModelWatchId::new();
        watch_registry
            .register_watch(&mut registration, endpoint, watch)
            .await
            .unwrap();
        let generation = watch_registry
            .begin_generation(&mut registration, endpoint, watch)
            .await
            .unwrap();
        watch_registry
            .register_targets(
                &mut registration,
                endpoint,
                watch,
                generation,
                &[ReadModelRef {
                    read_model_type: ReadModelTypeOwned::try_from("user").unwrap(),
                    id: ReadModelId::from("one"),
                }],
            )
            .await
            .unwrap();
        registration.commit().await.unwrap();

        let mut duplicate = factory.begin().await.unwrap();
        assert!(
            registry
                .register_endpoint(&mut duplicate, endpoint, lease)
                .await
                .is_err()
        );
        duplicate.rollback().await.unwrap();

        let mut rolled_back_removal = factory.begin().await.unwrap();
        registry
            .remove_endpoint(&mut rolled_back_removal, endpoint)
            .await
            .unwrap();
        rolled_back_removal.rollback().await.unwrap();
        let retained: i64 = sqlx::query_scalar("SELECT count(*) FROM read_model_watch_targets")
            .fetch_one(&pool)
            .await
            .unwrap();
        assert_eq!(retained, 1);

        let mut removal = factory.begin().await.unwrap();
        registry
            .remove_endpoint(&mut removal, endpoint)
            .await
            .unwrap();
        registry
            .remove_endpoint(&mut removal, endpoint)
            .await
            .unwrap();
        removal.commit().await.unwrap();
        let remaining: i64 = sqlx::query_scalar(
            r#"
            SELECT (SELECT count(*) FROM read_model_watch_endpoints)
                 + (SELECT count(*) FROM read_model_watches)
                 + (SELECT count(*) FROM read_model_watch_targets)
            "#,
        )
        .fetch_one(&pool)
        .await
        .unwrap();
        assert_eq!(remaining, 0);
    }

    #[sqlx::test(migrations = "migrations/postgresql")]
    #[ignore = "requires PostgreSQL"]
    async fn lease_extension_preserves_longer_leases_and_rejects_inactive_endpoints(pool: PgPool) {
        let registry = PgReadModelWatchEndpointRegistry::new();
        let factory = PgUnitOfWorkFactory::new(pool.clone());
        let endpoint = ReadModelWatchEndpointId::new();
        let lease = ReadModelWatchEndpointLeaseExpiresAt::from(Utc::now() + Duration::hours(1));
        let mut uow = factory.begin().await.unwrap();
        registry
            .register_endpoint(&mut uow, endpoint, lease)
            .await
            .unwrap();
        uow.commit().await.unwrap();

        let mut holding = factory.begin().await.unwrap();
        sqlx::query(
            r#"
            SELECT endpoint_id
            FROM read_model_watch_endpoints
            WHERE endpoint_id = $1
            FOR KEY SHARE
            "#,
        )
        .bind(endpoint.value())
        .fetch_one(holding.transaction_mut().as_mut())
        .await
        .unwrap();
        let mut extension = factory.begin().await.unwrap();
        sqlx::query("SET LOCAL lock_timeout = '1s'")
            .execute(extension.transaction_mut().as_mut())
            .await
            .unwrap();
        let longer = ReadModelWatchEndpointLeaseExpiresAt::from(lease.value() + Duration::hours(1));
        registry
            .extend_endpoint_lease(&mut extension, endpoint, longer)
            .await
            .unwrap();
        registry
            .extend_endpoint_lease(&mut extension, endpoint, lease)
            .await
            .unwrap();
        extension.commit().await.unwrap();
        holding.commit().await.unwrap();
        let stored: DateTime<Utc> = sqlx::query_scalar(
            "SELECT lease_until FROM read_model_watch_endpoints WHERE endpoint_id = $1",
        )
        .bind(endpoint.value())
        .fetch_one(&pool)
        .await
        .unwrap();
        assert_eq!(stored.timestamp_micros(), longer.value().timestamp_micros());

        let mut expired = factory.begin().await.unwrap();
        let expired_id = ReadModelWatchEndpointId::new();
        registry
            .register_endpoint(
                &mut expired,
                expired_id,
                ReadModelWatchEndpointLeaseExpiresAt::from(Utc::now() - Duration::hours(1)),
            )
            .await
            .unwrap();
        expired.commit().await.unwrap();
        let mut duplicate = factory.begin().await.unwrap();
        assert!(
            registry
                .register_endpoint(&mut duplicate, expired_id, longer)
                .await
                .is_err()
        );
        duplicate.rollback().await.unwrap();
        let mut inactive = factory.begin().await.unwrap();
        for id in [expired_id, ReadModelWatchEndpointId::new()] {
            let error = registry
                .extend_endpoint_lease(&mut inactive, id, longer)
                .await
                .unwrap_err();
            let ReadModelWatchEndpointRegistryError::Backend(source) = error;
            assert!(matches!(
                source.downcast_ref::<PgReadModelWatchEndpointRegistryError>(),
                Some(PgReadModelWatchEndpointRegistryError::EndpointInactive)
            ));
        }
        registry
            .remove_endpoint(&mut inactive, expired_id)
            .await
            .unwrap();
        inactive.commit().await.unwrap();
    }
}
