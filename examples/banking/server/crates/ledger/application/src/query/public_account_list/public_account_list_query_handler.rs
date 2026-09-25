use appletheia::application::authorization::AuthorizationPlan;
use appletheia::application::projection::{ProjectorDependencies, ProjectorSpec};
use appletheia::application::query::QueryHandler;
use appletheia::application::request_context::RequestContext;
use banking_iam_application::{OrganizationFragmentProjectorSpec, UserFragmentProjectorSpec};

use crate::projection::{AccountFragmentProjectorSpec, CurrencyFragmentProjectorSpec};
use crate::read_model::{PublicAccountList, PublicAccountListReader};

use super::{PublicAccountListQuery, PublicAccountListQueryHandlerError};

/// Handles public account list queries.
pub struct PublicAccountListQueryHandler<S>
where
    S: PublicAccountListReader,
{
    reader: S,
}

impl<S> PublicAccountListQueryHandler<S>
where
    S: PublicAccountListReader,
{
    pub fn new(reader: S) -> Self {
        Self { reader }
    }
}

impl<S> QueryHandler for PublicAccountListQueryHandler<S>
where
    S: PublicAccountListReader,
{
    type Query = PublicAccountListQuery;
    type Output = PublicAccountList;
    type Error = PublicAccountListQueryHandlerError;
    type Uow = S::Uow;

    const PROJECTOR_DEPENDENCIES: ProjectorDependencies<'static> = ProjectorDependencies::Some(&[
        AccountFragmentProjectorSpec::DESCRIPTOR,
        CurrencyFragmentProjectorSpec::DESCRIPTOR,
        UserFragmentProjectorSpec::DESCRIPTOR,
        OrganizationFragmentProjectorSpec::DESCRIPTOR,
    ]);

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
            .reader
            .list(uow, query.criteria, query.sort, query.page)
            .await?)
    }
}
