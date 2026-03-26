use appletheia::application::authorization::{
    AggregateRef, AuthorizationPlan, PrincipalRequirement, Relation, RelationshipRequirement,
};
use appletheia::application::command::{CommandHandled, CommandHandler};
use appletheia::application::projection::{ProjectorDependencies, ProjectorSpec};
use appletheia::application::repository::Repository;
use appletheia::application::request_context::RequestContext;
use appletheia::domain::Aggregate;
use banking_iam_application::RoleAssigneeRelationshipProjectorSpec;
use banking_iam_application::authorization::RoleAssigneeRelation;
use banking_iam_domain::{Role, RoleId, RoleName};
use banking_ledger_domain::currency_definition::CurrencyDefinition;

use super::{
    CurrencyDefinitionDefineCommand, CurrencyDefinitionDefineCommandHandlerError,
    CurrencyDefinitionDefineOutput,
};

/// Handles `CurrencyDefinitionDefineCommand`.
pub struct CurrencyDefinitionDefineCommandHandler<CDR>
where
    CDR: Repository<CurrencyDefinition>,
{
    currency_definition_repository: CDR,
}

impl<CDR> CurrencyDefinitionDefineCommandHandler<CDR>
where
    CDR: Repository<CurrencyDefinition>,
{
    pub fn new(currency_definition_repository: CDR) -> Self {
        Self {
            currency_definition_repository,
        }
    }

    fn admin_role_requirement()
    -> Result<PrincipalRequirement, CurrencyDefinitionDefineCommandHandlerError> {
        let role_name = RoleName::try_from("admin")?;
        let role_id = RoleId::from_name(&role_name);
        let aggregate = AggregateRef::from_id::<Role>(role_id);
        let relation = RoleAssigneeRelation::NAME;

        Ok(PrincipalRequirement::AuthenticatedWithRelationship {
            requirement: RelationshipRequirement::Check {
                aggregate,
                relation,
            },
            projector_dependencies: ProjectorDependencies::Some(&[
                RoleAssigneeRelationshipProjectorSpec::NAME,
            ]),
        })
    }
}

impl<CDR> CommandHandler for CurrencyDefinitionDefineCommandHandler<CDR>
where
    CDR: Repository<CurrencyDefinition>,
{
    type Command = CurrencyDefinitionDefineCommand;
    type Output = CurrencyDefinitionDefineOutput;
    type ReplayOutput = CurrencyDefinitionDefineOutput;
    type Error = CurrencyDefinitionDefineCommandHandlerError;
    type Uow = CDR::Uow;

    fn authorization_plan(
        &self,
        _command: &Self::Command,
    ) -> Result<AuthorizationPlan, Self::Error> {
        Ok(AuthorizationPlan::OnlyPrincipals(vec![
            Self::admin_role_requirement()?,
        ]))
    }

    async fn handle(
        &self,
        uow: &mut Self::Uow,
        request_context: &RequestContext,
        command: Self::Command,
    ) -> Result<CommandHandled<Self::Output, Self::ReplayOutput>, Self::Error> {
        let CurrencyDefinitionDefineCommand {
            symbol,
            name,
            decimals,
        } = command;
        let mut currency_definition = CurrencyDefinition::default();
        currency_definition.define(symbol, name, decimals)?;

        self.currency_definition_repository
            .save(uow, request_context, &mut currency_definition)
            .await?;

        let currency_definition_id = currency_definition
            .aggregate_id()
            .ok_or(CurrencyDefinitionDefineCommandHandlerError::MissingCurrencyDefinitionId)?;
        let output = CurrencyDefinitionDefineOutput::new(currency_definition_id);

        Ok(CommandHandled::same(output))
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
        ActorRef, CorrelationId, MessageId, Principal, RequestContext,
    };
    use appletheia::application::unit_of_work::{UnitOfWork, UnitOfWorkError};
    use appletheia::domain::Aggregate;
    use banking_iam_application::RoleAssigneeRelationshipProjectorSpec;
    use banking_iam_application::authorization::RoleAssigneeRelation;
    use banking_iam_domain::{Role, RoleId, RoleName};
    use banking_ledger_domain::core::{CurrencyDecimals, CurrencySymbol};
    use banking_ledger_domain::currency_definition::{
        CurrencyDefinition, CurrencyDefinitionId, CurrencyName,
    };
    use uuid::Uuid;

    use super::{
        CurrencyDefinitionDefineCommand, CurrencyDefinitionDefineCommandHandler,
        CurrencyDefinitionDefineOutput,
    };

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
            ActorRef::Subject {
                subject: subject.clone(),
            },
            Principal::Authenticated { subject },
        )
    }

    #[test]
    fn authorization_plan_requires_admin_role_assignee_relationship() {
        let repository = TestCurrencyDefinitionRepository::default();
        let handler = CurrencyDefinitionDefineCommandHandler::new(repository);
        let admin_role_name = RoleName::try_from("admin").expect("role name should be valid");
        let admin_role_id = RoleId::from_name(&admin_role_name);

        let plan = handler
            .authorization_plan(&CurrencyDefinitionDefineCommand {
                symbol: CurrencySymbol::try_from("usdc").expect("symbol should be valid"),
                name: CurrencyName::try_from("USD Coin").expect("name should be valid"),
                decimals: CurrencyDecimals::new(6),
            })
            .expect("authorization plan should build");

        assert_eq!(
            plan,
            AuthorizationPlan::OnlyPrincipals(vec![
                PrincipalRequirement::AuthenticatedWithRelationship {
                    requirement: RelationshipRequirement::Check {
                        aggregate: AggregateRef::from_id::<Role>(admin_role_id),
                        relation: RoleAssigneeRelation::NAME,
                    },
                    projector_dependencies: ProjectorDependencies::Some(&[
                        RoleAssigneeRelationshipProjectorSpec::NAME,
                    ]),
                },
            ])
        );
    }

    #[tokio::test]
    async fn handle_defines_currency_definition_and_returns_id() {
        let repository = TestCurrencyDefinitionRepository::default();
        let handler = CurrencyDefinitionDefineCommandHandler::new(repository.clone());
        let mut uow = TestUow;

        let handled = handler
            .handle(
                &mut uow,
                &request_context(),
                CurrencyDefinitionDefineCommand {
                    symbol: CurrencySymbol::try_from("usdc").expect("symbol should be valid"),
                    name: CurrencyName::try_from("USD Coin").expect("name should be valid"),
                    decimals: CurrencyDecimals::new(6),
                },
            )
            .await
            .expect("command should succeed");

        let output = handled.into_output();
        let saved = repository
            .currency_definition
            .lock()
            .expect("lock")
            .clone()
            .expect("currency definition should be saved");
        let saved_id = saved
            .aggregate_id()
            .expect("currency definition id should exist");

        assert_eq!(output, CurrencyDefinitionDefineOutput::new(saved_id));
        assert_eq!(saved.symbol().expect("symbol should exist").value(), "USDC");
        assert_eq!(saved.name().expect("name should exist").value(), "USD Coin");
        assert_eq!(saved.decimals().expect("decimals should exist").value(), 6);
    }
}
