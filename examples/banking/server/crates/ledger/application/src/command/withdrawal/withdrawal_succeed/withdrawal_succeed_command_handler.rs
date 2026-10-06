use appletheia::application::authorization::{AuthorizationPlan, PrincipalRequirement};
use appletheia::application::command::CommandHandler;
use appletheia::application::repository::Repository;
use appletheia::application::request_context::RequestContext;
use banking_ledger_domain::withdrawal::Withdrawal;

use super::{
    WithdrawalSucceedCommand, WithdrawalSucceedCommandHandlerError, WithdrawalSucceedOutput,
};

pub struct WithdrawalSucceedCommandHandler<WR>
where
    WR: Repository<Withdrawal>,
{
    withdrawal_repository: WR,
}

impl<WR> WithdrawalSucceedCommandHandler<WR>
where
    WR: Repository<Withdrawal>,
{
    pub fn new(withdrawal_repository: WR) -> Self {
        Self {
            withdrawal_repository,
        }
    }
}

impl<WR> CommandHandler for WithdrawalSucceedCommandHandler<WR>
where
    WR: Repository<Withdrawal>,
{
    type Command = WithdrawalSucceedCommand;
    type Output = WithdrawalSucceedOutput;
    type Error = WithdrawalSucceedCommandHandlerError;
    type Uow = WR::Uow;

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
            .withdrawal_repository
            .read(uow, command.withdrawal_id)
            .await?;

        withdrawal.succeed()?;
        self.withdrawal_repository
            .save(uow, request_context, &mut withdrawal)
            .await?;

        Ok(WithdrawalSucceedOutput {})
    }
}
