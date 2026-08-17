use appletheia::application::authorization::AuthorizationPlan;
use appletheia::application::projection::{ProjectorDependencies, ProjectorSpec};
use appletheia::application::query::QueryHandler;
use appletheia::application::request_context::RequestContext;

use crate::projection::UserFragmentProjectorSpec;
use crate::read_model::{UserPublicProfile, UserPublicProfileReader};

use super::{UserPublicProfileQuery, UserPublicProfileQueryHandlerError};

/// Handles public user profile queries.
pub struct UserPublicProfileQueryHandler<S>
where
    S: UserPublicProfileReader,
{
    reader: S,
}

impl<S> UserPublicProfileQueryHandler<S>
where
    S: UserPublicProfileReader,
{
    pub fn new(reader: S) -> Self {
        Self { reader }
    }
}

impl<S> QueryHandler for UserPublicProfileQueryHandler<S>
where
    S: UserPublicProfileReader,
{
    type Query = UserPublicProfileQuery;
    type Output = Option<UserPublicProfile>;
    type Error = UserPublicProfileQueryHandlerError;
    type Uow = S::Uow;

    const PROJECTOR_DEPENDENCIES: ProjectorDependencies<'static> =
        ProjectorDependencies::Some(&[UserFragmentProjectorSpec::DESCRIPTOR]);

    fn authorization_plan(&self, _query: &Self::Query) -> Result<AuthorizationPlan, Self::Error> {
        Ok(AuthorizationPlan::None)
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
    use appletheia::application::authorization::AuthorizationPlan;
    use appletheia::application::query::QueryHandler;
    use appletheia::application::unit_of_work::{UnitOfWork, UnitOfWorkError};
    use banking_iam_domain::UserId;

    use crate::read_model::{
        UserPublicProfile, UserPublicProfileReader, UserPublicProfileReaderError,
    };

    use super::{UserPublicProfileQuery, UserPublicProfileQueryHandler};

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
    struct TestUserPublicProfileReader;

    impl UserPublicProfileReader for TestUserPublicProfileReader {
        type Uow = TestUow;

        async fn find(
            &self,
            _uow: &mut Self::Uow,
            _user_id: UserId,
        ) -> Result<Option<UserPublicProfile>, UserPublicProfileReaderError> {
            Ok(None)
        }
    }

    #[test]
    fn authorization_plan_is_public() {
        let handler = UserPublicProfileQueryHandler::new(TestUserPublicProfileReader);

        let plan = handler
            .authorization_plan(&UserPublicProfileQuery {
                user_id: UserId::new(),
            })
            .expect("authorization plan should build");

        assert_eq!(plan, AuthorizationPlan::None);
    }
}
