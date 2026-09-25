use appletheia::application::authorization::{
    AuthorizationPlan, PrincipalRequirement, Relation, RelationshipRequirement,
};
use appletheia::application::command::CommandHandler;
use appletheia::application::repository::Repository;
use appletheia::application::request_context::RequestContext;
use appletheia::domain::{Aggregate, UniqueValue};
use banking_ledger_domain::currency::{Currency, CurrencyDefinition, CurrencyState};
use banking_ledger_domain::currency_registrar::CurrencyRegistrar;

use super::{CurrencyDefineCommand, CurrencyDefineCommandHandlerError, CurrencyDefineOutput};
use crate::authorization::CurrencyRegistrarCurrencyDefinerRelation;

pub struct CurrencyDefineCommandHandler<R>
where
    R: Repository<Currency>,
{
    repository: R,
}

impl<R> CurrencyDefineCommandHandler<R>
where
    R: Repository<Currency>,
{
    pub fn new(repository: R) -> Self {
        Self { repository }
    }
}

impl<R> CommandHandler for CurrencyDefineCommandHandler<R>
where
    R: Repository<Currency>,
{
    type Command = CurrencyDefineCommand;
    type Output = CurrencyDefineOutput;
    type Error = CurrencyDefineCommandHandlerError;
    type Uow = R::Uow;

    fn authorization_plan(
        &self,
        command: &Self::Command,
    ) -> Result<AuthorizationPlan, Self::Error> {
        Ok(AuthorizationPlan::OnlyPrincipals(vec![
            PrincipalRequirement::AuthenticatedWithRelationship(RelationshipRequirement::check::<
                CurrencyRegistrar,
            >(
                command.currency_registrar_id,
                CurrencyRegistrarCurrencyDefinerRelation::REF,
            )),
        ]))
    }

    async fn handle(
        &self,
        uow: &mut Self::Uow,
        request_context: &RequestContext,
        command: &Self::Command,
    ) -> Result<Self::Output, Self::Error> {
        let definition = CurrencyDefinition {
            currency_registrar_id: command.currency_registrar_id,
            code: command.code.clone(),
            decimals: command.decimals,
            description: command.description.clone(),
        };
        let unique_value = UniqueValue::from_strings([command.code.as_ref()])?;
        if self
            .repository
            .find_by_unique_value(uow, CurrencyState::CODE_KEY, &unique_value)
            .await?
            .is_some()
        {
            return Err(CurrencyDefineCommandHandlerError::DuplicateCode);
        }

        let mut currency = Currency::new();
        let currency_id = currency.aggregate_id();
        currency.define(definition)?;
        self.repository
            .save(uow, request_context, &mut currency)
            .await?;
        Ok(CurrencyDefineOutput { currency_id })
    }
}
