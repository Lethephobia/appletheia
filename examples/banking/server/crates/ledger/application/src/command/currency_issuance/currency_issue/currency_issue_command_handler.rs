use appletheia::application::authorization::{
    AuthorizationPlan, PrincipalRequirement, Relation, RelationshipRequirement,
};
use appletheia::application::command::{CommandHandled, CommandHandler};
use appletheia::application::repository::Repository;
use appletheia::application::request_context::RequestContext;
use banking_ledger_domain::account::Account;
use banking_ledger_domain::currency::Currency;
use banking_ledger_domain::currency_issuance::{
    CurrencyIssuance, CurrencyIssuanceIssueRejectionReason, CurrencyIssuanceIssueResult,
    CurrencyIssuanceRequest,
};

use super::{CurrencyIssueCommand, CurrencyIssueCommandHandlerError, CurrencyIssueOutput};
use crate::authorization::CurrencyIssuerRelation;

/// Handles `CurrencyIssueCommand`.
pub struct CurrencyIssueCommandHandler<AR, CDR, CIR>
where
    AR: Repository<Account, Uow = CDR::Uow>,
    CDR: Repository<Currency, Uow = CIR::Uow>,
    CIR: Repository<CurrencyIssuance>,
{
    account_repository: AR,
    currency_repository: CDR,
    currency_issuance_repository: CIR,
}

impl<AR, CDR, CIR> CurrencyIssueCommandHandler<AR, CDR, CIR>
where
    AR: Repository<Account, Uow = CDR::Uow>,
    CDR: Repository<Currency, Uow = CIR::Uow>,
    CIR: Repository<CurrencyIssuance>,
{
    pub fn new(
        account_repository: AR,
        currency_repository: CDR,
        currency_issuance_repository: CIR,
    ) -> Self {
        Self {
            account_repository,
            currency_repository,
            currency_issuance_repository,
        }
    }
}

impl<AR, CDR, CIR> CommandHandler for CurrencyIssueCommandHandler<AR, CDR, CIR>
where
    AR: Repository<Account, Uow = CDR::Uow>,
    CDR: Repository<Currency, Uow = CIR::Uow>,
    CIR: Repository<CurrencyIssuance>,
{
    type Command = CurrencyIssueCommand;
    type Output = CurrencyIssueOutput;
    type ReplayOutput = CurrencyIssueOutput;
    type Error = CurrencyIssueCommandHandlerError;
    type Uow = CIR::Uow;

    fn authorization_plan(
        &self,
        command: &Self::Command,
    ) -> Result<AuthorizationPlan, Self::Error> {
        Ok(AuthorizationPlan::OnlyPrincipals(vec![
            PrincipalRequirement::AuthenticatedWithRelationship(RelationshipRequirement::check::<
                Currency,
            >(
                command.currency_id,
                CurrencyIssuerRelation::REF,
            )),
        ]))
    }

    async fn handle(
        &self,
        uow: &mut Self::Uow,
        request_context: &RequestContext,
        command: &Self::Command,
    ) -> Result<CommandHandled<Self::Output, Self::ReplayOutput>, Self::Error> {
        let Some(destination_account) = self
            .account_repository
            .find(uow, command.destination_account_id)
            .await?
        else {
            return Err(CurrencyIssueCommandHandlerError::DestinationAccountNotFound);
        };
        let Some(currency) = self
            .currency_repository
            .find(uow, command.currency_id)
            .await?
        else {
            return Err(CurrencyIssueCommandHandlerError::CurrencyNotFound);
        };

        let mut currency_issuance = CurrencyIssuance::default();
        let request = CurrencyIssuanceRequest {
            currency_id: command.currency_id,
            destination_account_id: command.destination_account_id,
            amount: command.amount,
        };

        if destination_account.currency_id()? != &command.currency_id {
            let reason = CurrencyIssuanceIssueRejectionReason::CurrencyMismatch;
            let currency_issuance_id = currency_issuance.reject_issue(request, reason)?;

            self.currency_issuance_repository
                .save(uow, request_context, &mut currency_issuance)
                .await?;

            return Ok(CommandHandled::same(CurrencyIssueOutput::Rejected {
                currency_issuance_id,
                reason,
            }));
        }

        if !currency.is_provisioned()? {
            let reason = CurrencyIssuanceIssueRejectionReason::CurrencyProvisioningPending;
            let currency_issuance_id = currency_issuance.reject_issue(request, reason)?;

            self.currency_issuance_repository
                .save(uow, request_context, &mut currency_issuance)
                .await?;

            return Ok(CommandHandled::same(CurrencyIssueOutput::Rejected {
                currency_issuance_id,
                reason,
            }));
        }

        if !currency.is_active()? {
            let reason = CurrencyIssuanceIssueRejectionReason::CurrencyInactive;
            let currency_issuance_id = currency_issuance.reject_issue(request, reason)?;

            self.currency_issuance_repository
                .save(uow, request_context, &mut currency_issuance)
                .await?;

            return Ok(CommandHandled::same(CurrencyIssueOutput::Rejected {
                currency_issuance_id,
                reason,
            }));
        }

        let output = match currency_issuance.issue(request)? {
            CurrencyIssuanceIssueResult::Issued {
                currency_issuance_id,
            } => CurrencyIssueOutput::Issued {
                currency_issuance_id,
            },
            CurrencyIssuanceIssueResult::Rejected {
                currency_issuance_id,
                reason,
            } => CurrencyIssueOutput::Rejected {
                currency_issuance_id,
                reason,
            },
        };

        self.currency_issuance_repository
            .save(uow, request_context, &mut currency_issuance)
            .await?;

        Ok(CommandHandled::same(output))
    }
}
