use appletheia::application::authorization::{
    AuthorizationPlan, PrincipalRequirement, Relation, RelationshipRequirement,
};
use appletheia::application::command::CommandHandler;
use appletheia::application::repository::{Repository, RepositoryError};
use appletheia::application::request_context::RequestContext;
use appletheia::domain::Aggregate;
use banking_ledger_domain::account::Account;
use banking_ledger_domain::currency::Currency;
use banking_ledger_domain::deposit::Deposit;
use banking_ledger_domain::deposit::DepositError;
use banking_ledger_domain::token_binding::TokenBinding;

use super::{
    DepositSettlementPrepareCommand, DepositSettlementPrepareCommandHandlerError,
    DepositSettlementPrepareOutput,
};
use crate::authorization::AccountDepositRequesterRelation;
use crate::settlement::{DepositSettlementPrepareRequest, DepositSettlementPreparer};

pub struct DepositSettlementPrepareCommandHandler<R, DSP>
where
    R: Repository,
    DSP: DepositSettlementPreparer,
{
    repository: R,
    deposit_settlement_preparer: DSP,
}

impl<R, DSP> DepositSettlementPrepareCommandHandler<R, DSP>
where
    R: Repository,
    DSP: DepositSettlementPreparer,
{
    pub fn new(repository: R, deposit_settlement_preparer: DSP) -> Self {
        Self {
            repository,
            deposit_settlement_preparer,
        }
    }
}

impl<R, DSP> CommandHandler for DepositSettlementPrepareCommandHandler<R, DSP>
where
    R: Repository,
    DSP: DepositSettlementPreparer,
{
    type Command = DepositSettlementPrepareCommand;
    type Output = DepositSettlementPrepareOutput;
    type Error = DepositSettlementPrepareCommandHandlerError;
    type Uow = R::Uow;

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
            .repository
            .read::<Account>(uow, command.account_id)
            .await?;
        let currency = self
            .repository
            .read::<Currency>(uow, *account.currency_id()?)
            .await?;

        let mut deposit = Deposit::new();
        let deposit_id = deposit.aggregate_id();
        let binding = match self
            .repository
            .read::<TokenBinding>(uow, command.token_binding_id)
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
                return Err(DepositError::TokenBindingUnavailable.into());
            }
            Err(error) => return Err(error.into()),
        };
        let chain_network = binding.chain_network()?;
        let token_address = *binding.token_address()?;
        deposit.request(
            command.account_id,
            command.token_binding_id,
            command.token_owner_address,
            command.amount,
        )?;
        if let Some(note) = &command.note {
            deposit.set_note(Some(note.clone()))?;
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
                command.evm_authorization,
            ))
            .await?;
        self.repository
            .save::<Deposit>(uow, request_context, &mut deposit)
            .await?;

        Ok(DepositSettlementPrepareOutput {
            deposit_id,
            preparation,
        })
    }
}
