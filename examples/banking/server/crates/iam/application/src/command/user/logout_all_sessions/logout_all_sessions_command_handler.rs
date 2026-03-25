use appletheia::application::authentication::AuthTokenRevoker;
use appletheia::application::authorization::{AuthorizationPlan, PrincipalRequirement};
use appletheia::application::command::{CommandHandled, CommandHandler};
use appletheia::application::request_context::{Principal, RequestContext};

use super::{
    LogoutAllSessionsCommand, LogoutAllSessionsCommandHandlerError, LogoutAllSessionsOutput,
};

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
    type ReplayOutput = LogoutAllSessionsOutput;
    type Error = LogoutAllSessionsCommandHandlerError;
    type Uow = ATR::Uow;

    fn authorization_plan(
        &self,
        _command: &Self::Command,
    ) -> Result<AuthorizationPlan, Self::Error> {
        Ok(AuthorizationPlan::OnlyPrincipals(vec![
            PrincipalRequirement::Authenticated,
        ]))
    }

    async fn handle(
        &self,
        uow: &mut Self::Uow,
        request_context: &RequestContext,
        command: Self::Command,
    ) -> Result<CommandHandled<Self::Output, Self::ReplayOutput>, Self::Error> {
        let Principal::Authenticated { subject } = &request_context.principal else {
            return Err(LogoutAllSessionsCommandHandlerError::AuthenticatedPrincipalRequired);
        };

        self.auth_token_revoker
            .advance_revocation_cutoff(uow, subject, command.token_issued_at)
            .await?;

        Ok(CommandHandled::same(LogoutAllSessionsOutput))
    }
}
