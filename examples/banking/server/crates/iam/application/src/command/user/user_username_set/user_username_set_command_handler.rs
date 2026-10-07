use appletheia::application::authorization::{
    AuthorizationPlan, PrincipalRequirement, Relation, RelationshipRequirement,
};
use appletheia::application::command::CommandHandler;
use appletheia::application::repository::Repository;
use appletheia::application::request_context::RequestContext;
use appletheia::domain::{Aggregate, UniqueValue};
use banking_iam_domain::UserError;

use banking_iam_domain::{User, UserState, Username};

use super::{UserUsernameSetCommand, UserUsernameSetCommandHandlerError, UserUsernameSetOutput};
use crate::authorization::UserUsernameSetterRelation;

pub struct UserUsernameSetCommandHandler<R>
where
    R: Repository,
{
    repository: R,
}

impl<R> UserUsernameSetCommandHandler<R>
where
    R: Repository,
{
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    fn username_unique_value(
        username: &Username,
    ) -> Result<UniqueValue, UserUsernameSetCommandHandlerError> {
        Ok(UniqueValue::from_strings([username.as_ref()])?)
    }
}

impl<R> CommandHandler for UserUsernameSetCommandHandler<R>
where
    R: Repository,
{
    type Command = UserUsernameSetCommand;
    type Output = UserUsernameSetOutput;
    type Error = UserUsernameSetCommandHandlerError;
    type Uow = R::Uow;

    fn authorization_plan(
        &self,
        command: &Self::Command,
    ) -> Result<AuthorizationPlan, Self::Error> {
        Ok(AuthorizationPlan::OnlyPrincipals(vec![
            PrincipalRequirement::AuthenticatedWithRelationship(RelationshipRequirement::check::<
                User,
            >(
                command.user_id,
                UserUsernameSetterRelation::REF,
            )),
        ]))
    }

    async fn handle(
        &self,
        uow: &mut Self::Uow,
        request_context: &RequestContext,
        command: &Self::Command,
    ) -> Result<Self::Output, Self::Error> {
        let mut user = self.repository.read::<User>(uow, command.user_id).await?;

        let unique_value = Self::username_unique_value(&command.username)?;
        if self
            .repository
            .find_by_unique_value::<User>(uow, UserState::USERNAME_KEY, &unique_value)
            .await?
            .is_some_and(|existing| existing.aggregate_id() != command.user_id)
        {
            return Err(UserError::UsernameAlreadyTaken.into());
        }

        user.set_username(command.username.clone())?;

        self.repository
            .save(uow, request_context, &mut user)
            .await?;

        Ok(UserUsernameSetOutput {})
    }
}

#[cfg(test)]
mod tests {
    use appletheia::application::aggregate::SerializedAggregate;
    use appletheia::domain::{AggregateId, Event};
    use std::sync::{Arc, Mutex};

    use appletheia::application::aggregate::AggregateRef;
    use appletheia::application::command::CommandHandler;
    use appletheia::application::repository::{Repository, RepositoryError};
    use appletheia::application::request_context::{
        CorrelationId, MessageId, Principal, RequestContext,
    };
    use appletheia::application::unit_of_work::{UnitOfWork, UnitOfWorkError};
    use appletheia::domain::Aggregate;
    use banking_iam_domain::{User, UserId, UserIdentityProvider, UserIdentitySubject, Username};
    use uuid::Uuid;

    use super::{UserUsernameSetCommand, UserUsernameSetCommandHandler, UserUsernameSetOutput};

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
    struct TestRepository {
        user: Arc<Mutex<Option<User>>>,
    }
    impl TestRepository {
        fn copy_aggregate<S: Aggregate, T: Aggregate>(source: &S) -> T {
            let mut restored = SerializedAggregate::try_from_aggregate(source)
                .expect("test aggregate should serialize")
                .try_to_aggregate::<T>()
                .expect("test aggregate types should match");
            for event in source.uncommitted_events() {
                let id = T::Id::try_from_uuid(event.aggregate_id().value()).unwrap();
                let payload =
                    serde_json::from_value(serde_json::to_value(event.payload()).unwrap()).unwrap();
                restored
                    .core_mut()
                    .record_uncommitted_event(Event::from_persisted(
                        event.id(),
                        id,
                        event.aggregate_version(),
                        payload,
                        event.occurred_at(),
                    ));
            }
            restored
        }

        fn new(user: User) -> Self {
            Self {
                user: Arc::new(Mutex::new(Some(user))),
            }
        }
    }
    impl Repository for TestRepository {
        type Uow = TestUow;
        async fn read<A: Aggregate>(
            &self,
            _uow: &mut Self::Uow,
            id: A::Id,
        ) -> Result<A, RepositoryError<A>> {
            self.user
                .lock()
                .expect("lock")
                .as_ref()
                .map(|stored| Self::copy_aggregate::<_, A>(stored))
                .ok_or_else(|| RepositoryError::NotFound {
                    aggregate_type: A::TYPE,
                    aggregate_id: id,
                })
        }
        async fn read_at_version<A: Aggregate>(
            &self,
            _uow: &mut Self::Uow,
            id: A::Id,
            _at: appletheia::domain::AggregateVersion,
        ) -> Result<A, RepositoryError<A>> {
            self.user
                .lock()
                .expect("lock")
                .as_ref()
                .map(|stored| Self::copy_aggregate::<_, A>(stored))
                .ok_or_else(|| RepositoryError::NotFound {
                    aggregate_type: A::TYPE,
                    aggregate_id: id,
                })
        }
        async fn find_by_unique_value<A: Aggregate>(
            &self,
            _uow: &mut Self::Uow,
            _unique_key: appletheia::domain::UniqueKey,
            _unique_value: &appletheia::domain::UniqueValue,
        ) -> Result<Option<A>, RepositoryError<A>> {
            Ok(None)
        }
        async fn save<A: Aggregate>(
            &self,
            _uow: &mut Self::Uow,
            _request_context: &RequestContext,
            aggregate: &mut A,
        ) -> Result<(), RepositoryError<A>> {
            *self.user.lock().expect("lock") = Some(Self::copy_aggregate::<_, User>(aggregate));
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
        let mut user = User::new();
        user.register().expect("user should register");
        user.link_identity(
            UserIdentityProvider::try_from("https://accounts.example.com")
                .expect("provider should be valid"),
            UserIdentitySubject::try_from("user-123").expect("subject should be valid"),
            None,
        )
        .expect("identity should link");
        user
    }

    #[tokio::test]
    async fn handle_changes_username() {
        let user = registered_user();
        let user_id = user.aggregate_id();
        let repository = TestRepository::new(user);
        let handler = UserUsernameSetCommandHandler::new(repository);
        let mut uow = TestUow;

        let output = handler
            .handle(
                &mut uow,
                &request_context(user_id),
                &UserUsernameSetCommand {
                    user_id,
                    username: Username::try_from("alice").expect("username should be valid"),
                },
            )
            .await
            .expect("command should succeed");

        assert_eq!(output, UserUsernameSetOutput {});
        assert_eq!(
            serde_json::to_value(&output).expect("output should serialize"),
            serde_json::json!({})
        );
    }
}
