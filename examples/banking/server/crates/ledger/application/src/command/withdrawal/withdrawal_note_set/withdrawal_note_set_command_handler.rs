use appletheia::application::authorization::{
    AuthorizationPlan, PrincipalRequirement, Relation, RelationshipRequirement,
};
use appletheia::application::command::CommandHandler;
use appletheia::application::repository::Repository;
use appletheia::application::request_context::RequestContext;
use banking_ledger_domain::withdrawal::Withdrawal;

use super::{
    WithdrawalNoteSetCommand, WithdrawalNoteSetCommandHandlerError, WithdrawalNoteSetOutput,
};
use crate::authorization::WithdrawalNoteSetterRelation;

pub struct WithdrawalNoteSetCommandHandler<R>
where
    R: Repository<Withdrawal>,
{
    withdrawal_repository: R,
}

impl<R> WithdrawalNoteSetCommandHandler<R>
where
    R: Repository<Withdrawal>,
{
    pub fn new(withdrawal_repository: R) -> Self {
        Self {
            withdrawal_repository,
        }
    }
}

impl<R> CommandHandler for WithdrawalNoteSetCommandHandler<R>
where
    R: Repository<Withdrawal>,
{
    type Command = WithdrawalNoteSetCommand;
    type Output = WithdrawalNoteSetOutput;
    type Error = WithdrawalNoteSetCommandHandlerError;
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
                WithdrawalNoteSetterRelation::REF,
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
            .withdrawal_repository
            .read(uow, command.withdrawal_id)
            .await?;

        withdrawal.set_note(command.note.clone())?;

        self.withdrawal_repository
            .save(uow, request_context, &mut withdrawal)
            .await?;

        Ok(WithdrawalNoteSetOutput {})
    }
}
