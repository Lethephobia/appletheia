use appletheia::application::authorization::{
    AuthorizationPlan, PrincipalRequirement, Relation, RelationshipRequirement,
};
use appletheia::application::command::CommandHandler;
use appletheia::application::repository::{Repository, RepositoryError};
use appletheia::application::request_context::RequestContext;
use appletheia::domain::Aggregate;
use banking_ledger_domain::account::Account;
use banking_ledger_domain::currency::Currency;
use banking_ledger_domain::deposit::{
    Deposit, DepositRequest, DepositRequestRejectionReason, DepositRequestResult,
};
use banking_ledger_domain::token_binding::TokenBinding;

use super::{
    DepositSettlementPrepareCommand, DepositSettlementPrepareCommandHandlerError,
    DepositSettlementPrepareOutput,
};
use crate::authorization::AccountDepositRequesterRelation;
use crate::settlement::{DepositSettlementPrepareRequest, DepositSettlementPreparer};

pub struct DepositSettlementPrepareCommandHandler<DR, AR, CR, TBR, DSP>
where
    DR: Repository<Deposit>,
    AR: Repository<Account, Uow = DR::Uow>,
    CR: Repository<Currency, Uow = DR::Uow>,
    TBR: Repository<TokenBinding, Uow = DR::Uow>,
    DSP: DepositSettlementPreparer,
{
    deposit_repository: DR,
    account_repository: AR,
    currency_repository: CR,
    token_binding_repository: TBR,
    deposit_settlement_preparer: DSP,
}

impl<DR, AR, CR, TBR, DSP> DepositSettlementPrepareCommandHandler<DR, AR, CR, TBR, DSP>
where
    DR: Repository<Deposit>,
    AR: Repository<Account, Uow = DR::Uow>,
    CR: Repository<Currency, Uow = DR::Uow>,
    TBR: Repository<TokenBinding, Uow = DR::Uow>,
    DSP: DepositSettlementPreparer,
{
    pub fn new(
        deposit_repository: DR,
        account_repository: AR,
        currency_repository: CR,
        token_binding_repository: TBR,
        deposit_settlement_preparer: DSP,
    ) -> Self {
        Self {
            deposit_repository,
            account_repository,
            currency_repository,
            token_binding_repository,
            deposit_settlement_preparer,
        }
    }
}

impl<DR, AR, CR, TBR, DSP> CommandHandler
    for DepositSettlementPrepareCommandHandler<DR, AR, CR, TBR, DSP>
where
    DR: Repository<Deposit>,
    AR: Repository<Account, Uow = DR::Uow>,
    CR: Repository<Currency, Uow = DR::Uow>,
    TBR: Repository<TokenBinding, Uow = DR::Uow>,
    DSP: DepositSettlementPreparer,
{
    type Command = DepositSettlementPrepareCommand;
    type Output = DepositSettlementPrepareOutput;
    type Error = DepositSettlementPrepareCommandHandlerError;
    type Uow = DR::Uow;

    fn authorization_plan(
        &self,
        command: &Self::Command,
    ) -> Result<AuthorizationPlan, Self::Error> {
        Ok(AuthorizationPlan::OnlyPrincipals(vec![
            PrincipalRequirement::AuthenticatedWithRelationship(RelationshipRequirement::check::<
                Account,
            >(
                command.account_id,
                AccountDepositRequesterRelation::REF,
            )),
        ]))
    }

    async fn handle(
        &self,
        uow: &mut Self::Uow,
        request_context: &RequestContext,
        command: &Self::Command,
    ) -> Result<Self::Output, Self::Error> {
        let account = self
            .account_repository
            .read(uow, command.account_id)
            .await?;
        let currency = self
            .currency_repository
            .read(uow, *account.currency_id()?)
            .await?;

        let mut deposit = Deposit::new();
        let deposit_id = deposit.aggregate_id();
        let request = DepositRequest {
            account_id: command.account_id,
            token_binding_id: command.token_binding_id,
            token_owner_address: command.token_owner_address,
            amount: command.amount,
            note: command.note.clone(),
        };
        let binding = match self
            .token_binding_repository
            .read(uow, command.token_binding_id)
            .await
        {
            Ok(binding)
                if binding.is_active()?
                    && binding.is_deposit_enabled()?
                    && binding.currency_id()? == *account.currency_id()? =>
            {
                binding
            }
            Ok(_) | Err(RepositoryError::NotFound { .. }) => {
                let reason = DepositRequestRejectionReason::TokenBindingUnavailable;
                deposit.reject_request(request, reason)?;
                self.deposit_repository
                    .save(uow, request_context, &mut deposit)
                    .await?;
                return Ok(DepositSettlementPrepareOutput::Rejected { deposit_id, reason });
            }
            Err(error) => return Err(error.into()),
        };
        let chain_network = binding.chain_network()?;
        let token_address = *binding.token_address()?;
        let result = deposit.request(request)?;

        if let DepositRequestResult::Rejected { reason } = result {
            self.deposit_repository
                .save(uow, request_context, &mut deposit)
                .await?;
            return Ok(DepositSettlementPrepareOutput::Rejected { deposit_id, reason });
        }

        let preparation = self
            .deposit_settlement_preparer
            .prepare(DepositSettlementPrepareRequest::new(
                deposit_id,
                currency.decimals()?,
                chain_network,
                token_address,
                *deposit.token_owner_address()?,
                deposit.amount()?,
            ))
            .await?;
        self.deposit_repository
            .save(uow, request_context, &mut deposit)
            .await?;

        Ok(DepositSettlementPrepareOutput::Prepared {
            deposit_id,
            preparation,
        })
    }
}
