use appletheia::application::authorization::{AuthorizationPlan, PrincipalRequirement};
use appletheia::application::command::CommandHandler;
use appletheia::application::repository::Repository;
use appletheia::application::request_context::RequestContext;
use banking_ledger_domain::withdrawal::Withdrawal;

use super::{WithdrawalFailCommand, WithdrawalFailCommandHandlerError, WithdrawalFailOutput};

pub struct WithdrawalFailCommandHandler<R>
where
    R: Repository,
{
    repository: R,
}

impl<R> WithdrawalFailCommandHandler<R>
where
    R: Repository,
{
    pub fn new(repository: R) -> Self {
        Self { repository }
    }
}

impl<R> CommandHandler for WithdrawalFailCommandHandler<R>
where
    R: Repository,
{
    type Command = WithdrawalFailCommand;
    type Output = WithdrawalFailOutput;
    type Error = WithdrawalFailCommandHandlerError;
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

        withdrawal.fail(command.reason)?;
        self.repository
            .save(uow, request_context, &mut withdrawal)
            .await?;

        Ok(WithdrawalFailOutput {})
    }
}
