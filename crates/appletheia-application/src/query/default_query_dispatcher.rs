use crate::authorization::Authorizer;
use crate::projection::ProjectionConsistencyWaiter;
use crate::request_context::RequestContext;
use crate::unit_of_work::{UnitOfWork, UnitOfWorkFactory};

use super::{QueryConsistency, QueryDispatcher, QueryDispatcherError, QueryHandler, QueryOptions};

pub struct DefaultQueryDispatcher<W, U, AZ>
where
    W: ProjectionConsistencyWaiter,
    U: UnitOfWorkFactory,
    U::Uow: UnitOfWork,
{
    projection_consistency_waiter: W,
    uow_factory: U,
    authorizer: AZ,
}

impl<W, U, AZ> DefaultQueryDispatcher<W, U, AZ>
where
    W: ProjectionConsistencyWaiter,
    U: UnitOfWorkFactory,
    U::Uow: UnitOfWork,
    AZ: Authorizer,
{
    pub fn new(projection_consistency_waiter: W, uow_factory: U, authorizer: AZ) -> Self {
        Self {
            projection_consistency_waiter,
            uow_factory,
            authorizer,
        }
    }
}

impl<W, U, AZ> QueryDispatcher for DefaultQueryDispatcher<W, U, AZ>
where
    W: ProjectionConsistencyWaiter,
    U: UnitOfWorkFactory,
    U::Uow: UnitOfWork,
    AZ: Authorizer,
{
    type Uow = U::Uow;

    async fn dispatch<H>(
        &self,
        handler: &H,
        request_context: &RequestContext,
        query: H::Query,
        options: QueryOptions,
    ) -> Result<H::Output, QueryDispatcherError<H::Error>>
    where
        H: QueryHandler<Uow = Self::Uow>,
    {
        let authorization_plan = handler
            .authorization_plan(&query)
            .map_err(QueryDispatcherError::Handler)?;
        self.authorizer
            .authorize(&request_context.principal, &authorization_plan)
            .await?;

        match options.consistency {
            QueryConsistency::Eventual => {}
            QueryConsistency::AfterMessage {
                message_id,
                timeout,
                poll_interval,
            } => {
                self.projection_consistency_waiter
                    .wait_for_message(
                        message_id,
                        timeout,
                        poll_interval,
                        H::PROJECTOR_DEPENDENCIES,
                    )
                    .await?;
            }
            QueryConsistency::AfterEvents {
                event_ids,
                timeout,
                poll_interval,
            } => {
                self.projection_consistency_waiter
                    .wait_for_events(
                        &event_ids,
                        timeout,
                        poll_interval,
                        H::PROJECTOR_DEPENDENCIES,
                    )
                    .await?;
            }
        }

        let mut uow = self.uow_factory.begin().await?;

        let result = handler.handle(&mut uow, request_context, query).await;
        match result {
            Ok(output) => {
                uow.commit().await?;
                Ok(output)
            }
            Err(operation_error) => {
                let operation_error = uow
                    .rollback_with_operation_error(operation_error)
                    .await
                    .map_err(QueryDispatcherError::UnitOfWork)?;
                Err(QueryDispatcherError::Handler(operation_error))
            }
        }
    }
}
