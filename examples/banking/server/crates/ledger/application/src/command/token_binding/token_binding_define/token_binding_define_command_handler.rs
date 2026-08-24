use appletheia::application::authorization::{
    AuthorizationPlan, PrincipalRequirement, Relation, RelationshipRequirement,
};
use appletheia::application::command::CommandHandler;
use appletheia::application::repository::Repository;
use appletheia::application::request_context::RequestContext;
use appletheia::domain::{Aggregate, UniqueValue};
use banking_ledger_domain::currency::Currency;
use banking_ledger_domain::token_binding::{
    TokenBinding, TokenBindingDefineRejectionReason, TokenBindingDefinition, TokenBindingState,
};

use super::{
    TokenBindingDefineCommand, TokenBindingDefineCommandHandlerError, TokenBindingDefineOutput,
};
use crate::authorization::CurrencyTokenBindingDefinerRelation;
use crate::settlement::{TokenBindingSettlementValidationRequest, TokenBindingSettlementValidator};

pub struct TokenBindingDefineCommandHandler<CR, TBR, V>
where
    CR: Repository<Currency, Uow = TBR::Uow>,
    TBR: Repository<TokenBinding>,
    V: TokenBindingSettlementValidator,
{
    currency_repository: CR,
    token_binding_repository: TBR,
    settlement_validator: V,
}

impl<CR, TBR, V> TokenBindingDefineCommandHandler<CR, TBR, V>
where
    CR: Repository<Currency, Uow = TBR::Uow>,
    TBR: Repository<TokenBinding>,
    V: TokenBindingSettlementValidator,
{
    pub fn new(
        currency_repository: CR,
        token_binding_repository: TBR,
        settlement_validator: V,
    ) -> Self {
        Self {
            currency_repository,
            token_binding_repository,
            settlement_validator,
        }
    }
}

impl<CR, TBR, V> CommandHandler for TokenBindingDefineCommandHandler<CR, TBR, V>
where
    CR: Repository<Currency, Uow = TBR::Uow>,
    TBR: Repository<TokenBinding>,
    V: TokenBindingSettlementValidator,
{
    type Command = TokenBindingDefineCommand;
    type Output = TokenBindingDefineOutput;
    type Error = TokenBindingDefineCommandHandlerError;
    type Uow = TBR::Uow;

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
            .currency_repository
            .read(uow, command.currency_id)
            .await?;
        let mut token_binding = TokenBinding::new();
        let token_binding_id = token_binding.aggregate_id();
        let definition = TokenBindingDefinition {
            currency_id: command.currency_id,
            chain_network: command.chain_network,
            token_address: command.token_address,
        };
        let network_name = command.chain_network.network_name();
        let token_address = command.token_address.to_string();
        let unique_value = UniqueValue::from_strings([
            command.chain_network.chain_name(),
            network_name.as_str(),
            token_address.as_str(),
        ])?;
        if self
            .token_binding_repository
            .find_by_unique_value(uow, TokenBindingState::TOKEN_KEY, &unique_value)
            .await?
            .is_some()
        {
            let reason = TokenBindingDefineRejectionReason::DuplicateToken;
            token_binding.reject_define(definition, reason)?;
            self.token_binding_repository
                .save(uow, request_context, &mut token_binding)
                .await?;
            return Ok(TokenBindingDefineOutput::Rejected {
                token_binding_id,
                reason,
            });
        }
        self.settlement_validator
            .validate(TokenBindingSettlementValidationRequest {
                currency_decimals: currency.decimals()?,
                chain_network: command.chain_network,
                token_address: command.token_address,
            })
            .await?;
        token_binding.define(definition)?;
        self.token_binding_repository
            .save(uow, request_context, &mut token_binding)
            .await?;
        Ok(TokenBindingDefineOutput::Defined { token_binding_id })
    }
}
