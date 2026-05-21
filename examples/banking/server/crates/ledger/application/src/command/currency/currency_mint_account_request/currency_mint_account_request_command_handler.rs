use appletheia::application::authorization::{AuthorizationPlan, PrincipalRequirement};
use appletheia::application::command::{CommandHandled, CommandHandler};
use appletheia::application::repository::Repository;
use appletheia::application::request_context::RequestContext;
use banking_ledger_domain::currency::{Currency, CurrencyMintAccountCreationRequestResult};

use super::{
    CurrencyMintAccountRequestCommand, CurrencyMintAccountRequestCommandHandlerError,
    CurrencyMintAccountRequestOutput,
};

/// Handles `CurrencyMintAccountRequestCommand`.
pub struct CurrencyMintAccountRequestCommandHandler<CR>
where
    CR: Repository<Currency>,
{
    currency_repository: CR,
}

impl<CR> CurrencyMintAccountRequestCommandHandler<CR>
where
    CR: Repository<Currency>,
{
    pub fn new(currency_repository: CR) -> Self {
        Self {
            currency_repository,
        }
    }
}

impl<CR> CommandHandler for CurrencyMintAccountRequestCommandHandler<CR>
where
    CR: Repository<Currency>,
{
    type Command = CurrencyMintAccountRequestCommand;
    type Output = CurrencyMintAccountRequestOutput;
    type ReplayOutput = CurrencyMintAccountRequestOutput;
    type Error = CurrencyMintAccountRequestCommandHandlerError;
    type Uow = CR::Uow;

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
    ) -> Result<CommandHandled<Self::Output, Self::ReplayOutput>, Self::Error> {
        let Some(mut currency) = self
            .currency_repository
            .find(uow, command.currency_id)
            .await?
        else {
            return Err(CurrencyMintAccountRequestCommandHandlerError::CurrencyNotFound);
        };

        let result = currency.request_mint_account_creation()?;
        self.currency_repository
            .save(uow, request_context, &mut currency)
            .await?;

        let output = match result {
            CurrencyMintAccountCreationRequestResult::Requested => {
                CurrencyMintAccountRequestOutput::Requested
            }
            CurrencyMintAccountCreationRequestResult::Rejected { reason } => {
                CurrencyMintAccountRequestOutput::Rejected { reason }
            }
        };

        Ok(CommandHandled::same(output))
    }
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};

    use appletheia::application::command::CommandHandler;
    use appletheia::application::repository::{Repository, RepositoryError};
    use appletheia::application::request_context::{
        CorrelationId, MessageId, Principal, RequestContext,
    };
    use appletheia::application::unit_of_work::{UnitOfWork, UnitOfWorkError};
    use appletheia::domain::{Aggregate, AggregateVersion, UniqueKey, UniqueValue};
    use banking_iam_domain::UserId;
    use banking_ledger_domain::currency::{
        Currency, CurrencyDecimals, CurrencyEventPayload, CurrencyId, CurrencyMintAccount,
        CurrencyMintAccountAddress, CurrencyMintAccountCreationRequestRejectionReason,
        CurrencyMintTokenProgramId, CurrencyName, CurrencyOwner, CurrencySymbol,
    };
    use uuid::Uuid;

    use super::{
        CurrencyMintAccountRequestCommand, CurrencyMintAccountRequestCommandHandler,
        CurrencyMintAccountRequestOutput,
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

    #[derive(Clone)]
    struct TestCurrencyRepository {
        currency: Arc<Mutex<Option<Currency>>>,
        saved: Arc<Mutex<Option<Currency>>>,
    }

    impl TestCurrencyRepository {
        fn new(currency: Currency) -> Self {
            Self {
                currency: Arc::new(Mutex::new(Some(currency))),
                saved: Arc::new(Mutex::new(None)),
            }
        }
    }

    impl Repository<Currency> for TestCurrencyRepository {
        type Uow = TestUow;

        async fn find(
            &self,
            _uow: &mut Self::Uow,
            _id: CurrencyId,
        ) -> Result<Option<Currency>, RepositoryError<Currency>> {
            Ok(self.currency.lock().expect("lock").clone())
        }

        async fn find_at_version(
            &self,
            _uow: &mut Self::Uow,
            _id: CurrencyId,
            _at: Option<AggregateVersion>,
        ) -> Result<Option<Currency>, RepositoryError<Currency>> {
            Ok(self.currency.lock().expect("lock").clone())
        }

        async fn find_by_unique_value(
            &self,
            _uow: &mut Self::Uow,
            _unique_key: UniqueKey,
            _unique_value: &UniqueValue,
        ) -> Result<Option<Currency>, RepositoryError<Currency>> {
            Ok(None)
        }

        async fn save(
            &self,
            _uow: &mut Self::Uow,
            _request_context: &RequestContext,
            aggregate: &mut Currency,
        ) -> Result<(), RepositoryError<Currency>> {
            *self.saved.lock().expect("lock") = Some(aggregate.clone());
            *self.currency.lock().expect("lock") = Some(aggregate.clone());
            Ok(())
        }
    }

    fn request_context() -> RequestContext {
        RequestContext::new(
            CorrelationId::from(Uuid::now_v7()),
            MessageId::new(),
            Principal::System,
        )
        .expect("request context should be valid")
    }

    fn defined_currency() -> Currency {
        let mut currency = Currency::default();
        currency
            .define(
                CurrencyOwner::User(UserId::new()),
                CurrencySymbol::try_from("usdc").expect("symbol should be valid"),
                CurrencyName::try_from("USD Coin").expect("name should be valid"),
                CurrencyDecimals::new(6),
                None,
                None,
            )
            .expect("currency should be defined");
        currency.core_mut().clear_uncommitted_events();
        currency
    }

    fn mint_account() -> CurrencyMintAccount {
        CurrencyMintAccount::new(
            CurrencyMintAccountAddress::try_from("Mint111111111111111111111111111111111111")
                .expect("mint account address should be valid"),
            CurrencyMintTokenProgramId::try_from("TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb")
                .expect("token program ID should be valid"),
        )
    }

    #[tokio::test]
    async fn handle_persists_request_event() {
        let currency = defined_currency();
        let currency_id = currency.aggregate_id().expect("currency id should exist");
        let repository = TestCurrencyRepository::new(currency);
        let handler = CurrencyMintAccountRequestCommandHandler::new(repository.clone());
        let mut uow = TestUow;

        let handled = handler
            .handle(
                &mut uow,
                &request_context(),
                &CurrencyMintAccountRequestCommand { currency_id },
            )
            .await
            .expect("command should be handled");

        assert_eq!(
            handled.into_output(),
            CurrencyMintAccountRequestOutput::Requested
        );
        let saved = repository
            .saved
            .lock()
            .expect("lock")
            .clone()
            .expect("currency should be saved");
        assert_eq!(
            saved.uncommitted_events()[0].payload(),
            &CurrencyEventPayload::MintAccountCreationRequested
        );
    }

    #[tokio::test]
    async fn handle_is_idempotent_when_request_already_exists() {
        let mut currency = defined_currency();
        let currency_id = currency.aggregate_id().expect("currency id should exist");
        currency
            .request_mint_account_creation()
            .expect("request should succeed");
        currency.core_mut().clear_uncommitted_events();
        let repository = TestCurrencyRepository::new(currency);
        let handler = CurrencyMintAccountRequestCommandHandler::new(repository.clone());
        let mut uow = TestUow;

        let handled = handler
            .handle(
                &mut uow,
                &request_context(),
                &CurrencyMintAccountRequestCommand { currency_id },
            )
            .await
            .expect("command should be handled");

        assert_eq!(
            handled.into_output(),
            CurrencyMintAccountRequestOutput::Rejected {
                reason: CurrencyMintAccountCreationRequestRejectionReason::AlreadyRequested,
            }
        );
        let saved = repository
            .saved
            .lock()
            .expect("lock")
            .clone()
            .expect("currency should be saved");
        assert_eq!(
            saved.uncommitted_events()[0].payload(),
            &CurrencyEventPayload::MintAccountCreationRequestRejected {
                reason: CurrencyMintAccountCreationRequestRejectionReason::AlreadyRequested,
            }
        );
    }

    #[tokio::test]
    async fn handle_rejects_removed_currency() {
        let mut currency = defined_currency();
        let currency_id = currency.aggregate_id().expect("currency id should exist");
        currency.remove().expect("currency should be removed");
        currency.core_mut().clear_uncommitted_events();
        let repository = TestCurrencyRepository::new(currency);
        let handler = CurrencyMintAccountRequestCommandHandler::new(repository.clone());
        let mut uow = TestUow;

        let handled = handler
            .handle(
                &mut uow,
                &request_context(),
                &CurrencyMintAccountRequestCommand { currency_id },
            )
            .await
            .expect("command should be handled");

        assert_eq!(
            handled.into_output(),
            CurrencyMintAccountRequestOutput::Rejected {
                reason: CurrencyMintAccountCreationRequestRejectionReason::Removed,
            }
        );
    }

    #[tokio::test]
    async fn handle_rejects_already_recorded_currency() {
        let mut currency = defined_currency();
        let currency_id = currency.aggregate_id().expect("currency id should exist");
        currency
            .record_mint_account(mint_account())
            .expect("mint account should be recorded");
        currency.core_mut().clear_uncommitted_events();
        let repository = TestCurrencyRepository::new(currency);
        let handler = CurrencyMintAccountRequestCommandHandler::new(repository.clone());
        let mut uow = TestUow;

        let handled = handler
            .handle(
                &mut uow,
                &request_context(),
                &CurrencyMintAccountRequestCommand { currency_id },
            )
            .await
            .expect("command should be handled");

        assert_eq!(
            handled.into_output(),
            CurrencyMintAccountRequestOutput::Rejected {
                reason: CurrencyMintAccountCreationRequestRejectionReason::AlreadyRecorded,
            }
        );
        let saved = repository
            .saved
            .lock()
            .expect("lock")
            .clone()
            .expect("currency should be saved");
        assert_eq!(
            saved.uncommitted_events()[0].payload(),
            &CurrencyEventPayload::MintAccountCreationRequestRejected {
                reason: CurrencyMintAccountCreationRequestRejectionReason::AlreadyRecorded,
            }
        );
    }
}
