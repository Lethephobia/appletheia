use appletheia::application::authorization::{AuthorizationPlan, PrincipalRequirement};
use appletheia::application::command::CommandHandler;
use appletheia::application::repository::Repository;
use appletheia::application::request_context::RequestContext;
use appletheia::domain::{Aggregate, UniqueValue};
use banking_ledger_domain::currency_registrar::{
    CurrencyRegistrar, CurrencyRegistrarCreateRejectionReason, CurrencyRegistrarCreateResult,
    CurrencyRegistrarCreation, CurrencyRegistrarState,
};

use super::{
    CurrencyRegistrarCreateCommand, CurrencyRegistrarCreateCommandHandlerError,
    CurrencyRegistrarCreateOutput,
};

pub struct CurrencyRegistrarCreateCommandHandler<R>
where
    R: Repository<CurrencyRegistrar>,
{
    repository: R,
}

impl<R> CurrencyRegistrarCreateCommandHandler<R>
where
    R: Repository<CurrencyRegistrar>,
{
    pub fn new(repository: R) -> Self {
        Self { repository }
    }
}

impl<R> CommandHandler for CurrencyRegistrarCreateCommandHandler<R>
where
    R: Repository<CurrencyRegistrar>,
{
    type Command = CurrencyRegistrarCreateCommand;
    type Output = CurrencyRegistrarCreateOutput;
    type Error = CurrencyRegistrarCreateCommandHandlerError;
    type Uow = R::Uow;

    fn authorization_plan(
        &self,
        _command: &Self::Command,
    ) -> Result<AuthorizationPlan, Self::Error> {
        Ok(AuthorizationPlan::OnlyPrincipals(vec![
            PrincipalRequirement::System,
        ]))
    }

    async fn handle(
        &self,
        uow: &mut Self::Uow,
        request_context: &RequestContext,
        command: &Self::Command,
    ) -> Result<Self::Output, Self::Error> {
        let mut currency_registrar = CurrencyRegistrar::new();
        let currency_registrar_id = currency_registrar.aggregate_id();
        let creation = CurrencyRegistrarCreation {
            handle: command.handle.clone(),
            display_name: command.display_name.clone(),
            description: command.description.clone(),
        };
        let unique_value = UniqueValue::from_strings([command.handle.as_ref()])?;
        if self
            .repository
            .find_by_unique_value(uow, CurrencyRegistrarState::HANDLE_KEY, &unique_value)
            .await?
            .is_some()
        {
            let reason = CurrencyRegistrarCreateRejectionReason::HandleAlreadyTaken;
            currency_registrar.reject_create(creation, reason)?;
            self.repository
                .save(uow, request_context, &mut currency_registrar)
                .await?;
            return Ok(CurrencyRegistrarCreateOutput::Rejected {
                currency_registrar_id,
                reason,
            });
        }

        let result = currency_registrar.create(creation)?;
        self.repository
            .save(uow, request_context, &mut currency_registrar)
            .await?;

        Ok(match result {
            CurrencyRegistrarCreateResult::Created => CurrencyRegistrarCreateOutput::Created {
                currency_registrar_id,
            },
            CurrencyRegistrarCreateResult::Rejected { reason } => {
                CurrencyRegistrarCreateOutput::Rejected {
                    currency_registrar_id,
                    reason,
                }
            }
        })
    }
}
