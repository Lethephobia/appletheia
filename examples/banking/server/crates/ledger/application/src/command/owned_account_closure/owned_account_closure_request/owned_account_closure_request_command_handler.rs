use appletheia::application::authorization::{AuthorizationPlan, PrincipalRequirement};
use appletheia::application::command::{CommandHandled, CommandHandler};
use appletheia::application::repository::Repository;
use appletheia::application::request_context::RequestContext;
use appletheia::domain::Aggregate;
use banking_ledger_domain::owned_account_closure::{
    OwnedAccountClosure, OwnedAccountClosureRequest, OwnedAccountClosureRequestResult,
};

use super::{
    OwnedAccountClosureRequestCommand, OwnedAccountClosureRequestCommandHandlerError,
    OwnedAccountClosureRequestOutput,
};

/// Handles `OwnedAccountClosureRequestCommand`.
pub struct OwnedAccountClosureRequestCommandHandler<OACR>
where
    OACR: Repository<OwnedAccountClosure>,
{
    owned_account_closure_repository: OACR,
}

impl<OACR> OwnedAccountClosureRequestCommandHandler<OACR>
where
    OACR: Repository<OwnedAccountClosure>,
{
    pub fn new(owned_account_closure_repository: OACR) -> Self {
        Self {
            owned_account_closure_repository,
        }
    }
}

impl<OACR> CommandHandler for OwnedAccountClosureRequestCommandHandler<OACR>
where
    OACR: Repository<OwnedAccountClosure>,
{
    type Command = OwnedAccountClosureRequestCommand;
    type Output = OwnedAccountClosureRequestOutput;
    type ReplayOutput = OwnedAccountClosureRequestOutput;
    type Error = OwnedAccountClosureRequestCommandHandlerError;
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
        let mut owned_account_closure = OwnedAccountClosure::new();
        let owned_account_closure_id = owned_account_closure.aggregate_id();
        let result = owned_account_closure.request(OwnedAccountClosureRequest {
            owner: command.owner,
        })?;

        self.owned_account_closure_repository
            .save(uow, request_context, &mut owned_account_closure)
            .await?;

        let output = match result {
            OwnedAccountClosureRequestResult::Requested => OwnedAccountClosureRequestOutput {
                owned_account_closure_id,
            },
        };

        Ok(CommandHandled::same(output))
    }
}
