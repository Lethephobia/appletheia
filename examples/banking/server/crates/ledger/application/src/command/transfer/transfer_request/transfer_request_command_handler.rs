use appletheia::application::authorization::{
    AuthorizationPlan, PrincipalRequirement, Relation, RelationshipRequirement,
};
use appletheia::application::command::CommandHandler;
use appletheia::application::repository::Repository;
use appletheia::application::request_context::RequestContext;
use appletheia::domain::Aggregate;
use banking_ledger_domain::account::Account;
use banking_ledger_domain::transfer::{
    Transfer, TransferRequest, TransferRequestRejectionReason, TransferRequestResult,
};

use crate::authorization::AccountTransferRequesterRelation;

use super::{TransferRequestCommand, TransferRequestCommandHandlerError, TransferRequestOutput};

/// Handles `TransferRequestCommand`.
pub struct TransferRequestCommandHandler<AR, TR>
where
    AR: Repository<Account, Uow = TR::Uow>,
    TR: Repository<Transfer>,
{
    account_repository: AR,
    transfer_repository: TR,
}

impl<AR, TR> TransferRequestCommandHandler<AR, TR>
where
    AR: Repository<Account, Uow = TR::Uow>,
    TR: Repository<Transfer>,
{
    pub fn new(account_repository: AR, transfer_repository: TR) -> Self {
        Self {
            account_repository,
            transfer_repository,
        }
    }
}

impl<AR, TR> CommandHandler for TransferRequestCommandHandler<AR, TR>
where
    AR: Repository<Account, Uow = TR::Uow>,
    TR: Repository<Transfer>,
{
    type Command = TransferRequestCommand;
    type Output = TransferRequestOutput;
    type Error = TransferRequestCommandHandlerError;
    type Uow = TR::Uow;

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
            .account_repository
            .read(uow, command.from_account_id)
            .await?;
        let destination_account = self
            .account_repository
            .read(uow, command.to_account_id)
            .await?;

        let mut transfer = Transfer::new();
        let transfer_id = transfer.aggregate_id();
        let request = TransferRequest {
            from_account_id: command.from_account_id,
            to_account_id: command.to_account_id,
            amount: command.amount,
            note: command.note.clone(),
        };
        if source_account.currency_id()? != destination_account.currency_id()? {
            let reason = TransferRequestRejectionReason::CurrencyMismatch;
            transfer.reject_request(request, reason)?;

            self.transfer_repository
                .save(uow, request_context, &mut transfer)
                .await?;

            return Ok(TransferRequestOutput::Rejected {
                transfer_id,
                reason,
            });
        }

        let result = transfer.request(request)?;

        self.transfer_repository
            .save(uow, request_context, &mut transfer)
            .await?;

        let output = match result {
            TransferRequestResult::Requested => TransferRequestOutput::Requested { transfer_id },
            TransferRequestResult::Rejected { reason } => TransferRequestOutput::Rejected {
                transfer_id,
                reason,
            },
        };

        Ok(output)
    }
}
