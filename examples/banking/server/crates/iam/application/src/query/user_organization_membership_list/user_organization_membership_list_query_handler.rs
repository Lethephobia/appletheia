use appletheia::application::authorization::{
    AuthorizationPlan, PrincipalRequirement, Relation, RelationshipRequirement,
};
use appletheia::application::projection::{ProjectorDependencies, ProjectorSpec};
use appletheia::application::query::QueryHandler;
use appletheia::application::request_context::RequestContext;
use banking_iam_domain::User;

use crate::authorization::UserOwnerRelation;
use crate::projection::{
    OrganizationFragmentProjectorSpec, OrganizationMembershipFragmentProjectorSpec,
    UserFragmentProjectorSpec,
};
use crate::read_model::{UserOrganizationMembershipList, UserOrganizationMembershipListReader};

use super::{UserOrganizationMembershipListQuery, UserOrganizationMembershipListQueryHandlerError};

/// Handles user organization membership list queries.
pub struct UserOrganizationMembershipListQueryHandler<R>
where
    R: UserOrganizationMembershipListReader,
{
    reader: R,
}

impl<R> UserOrganizationMembershipListQueryHandler<R>
where
    R: UserOrganizationMembershipListReader,
{
    pub fn new(reader: R) -> Self {
        Self { reader }
    }
}

impl<R> QueryHandler for UserOrganizationMembershipListQueryHandler<R>
where
    R: UserOrganizationMembershipListReader,
{
    type Query = UserOrganizationMembershipListQuery;
    type Output = UserOrganizationMembershipList;
    type Error = UserOrganizationMembershipListQueryHandlerError;
    type Uow = R::Uow;

    const PROJECTOR_DEPENDENCIES: ProjectorDependencies<'static> = ProjectorDependencies::Some(&[
        OrganizationMembershipFragmentProjectorSpec::DESCRIPTOR,
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
            .list(uow, query.user_id, query.sort, query.page)
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
        UserOrganizationMembershipList, UserOrganizationMembershipListCursor,
        UserOrganizationMembershipListReader, UserOrganizationMembershipListReaderError,
        UserOrganizationMembershipListSortKey,
    };

    use super::{UserOrganizationMembershipListQuery, UserOrganizationMembershipListQueryHandler};

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

    impl UserOrganizationMembershipListReader for TestReader {
        type Uow = TestUow;

        async fn list(
            &self,
            _uow: &mut Self::Uow,
            _user_id: UserId,
            _sort: Sort<UserOrganizationMembershipListSortKey>,
            _page: CursorWindow<UserOrganizationMembershipListCursor>,
        ) -> Result<UserOrganizationMembershipList, UserOrganizationMembershipListReaderError>
        {
            panic!("reader is not exercised by this test")
        }
    }

    fn query(user_id: UserId) -> UserOrganizationMembershipListQuery {
        UserOrganizationMembershipListQuery {
            user_id,
            sort: Sort {
                key: UserOrganizationMembershipListSortKey::CreatedAt,
                direction: SortDirection::Desc,
            },
            page: CursorWindow::Forward {
                after: None,
                limit: PageSize::new(20).expect("page size should be valid"),
            },
        }
    }

    #[test]
    fn authorization_plan_requires_expected_relationship() {
        let handler = UserOrganizationMembershipListQueryHandler::new(TestReader);
        let user_id = UserId::new();

        let plan = handler
            .authorization_plan(&query(user_id))
            .expect("authorization plan should build");

        assert_eq!(
            plan,
            AuthorizationPlan::OnlyPrincipals(vec![
                PrincipalRequirement::AuthenticatedWithRelationship(
                    RelationshipRequirement::check::<User>(user_id, UserOwnerRelation::REF)
                ),
            ])
        );
    }
}
