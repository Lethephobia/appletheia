use appletheia::application::authorization::{AuthorizationPlan, PrincipalRequirement};
use appletheia::application::command::CommandHandler;
use appletheia::application::repository::Repository;
use appletheia::application::request_context::RequestContext;
use banking_ledger_domain::owned_account_closure::OwnedAccountClosure;

use super::{
    OwnedAccountClosureAccountFailedRecordCommand,
    OwnedAccountClosureAccountFailedRecordCommandHandlerError,
    OwnedAccountClosureAccountFailedRecordOutput,
};

pub struct OwnedAccountClosureAccountFailedRecordCommandHandler<R>
where
    R: Repository,
{
    repository: R,
}

impl<R> OwnedAccountClosureAccountFailedRecordCommandHandler<R>
where
    R: Repository,
{
    pub fn new(repository: R) -> Self {
        Self { repository }
    }
}

impl<R> CommandHandler for OwnedAccountClosureAccountFailedRecordCommandHandler<R>
where
    R: Repository,
{
    type Command = OwnedAccountClosureAccountFailedRecordCommand;
    type Output = OwnedAccountClosureAccountFailedRecordOutput;
    type Error = OwnedAccountClosureAccountFailedRecordCommandHandlerError;
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
        let mut owned_account_closure = self
            .repository
            .read::<OwnedAccountClosure>(uow, command.owned_account_closure_id)
            .await?;

        owned_account_closure.record_account_failed(command.account_id)?;
        if owned_account_closure.is_ready_to_complete()? {
            owned_account_closure.complete()?;
        }
        self.repository
            .save(uow, request_context, &mut owned_account_closure)
            .await?;

        Ok(OwnedAccountClosureAccountFailedRecordOutput {})
    }
}
