use appletheia::application::authorization::{
    AuthorizationPlan, PrincipalRequirement, Relation, RelationshipRequirement,
};
use appletheia::application::command::{CommandHandled, CommandHandler};
use appletheia::application::projection::{ProjectorDependencies, ProjectorSpec};
use appletheia::application::repository::Repository;
use appletheia::application::request_context::RequestContext;
use appletheia::domain::Aggregate;
use banking_iam_application::{
    OrganizationOwnerRelationshipProjectorSpec, OrganizationRoleRelationshipProjectorSpec,
};
use banking_ledger_domain::account::Account;
use banking_ledger_domain::currency::Currency;
use banking_ledger_domain::currency_issuance::CurrencyIssuance;

use super::{CurrencyIssueCommand, CurrencyIssueCommandHandlerError, CurrencyIssueOutput};
use crate::authorization::CurrencyIssuerRelation;
use crate::projection::CurrencyOwnerRelationshipProjectorSpec;

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
            PrincipalRequirement::AuthenticatedWithRelationship {
                requirement: RelationshipRequirement::check::<Currency>(
                    command.currency_id,
                    CurrencyIssuerRelation::REF,
                ),
                projector_dependencies: ProjectorDependencies::Some(&[
                    CurrencyOwnerRelationshipProjectorSpec::DESCRIPTOR,
                    OrganizationOwnerRelationshipProjectorSpec::DESCRIPTOR,
                    OrganizationRoleRelationshipProjectorSpec::DESCRIPTOR,
                ]),
            },
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

        if destination_account.currency_id()? != &command.currency_id {
            return Err(CurrencyIssueCommandHandlerError::CurrencyMismatch);
        }

        if !currency.is_active()? {
            return Err(CurrencyIssueCommandHandlerError::Currency(
                banking_ledger_domain::currency::CurrencyError::Inactive,
            ));
        }

        let mut currency_issuance = CurrencyIssuance::default();
        currency_issuance.issue(
            command.currency_id,
            command.destination_account_id,
            command.amount,
        )?;

        self.currency_issuance_repository
            .save(uow, request_context, &mut currency_issuance)
            .await?;

        let currency_issuance_id = currency_issuance
            .aggregate_id()
            .ok_or(CurrencyIssueCommandHandlerError::MissingCurrencyIssuanceId)?;

        Ok(CommandHandled::same(CurrencyIssueOutput::new(
            currency_issuance_id,
        )))
    }
}
