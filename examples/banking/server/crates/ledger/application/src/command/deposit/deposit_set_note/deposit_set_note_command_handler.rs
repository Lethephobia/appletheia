use crate::authorization::DepositAccountTransactorRelation;
use appletheia::application::authorization::{
    AuthorizationPlan, PrincipalRequirement, Relation, RelationshipRequirement,
};
use appletheia::application::command::CommandHandler;
use appletheia::application::repository::Repository;
use appletheia::application::request_context::RequestContext;
use banking_ledger_domain::deposit::Deposit;

use super::{DepositSetNoteCommand, DepositSetNoteCommandHandlerError, DepositSetNoteOutput};

pub struct DepositSetNoteCommandHandler<R>
where
    R: Repository,
{
    repository: R,
}

impl<R> DepositSetNoteCommandHandler<R>
where
    R: Repository,
{
    pub fn new(repository: R) -> Self {
        Self { repository }
    }
}

impl<R> CommandHandler for DepositSetNoteCommandHandler<R>
where
    R: Repository,
{
    type Command = DepositSetNoteCommand;
    type Output = DepositSetNoteOutput;
    type Error = DepositSetNoteCommandHandlerError;
    type Uow = R::Uow;

    fn authorization_plan(
        &self,
        command: &Self::Command,
    ) -> Result<AuthorizationPlan, Self::Error> {
        Ok(AuthorizationPlan::OnlyPrincipals(vec![
            PrincipalRequirement::AuthenticatedWithRelationship(RelationshipRequirement::check::<
                Deposit,
            >(
                command.deposit_id,
                DepositAccountTransactorRelation::REF,
            )),
        ]))
    }

    async fn handle(
        &self,
        uow: &mut Self::Uow,
        request_context: &RequestContext,
        command: &Self::Command,
    ) -> Result<Self::Output, Self::Error> {
        let mut deposit = self
            .repository
            .read::<Deposit>(uow, command.deposit_id)
            .await?;

        deposit.set_note(command.note.clone())?;

        self.repository
            .save(uow, request_context, &mut deposit)
            .await?;

        Ok(DepositSetNoteOutput {})
    }
}
