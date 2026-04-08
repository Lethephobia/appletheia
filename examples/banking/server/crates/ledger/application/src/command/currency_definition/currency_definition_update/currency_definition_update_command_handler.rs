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
    CurrencyDefinitionUpdateCommand, CurrencyDefinitionUpdateCommandHandlerError,
    CurrencyDefinitionUpdateOutput,
};
use crate::authorization::CurrencyDefinitionUpdaterRelation;
use crate::projection::CurrencyDefinitionOwnerRelationshipProjectorSpec;

/// Handles `CurrencyDefinitionUpdateCommand`.
pub struct CurrencyDefinitionUpdateCommandHandler<CDR>
where
    CDR: Repository<CurrencyDefinition>,
{
    currency_definition_repository: CDR,
}

impl<CDR> CurrencyDefinitionUpdateCommandHandler<CDR>
where
    CDR: Repository<CurrencyDefinition>,
{
    pub fn new(currency_definition_repository: CDR) -> Self {
        Self {
            currency_definition_repository,
        }
    }
}

impl<CDR> CommandHandler for CurrencyDefinitionUpdateCommandHandler<CDR>
where
    CDR: Repository<CurrencyDefinition>,
{
    type Command = CurrencyDefinitionUpdateCommand;
    type Output = CurrencyDefinitionUpdateOutput;
    type ReplayOutput = CurrencyDefinitionUpdateOutput;
    type Error = CurrencyDefinitionUpdateCommandHandlerError;
    type Uow = CDR::Uow;

    fn authorization_plan(
        &self,
        command: &Self::Command,
    ) -> Result<AuthorizationPlan, Self::Error> {
        Ok(AuthorizationPlan::OnlyPrincipals(vec![
            PrincipalRequirement::AuthenticatedWithRelationship {
                requirement: RelationshipRequirement::check::<CurrencyDefinition>(
                    command.currency_definition_id,
                    CurrencyDefinitionUpdaterRelation::REF,
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
            return Err(CurrencyDefinitionUpdateCommandHandlerError::CurrencyDefinitionNotFound);
        };

        if let Some(symbol) = command.symbol.clone() {
            currency_definition.change_symbol(symbol)?;
        }

        if let Some(name) = command.name.clone() {
            currency_definition.change_name(name)?;
        }

        self.currency_definition_repository
            .save(uow, request_context, &mut currency_definition)
            .await?;

        Ok(CommandHandled::same(CurrencyDefinitionUpdateOutput))
    }
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};

    use appletheia::application::authorization::{
        AggregateRef, AuthorizationPlan, PrincipalRequirement, Relation, RelationshipRequirement,
    };
    use appletheia::application::command::CommandHandler;
    use appletheia::application::projection::{ProjectorDependencies, ProjectorSpec};
    use appletheia::application::repository::{Repository, RepositoryError};
    use appletheia::application::request_context::{
        CorrelationId, MessageId, Principal, RequestContext,
    };
    use appletheia::application::unit_of_work::{UnitOfWork, UnitOfWorkError};
    use appletheia::domain::Aggregate;
    use banking_iam_application::OrganizationOwnerRelationshipProjectorSpec;
    use banking_iam_domain::UserId;
    use banking_ledger_domain::core::{CurrencyDecimals, CurrencySymbol};
    use banking_ledger_domain::currency_definition::{
        CurrencyDefinition, CurrencyDefinitionId, CurrencyDefinitionOwner, CurrencyName,
    };
    use uuid::Uuid;

    use super::{
        CurrencyDefinitionUpdateCommand, CurrencyDefinitionUpdateCommandHandler,
        CurrencyDefinitionUpdateOutput,
    };
    use crate::authorization::CurrencyDefinitionUpdaterRelation;
    use crate::projection::CurrencyDefinitionOwnerRelationshipProjectorSpec;

    #[derive(Default)]
    struct TestUow;

    impl UnitOfWork for TestUow {
        async fn commit(self) -> Result<(), UnitOfWorkError> {
            Ok(())
        }

        async fn rollback(self) -> Result<(), UnitOfWorkError> {
            Ok(())
        }
    }

    #[derive(Clone, Default)]
    struct TestCurrencyDefinitionRepository {
        currency_definition: Arc<Mutex<Option<CurrencyDefinition>>>,
    }

    impl TestCurrencyDefinitionRepository {
        fn new(currency_definition: CurrencyDefinition) -> Self {
            Self {
                currency_definition: Arc::new(Mutex::new(Some(currency_definition))),
            }
        }
    }

    impl Repository<CurrencyDefinition> for TestCurrencyDefinitionRepository {
        type Uow = TestUow;

        async fn find(
            &self,
            _uow: &mut Self::Uow,
            _id: CurrencyDefinitionId,
        ) -> Result<Option<CurrencyDefinition>, RepositoryError<CurrencyDefinition>> {
            Ok(self.currency_definition.lock().expect("lock").clone())
        }

        async fn find_at_version(
            &self,
            _uow: &mut Self::Uow,
            _id: CurrencyDefinitionId,
            _at: Option<appletheia::domain::AggregateVersion>,
        ) -> Result<Option<CurrencyDefinition>, RepositoryError<CurrencyDefinition>> {
            Ok(self.currency_definition.lock().expect("lock").clone())
        }

        async fn find_by_unique_value(
            &self,
            _uow: &mut Self::Uow,
            _unique_key: appletheia::domain::UniqueKey,
            _unique_value: &appletheia::domain::UniqueValue,
        ) -> Result<Option<CurrencyDefinition>, RepositoryError<CurrencyDefinition>> {
            Ok(None)
        }

        async fn save(
            &self,
            _uow: &mut Self::Uow,
            _request_context: &RequestContext,
            aggregate: &mut CurrencyDefinition,
        ) -> Result<(), RepositoryError<CurrencyDefinition>> {
            *self.currency_definition.lock().expect("lock") = Some(aggregate.clone());
            Ok(())
        }
    }

    fn request_context() -> RequestContext {
        let subject = AggregateRef::new(
            appletheia::application::event::AggregateTypeOwned::try_from("user")
                .expect("aggregate type should be valid"),
            appletheia::application::event::AggregateIdValue::from(Uuid::now_v7()),
        );

        RequestContext::new(
            CorrelationId::from(Uuid::now_v7()),
            MessageId::new(),
            Principal::Authenticated { subject },
        )
        .expect("request context should be valid")
    }

    fn defined_currency_definition() -> CurrencyDefinition {
        let mut currency_definition = CurrencyDefinition::default();
        currency_definition
            .define(
                CurrencyDefinitionOwner::User(UserId::new()),
                CurrencySymbol::try_from("usdc").expect("symbol should be valid"),
                CurrencyName::try_from("USD Coin").expect("name should be valid"),
                CurrencyDecimals::new(6),
            )
            .expect("definition should succeed");
        currency_definition
    }

    #[test]
    fn authorization_plan_requires_updater_relationship() {
        let currency_definition = defined_currency_definition();
        let currency_definition_id = currency_definition
            .aggregate_id()
            .expect("currency definition id should exist");
        let repository = TestCurrencyDefinitionRepository::new(currency_definition);
        let handler = CurrencyDefinitionUpdateCommandHandler::new(repository);

        let plan = handler
            .authorization_plan(&CurrencyDefinitionUpdateCommand {
                currency_definition_id,
                symbol: None,
                name: None,
            })
            .expect("authorization plan should build");

        assert_eq!(
            plan,
            AuthorizationPlan::OnlyPrincipals(vec![
                PrincipalRequirement::AuthenticatedWithRelationship {
                    requirement: RelationshipRequirement::check::<CurrencyDefinition>(
                        currency_definition_id,
                        CurrencyDefinitionUpdaterRelation::REF,
                    ),
                    projector_dependencies: ProjectorDependencies::Some(&[
                        CurrencyDefinitionOwnerRelationshipProjectorSpec::DESCRIPTOR,
                        OrganizationOwnerRelationshipProjectorSpec::DESCRIPTOR,
                    ]),
                },
            ])
        );
    }

    #[tokio::test]
    async fn handle_updates_only_specified_fields() {
        let currency_definition = defined_currency_definition();
        let currency_definition_id = currency_definition
            .aggregate_id()
            .expect("currency definition id should exist");
        let repository = TestCurrencyDefinitionRepository::new(currency_definition);
        let handler = CurrencyDefinitionUpdateCommandHandler::new(repository.clone());
        let mut uow = TestUow;

        let handled = handler
            .handle(
                &mut uow,
                &request_context(),
                &CurrencyDefinitionUpdateCommand {
                    currency_definition_id,
                    symbol: None,
                    name: Some(
                        CurrencyName::try_from("USD Coin Updated").expect("name should be valid"),
                    ),
                },
            )
            .await
            .expect("command should succeed");

        assert_eq!(handled.into_output(), CurrencyDefinitionUpdateOutput);
    }
}
