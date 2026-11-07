use crate::authorization::WithdrawalAccountTransactorRelation;
use appletheia::application::authorization::{
    AuthorizationPlan, PrincipalRequirement, Relation, RelationshipRequirement,
};
use appletheia::application::command::CommandHandler;
use appletheia::application::repository::Repository;
use appletheia::application::request_context::RequestContext;
use banking_ledger_domain::withdrawal::Withdrawal;

use super::{
    WithdrawalSetNoteCommand, WithdrawalSetNoteCommandHandlerError, WithdrawalSetNoteOutput,
};

pub struct WithdrawalSetNoteCommandHandler<R>
where
    R: Repository,
{
    repository: R,
}

impl<R> WithdrawalSetNoteCommandHandler<R>
where
    R: Repository,
{
    pub fn new(repository: R) -> Self {
        Self { repository }
    }
}

impl<R> CommandHandler for WithdrawalSetNoteCommandHandler<R>
where
    R: Repository,
{
    type Command = WithdrawalSetNoteCommand;
    type Output = WithdrawalSetNoteOutput;
    type Error = WithdrawalSetNoteCommandHandlerError;
    type Uow = R::Uow;

    fn authorization_plan(
        &self,
        command: &Self::Command,
    ) -> Result<AuthorizationPlan, Self::Error> {
        Ok(AuthorizationPlan::OnlyPrincipals(vec![
            PrincipalRequirement::AuthenticatedWithRelationship(RelationshipRequirement::check::<
                Withdrawal,
            >(
                command.withdrawal_id,
                WithdrawalAccountTransactorRelation::REF,
            )),
        ]))
    }

    async fn handle(
        &self,
        uow: &mut Self::Uow,
        request_context: &RequestContext,
        command: &Self::Command,
    ) -> Result<Self::Output, Self::Error> {
        let mut withdrawal = self
            .repository
            .read::<Withdrawal>(uow, command.withdrawal_id)
            .await?;

        withdrawal.set_note(command.note.clone())?;

        self.repository
            .save(uow, request_context, &mut withdrawal)
            .await?;

        Ok(WithdrawalSetNoteOutput {})
    }
}
