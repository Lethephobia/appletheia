use appletheia::application::authorization::{
    AggregateRef, AuthorizationPlan, PrincipalRequirement, Relation, RelationshipRequirement,
};
use appletheia::application::command::{CommandHandled, CommandHandler};
use appletheia::application::projection::{ProjectorDependencies, ProjectorSpec};
use appletheia::application::repository::Repository;
use appletheia::application::request_context::RequestContext;
use banking_iam_domain::User;

use super::{UserProfileReadyCommand, UserProfileReadyCommandHandlerError, UserProfileReadyOutput};
use crate::authorization::UserProfileEditorRelation;
use crate::projection::UserOwnerRelationshipProjectorSpec;

/// Handles `UserProfileReadyCommand`.
pub struct UserProfileReadyCommandHandler<UR>
where
    UR: Repository<User>,
{
    user_repository: UR,
}

impl<UR> UserProfileReadyCommandHandler<UR>
where
    UR: Repository<User>,
{
    pub fn new(user_repository: UR) -> Self {
        Self { user_repository }
    }
}

impl<UR> CommandHandler for UserProfileReadyCommandHandler<UR>
where
    UR: Repository<User>,
{
    type Command = UserProfileReadyCommand;
    type Output = UserProfileReadyOutput;
    type ReplayOutput = UserProfileReadyOutput;
    type Error = UserProfileReadyCommandHandlerError;
    type Uow = UR::Uow;

    fn authorization_plan(
        &self,
        command: &Self::Command,
    ) -> Result<AuthorizationPlan, Self::Error> {
        Ok(AuthorizationPlan::OnlyPrincipals(vec![
            PrincipalRequirement::AuthenticatedWithRelationship {
                requirement: RelationshipRequirement::Check {
                    aggregate: AggregateRef::from_id::<User>(command.user_id),
                    relation: UserProfileEditorRelation::NAME,
                },
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
            return Err(UserProfileReadyCommandHandlerError::UserNotFound);
        };

        user.ready_profile(command.username.clone(), command.display_name.clone())?;

        self.user_repository
            .save(uow, request_context, &mut user)
            .await?;

        let username = user
            .username()?
            .cloned()
            .ok_or(UserProfileReadyCommandHandlerError::UserProfileNotReady)?;
        let display_name = user
            .display_name()?
            .cloned()
            .ok_or(UserProfileReadyCommandHandlerError::UserProfileNotReady)?;
        let output = UserProfileReadyOutput::new(command.user_id, username, display_name);

        Ok(CommandHandled::same(output))
    }
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};

    use appletheia::application::authorization::{
        AggregateRef, AuthorizationPlan, PrincipalRequirement, Relation, RelationshipRequirement,
    };
    use appletheia::application::command::CommandHandler;
    use appletheia::application::projection::{ProjectorDependencies, ProjectorSpec};
    use appletheia::application::repository::{Repository, RepositoryError};
    use appletheia::application::request_context::{
        ActorRef, CorrelationId, MessageId, Principal, RequestContext,
    };
    use appletheia::application::unit_of_work::{UnitOfWork, UnitOfWorkError};
    use appletheia::domain::Aggregate;
    use banking_iam_domain::{
        User, UserDisplayName, UserId, UserIdentity, UserIdentityProvider, UserIdentitySubject,
        Username,
    };
    use uuid::Uuid;

    use super::{UserProfileReadyCommand, UserProfileReadyCommandHandler, UserProfileReadyOutput};
    use crate::authorization::UserProfileEditorRelation;
    use crate::projection::UserOwnerRelationshipProjectorSpec;

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
        let subject = AggregateRef::from_id::<User>(user_id);

        RequestContext::new(
            CorrelationId::from(Uuid::now_v7()),
            MessageId::new(),
            ActorRef::Subject {
                subject: subject.clone(),
            },
            Principal::Authenticated { subject },
        )
    }

    fn registered_user() -> User {
        let identity = UserIdentity::new(
            UserIdentityProvider::try_from("https://accounts.example.com")
                .expect("provider should be valid"),
            UserIdentitySubject::try_from("user-123").expect("subject should be valid"),
            None,
        );
        let mut user = User::default();
        user.register(identity).expect("user should register");
        user
    }

    #[test]
    fn authorization_plan_requires_profile_editor_relationship() {
        let user = registered_user();
        let user_id = user.aggregate_id().expect("user id should exist");
        let repository = TestUserRepository::new(user);
        let handler = UserProfileReadyCommandHandler::new(repository);
        let command = UserProfileReadyCommand {
            user_id,
            username: Username::try_from("alice").expect("username should be valid"),
            display_name: UserDisplayName::try_from("Alice").expect("display name should be valid"),
        };

        let plan = handler
            .authorization_plan(&command)
            .expect("authorization plan should build");

        assert_eq!(
            plan,
            AuthorizationPlan::OnlyPrincipals(vec![
                PrincipalRequirement::AuthenticatedWithRelationship {
                    requirement: RelationshipRequirement::Check {
                        aggregate: AggregateRef::from_id::<User>(user_id),
                        relation: UserProfileEditorRelation::NAME,
                    },
                    projector_dependencies: ProjectorDependencies::Some(&[
                        UserOwnerRelationshipProjectorSpec::DESCRIPTOR,
                    ]),
                },
            ])
        );
    }

    #[tokio::test]
    async fn handle_readies_profile_and_returns_updated_values() {
        let user = registered_user();
        let user_id = user.aggregate_id().expect("user id should exist");
        let repository = TestUserRepository::new(user);
        let handler = UserProfileReadyCommandHandler::new(repository.clone());
        let mut uow = TestUow;
        let request_context = request_context(user_id);

        let handled = handler
            .handle(
                &mut uow,
                &request_context,
                &UserProfileReadyCommand {
                    user_id,
                    username: Username::try_from("alice").expect("username should be valid"),
                    display_name: UserDisplayName::try_from("Alice")
                        .expect("display name should be valid"),
                },
            )
            .await
            .expect("command should succeed");

        assert_eq!(
            handled.into_output(),
            UserProfileReadyOutput::new(
                user_id,
                Username::try_from("alice").expect("username should be valid"),
                UserDisplayName::try_from("Alice").expect("display name should be valid"),
            )
        );
    }
}
