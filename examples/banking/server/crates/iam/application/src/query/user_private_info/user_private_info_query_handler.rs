use appletheia::application::authorization::{
    AuthorizationPlan, PrincipalRequirement, Relation, RelationshipRequirement,
};
use appletheia::application::projection::{ProjectorDependencies, ProjectorSpec};
use appletheia::application::query::QueryHandler;
use appletheia::application::request_context::RequestContext;
use banking_iam_domain::User;

use crate::authorization::UserOwnerRelation;
use crate::projection::{UserFragmentProjectorSpec, UserIdentityFragmentProjectorSpec};
use crate::read_model::{UserPrivateInfo, UserPrivateInfoReader};

use super::{UserPrivateInfoQuery, UserPrivateInfoQueryHandlerError};

/// Handles user-private information queries.
pub struct UserPrivateInfoQueryHandler<S>
where
    S: UserPrivateInfoReader,
{
    reader: S,
}

impl<S> UserPrivateInfoQueryHandler<S>
where
    S: UserPrivateInfoReader,
{
    pub fn new(reader: S) -> Self {
        Self { reader }
    }
}

impl<S> QueryHandler for UserPrivateInfoQueryHandler<S>
where
    S: UserPrivateInfoReader,
{
    type Query = UserPrivateInfoQuery;
    type Output = Option<UserPrivateInfo>;
    type Error = UserPrivateInfoQueryHandlerError;
    type Uow = S::Uow;

    const PROJECTOR_DEPENDENCIES: ProjectorDependencies<'static> = ProjectorDependencies::Some(&[
        UserFragmentProjectorSpec::DESCRIPTOR,
        UserIdentityFragmentProjectorSpec::DESCRIPTOR,
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
        uow: &mut S::Uow,
        _request_context: &RequestContext,
        query: Self::Query,
    ) -> Result<Self::Output, Self::Error> {
        Ok(self.reader.find(uow, query.user_id).await?)
    }
}

#[cfg(test)]
mod tests {
    use appletheia::application::authorization::{
        AuthorizationPlan, PrincipalRequirement, Relation, RelationshipRequirement,
    };
    use appletheia::application::query::QueryHandler;
    use appletheia::application::unit_of_work::{UnitOfWork, UnitOfWorkError};
    use banking_iam_domain::{User, UserId};

    use crate::authorization::UserOwnerRelation;
    use crate::read_model::{UserPrivateInfo, UserPrivateInfoReader, UserPrivateInfoReaderError};

    use super::{UserPrivateInfoQuery, UserPrivateInfoQueryHandler};

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
    struct TestUserPrivateInfoReader;

    impl UserPrivateInfoReader for TestUserPrivateInfoReader {
        type Uow = TestUow;

        async fn find(
            &self,
            _uow: &mut Self::Uow,
            _user_id: UserId,
        ) -> Result<Option<UserPrivateInfo>, UserPrivateInfoReaderError> {
            Ok(None)
        }
    }

    #[test]
    fn authorization_plan_requires_user_owner_relationship() {
        let handler = UserPrivateInfoQueryHandler::new(TestUserPrivateInfoReader);
        let user_id = UserId::new();

        let plan = handler
            .authorization_plan(&UserPrivateInfoQuery { user_id })
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
