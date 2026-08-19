use appletheia::application::authorization::{
    AuthorizationPlan, PrincipalRequirement, Relation, RelationshipRequirement,
};
use appletheia::application::projection::{ProjectorDependencies, ProjectorSpec};
use appletheia::application::query::QueryHandler;
use appletheia::application::request_context::RequestContext;
use banking_iam_domain::User;

use crate::authorization::UserOwnerRelation;
use crate::projection::{
    OrganizationFragmentProjectorSpec, OrganizationJoinRequestFragmentProjectorSpec,
    UserFragmentProjectorSpec,
};
use crate::read_model::{UserOrganizationJoinRequestList, UserOrganizationJoinRequestListReader};

use super::{
    UserOrganizationJoinRequestListQuery, UserOrganizationJoinRequestListQueryHandlerError,
};

/// Handles user organization join request list queries.
pub struct UserOrganizationJoinRequestListQueryHandler<R>
where
    R: UserOrganizationJoinRequestListReader,
{
    reader: R,
}

impl<R> UserOrganizationJoinRequestListQueryHandler<R>
where
    R: UserOrganizationJoinRequestListReader,
{
    pub fn new(reader: R) -> Self {
        Self { reader }
    }
}

impl<R> QueryHandler for UserOrganizationJoinRequestListQueryHandler<R>
where
    R: UserOrganizationJoinRequestListReader,
{
    type Query = UserOrganizationJoinRequestListQuery;
    type Output = UserOrganizationJoinRequestList;
    type Error = UserOrganizationJoinRequestListQueryHandlerError;
    type Uow = R::Uow;

    const PROJECTOR_DEPENDENCIES: ProjectorDependencies<'static> = ProjectorDependencies::Some(&[
        OrganizationJoinRequestFragmentProjectorSpec::DESCRIPTOR,
        OrganizationFragmentProjectorSpec::DESCRIPTOR,
        UserFragmentProjectorSpec::DESCRIPTOR,
    ]);

    fn authorization_plan(&self, query: &Self::Query) -> Result<AuthorizationPlan, Self::Error> {
        Ok(AuthorizationPlan::OnlyPrincipals(vec![
            PrincipalRequirement::AuthenticatedWithRelationship(RelationshipRequirement::check::<
                User,
            >(
                query.user_id,
                UserOwnerRelation::REF,
            )),
        ]))
    }

    async fn handle(
        &self,
        uow: &mut Self::Uow,
        _request_context: &RequestContext,
        query: Self::Query,
    ) -> Result<Self::Output, Self::Error> {
        Ok(self
            .reader
            .list(uow, query.user_id, query.criteria, query.sort, query.page)
            .await?)
    }
}

#[cfg(test)]
mod tests {
    use appletheia::application::authorization::{
        AuthorizationPlan, PrincipalRequirement, Relation, RelationshipRequirement,
    };
    use appletheia::application::query::QueryHandler;
    use appletheia::application::read_model::pagination::{
        CursorWindow, PageSize, Sort, SortDirection,
    };
    use appletheia::application::unit_of_work::{UnitOfWork, UnitOfWorkError};
    use banking_iam_domain::{User, UserId};

    use crate::authorization::UserOwnerRelation;
    use crate::read_model::{
        UserOrganizationJoinRequestList, UserOrganizationJoinRequestListCriteria,
        UserOrganizationJoinRequestListCursor, UserOrganizationJoinRequestListReader,
        UserOrganizationJoinRequestListReaderError, UserOrganizationJoinRequestListSortKey,
    };

    use super::{
        UserOrganizationJoinRequestListQuery, UserOrganizationJoinRequestListQueryHandler,
    };

    struct TestUow;

    impl UnitOfWork for TestUow {
        async fn commit(self) -> Result<(), UnitOfWorkError> {
            Ok(())
        }

        async fn rollback(self) -> Result<(), UnitOfWorkError> {
            Ok(())
        }
    }

    struct TestReader;

    impl UserOrganizationJoinRequestListReader for TestReader {
        type Uow = TestUow;

        async fn list(
            &self,
            _uow: &mut Self::Uow,
            _scope_id: UserId,
            _criteria: UserOrganizationJoinRequestListCriteria,
            _sort: Sort<UserOrganizationJoinRequestListSortKey>,
            _page: CursorWindow<UserOrganizationJoinRequestListCursor>,
        ) -> Result<UserOrganizationJoinRequestList, UserOrganizationJoinRequestListReaderError>
        {
            panic!("reader is not exercised by this test")
        }
    }

    #[test]
    fn authorization_plan_requires_expected_relationship() {
        let handler = UserOrganizationJoinRequestListQueryHandler::new(TestReader);
        let scope_id = UserId::new();
        let query = UserOrganizationJoinRequestListQuery {
            user_id: scope_id,
            criteria: UserOrganizationJoinRequestListCriteria::default(),
            sort: Sort {
                key: UserOrganizationJoinRequestListSortKey::CreatedAt,
                direction: SortDirection::Desc,
            },
            page: CursorWindow::Forward {
                after: None,
                limit: PageSize::new(20).expect("page size should be valid"),
            },
        };

        let plan = handler
            .authorization_plan(&query)
            .expect("authorization plan should build");

        assert_eq!(
            plan,
            AuthorizationPlan::OnlyPrincipals(vec![
                PrincipalRequirement::AuthenticatedWithRelationship(
                    RelationshipRequirement::check::<User>(scope_id, UserOwnerRelation::REF)
                ),
            ])
        );
    }
}
