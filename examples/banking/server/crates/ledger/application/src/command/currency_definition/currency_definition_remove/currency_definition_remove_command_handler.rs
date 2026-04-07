use appletheia::application::authorization::{
    AuthorizationPlan, PrincipalRequirement, Relation, RelationshipRequirement,
};
use appletheia::application::command::{CommandHandled, CommandHandler};
use appletheia::application::projection::{ProjectorDependencies, ProjectorSpec};
use appletheia::application::repository::Repository;
use appletheia::application::request_context::RequestContext;
use banking_iam_application::OrganizationOwnerRelationshipProjectorSpec;
use banking_ledger_domain::currency_definition::CurrencyDefinition;

use super::{
    CurrencyDefinitionRemoveCommand, CurrencyDefinitionRemoveCommandHandlerError,
    CurrencyDefinitionRemoveOutput,
};
use crate::authorization::CurrencyDefinitionRemoverRelation;
use crate::projection::CurrencyDefinitionOwnerRelationshipProjectorSpec;

/// Handles `CurrencyDefinitionRemoveCommand`.
pub struct CurrencyDefinitionRemoveCommandHandler<CDR>
where
    CDR: Repository<CurrencyDefinition>,
{
    currency_definition_repository: CDR,
}

impl<CDR> CurrencyDefinitionRemoveCommandHandler<CDR>
where
    CDR: Repository<CurrencyDefinition>,
{
    pub fn new(currency_definition_repository: CDR) -> Self {
        Self {
            currency_definition_repository,
        }
    }
}

impl<CDR> CommandHandler for CurrencyDefinitionRemoveCommandHandler<CDR>
where
    CDR: Repository<CurrencyDefinition>,
{
    type Command = CurrencyDefinitionRemoveCommand;
    type Output = CurrencyDefinitionRemoveOutput;
    type ReplayOutput = CurrencyDefinitionRemoveOutput;
    type Error = CurrencyDefinitionRemoveCommandHandlerError;
    type Uow = CDR::Uow;

    fn authorization_plan(
        &self,
        command: &Self::Command,
    ) -> Result<AuthorizationPlan, Self::Error> {
        Ok(AuthorizationPlan::OnlyPrincipals(vec![
            PrincipalRequirement::AuthenticatedWithRelationship {
                requirement: RelationshipRequirement::check::<CurrencyDefinition>(
                    command.currency_definition_id,
                    CurrencyDefinitionRemoverRelation::REF,
                ),
                projector_dependencies: ProjectorDependencies::Some(&[
                    CurrencyDefinitionOwnerRelationshipProjectorSpec::DESCRIPTOR,
                    OrganizationOwnerRelationshipProjectorSpec::DESCRIPTOR,
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
        let Some(mut currency_definition) = self
            .currency_definition_repository
            .find(uow, command.currency_definition_id)
            .await?
        else {
            return Err(CurrencyDefinitionRemoveCommandHandlerError::CurrencyDefinitionNotFound);
        };

        currency_definition.remove()?;

        self.currency_definition_repository
            .save(uow, request_context, &mut currency_definition)
            .await?;

        Ok(CommandHandled::same(CurrencyDefinitionRemoveOutput))
    }
}
