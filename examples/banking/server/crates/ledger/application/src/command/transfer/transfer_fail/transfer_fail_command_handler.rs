use appletheia::application::authorization::{AuthorizationPlan, PrincipalRequirement};
use appletheia::application::command::CommandHandler;
use appletheia::application::repository::Repository;
use appletheia::application::request_context::RequestContext;
use banking_ledger_domain::transfer::Transfer;

use super::{TransferFailCommand, TransferFailCommandHandlerError, TransferFailOutput};

pub struct TransferFailCommandHandler<R>
where
    R: Repository,
{
    repository: R,
}

impl<R> TransferFailCommandHandler<R>
where
    R: Repository,
{
    pub fn new(repository: R) -> Self {
        Self { repository }
    }
}

impl<R> CommandHandler for TransferFailCommandHandler<R>
where
    R: Repository,
{
    type Command = TransferFailCommand;
    type Output = TransferFailOutput;
    type Error = TransferFailCommandHandlerError;
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
        let mut transfer = self
            .repository
            .read::<Transfer>(uow, command.transfer_id)
            .await?;

        transfer.fail(command.reason)?;
        self.repository
            .save(uow, request_context, &mut transfer)
            .await?;

        Ok(TransferFailOutput {})
    }
}
