use appletheia::application::authorization::{AuthorizationPlan, PrincipalRequirement};
use appletheia::application::command::{CommandHandled, CommandHandler};
use appletheia::application::repository::Repository;
use appletheia::application::request_context::RequestContext;
use banking_ledger_domain::owned_account_closure::OwnedAccountClosure;

use super::{
    OwnedAccountClosureAccountCloseRecordCommand,
    OwnedAccountClosureAccountCloseRecordCommandHandlerError,
    OwnedAccountClosureAccountCloseRecordOutput,
};

/// Handles `OwnedAccountClosureAccountCloseRecordCommand`.
pub struct OwnedAccountClosureAccountCloseRecordCommandHandler<OACR>
where
    OACR: Repository<OwnedAccountClosure>,
{
    owned_account_closure_repository: OACR,
}

impl<OACR> OwnedAccountClosureAccountCloseRecordCommandHandler<OACR>
where
    OACR: Repository<OwnedAccountClosure>,
{
    pub fn new(owned_account_closure_repository: OACR) -> Self {
        Self {
            owned_account_closure_repository,
        }
    }
}

impl<OACR> CommandHandler for OwnedAccountClosureAccountCloseRecordCommandHandler<OACR>
where
    OACR: Repository<OwnedAccountClosure>,
{
    type Command = OwnedAccountClosureAccountCloseRecordCommand;
    type Output = OwnedAccountClosureAccountCloseRecordOutput;
    type ReplayOutput = OwnedAccountClosureAccountCloseRecordOutput;
    type Error = OwnedAccountClosureAccountCloseRecordCommandHandlerError;
    type Uow = OACR::Uow;

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
        let Some(mut owned_account_closure) = self
            .owned_account_closure_repository
            .find(uow, command.owned_account_closure_id)
            .await?
        else {
            return Err(
                OwnedAccountClosureAccountCloseRecordCommandHandlerError::OwnedAccountClosureNotFound,
            );
        };

        let result = owned_account_closure.record_account_close(command.account_id)?;
        self.owned_account_closure_repository
            .save(uow, request_context, &mut owned_account_closure)
            .await?;

        Ok(CommandHandled::same(
            OwnedAccountClosureAccountCloseRecordOutput::from(result),
        ))
    }
}
