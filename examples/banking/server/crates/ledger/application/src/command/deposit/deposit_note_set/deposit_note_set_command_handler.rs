use appletheia::application::authorization::{
    AuthorizationPlan, PrincipalRequirement, Relation, RelationshipRequirement,
};
use appletheia::application::command::CommandHandler;
use appletheia::application::repository::Repository;
use appletheia::application::request_context::RequestContext;
use banking_ledger_domain::deposit::Deposit;

use super::{DepositNoteSetCommand, DepositNoteSetCommandHandlerError, DepositNoteSetOutput};
use crate::authorization::DepositNoteSetterRelation;

pub struct DepositNoteSetCommandHandler<R>
where
    R: Repository<Deposit>,
{
    deposit_repository: R,
}

impl<R> DepositNoteSetCommandHandler<R>
where
    R: Repository<Deposit>,
{
    pub fn new(deposit_repository: R) -> Self {
        Self { deposit_repository }
    }
}

impl<R> CommandHandler for DepositNoteSetCommandHandler<R>
where
    R: Repository<Deposit>,
{
    type Command = DepositNoteSetCommand;
    type Output = DepositNoteSetOutput;
    type Error = DepositNoteSetCommandHandlerError;
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
                DepositNoteSetterRelation::REF,
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
            .deposit_repository
            .read(uow, command.deposit_id)
            .await?;

        deposit.set_note(command.note.clone())?;

        self.deposit_repository
            .save(uow, request_context, &mut deposit)
            .await?;

        Ok(DepositNoteSetOutput {})
    }
}
