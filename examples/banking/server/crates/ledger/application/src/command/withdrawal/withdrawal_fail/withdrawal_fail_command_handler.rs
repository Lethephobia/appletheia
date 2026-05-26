use appletheia::application::authorization::{AuthorizationPlan, PrincipalRequirement};
use appletheia::application::command::{CommandHandled, CommandHandler};
use appletheia::application::repository::Repository;
use appletheia::application::request_context::RequestContext;
use banking_ledger_domain::withdrawal::{Withdrawal, WithdrawalFailResult};

use super::{WithdrawalFailCommand, WithdrawalFailCommandHandlerError, WithdrawalFailOutput};

/// Handles `WithdrawalFailCommand`.
pub struct WithdrawalFailCommandHandler<WR>
where
    WR: Repository<Withdrawal>,
{
    withdrawal_repository: WR,
}

impl<WR> WithdrawalFailCommandHandler<WR>
where
    WR: Repository<Withdrawal>,
{
    pub fn new(withdrawal_repository: WR) -> Self {
        Self {
            withdrawal_repository,
        }
    }
}

impl<WR> CommandHandler for WithdrawalFailCommandHandler<WR>
where
    WR: Repository<Withdrawal>,
{
    type Command = WithdrawalFailCommand;
    type Output = WithdrawalFailOutput;
    type ReplayOutput = WithdrawalFailOutput;
    type Error = WithdrawalFailCommandHandlerError;
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
    ) -> Result<CommandHandled<Self::Output, Self::ReplayOutput>, Self::Error> {
        let Some(mut withdrawal) = self
            .withdrawal_repository
            .find(uow, command.withdrawal_id)
            .await?
        else {
            return Err(WithdrawalFailCommandHandlerError::WithdrawalNotFound);
        };

        let result = withdrawal.fail(command.reason)?;
        self.withdrawal_repository
            .save(uow, request_context, &mut withdrawal)
            .await?;

        let output = match result {
            WithdrawalFailResult::Failed => WithdrawalFailOutput::Failed,
            WithdrawalFailResult::Rejected { reason } => WithdrawalFailOutput::Rejected { reason },
        };

        Ok(CommandHandled::same(output))
    }
}
