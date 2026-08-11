use appletheia::application::authorization::{AuthorizationPlan, PrincipalRequirement};
use appletheia::application::command::CommandHandler;
use appletheia::application::repository::Repository;
use appletheia::application::request_context::RequestContext;
use banking_ledger_domain::deposit::{Deposit, DepositTokenTransferResult};

use crate::mint::TokenDepositVerifier;

use super::{
    DepositTokenTransferRecordCommand, DepositTokenTransferRecordCommandHandlerError,
    DepositTokenTransferRecordOutput,
};

/// Handles `DepositTokenTransferRecordCommand`.
pub struct DepositTokenTransferRecordCommandHandler<DR, TDV>
where
    DR: Repository<Deposit>,
    TDV: TokenDepositVerifier,
{
    deposit_repository: DR,
    token_deposit_verifier: TDV,
}

impl<DR, TDV> DepositTokenTransferRecordCommandHandler<DR, TDV>
where
    DR: Repository<Deposit>,
    TDV: TokenDepositVerifier,
{
    pub fn new(deposit_repository: DR, token_deposit_verifier: TDV) -> Self {
        Self {
            deposit_repository,
            token_deposit_verifier,
        }
    }
}

impl<DR, TDV> CommandHandler for DepositTokenTransferRecordCommandHandler<DR, TDV>
where
    DR: Repository<Deposit>,
    TDV: TokenDepositVerifier,
{
    type Command = DepositTokenTransferRecordCommand;
    type Output = DepositTokenTransferRecordOutput;
    type Error = DepositTokenTransferRecordCommandHandlerError;
    type Uow = DR::Uow;

    fn authorization_plan(
        &self,
        _command: &Self::Command,
    ) -> Result<AuthorizationPlan, Self::Error> {
        Ok(AuthorizationPlan::OnlyPrincipals(vec![
            PrincipalRequirement::Authenticated,
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

        self.token_deposit_verifier
            .verify(command.deposit_id)
            .await?;
        let result = deposit.record_token_transfer()?;
        self.deposit_repository
            .save(uow, request_context, &mut deposit)
            .await?;

        let output = match result {
            DepositTokenTransferResult::TokenTransferred => {
                DepositTokenTransferRecordOutput::TokenTransferred
            }
            DepositTokenTransferResult::Rejected { .. } => {
                DepositTokenTransferRecordOutput::Rejected
            }
        };

        Ok(output)
    }
}
