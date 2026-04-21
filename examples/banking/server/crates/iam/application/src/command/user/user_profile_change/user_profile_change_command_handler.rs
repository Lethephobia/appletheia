use appletheia::application::authorization::{
    AuthorizationPlan, PrincipalRequirement, Relation, RelationshipRequirement,
};
use appletheia::application::command::{CommandHandled, CommandHandler};
use appletheia::application::projection::{ProjectorDependencies, ProjectorSpec};
use appletheia::application::repository::Repository;
use appletheia::application::request_context::RequestContext;
use banking_iam_domain::User;

use super::{
    UserProfileChangeCommand, UserProfileChangeCommandHandlerError, UserProfileChangeOutput,
};
use crate::authorization::UserProfileChangerRelation;
use crate::projection::UserOwnerRelationshipProjectorSpec;

/// Handles `UserProfileChangeCommand`.
pub struct UserProfileChangeCommandHandler<UR>
where
    UR: Repository<User>,
{
    user_repository: UR,
}

impl<UR> UserProfileChangeCommandHandler<UR>
where
    UR: Repository<User>,
{
    pub fn new(user_repository: UR) -> Self {
        Self { user_repository }
    }
}

impl<UR> CommandHandler for UserProfileChangeCommandHandler<UR>
where
    UR: Repository<User>,
{
    type Command = UserProfileChangeCommand;
    type Output = UserProfileChangeOutput;
    type ReplayOutput = UserProfileChangeOutput;
    type Error = UserProfileChangeCommandHandlerError;
    type Uow = UR::Uow;

    fn authorization_plan(
        &self,
        command: &Self::Command,
    ) -> Result<AuthorizationPlan, Self::Error> {
        Ok(AuthorizationPlan::OnlyPrincipals(vec![
            PrincipalRequirement::AuthenticatedWithRelationship {
                requirement: RelationshipRequirement::check::<User>(
                    command.user_id,
                    UserProfileChangerRelation::REF,
                ),
                projector_dependencies: ProjectorDependencies::Some(&[
                    UserOwnerRelationshipProjectorSpec::DESCRIPTOR,
                ]),
            },
        ]))
    }

    async fn handle(
        &self,
        uow: &mut Self::Uow,
        request_context: &RequestContext,
        command: &Self::Command,
    ) -> Result<CommandHandled<Self::Output, Self::ReplayOutput>, Self::Error> {
        let Some(mut user) = self.user_repository.find(uow, command.user_id).await? else {
            return Err(UserProfileChangeCommandHandlerError::UserNotFound);
        };

        user.change_profile(command.profile.clone())?;

        self.user_repository
            .save(uow, request_context, &mut user)
            .await?;

        Ok(CommandHandled::same(UserProfileChangeOutput))
    }
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};

    use appletheia::application::authorization::AggregateRef;
    use appletheia::application::command::CommandHandler;
    use appletheia::application::repository::{Repository, RepositoryError};
    use appletheia::application::request_context::{
        CorrelationId, MessageId, Principal, RequestContext,
    };
    use appletheia::application::unit_of_work::{UnitOfWork, UnitOfWorkError};
    use appletheia::domain::Aggregate;
    use banking_iam_domain::{
        User, UserDisplayName, UserId, UserIdentity, UserIdentityProvider, UserIdentitySubject,
        UserPictureUrl, UserProfile,
    };
    use uuid::Uuid;

    use super::{
        UserProfileChangeCommand, UserProfileChangeCommandHandler, UserProfileChangeOutput,
    };

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

    #[derive(Clone, Default)]
    struct TestUserRepository {
        user: Arc<Mutex<Option<User>>>,
    }

    impl TestUserRepository {
        fn new(user: User) -> Self {
            Self {
                user: Arc::new(Mutex::new(Some(user))),
            }
        }
    }

    impl Repository<User> for TestUserRepository {
        type Uow = TestUow;
        async fn find(
            &self,
            _uow: &mut Self::Uow,
            _id: UserId,
        ) -> Result<Option<User>, RepositoryError<User>> {
            Ok(self.user.lock().expect("lock").clone())
        }
        async fn find_at_version(
            &self,
            _uow: &mut Self::Uow,
            _id: UserId,
            _at: Option<appletheia::domain::AggregateVersion>,
        ) -> Result<Option<User>, RepositoryError<User>> {
            Ok(self.user.lock().expect("lock").clone())
        }
        async fn find_by_unique_value(
            &self,
            _uow: &mut Self::Uow,
            _unique_key: appletheia::domain::UniqueKey,
            _unique_value: &appletheia::domain::UniqueValue,
        ) -> Result<Option<User>, RepositoryError<User>> {
            Ok(None)
        }
        async fn save(
            &self,
            _uow: &mut Self::Uow,
            _request_context: &RequestContext,
            aggregate: &mut User,
        ) -> Result<(), RepositoryError<User>> {
            *self.user.lock().expect("lock") = Some(aggregate.clone());
            Ok(())
        }
    }

    fn request_context(user_id: UserId) -> RequestContext {
        RequestContext::new(
            CorrelationId::from(Uuid::now_v7()),
            MessageId::new(),
            Principal::Authenticated {
                subject: AggregateRef::from_id::<User>(user_id),
            },
        )
        .expect("request context should be valid")
    }

    fn registered_user() -> User {
        let mut user = User::default();
        user.register(UserIdentity::new(
            UserIdentityProvider::try_from("https://accounts.example.com")
                .expect("provider should be valid"),
            UserIdentitySubject::try_from("user-123").expect("subject should be valid"),
            None,
        ))
        .expect("user should register");
        user
    }

    #[tokio::test]
    async fn handle_changes_profile() {
        let user = registered_user();
        let user_id = user.aggregate_id().expect("user id should exist");
        let repository = TestUserRepository::new(user);
        let handler = UserProfileChangeCommandHandler::new(repository.clone());
        let mut uow = TestUow;

        let handled = handler
            .handle(
                &mut uow,
                &request_context(user_id),
                &UserProfileChangeCommand {
                    user_id,
                    profile: UserProfile::new(
                        UserDisplayName::try_from("Alice Example")
                            .expect("display name should be valid"),
                        None,
                        Some(
                            UserPictureUrl::try_from("https://cdn.example.com/alice.png")
                                .expect("picture URL should be valid"),
                        ),
                    ),
                },
            )
            .await
            .expect("command should succeed");

        assert_eq!(handled.into_output(), UserProfileChangeOutput);
    }
}
