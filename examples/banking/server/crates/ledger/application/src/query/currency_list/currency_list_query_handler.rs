use appletheia::application::authorization::AuthorizationPlan;
use appletheia::application::projection::{ProjectorDependencies, ProjectorSpec};
use appletheia::application::query::QueryHandler;
use appletheia::application::request_context::RequestContext;

use crate::projection::CurrencyListProjectorSpec;
use crate::read_model::{CurrencyList, CurrencyListReader};

use super::{CurrencyListQuery, CurrencyListQueryHandlerError};

/// Handles public currency list queries.
pub struct CurrencyListQueryHandler<S>
where
    S: CurrencyListReader,
{
    store: S,
}

impl<S> CurrencyListQueryHandler<S>
where
    S: CurrencyListReader,
{
    pub fn new(store: S) -> Self {
        Self { store }
    }
}

impl<S> QueryHandler for CurrencyListQueryHandler<S>
where
    S: CurrencyListReader,
{
    type Query = CurrencyListQuery;
    type Output = CurrencyList;
    type Error = CurrencyListQueryHandlerError;
    type Uow = S::Uow;

    const PROJECTOR_DEPENDENCIES: ProjectorDependencies<'static> =
        ProjectorDependencies::Some(&[CurrencyListProjectorSpec::DESCRIPTOR]);

    fn authorization_plan(&self, _query: &Self::Query) -> Result<AuthorizationPlan, Self::Error> {
        Ok(AuthorizationPlan::None)
    }

    async fn handle(
        &self,
        uow: &mut S::Uow,
        _request_context: &RequestContext,
        query: Self::Query,
    ) -> Result<Self::Output, Self::Error> {
        Ok(self
            .store
            .list(uow, query.criteria, query.cursor_options, query.limit)
            .await?)
    }
}
