use appletheia::application::authorization::{AuthorizationPlan, PrincipalRequirement};
use appletheia::application::command::CommandHandler;
use appletheia::application::repository::Repository;
use appletheia::application::request_context::RequestContext;
use banking_ledger_domain::withdrawal::Withdrawal;

use super::{
    WithdrawalSucceedCommand, WithdrawalSucceedCommandHandlerError, WithdrawalSucceedOutput,
};

pub struct WithdrawalSucceedCommandHandler<R>
where
    R: Repository,
{
    repository: R,
}

impl<R> WithdrawalSucceedCommandHandler<R>
where
    R: Repository,
{
    pub fn new(repository: R) -> Self {
        Self { repository }
    }
}

impl<R> CommandHandler for WithdrawalSucceedCommandHandler<R>
where
    R: Repository,
{
    type Command = WithdrawalSucceedCommand;
    type Output = WithdrawalSucceedOutput;
    type Error = WithdrawalSucceedCommandHandlerError;
    type Uow = R::Uow;

    fn authorization_plan(
        &self,
        _command: &Self::Command,
    ) -> Result<AuthorizationPlan, Self::Error> {
        Ok(AuthorizationPlan::OnlyPrincipals(vec![
            PrincipalRequirement::System,
        ]))
    }

    async fn handle(
        &self,
        uow: &mut Self::Uow,
        request_context: &RequestContext,
        command: &Self::Command,
    ) -> Result<Self::Output, Self::Error> {
        let mut withdrawal = self
            .repository
            .read::<Withdrawal>(uow, command.withdrawal_id)
            .await?;

        withdrawal.succeed()?;
        self.repository
            .save::<Withdrawal>(uow, request_context, &mut withdrawal)
            .await?;

        Ok(WithdrawalSucceedOutput {})
    }
}
