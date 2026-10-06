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
    R: Repository<Transfer>,
{
    transfer_repository: R,
}

impl<R> TransferNoteSetCommandHandler<R>
where
    R: Repository<Transfer>,
{
    pub fn new(transfer_repository: R) -> Self {
        Self {
            transfer_repository,
        }
    }
}

impl<R> CommandHandler for TransferNoteSetCommandHandler<R>
where
    R: Repository<Transfer>,
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
            .transfer_repository
            .read(uow, command.transfer_id)
            .await?;

        transfer.set_note(command.note.clone())?;

        self.transfer_repository
            .save(uow, request_context, &mut transfer)
            .await?;

        Ok(TransferNoteSetOutput {})
    }
}
