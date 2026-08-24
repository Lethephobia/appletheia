use appletheia::application::authorization::AuthorizationPlan;
use appletheia::application::projection::{ProjectorDependencies, ProjectorSpec};
use appletheia::application::query::QueryHandler;
use appletheia::application::request_context::RequestContext;

use super::{CurrencyListQuery, CurrencyListQueryHandlerError};
use crate::projection::CurrencyFragmentProjectorSpec;
use crate::read_model::{CurrencyList, CurrencyListReader};

pub struct CurrencyListQueryHandler<R>
where
    R: CurrencyListReader,
{
    reader: R,
}

impl<R> CurrencyListQueryHandler<R>
where
    R: CurrencyListReader,
{
    pub fn new(reader: R) -> Self {
        Self { reader }
    }
}

impl<R> QueryHandler for CurrencyListQueryHandler<R>
where
    R: CurrencyListReader,
{
    type Query = CurrencyListQuery;
    type Output = CurrencyList;
    type Error = CurrencyListQueryHandlerError;
    type Uow = R::Uow;

    const PROJECTOR_DEPENDENCIES: ProjectorDependencies<'static> =
        ProjectorDependencies::Some(&[CurrencyFragmentProjectorSpec::DESCRIPTOR]);

    fn authorization_plan(&self, _query: &Self::Query) -> Result<AuthorizationPlan, Self::Error> {
        Ok(AuthorizationPlan::None)
    }

    async fn handle(
        &self,
        uow: &mut Self::Uow,
        _request_context: &RequestContext,
        query: Self::Query,
    ) -> Result<Self::Output, Self::Error> {
        Ok(self.reader.list(uow, query.include_inactive).await?)
    }
}
