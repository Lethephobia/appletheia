use appletheia::application::authentication::AuthTokenRevoker;
use appletheia::application::authorization::{AuthorizationPlan, PrincipalRequirement};
use appletheia::application::command::{CommandHandled, CommandHandler};
use appletheia::application::request_context::RequestContext;

use super::{LogoutCommand, LogoutCommandHandlerError, LogoutOutput};

/// Handles `LogoutCommand`.
pub struct LogoutCommandHandler<ATR>
where
    ATR: AuthTokenRevoker,
{
    auth_token_revoker: ATR,
}

impl<ATR> LogoutCommandHandler<ATR>
where
    ATR: AuthTokenRevoker,
{
    pub fn new(auth_token_revoker: ATR) -> Self {
        Self { auth_token_revoker }
    }
}

impl<ATR> CommandHandler for LogoutCommandHandler<ATR>
where
    ATR: AuthTokenRevoker,
{
    type Command = LogoutCommand;
    type Output = LogoutOutput;
    type ReplayOutput = LogoutOutput;
    type Error = LogoutCommandHandlerError;
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
        _request_context: &RequestContext,
        command: &Self::Command,
    ) -> Result<CommandHandled<Self::Output, Self::ReplayOutput>, Self::Error> {
        self.auth_token_revoker
            .revoke_token(uow, command.token_id, command.token_expires_at)
            .await?;

        Ok(CommandHandled::same(LogoutOutput))
    }
}
