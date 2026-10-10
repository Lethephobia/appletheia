use appletheia::application::authorization::{
    AuthorizationPlan, PrincipalRequirement, Relation, RelationshipRequirement,
};
use appletheia::application::command::CommandHandler;
use appletheia::application::repository::Repository;
use appletheia::application::request_context::RequestContext;
use appletheia::domain::Aggregate;
use banking_ledger_domain::account::Account;
use banking_ledger_domain::transfer::Transfer;
use banking_ledger_domain::transfer::TransferError;

use crate::authorization::AccountTransferRequesterRelation;

use super::{TransferRequestCommand, TransferRequestCommandHandlerError, TransferRequestOutput};

pub struct TransferRequestCommandHandler<R>
where
    R: Repository,
{
    repository: R,
}

impl<R> TransferRequestCommandHandler<R>
where
    R: Repository,
{
    pub fn new(repository: R) -> Self {
        Self { repository }
    }
}

impl<R> CommandHandler for TransferRequestCommandHandler<R>
where
    R: Repository,
{
    type Command = TransferRequestCommand;
    type Output = TransferRequestOutput;
    type Error = TransferRequestCommandHandlerError;
    type Uow = R::Uow;

    fn authorization_plan(
        &self,
        command: &Self::Command,
    ) -> Result<AuthorizationPlan, Self::Error> {
        Ok(AuthorizationPlan::OnlyPrincipals(vec![
            PrincipalRequirement::AuthenticatedWithRelationship(RelationshipRequirement::check::<
                Account,
            >(
                command.from_account_id,
                AccountTransferRequesterRelation::REF,
            )),
        ]))
    }

    async fn handle(
        &self,
        uow: &mut Self::Uow,
        request_context: &RequestContext,
        command: &Self::Command,
    ) -> Result<Self::Output, Self::Error> {
        let source_account = self
            .repository
            .read_shared::<Account>(uow, command.from_account_id)
            .await?;
        let destination_account = self
            .repository
            .read_shared::<Account>(uow, command.to_account_id)
            .await?;

        let mut transfer = Transfer::new();
        let transfer_id = transfer.aggregate_id();
        if source_account.currency_id()? != destination_account.currency_id()? {
            return Err(TransferError::CurrencyMismatch.into());
        }

        transfer.request(
            command.from_account_id,
            command.to_account_id,
            command.amount,
        )?;
        if let Some(note) = &command.note {
            transfer.set_note(Some(note.clone()))?;
        }

        self.repository
            .save(uow, request_context, &mut transfer)
            .await?;

        Ok(TransferRequestOutput { transfer_id })
    }
}
