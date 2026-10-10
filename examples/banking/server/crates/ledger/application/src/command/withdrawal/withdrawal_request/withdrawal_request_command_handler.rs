use appletheia::application::authorization::{
    AuthorizationPlan, PrincipalRequirement, Relation, RelationshipRequirement,
};
use appletheia::application::command::CommandHandler;
use appletheia::application::repository::{Repository, RepositoryError};
use appletheia::application::request_context::RequestContext;
use appletheia::domain::Aggregate;
use banking_ledger_domain::account::Account;
use banking_ledger_domain::token_binding::TokenBinding;
use banking_ledger_domain::withdrawal::Withdrawal;
use banking_ledger_domain::withdrawal::WithdrawalError;

use super::{
    WithdrawalRequestCommand, WithdrawalRequestCommandHandlerError, WithdrawalRequestOutput,
};
use crate::authorization::AccountWithdrawalRequesterRelation;

pub struct WithdrawalRequestCommandHandler<R>
where
    R: Repository,
{
    repository: R,
}

impl<R> WithdrawalRequestCommandHandler<R>
where
    R: Repository,
{
    pub fn new(repository: R) -> Self {
        Self { repository }
    }
}

impl<R> CommandHandler for WithdrawalRequestCommandHandler<R>
where
    R: Repository,
{
    type Command = WithdrawalRequestCommand;
    type Output = WithdrawalRequestOutput;
    type Error = WithdrawalRequestCommandHandlerError;
    type Uow = R::Uow;

    fn authorization_plan(
        &self,
        command: &Self::Command,
    ) -> Result<AuthorizationPlan, Self::Error> {
        Ok(AuthorizationPlan::OnlyPrincipals(vec![
            PrincipalRequirement::AuthenticatedWithRelationship(RelationshipRequirement::check::<
                Account,
            >(
                command.account_id,
                AccountWithdrawalRequesterRelation::REF,
            )),
        ]))
    }

    async fn handle(
        &self,
        uow: &mut Self::Uow,
        request_context: &RequestContext,
        command: &Self::Command,
    ) -> Result<Self::Output, Self::Error> {
        let mut withdrawal = Withdrawal::new();
        let withdrawal_id = withdrawal.aggregate_id();
        let account = self
            .repository
            .read_shared::<Account>(uow, command.account_id)
            .await?;
        match self
            .repository
            .read_shared::<TokenBinding>(uow, command.token_binding_id)
            .await
        {
            Ok(token_binding)
                if token_binding.is_active()?
                    && token_binding.is_withdrawal_enabled()?
                    && token_binding.currency_id()? == *account.currency_id()? => {}
            Ok(_) | Err(RepositoryError::NotFound { .. }) => {
                return Err(WithdrawalError::TokenBindingUnavailable.into());
            }
            Err(error) => return Err(error.into()),
        }
        withdrawal.request(
            command.account_id,
            command.token_binding_id,
            command.token_owner_address,
            command.amount,
        )?;
        if let Some(note) = &command.note {
            withdrawal.set_note(Some(note.clone()))?;
        }

        self.repository
            .save(uow, request_context, &mut withdrawal)
            .await?;

        Ok(WithdrawalRequestOutput { withdrawal_id })
    }
}
