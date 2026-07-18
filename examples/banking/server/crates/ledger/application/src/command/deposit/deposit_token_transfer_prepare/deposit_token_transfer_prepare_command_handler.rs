use appletheia::application::authorization::{
    AuthorizationPlan, PrincipalRequirement, Relation, RelationshipRequirement,
};
use appletheia::application::command::{CommandHandled, CommandHandler};
use appletheia::application::repository::Repository;
use appletheia::application::request_context::RequestContext;
use appletheia::domain::Aggregate;
use banking_ledger_domain::account::Account;
use banking_ledger_domain::currency::Currency;
use banking_ledger_domain::deposit::{
    Deposit, DepositRequest, DepositRequestRejectionReason, DepositRequestResult,
};

use super::{
    DepositTokenTransferPrepareCommand, DepositTokenTransferPrepareCommandHandlerError,
    DepositTokenTransferPrepareOutput,
};
use crate::authorization::AccountDepositRequesterRelation;
use crate::mint::{
    TokenAccountOwnerAddressValidationResult, TokenAccountOwnerAddressValidator,
    TokenAccountOwnerAddressValidatorError, TokenDepositPrepareRequest, TokenDepositPreparer,
};

/// Handles `DepositTokenTransferPrepareCommand`.
pub struct DepositTokenTransferPrepareCommandHandler<DR, AR, CR, TAOV, PTDP>
where
    DR: Repository<Deposit>,
    AR: Repository<Account, Uow = DR::Uow>,
    CR: Repository<Currency, Uow = AR::Uow>,
    TAOV: TokenAccountOwnerAddressValidator,
    PTDP: TokenDepositPreparer,
{
    deposit_repository: DR,
    account_repository: AR,
    currency_repository: CR,
    token_account_owner_address_validator: TAOV,
    token_deposit_preparer: PTDP,
}

impl<DR, AR, CR, TAOV, PTDP> DepositTokenTransferPrepareCommandHandler<DR, AR, CR, TAOV, PTDP>
where
    DR: Repository<Deposit>,
    AR: Repository<Account, Uow = DR::Uow>,
    CR: Repository<Currency, Uow = AR::Uow>,
    TAOV: TokenAccountOwnerAddressValidator,
    PTDP: TokenDepositPreparer,
{
    pub fn new(
        deposit_repository: DR,
        account_repository: AR,
        currency_repository: CR,
        token_account_owner_address_validator: TAOV,
        token_deposit_preparer: PTDP,
    ) -> Self {
        Self {
            deposit_repository,
            account_repository,
            currency_repository,
            token_account_owner_address_validator,
            token_deposit_preparer,
        }
    }
}

impl<DR, AR, CR, TAOV, PTDP> CommandHandler
    for DepositTokenTransferPrepareCommandHandler<DR, AR, CR, TAOV, PTDP>
where
    DR: Repository<Deposit>,
    AR: Repository<Account, Uow = DR::Uow>,
    CR: Repository<Currency, Uow = AR::Uow>,
    TAOV: TokenAccountOwnerAddressValidator,
    PTDP: TokenDepositPreparer,
{
    type Command = DepositTokenTransferPrepareCommand;
    type Output = DepositTokenTransferPrepareOutput;
    type ReplayOutput = DepositTokenTransferPrepareOutput;
    type Error = DepositTokenTransferPrepareCommandHandlerError;
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
    ) -> Result<CommandHandled<Self::Output, Self::ReplayOutput>, Self::Error> {
        let account = self
            .account_repository
            .read(uow, command.account_id)
            .await?;
        let currency_id = *account.currency_id()?;

        let mut deposit = Deposit::new();
        let deposit_id = deposit.aggregate_id();
        let request = DepositRequest {
            account_id: command.account_id,
            currency_id,
            token_account_owner_address: command.token_account_owner_address.clone(),
            amount: command.amount,
        };

        match self
            .token_account_owner_address_validator
            .validate(&command.token_account_owner_address)
            .await
        {
            Ok(TokenAccountOwnerAddressValidationResult::Valid) => {}
            Ok(TokenAccountOwnerAddressValidationResult::Invalid) => {
                let reason = DepositRequestRejectionReason::InvalidTokenAccountOwnerAddress;
                deposit.reject_request(request, reason)?;
                self.deposit_repository
                    .save(uow, request_context, &mut deposit)
                    .await?;
                return Ok(CommandHandled::same(
                    DepositTokenTransferPrepareOutput::Rejected { deposit_id, reason },
                ));
            }
            Err(error @ TokenAccountOwnerAddressValidatorError::Backend(_)) => {
                return Err(error.into());
            }
        }

        let currency = self.currency_repository.read(uow, currency_id).await?;
        let Some(mint_account) = currency.mint_account()? else {
            let reason = DepositRequestRejectionReason::CurrencyUnprovisioned;
            deposit.reject_request(request, reason)?;
            self.deposit_repository
                .save(uow, request_context, &mut deposit)
                .await?;
            return Ok(CommandHandled::same(
                DepositTokenTransferPrepareOutput::Rejected { deposit_id, reason },
            ));
        };

        let result = deposit.request(request)?;
        match result {
            DepositRequestResult::Requested => {}
            DepositRequestResult::Rejected { reason } => {
                self.deposit_repository
                    .save(uow, request_context, &mut deposit)
                    .await?;
                return Ok(CommandHandled::same(
                    DepositTokenTransferPrepareOutput::Rejected { deposit_id, reason },
                ));
            }
        }

        let request = TokenDepositPrepareRequest::new(
            deposit_id,
            currency_id,
            mint_account.clone(),
            command.token_account_owner_address.clone(),
            command.amount,
        );
        let preparation = self.token_deposit_preparer.prepare(request).await?;
        self.deposit_repository
            .save(uow, request_context, &mut deposit)
            .await?;

        Ok(CommandHandled::same(
            DepositTokenTransferPrepareOutput::Prepared {
                deposit_id,
                preparation,
            },
        ))
    }
}
