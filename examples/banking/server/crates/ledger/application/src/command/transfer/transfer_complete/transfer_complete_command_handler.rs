use appletheia::application::authorization::{AuthorizationPlan, PrincipalRequirement};
use appletheia::application::command::{CommandHandled, CommandHandler};
use appletheia::application::repository::Repository;
use appletheia::application::request_context::RequestContext;
use banking_ledger_domain::transfer::Transfer;

use super::{TransferCompleteCommand, TransferCompleteCommandHandlerError, TransferCompleteOutput};

/// Handles `TransferCompleteCommand`.
pub struct TransferCompleteCommandHandler<TR>
where
    TR: Repository<Transfer>,
{
    transfer_repository: TR,
}

impl<TR> TransferCompleteCommandHandler<TR>
where
    TR: Repository<Transfer>,
{
    pub fn new(transfer_repository: TR) -> Self {
        Self {
            transfer_repository,
        }
    }
}

impl<TR> CommandHandler for TransferCompleteCommandHandler<TR>
where
    TR: Repository<Transfer>,
{
    type Command = TransferCompleteCommand;
    type Output = TransferCompleteOutput;
    type ReplayOutput = TransferCompleteOutput;
    type Error = TransferCompleteCommandHandlerError;
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
        let Some(mut transfer) = self
            .transfer_repository
            .find(uow, command.transfer_id)
            .await?
        else {
            return Err(TransferCompleteCommandHandlerError::TransferNotFound);
        };

        transfer.complete()?;
        self.transfer_repository
            .save(uow, request_context, &mut transfer)
            .await?;

        Ok(CommandHandled::same(TransferCompleteOutput))
    }
}
