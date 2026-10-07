use appletheia::application::authorization::{
    AuthorizationPlan, PrincipalRequirement, Relation, RelationshipRequirement,
};
use appletheia::application::command::CommandHandler;
use appletheia::application::repository::Repository;
use appletheia::application::request_context::RequestContext;
use appletheia::domain::{Aggregate, UniqueValue};
use banking_ledger_domain::currency::Currency;
use banking_ledger_domain::token_binding::TokenBindingError;
use banking_ledger_domain::token_binding::{TokenBinding, TokenBindingState};

use super::{
    TokenBindingDefineCommand, TokenBindingDefineCommandHandlerError, TokenBindingDefineOutput,
};
use crate::authorization::CurrencyTokenBindingDefinerRelation;
use crate::settlement::{TokenBindingSettlementValidationRequest, TokenBindingSettlementValidator};

pub struct TokenBindingDefineCommandHandler<R, V>
where
    R: Repository,
    V: TokenBindingSettlementValidator,
{
    repository: R,
    settlement_validator: V,
}

impl<R, V> TokenBindingDefineCommandHandler<R, V>
where
    R: Repository,
    V: TokenBindingSettlementValidator,
{
    pub fn new(repository: R, settlement_validator: V) -> Self {
        Self {
            repository,
            settlement_validator,
        }
    }
}

impl<R, V> CommandHandler for TokenBindingDefineCommandHandler<R, V>
where
    R: Repository,
    V: TokenBindingSettlementValidator,
{
    type Command = TokenBindingDefineCommand;
    type Output = TokenBindingDefineOutput;
    type Error = TokenBindingDefineCommandHandlerError;
    type Uow = R::Uow;

    fn authorization_plan(
        &self,
        command: &Self::Command,
    ) -> Result<AuthorizationPlan, Self::Error> {
        Ok(AuthorizationPlan::OnlyPrincipals(vec![
            PrincipalRequirement::AuthenticatedWithRelationship(RelationshipRequirement::check::<
                Currency,
            >(
                command.currency_id,
                CurrencyTokenBindingDefinerRelation::REF,
            )),
        ]))
    }

    async fn handle(
        &self,
        uow: &mut Self::Uow,
        request_context: &RequestContext,
        command: &Self::Command,
    ) -> Result<Self::Output, Self::Error> {
        let currency = self
            .repository
            .read::<Currency>(uow, command.currency_id)
            .await?;
        let mut token_binding = TokenBinding::new();
        let token_binding_id = token_binding.aggregate_id();
        let token_address = command.token_address.to_string();
        let unique_value = UniqueValue::from_strings([
            command.chain_network.chain_name(),
            token_address.as_str(),
        ])?;
        if self
            .repository
            .find_by_unique_value::<TokenBinding>(uow, TokenBindingState::TOKEN_KEY, &unique_value)
            .await?
            .is_some()
        {
            return Err(TokenBindingError::TokenAlreadyBound.into());
        }
        self.settlement_validator
            .validate(TokenBindingSettlementValidationRequest {
                currency_decimals: currency.decimals()?,
                chain_network: command.chain_network,
                token_address: command.token_address,
            })
            .await?;
        token_binding.define(
            command.currency_id,
            command.chain_network,
            command.token_address,
            command.deposit_enabled,
            command.withdrawal_enabled,
        )?;
        self.repository
            .save(uow, request_context, &mut token_binding)
            .await?;
        Ok(TokenBindingDefineOutput { token_binding_id })
    }
}
