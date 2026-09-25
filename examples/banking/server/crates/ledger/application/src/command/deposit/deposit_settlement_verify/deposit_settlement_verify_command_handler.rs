use appletheia::application::authorization::{AuthorizationPlan, PrincipalRequirement};
use appletheia::application::command::CommandHandler;
use appletheia::application::repository::{Repository, RepositoryError};
use appletheia::application::request_context::RequestContext;
use banking_ledger_domain::account::Account;
use banking_ledger_domain::currency::Currency;
use banking_ledger_domain::deposit::{
    Deposit, DepositSettlementVerifyRejectionReason, DepositSettlementVerifyResult,
};
use banking_ledger_domain::token_binding::TokenBinding;

use crate::settlement::{DepositSettlementVerifier, DepositSettlementVerifyRequest};

use super::{
    DepositSettlementVerifyCommand, DepositSettlementVerifyCommandHandlerError,
    DepositSettlementVerifyOutput,
};

/// Handles `DepositSettlementVerifyCommand`.
pub struct DepositSettlementVerifyCommandHandler<DR, AR, CR, TBR, TDV>
where
    DR: Repository<Deposit>,
    AR: Repository<Account, Uow = DR::Uow>,
    CR: Repository<Currency, Uow = DR::Uow>,
    TBR: Repository<TokenBinding, Uow = DR::Uow>,
    TDV: DepositSettlementVerifier,
{
    deposit_repository: DR,
    account_repository: AR,
    currency_repository: CR,
    token_binding_repository: TBR,
    deposit_settlement_verifier: TDV,
}

impl<DR, AR, CR, TBR, TDV> DepositSettlementVerifyCommandHandler<DR, AR, CR, TBR, TDV>
where
    DR: Repository<Deposit>,
    AR: Repository<Account, Uow = DR::Uow>,
    CR: Repository<Currency, Uow = DR::Uow>,
    TBR: Repository<TokenBinding, Uow = DR::Uow>,
    TDV: DepositSettlementVerifier,
{
    pub fn new(
        deposit_repository: DR,
        account_repository: AR,
        currency_repository: CR,
        token_binding_repository: TBR,
        deposit_settlement_verifier: TDV,
    ) -> Self {
        Self {
            deposit_repository,
            account_repository,
            currency_repository,
            token_binding_repository,
            deposit_settlement_verifier,
        }
    }
}

impl<DR, AR, CR, TBR, TDV> CommandHandler
    for DepositSettlementVerifyCommandHandler<DR, AR, CR, TBR, TDV>
where
    DR: Repository<Deposit>,
    AR: Repository<Account, Uow = DR::Uow>,
    CR: Repository<Currency, Uow = DR::Uow>,
    TBR: Repository<TokenBinding, Uow = DR::Uow>,
    TDV: DepositSettlementVerifier,
{
    type Command = DepositSettlementVerifyCommand;
    type Output = DepositSettlementVerifyOutput;
    type Error = DepositSettlementVerifyCommandHandlerError;
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
        let account = self
            .account_repository
            .read(uow, *deposit.account_id()?)
            .await?;
        let currency = self
            .currency_repository
            .read(uow, *account.currency_id()?)
            .await?;
        let token_binding = match self
            .token_binding_repository
            .read(uow, deposit.token_binding_id()?)
            .await
        {
            Ok(token_binding)
                if token_binding.is_active()?
                    && token_binding.is_deposit_enabled()?
                    && token_binding.currency_id()? == *account.currency_id()? =>
            {
                token_binding
            }
            Ok(_) | Err(RepositoryError::NotFound { .. }) => {
                let reason = DepositSettlementVerifyRejectionReason::TokenBindingUnavailable;
                deposit.reject_settlement_verify(command.transaction_id, reason)?;
                self.deposit_repository
                    .save(uow, request_context, &mut deposit)
                    .await?;
                return Ok(DepositSettlementVerifyOutput::Rejected);
            }
            Err(error) => return Err(error.into()),
        };
        let chain_network = token_binding.chain_network()?;
        if !command.transaction_id.matches_network(chain_network) {
            let reason = DepositSettlementVerifyRejectionReason::ChainMismatch;
            deposit.reject_settlement_verify(command.transaction_id, reason)?;
            self.deposit_repository
                .save(uow, request_context, &mut deposit)
                .await?;
            return Ok(DepositSettlementVerifyOutput::Rejected);
        }

        let verification = self
            .deposit_settlement_verifier
            .verify(DepositSettlementVerifyRequest {
                deposit_id: command.deposit_id,
                currency_decimals: currency.decimals()?,
                chain_network,
                token_address: *token_binding.token_address()?,
                token_owner_address: *deposit.token_owner_address()?,
                amount: deposit.amount()?,
                transaction_id: command.transaction_id,
            })
            .await?;
        let result = deposit.record_settlement_verified(verification.transaction_id)?;
        self.deposit_repository
            .save(uow, request_context, &mut deposit)
            .await?;

        let output = match result {
            DepositSettlementVerifyResult::Verified => DepositSettlementVerifyOutput::Verified,
            DepositSettlementVerifyResult::Rejected { .. } => {
                DepositSettlementVerifyOutput::Rejected
            }
        };

        Ok(output)
    }
}
