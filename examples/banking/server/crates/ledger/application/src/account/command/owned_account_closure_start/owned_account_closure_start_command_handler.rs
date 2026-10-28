use appletheia::application::authorization::{AuthorizationPlan, PrincipalRequirement};
use appletheia::application::command::CommandHandler;
use appletheia::application::repository::Repository;
use appletheia::application::request_context::RequestContext;
use appletheia::domain::Aggregate;
use banking_ledger_domain::owned_account_closure::OwnedAccountClosure;

use super::{
    OwnedAccountClosureStartCommand, OwnedAccountClosureStartCommandHandlerError,
    OwnedAccountClosureStartOutput,
};

pub struct OwnedAccountClosureStartCommandHandler<R>
where
    R: Repository,
{
    repository: R,
}

impl<R> OwnedAccountClosureStartCommandHandler<R>
where
    R: Repository,
{
    pub fn new(repository: R) -> Self {
        Self { repository }
    }
}

impl<R> CommandHandler for OwnedAccountClosureStartCommandHandler<R>
where
    R: Repository,
{
    type Command = OwnedAccountClosureStartCommand;
    type Output = OwnedAccountClosureStartOutput;
    type Error = OwnedAccountClosureStartCommandHandlerError;
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
        let mut owned_account_closure = OwnedAccountClosure::new();
        let owned_account_closure_id = owned_account_closure.aggregate_id();
        owned_account_closure.start(command.owner)?;

        self.repository
            .save(uow, request_context, &mut owned_account_closure)
            .await?;

        Ok(OwnedAccountClosureStartOutput {
            owned_account_closure_id,
        })
    }
}
