use appletheia::application::authorization::{
    AuthorizationPlan, PrincipalRequirement, Relation, RelationshipRequirement,
};
use appletheia::application::command::{CommandHandled, CommandHandler};
use appletheia::application::repository::Repository;
use appletheia::application::request_context::RequestContext;
use appletheia::domain::Aggregate;
use appletheia::domain::UniqueValue;
use banking_iam_application::authorization::{
    OrganizationFinanceManagerRelation, UserOwnerRelation,
};
use banking_iam_domain::{Organization, User};
use banking_ledger_domain::currency::{
    Currency, CurrencyDefineResult, CurrencyError, CurrencyOwner, CurrencyState, CurrencySymbol,
};

use super::{CurrencyDefineCommand, CurrencyDefineCommandHandlerError, CurrencyDefineOutput};

/// Handles `CurrencyDefineCommand`.
pub struct CurrencyDefineCommandHandler<CDR>
where
    CDR: Repository<Currency>,
{
    currency_repository: CDR,
}

impl<CDR> CurrencyDefineCommandHandler<CDR>
where
    CDR: Repository<Currency>,
{
    pub fn new(currency_repository: CDR) -> Self {
        Self {
            currency_repository,
        }
    }

    fn symbol_unique_value(
        symbol: &CurrencySymbol,
    ) -> Result<UniqueValue, CurrencyDefineCommandHandlerError> {
        Ok(UniqueValue::from_strings([symbol.as_ref()])?)
    }
}

impl<CDR> CommandHandler for CurrencyDefineCommandHandler<CDR>
where
    CDR: Repository<Currency>,
{
    type Command = CurrencyDefineCommand;
    type Output = CurrencyDefineOutput;
    type ReplayOutput = CurrencyDefineOutput;
    type Error = CurrencyDefineCommandHandlerError;
    type Uow = CDR::Uow;

    fn authorization_plan(
        &self,
        command: &Self::Command,
    ) -> Result<AuthorizationPlan, Self::Error> {
        match command.owner {
            CurrencyOwner::User(user_id) => Ok(AuthorizationPlan::OnlyPrincipals(vec![
                PrincipalRequirement::AuthenticatedWithRelationship(
                    RelationshipRequirement::check::<User>(user_id, UserOwnerRelation::REF),
                ),
            ])),
            CurrencyOwner::Organization(organization_id) => {
                Ok(AuthorizationPlan::OnlyPrincipals(vec![
                    PrincipalRequirement::AuthenticatedWithRelationship(
                        RelationshipRequirement::check::<Organization>(
                            organization_id,
                            OrganizationFinanceManagerRelation::REF,
                        ),
                    ),
                ]))
            }
        }
    }

    async fn handle(
        &self,
        uow: &mut Self::Uow,
        request_context: &RequestContext,
        command: &Self::Command,
    ) -> Result<CommandHandled<Self::Output, Self::ReplayOutput>, Self::Error> {
        let CurrencyDefineCommand {
            owner,
            symbol,
            name,
            decimals,
            description,
            image,
        } = command.clone();

        let unique_value = Self::symbol_unique_value(&symbol)?;
        if self
            .currency_repository
            .find_by_unique_value(uow, CurrencyState::SYMBOL_KEY, &unique_value)
            .await?
            .is_some()
        {
            return Err(CurrencyError::SymbolAlreadyTaken.into());
        }

        let mut currency = Currency::new();
        let currency_id = currency.aggregate_id();
        let result = currency.define(owner, symbol, name, decimals, description, image)?;

        self.currency_repository
            .save(uow, request_context, &mut currency)
            .await?;

        let output = match result {
            CurrencyDefineResult::Defined => CurrencyDefineOutput::new(currency_id),
        };

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

    use appletheia::application::repository::{Repository, RepositoryError};
    use appletheia::application::request_context::{
        CorrelationId, MessageId, Principal, RequestContext,
    };
    use appletheia::application::unit_of_work::{UnitOfWork, UnitOfWorkError};
    use appletheia::domain::Aggregate;
    use banking_iam_application::authorization::{
        OrganizationFinanceManagerRelation, UserOwnerRelation,
    };

    use banking_iam_domain::{Organization, OrganizationId, User, UserId};
    use banking_ledger_domain::currency::{
        Currency, CurrencyDecimals, CurrencyId, CurrencyName, CurrencyOwner, CurrencySymbol,
    };
    use uuid::Uuid;

    use super::{CurrencyDefineCommand, CurrencyDefineCommandHandler, CurrencyDefineOutput};

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
    struct TestCurrencyRepository {
        currency: Arc<Mutex<Option<Currency>>>,
    }

    impl Repository<Currency> for TestCurrencyRepository {
        type Uow = TestUow;

        async fn read(
            &self,
            _uow: &mut Self::Uow,
            _id: CurrencyId,
        ) -> Result<Currency, RepositoryError<Currency>> {
            self.currency
                .lock()
                .expect("lock")
                .clone()
                .ok_or_else(|| RepositoryError::NotFound {
                    aggregate_type: Currency::TYPE,
                    aggregate_id: _id,
                })
        }

        async fn read_at_version(
            &self,
            _uow: &mut Self::Uow,
            _id: CurrencyId,
            _at: appletheia::domain::AggregateVersion,
        ) -> Result<Currency, RepositoryError<Currency>> {
            self.currency
                .lock()
                .expect("lock")
                .clone()
                .ok_or_else(|| RepositoryError::NotFound {
                    aggregate_type: Currency::TYPE,
                    aggregate_id: _id,
                })
        }

        async fn find_by_unique_value(
            &self,
            _uow: &mut Self::Uow,
            _unique_key: appletheia::domain::UniqueKey,
            _unique_value: &appletheia::domain::UniqueValue,
        ) -> Result<Option<Currency>, RepositoryError<Currency>> {
            Ok(None)
        }

        async fn save(
            &self,
            _uow: &mut Self::Uow,
            _request_context: &RequestContext,
            aggregate: &mut Currency,
        ) -> Result<(), RepositoryError<Currency>> {
            *self.currency.lock().expect("lock") = Some(aggregate.clone());
            Ok(())
        }
    }

    fn request_context() -> (RequestContext, UserId) {
        let user_id = UserId::new();
        let subject = AggregateRef::from_id::<User>(user_id);

        (
            RequestContext::new(
                CorrelationId::from(Uuid::now_v7()),
                MessageId::new(),
                Principal::Authenticated { subject },
            )
            .expect("request context should be valid"),
            user_id,
        )
    }

    fn user_owner(user_id: UserId) -> CurrencyOwner {
        CurrencyOwner::User(user_id)
    }

    fn organization_owner(organization_id: OrganizationId) -> CurrencyOwner {
        CurrencyOwner::Organization(organization_id)
    }

    #[test]
    fn authorization_plan_requires_user_owner_relationship_when_user_owner_is_specified() {
        let repository = TestCurrencyRepository::default();
        let handler = CurrencyDefineCommandHandler::new(repository);
        let user_id = UserId::new();

        let plan = handler
            .authorization_plan(&CurrencyDefineCommand {
                owner: user_owner(user_id),
                symbol: CurrencySymbol::try_from("usdc").expect("symbol should be valid"),
                name: CurrencyName::try_from("USD Coin").expect("name should be valid"),
                decimals: CurrencyDecimals::new(6),
                description: None,
                image: None,
            })
            .expect("authorization plan should build");

        assert_eq!(
            plan,
            AuthorizationPlan::OnlyPrincipals(vec![
                PrincipalRequirement::AuthenticatedWithRelationship(
                    RelationshipRequirement::check::<User>(user_id, UserOwnerRelation::REF)
                ),
            ])
        );
    }

    #[test]
    fn authorization_plan_requires_organization_definer_relationship_when_organization_is_specified()
     {
        let repository = TestCurrencyRepository::default();
        let handler = CurrencyDefineCommandHandler::new(repository);
        let organization_id = OrganizationId::new();

        let plan = handler
            .authorization_plan(&CurrencyDefineCommand {
                owner: organization_owner(organization_id),
                symbol: CurrencySymbol::try_from("usdc").expect("symbol should be valid"),
                name: CurrencyName::try_from("USD Coin").expect("name should be valid"),
                decimals: CurrencyDecimals::new(6),
                description: None,
                image: None,
            })
            .expect("authorization plan should build");

        assert_eq!(
            plan,
            AuthorizationPlan::OnlyPrincipals(vec![
                PrincipalRequirement::AuthenticatedWithRelationship(
                    RelationshipRequirement::check::<Organization>(
                        organization_id,
                        OrganizationFinanceManagerRelation::REF,
                    )
                ),
            ])
        );
    }

    #[tokio::test]
    async fn handle_defines_currency_and_returns_id() {
        let repository = TestCurrencyRepository::default();
        let handler = CurrencyDefineCommandHandler::new(repository.clone());
        let mut uow = TestUow;

        let (request_context, user_id) = request_context();
        let expected_owner = user_owner(user_id);
        let handled = handler
            .handle(
                &mut uow,
                &request_context,
                &CurrencyDefineCommand {
                    owner: expected_owner,
                    symbol: CurrencySymbol::try_from("usdc").expect("symbol should be valid"),
                    name: CurrencyName::try_from("USD Coin").expect("name should be valid"),
                    decimals: CurrencyDecimals::new(6),
                    description: None,
                    image: None,
                },
            )
            .await
            .expect("command should succeed");

        let output = handled.into_output();
        let saved = repository
            .currency
            .lock()
            .expect("lock")
            .clone()
            .expect("currency should be saved");
        let saved_id = saved.aggregate_id();

        assert_eq!(output, CurrencyDefineOutput::new(saved_id));
        assert_eq!(saved.symbol().expect("symbol should exist").value(), "USDC");
        assert_eq!(saved.name().expect("name should exist").value(), "USD Coin");
        assert_eq!(saved.decimals().expect("decimals should exist").value(), 6);
        assert_eq!(saved.owner().expect("owner should exist"), expected_owner);
    }

    #[tokio::test]
    async fn handle_defines_organization_owned_currency_when_organization_is_specified() {
        let repository = TestCurrencyRepository::default();
        let handler = CurrencyDefineCommandHandler::new(repository.clone());
        let mut uow = TestUow;
        let organization_id = OrganizationId::new();
        let expected_owner = organization_owner(organization_id);
        let (request_context, _) = request_context();

        let handled = handler
            .handle(
                &mut uow,
                &request_context,
                &CurrencyDefineCommand {
                    owner: expected_owner,
                    symbol: CurrencySymbol::try_from("usdc").expect("symbol should be valid"),
                    name: CurrencyName::try_from("USD Coin").expect("name should be valid"),
                    decimals: CurrencyDecimals::new(6),
                    description: None,
                    image: None,
                },
            )
            .await
            .expect("command should succeed");

        let output = handled.into_output();
        let saved = repository
            .currency
            .lock()
            .expect("lock")
            .clone()
            .expect("currency should be saved");
        let saved_id = saved.aggregate_id();

        assert_eq!(output, CurrencyDefineOutput::new(saved_id));
        assert_eq!(saved.owner().expect("owner should exist"), expected_owner);
    }
}
