use appletheia::application::authorization::{
    AuthorizationPlan, PrincipalRequirement, Relation, RelationshipRequirement,
};
use appletheia::application::command::CommandHandler;
use appletheia::application::repository::Repository;
use appletheia::application::request_context::RequestContext;
use banking_ledger_domain::transfer::Transfer;

use super::{TransferNoteSetCommand, TransferNoteSetCommandHandlerError, TransferNoteSetOutput};
use crate::authorization::TransferNoteSetterRelation;

pub struct TransferNoteSetCommandHandler<R>
where
    R: Repository,
{
    repository: R,
}

impl<R> TransferNoteSetCommandHandler<R>
where
    R: Repository,
{
    pub fn new(repository: R) -> Self {
        Self { repository }
    }
}

impl<R> CommandHandler for TransferNoteSetCommandHandler<R>
where
    R: Repository,
{
    type Command = TransferNoteSetCommand;
    type Output = TransferNoteSetOutput;
    type Error = TransferNoteSetCommandHandlerError;
    type Uow = R::Uow;

    fn authorization_plan(
        &self,
        command: &Self::Command,
    ) -> Result<AuthorizationPlan, Self::Error> {
        Ok(AuthorizationPlan::OnlyPrincipals(vec![
            PrincipalRequirement::AuthenticatedWithRelationship(RelationshipRequirement::check::<
                Transfer,
            >(
                command.transfer_id,
                TransferNoteSetterRelation::REF,
            )),
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

        transfer.set_note(command.note.clone())?;

        self.repository
            .save(uow, request_context, &mut transfer)
            .await?;

        Ok(TransferNoteSetOutput {})
    }
}
