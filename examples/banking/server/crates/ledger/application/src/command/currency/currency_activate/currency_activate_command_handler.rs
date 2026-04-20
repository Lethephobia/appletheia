use appletheia::application::authorization::{
    AuthorizationPlan, PrincipalRequirement, Relation, RelationshipRequirement,
};
use appletheia::application::command::{CommandHandled, CommandHandler};
use appletheia::application::projection::{ProjectorDependencies, ProjectorSpec};
use appletheia::application::repository::Repository;
use appletheia::application::request_context::RequestContext;
use banking_iam_application::{
    OrganizationOwnerRelationshipProjectorSpec, OrganizationRoleRelationshipProjectorSpec,
};
use banking_ledger_domain::currency::Currency;

use super::{CurrencyActivateCommand, CurrencyActivateCommandHandlerError, CurrencyActivateOutput};
use crate::authorization::CurrencyActivatorRelation;
use crate::projection::CurrencyOwnerRelationshipProjectorSpec;

/// Handles `CurrencyActivateCommand`.
pub struct CurrencyActivateCommandHandler<CDR>
where
    CDR: Repository<Currency>,
{
    currency_repository: CDR,
}

impl<CDR> CurrencyActivateCommandHandler<CDR>
where
    CDR: Repository<Currency>,
{
    pub fn new(currency_repository: CDR) -> Self {
        Self {
            currency_repository,
        }
    }
}

impl<CDR> CommandHandler for CurrencyActivateCommandHandler<CDR>
where
    CDR: Repository<Currency>,
{
    type Command = CurrencyActivateCommand;
    type Output = CurrencyActivateOutput;
    type ReplayOutput = CurrencyActivateOutput;
    type Error = CurrencyActivateCommandHandlerError;
    type Uow = CDR::Uow;

    fn authorization_plan(
        &self,
        command: &Self::Command,
    ) -> Result<AuthorizationPlan, Self::Error> {
        Ok(AuthorizationPlan::OnlyPrincipals(vec![
            PrincipalRequirement::AuthenticatedWithRelationship {
                requirement: RelationshipRequirement::check::<Currency>(
                    command.currency_id,
                    CurrencyActivatorRelation::REF,
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
        let Some(mut currency) = self
            .currency_repository
            .find(uow, command.currency_id)
            .await?
        else {
            return Err(CurrencyActivateCommandHandlerError::CurrencyNotFound);
        };

        currency.activate()?;

        self.currency_repository
            .save(uow, request_context, &mut currency)
            .await?;

        Ok(CommandHandled::same(CurrencyActivateOutput))
    }
}
