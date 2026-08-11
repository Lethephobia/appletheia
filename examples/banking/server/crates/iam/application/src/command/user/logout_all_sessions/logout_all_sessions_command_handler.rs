use appletheia::application::authentication::AuthTokenRevoker;
use appletheia::application::authorization::{
    AggregateRef, AuthorizationPlan, PrincipalRequirement, Relation, RelationshipRequirement,
};
use appletheia::application::command::CommandHandler;
use appletheia::application::request_context::RequestContext;
use banking_iam_domain::User;

use super::{
    LogoutAllSessionsCommand, LogoutAllSessionsCommandHandlerError, LogoutAllSessionsOutput,
};
use crate::authorization::UserOwnerRelation;

/// Handles `LogoutAllSessionsCommand`.
pub struct LogoutAllSessionsCommandHandler<ATR>
where
    ATR: AuthTokenRevoker,
{
    auth_token_revoker: ATR,
}

impl<ATR> LogoutAllSessionsCommandHandler<ATR>
where
    ATR: AuthTokenRevoker,
{
    pub fn new(auth_token_revoker: ATR) -> Self {
        Self { auth_token_revoker }
    }
}

impl<ATR> CommandHandler for LogoutAllSessionsCommandHandler<ATR>
where
    ATR: AuthTokenRevoker,
{
    type Command = LogoutAllSessionsCommand;
    type Output = LogoutAllSessionsOutput;
    type Error = LogoutAllSessionsCommandHandlerError;
    type Uow = ATR::Uow;

    fn authorization_plan(
        &self,
        command: &Self::Command,
    ) -> Result<AuthorizationPlan, Self::Error> {
        Ok(AuthorizationPlan::OnlyPrincipals(vec![
            PrincipalRequirement::AuthenticatedWithRelationship(RelationshipRequirement::check::<
                User,
            >(
                command.user_id,
                UserOwnerRelation::REF,
            )),
        ]))
    }

    async fn handle(
        &self,
        uow: &mut Self::Uow,
        _request_context: &RequestContext,
        command: &Self::Command,
    ) -> Result<Self::Output, Self::Error> {
        let subject = AggregateRef::from_id::<User>(command.user_id);

        self.auth_token_revoker
            .advance_revocation_cutoff(uow, &subject, command.token_issued_at)
            .await?;

        Ok(LogoutAllSessionsOutput)
    }
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};

    use appletheia::application::authentication::{
        AuthTokenExpiresAt, AuthTokenId, AuthTokenIssuedAt, AuthTokenRevocationError,
        AuthTokenRevoker,
    };
    use appletheia::application::authorization::{
        AggregateRef, AuthorizationPlan, PrincipalRequirement, Relation, RelationshipRequirement,
    };
    use appletheia::application::command::CommandHandler;
    use appletheia::application::request_context::{
        CorrelationId, MessageId, Principal, RequestContext,
    };
    use appletheia::application::unit_of_work::{UnitOfWork, UnitOfWorkError};
    use banking_iam_domain::{User, UserId};
    use uuid::Uuid;

    use crate::authorization::UserOwnerRelation;

    use super::{
        LogoutAllSessionsCommand, LogoutAllSessionsCommandHandler, LogoutAllSessionsOutput,
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
    struct TestAuthTokenRevoker {
        cutoff: Arc<Mutex<Option<(AggregateRef, AuthTokenIssuedAt)>>>,
    }

    impl TestAuthTokenRevoker {
        fn cutoff(&self) -> Option<(AggregateRef, AuthTokenIssuedAt)> {
            self.cutoff.lock().expect("lock").clone()
        }
    }

    impl AuthTokenRevoker for TestAuthTokenRevoker {
        type Uow = TestUow;

        async fn revoke_token(
            &self,
            _uow: &mut Self::Uow,
            _token_id: AuthTokenId,
            _expires_at: AuthTokenExpiresAt,
        ) -> Result<(), AuthTokenRevocationError> {
            Ok(())
        }

        async fn advance_revocation_cutoff(
            &self,
            _uow: &mut Self::Uow,
            subject: &AggregateRef,
            issued_at: AuthTokenIssuedAt,
        ) -> Result<(), AuthTokenRevocationError> {
            *self.cutoff.lock().expect("lock") = Some((subject.clone(), issued_at));
            Ok(())
        }
    }

    fn request_context() -> RequestContext {
        RequestContext::new(
            CorrelationId::from(Uuid::now_v7()),
            MessageId::new(),
            Principal::Anonymous,
        )
        .expect("request context should be valid")
    }

    #[test]
    fn authorization_plan_requires_user_owner_relationship() {
        let user_id = UserId::new();
        let handler = LogoutAllSessionsCommandHandler::new(TestAuthTokenRevoker::default());
        let command = LogoutAllSessionsCommand {
            user_id,
            token_issued_at: AuthTokenIssuedAt::default(),
        };

        let plan = handler
            .authorization_plan(&command)
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

    #[tokio::test]
    async fn handle_advances_revocation_cutoff_for_command_user() {
        let user_id = UserId::new();
        let issued_at = AuthTokenIssuedAt::default();
        let revoker = TestAuthTokenRevoker::default();
        let handler = LogoutAllSessionsCommandHandler::new(revoker.clone());
        let mut uow = TestUow;

        let handled = handler
            .handle(
                &mut uow,
                &request_context(),
                &LogoutAllSessionsCommand {
                    user_id,
                    token_issued_at: issued_at,
                },
            )
            .await
            .expect("command should succeed");

        assert_eq!(handled, LogoutAllSessionsOutput);
        assert_eq!(
            revoker.cutoff(),
            Some((AggregateRef::from_id::<User>(user_id), issued_at))
        );
    }
}
