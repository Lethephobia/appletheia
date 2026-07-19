use appletheia::application::authorization::{AuthorizationPlan, PrincipalRequirement};
use appletheia::application::projection::{ProjectorDependencies, ProjectorSpec};
use appletheia::application::query::QueryHandler;
use appletheia::application::request_context::RequestContext;

use crate::projection::PublicOrganizationListProjectorSpec;
use crate::read_model::{PublicOrganizationList, PublicOrganizationListReader};

use super::{PublicOrganizationListQuery, PublicOrganizationListQueryHandlerError};

/// Handles public organization list queries.
pub struct PublicOrganizationListQueryHandler<S>
where
    S: PublicOrganizationListReader,
{
    reader: S,
}

impl<S> PublicOrganizationListQueryHandler<S>
where
    S: PublicOrganizationListReader,
{
    pub fn new(reader: S) -> Self {
        Self { reader }
    }
}

impl<S> QueryHandler for PublicOrganizationListQueryHandler<S>
where
    S: PublicOrganizationListReader,
{
    type Query = PublicOrganizationListQuery;
    type Output = PublicOrganizationList;
    type Error = PublicOrganizationListQueryHandlerError;
    type Uow = S::Uow;

    const PROJECTOR_DEPENDENCIES: ProjectorDependencies<'static> =
        ProjectorDependencies::Some(&[PublicOrganizationListProjectorSpec::DESCRIPTOR]);

    fn authorization_plan(&self, _query: &Self::Query) -> Result<AuthorizationPlan, Self::Error> {
        Ok(AuthorizationPlan::OnlyPrincipals(vec![
            PrincipalRequirement::Authenticated,
        ]))
    }

    async fn handle(
        &self,
        uow: &mut S::Uow,
        _request_context: &RequestContext,
        query: Self::Query,
    ) -> Result<Self::Output, Self::Error> {
        Ok(self
            .reader
            .list(uow, query.criteria, query.cursor_options, query.limit)
            .await?)
    }
}

#[cfg(test)]
mod tests {
    use appletheia::application::authorization::{AuthorizationPlan, PrincipalRequirement};
    use appletheia::application::query::QueryHandler;
    use appletheia::application::unit_of_work::{UnitOfWork, UnitOfWorkError};
    use banking_shared_kernel_application::read_model::{CursorOptions, PageSize};

    use crate::read_model::{
        PublicOrganizationList, PublicOrganizationListCriteria, PublicOrganizationListCursor,
        PublicOrganizationListReader, PublicOrganizationListReaderError,
        PublicOrganizationListSortKey,
    };

    use super::{PublicOrganizationListQuery, PublicOrganizationListQueryHandler};

    #[derive(Default)]
    struct TestUow;

    impl UnitOfWork for TestUow {
        async fn commit(self) -> Result<(), UnitOfWorkError> {
            Ok(())
        }

        async fn rollback(self) -> Result<(), UnitOfWorkError> {
            Ok(())
        }
    }

    #[derive(Default)]
    struct TestPublicOrganizationListReader;

    impl PublicOrganizationListReader for TestPublicOrganizationListReader {
        type Uow = TestUow;

        async fn list(
            &self,
            _uow: &mut Self::Uow,
            _criteria: PublicOrganizationListCriteria,
            _cursor_options: Option<
                CursorOptions<PublicOrganizationListSortKey, PublicOrganizationListCursor>,
            >,
            _limit: PageSize,
        ) -> Result<PublicOrganizationList, PublicOrganizationListReaderError> {
            Ok(PublicOrganizationList {
                items: Vec::new(),
                next_cursor: None,
            })
        }
    }

    #[test]
    fn authorization_plan_requires_authentication() {
        let handler = PublicOrganizationListQueryHandler::new(TestPublicOrganizationListReader);
        let query = PublicOrganizationListQuery {
            criteria: PublicOrganizationListCriteria::default(),
            cursor_options: None,
            limit: PageSize::new(20).expect("page size should be valid"),
        };

        let plan = handler
            .authorization_plan(&query)
            .expect("authorization plan should build");

        assert_eq!(
            plan,
            AuthorizationPlan::OnlyPrincipals(vec![PrincipalRequirement::Authenticated])
        );
    }
}
