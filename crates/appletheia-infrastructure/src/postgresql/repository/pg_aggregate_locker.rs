use crate::postgresql::unit_of_work::PgUnitOfWork;
use appletheia_application::{
    aggregate::AggregateRef,
    repository::{AggregateLockMode, AggregateLocker, AggregateLockerError},
};

use sqlx::{Postgres, QueryBuilder};

#[derive(Default)]
pub struct PgAggregateLocker;

impl PgAggregateLocker {
    pub fn new() -> Self {
        Self
    }
}

impl AggregateLocker for PgAggregateLocker {
    type Uow = PgUnitOfWork;

    async fn lock(
        &self,
        uow: &mut Self::Uow,
        aggregate: &AggregateRef,
        lock_mode: AggregateLockMode,
    ) -> Result<(), AggregateLockerError> {
        if let Some(held) = uow.aggregate_lock_mode(aggregate) {
            if held == AggregateLockMode::Shared && lock_mode == AggregateLockMode::Exclusive {
                return Err(AggregateLockerError::SharedLockUpgrade {
                    aggregate: aggregate.clone(),
                });
            }
            return Ok(());
        }

        let mut query = QueryBuilder::<Postgres>::new(
            "SELECT aggregate_id FROM aggregate_locks WHERE aggregate_type = ",
        );
        query
            .push_bind(aggregate.aggregate_type.value())
            .push(" AND aggregate_id = ")
            .push_bind(aggregate.aggregate_id.value())
            .push(match lock_mode {
                AggregateLockMode::Shared => " FOR SHARE",
                AggregateLockMode::Exclusive => " FOR UPDATE",
            });
        let row = query
            .build()
            .fetch_optional(uow.transaction_mut().as_mut())
            .await
            .map_err(|source| AggregateLockerError::Persistence(Box::new(source)))?;
        if row.is_none() {
            return Err(AggregateLockerError::NotFound {
                aggregate: aggregate.clone(),
            });
        }
        uow.record_aggregate_lock(aggregate.clone(), lock_mode);
        Ok(())
    }

    async fn create_if_not_exists(
        &self,
        uow: &mut Self::Uow,
        aggregate: &AggregateRef,
    ) -> Result<(), AggregateLockerError> {
        if let Some(held) = uow.aggregate_lock_mode(aggregate) {
            if held == AggregateLockMode::Shared {
                return Err(AggregateLockerError::SharedLockUpgrade {
                    aggregate: aggregate.clone(),
                });
            }
            return Ok(());
        }

        let row = sqlx::query(
            "SELECT aggregate_id FROM aggregate_locks WHERE aggregate_type = $1 AND aggregate_id = $2 FOR UPDATE",
        )
        .bind(aggregate.aggregate_type.value())
        .bind(aggregate.aggregate_id.value())
        .fetch_optional(uow.transaction_mut().as_mut())
        .await
        .map_err(|source| AggregateLockerError::Persistence(Box::new(source)))?;
        if row.is_none() {
            sqlx::query(
                "INSERT INTO aggregate_locks (aggregate_type, aggregate_id) VALUES ($1, $2)",
            )
            .bind(aggregate.aggregate_type.value())
            .bind(aggregate.aggregate_id.value())
            .execute(uow.transaction_mut().as_mut())
            .await
            .map_err(|source| AggregateLockerError::Persistence(Box::new(source)))?;
        }
        uow.record_aggregate_lock(aggregate.clone(), AggregateLockMode::Exclusive);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::postgresql::unit_of_work::PgUnitOfWorkFactory;
    use appletheia_application::{
        aggregate::{AggregateIdValue, AggregateTypeOwned},
        unit_of_work::{UnitOfWork, UnitOfWorkFactory},
    };
    use sqlx::PgPool;
    use uuid::Uuid;

    fn aggregate_ref() -> AggregateRef {
        AggregateRef::new(
            AggregateTypeOwned::try_from("counter").expect("valid type"),
            AggregateIdValue::from(Uuid::now_v7()),
        )
    }

    #[sqlx::test(migrations = "migrations/postgresql")]
    #[ignore = "requires PostgreSQL"]
    async fn missing_lock_is_not_created_and_creation_rolls_back(pool: PgPool) {
        let factory = PgUnitOfWorkFactory::new(pool.clone());
        let locker = PgAggregateLocker::new();
        let aggregate = aggregate_ref();
        let mut uow = factory.begin().await.unwrap();
        assert!(matches!(
            locker
                .lock(&mut uow, &aggregate, AggregateLockMode::Shared)
                .await,
            Err(AggregateLockerError::NotFound { .. })
        ));
        let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM aggregate_locks")
            .fetch_one(uow.transaction_mut().as_mut())
            .await
            .unwrap();
        assert_eq!(count, 0);
        assert_eq!(uow.aggregate_lock_mode(&aggregate), None);
        locker
            .create_if_not_exists(&mut uow, &aggregate)
            .await
            .unwrap();
        locker
            .create_if_not_exists(&mut uow, &aggregate)
            .await
            .unwrap();
        assert_eq!(
            uow.aggregate_lock_mode(&aggregate),
            Some(AggregateLockMode::Exclusive)
        );
        locker
            .lock(&mut uow, &aggregate, AggregateLockMode::Shared)
            .await
            .unwrap();
        uow.rollback().await.unwrap();
        let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM aggregate_locks")
            .fetch_one(&pool)
            .await
            .unwrap();
        assert_eq!(count, 0);
    }

    #[sqlx::test(migrations = "migrations/postgresql")]
    #[ignore = "requires PostgreSQL"]
    async fn concurrent_creation_waits_then_rejects_duplicate(pool: PgPool) {
        let factory = PgUnitOfWorkFactory::new(pool);
        let locker = PgAggregateLocker::new();
        let aggregate = aggregate_ref();
        let mut first = factory.begin().await.unwrap();
        locker
            .create_if_not_exists(&mut first, &aggregate)
            .await
            .unwrap();
        let mut second = factory.begin().await.unwrap();
        sqlx::query("SET LOCAL lock_timeout = '5s'")
            .execute(second.transaction_mut().as_mut())
            .await
            .unwrap();
        {
            let waiting = locker.create_if_not_exists(&mut second, &aggregate);
            tokio::pin!(waiting);
            assert!(
                tokio::time::timeout(std::time::Duration::from_millis(100), &mut waiting)
                    .await
                    .is_err()
            );
            first.commit().await.unwrap();
            let error = waiting.await.unwrap_err();
            let AggregateLockerError::Persistence(source) = error else {
                panic!("expected duplicate key")
            };
            let database_error = source.downcast_ref::<sqlx::Error>().unwrap();
            assert_eq!(
                database_error
                    .as_database_error()
                    .unwrap()
                    .code()
                    .as_deref(),
                Some("23505")
            );
        }
        second.rollback().await.unwrap();
    }

    #[sqlx::test(migrations = "migrations/postgresql")]
    #[ignore = "requires PostgreSQL"]
    async fn shared_locks_coexist_and_block_exclusive_locks(pool: PgPool) {
        let factory = PgUnitOfWorkFactory::new(pool);
        let locker = PgAggregateLocker::new();
        let aggregate = aggregate_ref();
        let mut seed = factory.begin().await.unwrap();
        locker
            .create_if_not_exists(&mut seed, &aggregate)
            .await
            .unwrap();
        seed.commit().await.unwrap();

        let mut first = factory.begin().await.unwrap();
        let mut second = factory.begin().await.unwrap();
        locker
            .lock(&mut first, &aggregate, AggregateLockMode::Shared)
            .await
            .unwrap();
        sqlx::query("SET LOCAL lock_timeout = '100ms'")
            .execute(second.transaction_mut().as_mut())
            .await
            .unwrap();
        locker
            .lock(&mut second, &aggregate, AggregateLockMode::Shared)
            .await
            .unwrap();
        assert!(matches!(
            locker
                .lock(&mut second, &aggregate, AggregateLockMode::Exclusive)
                .await,
            Err(AggregateLockerError::SharedLockUpgrade { .. })
        ));
        assert!(matches!(
            locker.create_if_not_exists(&mut second, &aggregate).await,
            Err(AggregateLockerError::SharedLockUpgrade { .. })
        ));
        second.rollback().await.unwrap();

        let mut writer = factory.begin().await.unwrap();
        sqlx::query("SET LOCAL lock_timeout = '100ms'")
            .execute(writer.transaction_mut().as_mut())
            .await
            .unwrap();
        let error = locker
            .lock(&mut writer, &aggregate, AggregateLockMode::Exclusive)
            .await
            .unwrap_err();
        let AggregateLockerError::Persistence(source) = error else {
            panic!("expected database lock timeout")
        };
        let error = source.downcast_ref::<sqlx::Error>().unwrap();
        assert_eq!(
            error.as_database_error().unwrap().code().as_deref(),
            Some("55P03")
        );
        writer.rollback().await.unwrap();
        first.rollback().await.unwrap();

        let mut writer = factory.begin().await.unwrap();
        locker
            .lock(&mut writer, &aggregate, AggregateLockMode::Exclusive)
            .await
            .unwrap();
        locker
            .lock(&mut writer, &aggregate, AggregateLockMode::Shared)
            .await
            .unwrap();
        assert_eq!(
            writer.aggregate_lock_mode(&aggregate),
            Some(AggregateLockMode::Exclusive)
        );
        writer.commit().await.unwrap();
    }

    #[sqlx::test(migrations = "migrations/postgresql")]
    #[ignore = "requires PostgreSQL"]
    async fn create_if_not_exists_locks_existing_row_and_sees_committed_changes(pool: PgPool) {
        let factory = PgUnitOfWorkFactory::new(pool.clone());
        let locker = PgAggregateLocker::new();
        let aggregate = aggregate_ref();
        sqlx::query("CREATE TABLE lock_test_values (value INTEGER NOT NULL)")
            .execute(&pool)
            .await
            .unwrap();
        let mut seed = factory.begin().await.unwrap();
        locker
            .create_if_not_exists(&mut seed, &aggregate)
            .await
            .unwrap();
        seed.commit().await.unwrap();
        let mut first = factory.begin().await.unwrap();
        locker
            .lock(&mut first, &aggregate, AggregateLockMode::Exclusive)
            .await
            .unwrap();
        sqlx::query("INSERT INTO lock_test_values VALUES (42)")
            .execute(first.transaction_mut().as_mut())
            .await
            .unwrap();
        let mut second = factory.begin().await.unwrap();
        sqlx::query("SET LOCAL lock_timeout = '5s'")
            .execute(second.transaction_mut().as_mut())
            .await
            .unwrap();
        {
            let waiting = locker.create_if_not_exists(&mut second, &aggregate);
            tokio::pin!(waiting);
            assert!(
                tokio::time::timeout(std::time::Duration::from_millis(100), &mut waiting)
                    .await
                    .is_err()
            );
            first.commit().await.unwrap();
            waiting.await.unwrap();
        }
        let value: i32 = sqlx::query_scalar("SELECT value FROM lock_test_values")
            .fetch_one(second.transaction_mut().as_mut())
            .await
            .unwrap();
        assert_eq!(value, 42);
        second.commit().await.unwrap();
        let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM aggregate_locks")
            .fetch_one(&pool)
            .await
            .unwrap();
        assert_eq!(count, 1);
    }
}
