use appletheia::application::authorization::{
    AuthorizationPlan, PrincipalRequirement, Relation, RelationshipRequirement,
};
use appletheia::application::projection::{ProjectorDependencies, ProjectorSpec};
use appletheia::application::query::QueryHandler;
use appletheia::application::request_context::RequestContext;
use banking_iam_domain::User;

use crate::authorization::UserOwnerRelation;
use crate::projection::{
    OrganizationFragmentProjectorSpec, OrganizationInvitationFragmentProjectorSpec,
    UserFragmentProjectorSpec,
};
use crate::read_model::{UserOrganizationInvitationList, UserOrganizationInvitationListReader};

use super::{UserOrganizationInvitationListQuery, UserOrganizationInvitationListQueryHandlerError};

/// Handles user organization invitation list queries.
pub struct UserOrganizationInvitationListQueryHandler<R>
where
    R: UserOrganizationInvitationListReader,
{
    reader: R,
}

impl<R> UserOrganizationInvitationListQueryHandler<R>
where
    R: UserOrganizationInvitationListReader,
{
    pub fn new(reader: R) -> Self {
        Self { reader }
    }
}

impl<R> QueryHandler for UserOrganizationInvitationListQueryHandler<R>
where
    R: UserOrganizationInvitationListReader,
{
    type Query = UserOrganizationInvitationListQuery;
    type Output = UserOrganizationInvitationList;
    type Error = UserOrganizationInvitationListQueryHandlerError;
    type Uow = R::Uow;

    const PROJECTOR_DEPENDENCIES: ProjectorDependencies<'static> = ProjectorDependencies::Some(&[
        OrganizationInvitationFragmentProjectorSpec::DESCRIPTOR,
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
        UserOrganizationInvitationList, UserOrganizationInvitationListCriteria,
        UserOrganizationInvitationListCursor, UserOrganizationInvitationListReader,
        UserOrganizationInvitationListReaderError, UserOrganizationInvitationListSortKey,
    };

    use super::{UserOrganizationInvitationListQuery, UserOrganizationInvitationListQueryHandler};

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

    impl UserOrganizationInvitationListReader for TestReader {
        type Uow = TestUow;

        async fn list(
            &self,
            _uow: &mut Self::Uow,
            _scope_id: UserId,
            _criteria: UserOrganizationInvitationListCriteria,
            _sort: Sort<UserOrganizationInvitationListSortKey>,
            _page: CursorWindow<UserOrganizationInvitationListCursor>,
        ) -> Result<UserOrganizationInvitationList, UserOrganizationInvitationListReaderError>
        {
            panic!("reader is not exercised by this test")
        }
    }

    #[test]
    fn authorization_plan_requires_expected_relationship() {
        let handler = UserOrganizationInvitationListQueryHandler::new(TestReader);
        let scope_id = UserId::new();
        let query = UserOrganizationInvitationListQuery {
            user_id: scope_id,
            criteria: UserOrganizationInvitationListCriteria::default(),
            sort: Sort {
                key: UserOrganizationInvitationListSortKey::CreatedAt,
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
