use appletheia::application::authorization::{
    AggregateRef, AuthorizationPlan, PrincipalRequirement, Relation, RelationRefOwned,
    RelationshipRequirement,
};
use appletheia::application::command::{CommandHandled, CommandHandler};
use appletheia::application::projection::{ProjectorDependencies, ProjectorSpec};
use appletheia::application::repository::Repository;
use appletheia::application::request_context::RequestContext;
use banking_ledger_domain::currency_definition::CurrencyDefinition;

use super::{
    CurrencyDefinitionDeactivateCommand, CurrencyDefinitionDeactivateCommandHandlerError,
    CurrencyDefinitionDeactivateOutput,
};
use crate::authorization::CurrencyDefinitionDeactivatorRelation;
use crate::projection::CurrencyDefinitionOwnerRelationshipProjectorSpec;

/// Handles `CurrencyDefinitionDeactivateCommand`.
pub struct CurrencyDefinitionDeactivateCommandHandler<CDR>
where
    CDR: Repository<CurrencyDefinition>,
{
    currency_definition_repository: CDR,
}

impl<CDR> CurrencyDefinitionDeactivateCommandHandler<CDR>
where
    CDR: Repository<CurrencyDefinition>,
{
    pub fn new(currency_definition_repository: CDR) -> Self {
        Self {
            currency_definition_repository,
        }
    }
}

impl<CDR> CommandHandler for CurrencyDefinitionDeactivateCommandHandler<CDR>
where
    CDR: Repository<CurrencyDefinition>,
{
    type Command = CurrencyDefinitionDeactivateCommand;
    type Output = CurrencyDefinitionDeactivateOutput;
    type ReplayOutput = CurrencyDefinitionDeactivateOutput;
    type Error = CurrencyDefinitionDeactivateCommandHandlerError;
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
                    relation: RelationRefOwned::from(CurrencyDefinitionDeactivatorRelation::REF),
                },
                projector_dependencies: ProjectorDependencies::Some(&[
                    CurrencyDefinitionOwnerRelationshipProjectorSpec::DESCRIPTOR,
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
            return Err(
                CurrencyDefinitionDeactivateCommandHandlerError::CurrencyDefinitionNotFound,
            );
        };

        currency_definition.deactivate()?;

        self.currency_definition_repository
            .save(uow, request_context, &mut currency_definition)
            .await?;

        Ok(CommandHandled::same(CurrencyDefinitionDeactivateOutput))
    }
}
