use appletheia::application::authorization::{AuthorizationPlan, PrincipalRequirement};
use appletheia::application::command::CommandHandler;
use appletheia::application::repository::Repository;
use appletheia::application::request_context::RequestContext;
use banking_ledger_domain::owned_account_closure::{
    OwnedAccountClosure, OwnedAccountClosureCompleteResult,
};

use super::{
    OwnedAccountClosureCompleteCommand, OwnedAccountClosureCompleteCommandHandlerError,
    OwnedAccountClosureCompleteOutput,
};

/// Handles `OwnedAccountClosureCompleteCommand`.
pub struct OwnedAccountClosureCompleteCommandHandler<OACR>
where
    OACR: Repository<OwnedAccountClosure>,
{
    owned_account_closure_repository: OACR,
}

impl<OACR> OwnedAccountClosureCompleteCommandHandler<OACR>
where
    OACR: Repository<OwnedAccountClosure>,
{
    pub fn new(owned_account_closure_repository: OACR) -> Self {
        Self {
            owned_account_closure_repository,
        }
    }
}

impl<OACR> CommandHandler for OwnedAccountClosureCompleteCommandHandler<OACR>
where
    OACR: Repository<OwnedAccountClosure>,
{
    type Command = OwnedAccountClosureCompleteCommand;
    type Output = OwnedAccountClosureCompleteOutput;
    type Error = OwnedAccountClosureCompleteCommandHandlerError;
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
    ) -> Result<Self::Output, Self::Error> {
        let mut owned_account_closure = self
            .owned_account_closure_repository
            .read(uow, command.owned_account_closure_id)
            .await?;

        let result = owned_account_closure.complete()?;
        self.owned_account_closure_repository
            .save(uow, request_context, &mut owned_account_closure)
            .await?;

        let output = match result {
            OwnedAccountClosureCompleteResult::Completed => {
                OwnedAccountClosureCompleteOutput::Completed
            }
            OwnedAccountClosureCompleteResult::Rejected { reason } => {
                OwnedAccountClosureCompleteOutput::Rejected { reason }
            }
        };

        Ok(output)
    }
}
