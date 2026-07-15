use appletheia::application::authorization::{AuthorizationPlan, PrincipalRequirement};
use appletheia::application::command::{CommandHandled, CommandHandler};
use appletheia::application::repository::Repository;
use appletheia::application::request_context::RequestContext;
use banking_ledger_domain::transfer::{Transfer, TransferFailResult};

use super::{TransferFailCommand, TransferFailCommandHandlerError, TransferFailOutput};

/// Handles `TransferFailCommand`.
pub struct TransferFailCommandHandler<TR>
where
    TR: Repository<Transfer>,
{
    transfer_repository: TR,
}

impl<TR> TransferFailCommandHandler<TR>
where
    TR: Repository<Transfer>,
{
    pub fn new(transfer_repository: TR) -> Self {
        Self {
            transfer_repository,
        }
    }
}

impl<TR> CommandHandler for TransferFailCommandHandler<TR>
where
    TR: Repository<Transfer>,
{
    type Command = TransferFailCommand;
    type Output = TransferFailOutput;
    type ReplayOutput = TransferFailOutput;
    type Error = TransferFailCommandHandlerError;
    type Uow = TR::Uow;

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
        let mut transfer = self
            .transfer_repository
            .read(uow, command.transfer_id)
            .await?;

        let result = transfer.fail(command.reason)?;
        self.transfer_repository
            .save(uow, request_context, &mut transfer)
            .await?;

        let output = match result {
            TransferFailResult::Failed => TransferFailOutput::Failed,
            TransferFailResult::Rejected { reason } => TransferFailOutput::Rejected { reason },
        };

        Ok(CommandHandled::same(output))
    }
}
