use appletheia_application::read_model::{
    ReadModelRef,
    watch::{
        ReadModelWatchEndpointId, ReadModelWatchEndpointPage, ReadModelWatchEndpointPageSize,
        ReadModelWatchId, ReadModelWatchLookup, ReadModelWatchLookupError, ReadModelWatchPage,
        ReadModelWatchPageSize,
    },
};
use chrono::Utc;
use sqlx::{Postgres, QueryBuilder};
use uuid::Uuid;

use crate::postgresql::unit_of_work::PgUnitOfWork;

#[derive(Default)]
pub struct PgReadModelWatchLookup;

impl PgReadModelWatchLookup {
    pub fn new() -> Self {
        Self
    }
}

impl ReadModelWatchLookup for PgReadModelWatchLookup {
    type Uow = PgUnitOfWork;

    async fn find_endpoint_ids_by_target(
        &self,
        uow: &mut Self::Uow,
        target: &ReadModelRef,
        cursor: Option<ReadModelWatchEndpointId>,
        limit: ReadModelWatchEndpointPageSize,
    ) -> Result<ReadModelWatchEndpointPage, ReadModelWatchLookupError> {
        let now = Utc::now();
        let mut query_builder = QueryBuilder::<Postgres>::new(
            r#"
            SELECT DISTINCT endpoint.endpoint_id
            FROM read_model_watch_targets AS target
            JOIN read_model_watches AS watch ON watch.watch_id = target.watch_id
            JOIN read_model_watch_endpoints AS endpoint ON endpoint.endpoint_id = watch.endpoint_id
            WHERE target.read_model_type = "#,
        );
        query_builder
            .push_bind(target.read_model_type.value())
            .push(" AND target.read_model_id = ")
            .push_bind(target.id.value())
            .push(" AND endpoint.lease_until > ")
            .push_bind(now)
            .push(" AND watch.lease_until > ")
            .push_bind(now);

        if let Some(cursor) = cursor {
            query_builder
                .push(" AND endpoint.endpoint_id > ")
                .push_bind(cursor.value());
        }
        let ids = query_builder
            .push(" ORDER BY endpoint.endpoint_id LIMIT ")
            .push_bind(limit.as_i64() + 1)
            .build_query_scalar::<Uuid>()
            .fetch_all(uow.transaction_mut().as_mut())
            .await
            .map_err(|source| ReadModelWatchLookupError::Backend(Box::new(source)))?;
        let mut endpoint_ids = ids
            .into_iter()
            .map(ReadModelWatchEndpointId::try_from)
            .collect::<Result<Vec<_>, _>>()
            .map_err(|source| ReadModelWatchLookupError::Backend(Box::new(source)))?;
        let page_limit = limit.as_usize();
        let next_cursor = if endpoint_ids.len() > page_limit {
            endpoint_ids.truncate(page_limit);
            endpoint_ids.last().copied()
        } else {
            None
        };
        Ok(ReadModelWatchEndpointPage::new(endpoint_ids, next_cursor))
    }

    async fn find_watch_ids_by_target(
        &self,
        uow: &mut Self::Uow,
        endpoint_id: ReadModelWatchEndpointId,
        target: &ReadModelRef,
        cursor: Option<ReadModelWatchId>,
        limit: ReadModelWatchPageSize,
    ) -> Result<ReadModelWatchPage, ReadModelWatchLookupError> {
        let now = Utc::now();
        let mut query_builder = QueryBuilder::<Postgres>::new(
            r#"
            SELECT DISTINCT watch.watch_id
            FROM read_model_watch_targets AS target
            JOIN read_model_watches AS watch ON watch.watch_id = target.watch_id
            JOIN read_model_watch_endpoints AS endpoint ON endpoint.endpoint_id = watch.endpoint_id
            WHERE target.read_model_type = "#,
        );
        query_builder
            .push_bind(target.read_model_type.value())
            .push(" AND target.read_model_id = ")
            .push_bind(target.id.value())
            .push(" AND endpoint.lease_until > ")
            .push_bind(now)
            .push(" AND watch.lease_until > ")
            .push_bind(now);

        query_builder
            .push(" AND endpoint.endpoint_id = ")
            .push_bind(endpoint_id.value());

        if let Some(cursor) = cursor {
            query_builder
                .push(" AND watch.watch_id > ")
                .push_bind(cursor.value());
        }
        let ids = query_builder
            .push(" ORDER BY watch.watch_id LIMIT ")
            .push_bind(limit.as_i64() + 1)
            .build_query_scalar::<Uuid>()
            .fetch_all(uow.transaction_mut().as_mut())
            .await
            .map_err(|source| ReadModelWatchLookupError::Backend(Box::new(source)))?;
        let mut watch_ids = ids
            .into_iter()
            .map(ReadModelWatchId::try_from)
            .collect::<Result<Vec<_>, _>>()
            .map_err(|source| ReadModelWatchLookupError::Backend(Box::new(source)))?;
        let page_limit = limit.as_usize();
        let next_cursor = if watch_ids.len() > page_limit {
            watch_ids.truncate(page_limit);
            watch_ids.last().copied()
        } else {
            None
        };
        Ok(ReadModelWatchPage::new(watch_ids, next_cursor))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::num::NonZeroU32;

    use appletheia_application::read_model::{ReadModelId, ReadModelTypeOwned};
    use appletheia_application::unit_of_work::{UnitOfWork, UnitOfWorkFactory};
    use chrono::Duration;
    use sqlx::PgPool;

    use crate::postgresql::unit_of_work::PgUnitOfWorkFactory;

    #[sqlx::test(migrations = "migrations/postgresql")]
    #[ignore = "requires PostgreSQL"]
    async fn lookup_paginates_distinct_active_targets_across_retained_generations(pool: PgPool) {
        let lookup = PgReadModelWatchLookup::new();
        let factory = PgUnitOfWorkFactory::new(pool.clone());
        let target = ReadModelRef {
            read_model_type: ReadModelTypeOwned::try_from("user").unwrap(),
            id: ReadModelId::from("one"),
        };
        let mut endpoints = [
            ReadModelWatchEndpointId::new(),
            ReadModelWatchEndpointId::new(),
            ReadModelWatchEndpointId::new(),
        ];
        endpoints.sort();
        let mut watches = [
            ReadModelWatchId::new(),
            ReadModelWatchId::new(),
            ReadModelWatchId::new(),
            ReadModelWatchId::new(),
            ReadModelWatchId::new(),
        ];
        watches.sort();
        let active_until = Utc::now() + Duration::hours(1);
        let expired_until = Utc::now() - Duration::hours(1);
        for (index, endpoint) in endpoints.iter().enumerate() {
            sqlx::query(
                "INSERT INTO read_model_watch_endpoints (endpoint_id, lease_until) VALUES ($1, $2)",
            )
            .bind(endpoint.value())
            .bind(if index == 2 {
                expired_until
            } else {
                active_until
            })
            .execute(&pool)
            .await
            .unwrap();
        }
        for (index, watch) in watches.iter().enumerate() {
            let endpoint = match index {
                0 | 1 | 4 => endpoints[0],
                2 => endpoints[1],
                _ => endpoints[2],
            };
            sqlx::query(
                r#"
                INSERT INTO read_model_watches (watch_id, endpoint_id, lease_until, generation)
                VALUES ($1, $2, $3, 2)
                "#,
            )
            .bind(watch.value())
            .bind(endpoint.value())
            .bind(if index == 4 {
                expired_until
            } else {
                active_until
            })
            .execute(&pool)
            .await
            .unwrap();
            // The second watch matches only an older, retained generation.
            for generation in 1..=if index == 1 { 1 } else { 2 } {
                sqlx::query(
                    r#"
                    INSERT INTO read_model_watch_targets
                        (watch_id, generation, read_model_type, read_model_id)
                    VALUES ($1, $2, $3, $4)
                    "#,
                )
                .bind(watch.value())
                .bind(i64::from(generation))
                .bind(target.read_model_type.value())
                .bind(target.id.value())
                .execute(&pool)
                .await
                .unwrap();
            }
        }
        let endpoint_limit = ReadModelWatchEndpointPageSize::new(NonZeroU32::new(1).unwrap());
        let watch_limit = ReadModelWatchPageSize::new(NonZeroU32::new(1).unwrap());
        let mut uow = factory.begin().await.unwrap();
        let first_endpoints = lookup
            .find_endpoint_ids_by_target(&mut uow, &target, None, endpoint_limit)
            .await
            .unwrap();
        assert_eq!(first_endpoints.endpoint_ids, [endpoints[0]]);
        assert_eq!(first_endpoints.next_cursor, Some(endpoints[0]));
        let last_endpoints = lookup
            .find_endpoint_ids_by_target(
                &mut uow,
                &target,
                first_endpoints.next_cursor,
                endpoint_limit,
            )
            .await
            .unwrap();
        assert_eq!(last_endpoints.endpoint_ids, [endpoints[1]]);
        assert!(last_endpoints.is_last());
        let first_watches = lookup
            .find_watch_ids_by_target(&mut uow, endpoints[0], &target, None, watch_limit)
            .await
            .unwrap();
        assert_eq!(first_watches.watch_ids, [watches[0]]);
        assert_eq!(first_watches.next_cursor, Some(watches[0]));
        let last_watches = lookup
            .find_watch_ids_by_target(
                &mut uow,
                endpoints[0],
                &target,
                first_watches.next_cursor,
                watch_limit,
            )
            .await
            .unwrap();
        assert_eq!(last_watches.watch_ids, [watches[1]]);
        assert!(last_watches.is_last());
        let exhausted = lookup
            .find_watch_ids_by_target(
                &mut uow,
                endpoints[0],
                &target,
                Some(watches[1]),
                watch_limit,
            )
            .await
            .unwrap();
        assert!(exhausted.watch_ids.is_empty());
        assert!(exhausted.is_last());
        for endpoint in [endpoints[2], ReadModelWatchEndpointId::new()] {
            let empty = lookup
                .find_watch_ids_by_target(&mut uow, endpoint, &target, None, watch_limit)
                .await
                .unwrap();
            assert!(empty.watch_ids.is_empty());
            assert!(empty.is_last());
        }
        for missing in [
            ReadModelRef {
                read_model_type: ReadModelTypeOwned::try_from("account").unwrap(),
                id: target.id.clone(),
            },
            ReadModelRef {
                read_model_type: target.read_model_type.clone(),
                id: ReadModelId::from("missing"),
            },
        ] {
            let empty_endpoints = lookup
                .find_endpoint_ids_by_target(&mut uow, &missing, None, endpoint_limit)
                .await
                .unwrap();
            assert!(empty_endpoints.endpoint_ids.is_empty());
            assert!(empty_endpoints.is_last());
            let empty_watches = lookup
                .find_watch_ids_by_target(&mut uow, endpoints[0], &missing, None, watch_limit)
                .await
                .unwrap();
            assert!(empty_watches.watch_ids.is_empty());
            assert!(empty_watches.is_last());
        }
        uow.commit().await.unwrap();
    }
}
