use appletheia::application::authorization::{
    AggregateRef, AuthorizationPlan, PrincipalRequirement, Relation, RelationshipRequirement,
};
use appletheia::application::command::{CommandHandled, CommandHandler};
use appletheia::application::projection::{ProjectorDependencies, ProjectorSpec};
use appletheia::application::repository::Repository;
use appletheia::application::request_context::RequestContext;
use banking_iam_application::RoleAssigneeRelationshipProjectorSpec;
use banking_ledger_domain::currency_definition::CurrencyDefinition;

use super::{
    CurrencyDefinitionActivateCommand, CurrencyDefinitionActivateCommandHandlerError,
    CurrencyDefinitionActivateOutput,
};
use crate::authorization::CurrencyDefinitionActivatorRelation;
use crate::projection::CurrencyDefinitionStatusManagerRelationshipProjectorSpec;

/// Handles `CurrencyDefinitionActivateCommand`.
pub struct CurrencyDefinitionActivateCommandHandler<CDR>
where
    CDR: Repository<CurrencyDefinition>,
{
    currency_definition_repository: CDR,
}

impl<CDR> CurrencyDefinitionActivateCommandHandler<CDR>
where
    CDR: Repository<CurrencyDefinition>,
{
    pub fn new(currency_definition_repository: CDR) -> Self {
        Self {
            currency_definition_repository,
        }
    }
}

impl<CDR> CommandHandler for CurrencyDefinitionActivateCommandHandler<CDR>
where
    CDR: Repository<CurrencyDefinition>,
{
    type Command = CurrencyDefinitionActivateCommand;
    type Output = CurrencyDefinitionActivateOutput;
    type ReplayOutput = CurrencyDefinitionActivateOutput;
    type Error = CurrencyDefinitionActivateCommandHandlerError;
    type Uow = CDR::Uow;

    fn authorization_plan(
        &self,
        command: &Self::Command,
    ) -> Result<AuthorizationPlan, Self::Error> {
        Ok(AuthorizationPlan::OnlyPrincipals(vec![
            PrincipalRequirement::AuthenticatedWithRelationship {
                requirement: RelationshipRequirement::Check {
                    aggregate: AggregateRef::from_id::<CurrencyDefinition>(
                        command.currency_definition_id,
                    ),
                    relation: CurrencyDefinitionActivatorRelation::NAME,
                },
                projector_dependencies: ProjectorDependencies::Some(&[
                    RoleAssigneeRelationshipProjectorSpec::DESCRIPTOR,
                    CurrencyDefinitionStatusManagerRelationshipProjectorSpec::DESCRIPTOR,
                ]),
            },
        ]))
    }

    async fn handle(
        &self,
        uow: &mut Self::Uow,
        request_context: &RequestContext,
        command: Self::Command,
    ) -> Result<CommandHandled<Self::Output, Self::ReplayOutput>, Self::Error> {
        let Some(mut currency_definition) = self
            .currency_definition_repository
            .find(uow, command.currency_definition_id)
            .await?
        else {
            return Err(CurrencyDefinitionActivateCommandHandlerError::CurrencyDefinitionNotFound);
        };

        currency_definition.activate()?;

        self.currency_definition_repository
            .save(uow, request_context, &mut currency_definition)
            .await?;

        Ok(CommandHandled::same(CurrencyDefinitionActivateOutput))
    }
}
