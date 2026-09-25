use appletheia::application::authorization::{
    AuthorizationPlan, PrincipalRequirement, Relation, RelationshipRequirement,
};
use appletheia::application::command::CommandHandler;
use appletheia::application::repository::Repository;
use appletheia::application::request_context::RequestContext;
use appletheia::domain::{Aggregate, UniqueValue};
use banking_ledger_domain::currency_registrar::{
    CurrencyRegistrar, CurrencyRegistrarHandle, CurrencyRegistrarHandleChangeRejectionReason,
    CurrencyRegistrarHandleChangeResult, CurrencyRegistrarState,
};

use crate::authorization::CurrencyRegistrarMemberRelation;

use super::{
    CurrencyRegistrarHandleChangeCommand, CurrencyRegistrarHandleChangeCommandHandlerError,
    CurrencyRegistrarHandleChangeOutput,
};

pub struct CurrencyRegistrarHandleChangeCommandHandler<R>
where
    R: Repository<CurrencyRegistrar>,
{
    repository: R,
}

impl<R> CurrencyRegistrarHandleChangeCommandHandler<R>
where
    R: Repository<CurrencyRegistrar>,
{
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    fn handle_unique_value(
        handle: &CurrencyRegistrarHandle,
    ) -> Result<UniqueValue, CurrencyRegistrarHandleChangeCommandHandlerError> {
        Ok(UniqueValue::from_strings([handle.as_ref()])?)
    }
}

impl<R> CommandHandler for CurrencyRegistrarHandleChangeCommandHandler<R>
where
    R: Repository<CurrencyRegistrar>,
{
    type Command = CurrencyRegistrarHandleChangeCommand;
    type Output = CurrencyRegistrarHandleChangeOutput;
    type Error = CurrencyRegistrarHandleChangeCommandHandlerError;
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
                CurrencyRegistrarMemberRelation::REF,
            )),
        ]))
    }

    async fn handle(
        &self,
        uow: &mut Self::Uow,
        request_context: &RequestContext,
        command: &Self::Command,
    ) -> Result<Self::Output, Self::Error> {
        let mut registrar = self
            .repository
            .read(uow, command.currency_registrar_id)
            .await?;

        let unique_value = Self::handle_unique_value(&command.handle)?;
        let result = if self
            .repository
            .find_by_unique_value(uow, CurrencyRegistrarState::HANDLE_KEY, &unique_value)
            .await?
            .is_some_and(|existing| existing.aggregate_id() != command.currency_registrar_id)
        {
            registrar.reject_change_handle(
                command.handle.clone(),
                CurrencyRegistrarHandleChangeRejectionReason::AlreadyTaken,
            )?
        } else {
            registrar.change_handle(command.handle.clone())?
        };

        self.repository
            .save(uow, request_context, &mut registrar)
            .await?;

        Ok(match result {
            CurrencyRegistrarHandleChangeResult::Changed => {
                CurrencyRegistrarHandleChangeOutput::Changed
            }
            CurrencyRegistrarHandleChangeResult::Rejected { reason } => {
                CurrencyRegistrarHandleChangeOutput::Rejected { reason }
            }
        })
    }
}
