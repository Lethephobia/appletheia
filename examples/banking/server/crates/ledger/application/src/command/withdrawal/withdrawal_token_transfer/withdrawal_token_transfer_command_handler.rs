use crate::mint::{PoolTokenTransferExecutor, PoolTokenTransferRequest};
use appletheia::application::authorization::{AuthorizationPlan, PrincipalRequirement};
use appletheia::application::command::{CommandHandled, CommandHandler};
use appletheia::application::repository::Repository;
use appletheia::application::request_context::RequestContext;
use banking_ledger_domain::currency::Currency;
use banking_ledger_domain::withdrawal::{
    Withdrawal, WithdrawalTokenTransferRejectionReason, WithdrawalTokenTransferResult,
};

use super::{
    WithdrawalTokenTransferCommand, WithdrawalTokenTransferCommandHandlerError,
    WithdrawalTokenTransferOutput,
};

/// Handles `WithdrawalTokenTransferCommand`.
pub struct WithdrawalTokenTransferCommandHandler<WR, CR, PTTE>
where
    WR: Repository<Withdrawal>,
    CR: Repository<Currency, Uow = WR::Uow>,
    PTTE: PoolTokenTransferExecutor,
{
    withdrawal_repository: WR,
    currency_repository: CR,
    pool_token_transfer_executor: PTTE,
}

impl<WR, CR, PTTE> WithdrawalTokenTransferCommandHandler<WR, CR, PTTE>
where
    WR: Repository<Withdrawal>,
    CR: Repository<Currency, Uow = WR::Uow>,
    PTTE: PoolTokenTransferExecutor,
{
    pub fn new(
        withdrawal_repository: WR,
        currency_repository: CR,
        pool_token_transfer_executor: PTTE,
    ) -> Self {
        Self {
            withdrawal_repository,
            currency_repository,
            pool_token_transfer_executor,
        }
    }
}

impl<WR, CR, PTTE> CommandHandler for WithdrawalTokenTransferCommandHandler<WR, CR, PTTE>
where
    WR: Repository<Withdrawal>,
    CR: Repository<Currency, Uow = WR::Uow>,
    PTTE: PoolTokenTransferExecutor,
{
    type Command = WithdrawalTokenTransferCommand;
    type Output = WithdrawalTokenTransferOutput;
    type ReplayOutput = WithdrawalTokenTransferOutput;
    type Error = WithdrawalTokenTransferCommandHandlerError;
    type Uow = WR::Uow;

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
        let mut withdrawal = self
            .withdrawal_repository
            .read(uow, command.withdrawal_id)
            .await?;
        let currency_id = *withdrawal.currency_id()?;
        let currency = self.currency_repository.read(uow, currency_id).await?;
        let Some(mint_account) = currency.mint_account()? else {
            let reason = WithdrawalTokenTransferRejectionReason::CurrencyUnprovisioned;
            withdrawal.reject_token_transfer(reason)?;
            self.withdrawal_repository
                .save(uow, request_context, &mut withdrawal)
                .await?;
            return Ok(CommandHandled::same(
                WithdrawalTokenTransferOutput::Rejected { reason },
            ));
        };

        let request = PoolTokenTransferRequest::new(
            command.withdrawal_id,
            currency_id,
            mint_account.clone(),
            withdrawal.token_account_owner_address()?.clone(),
            *withdrawal.amount()?,
        );

        self.pool_token_transfer_executor.execute(request).await?;
        let result = withdrawal.record_token_transfer()?;
        self.withdrawal_repository
            .save(uow, request_context, &mut withdrawal)
            .await?;
        let output = match result {
            WithdrawalTokenTransferResult::TokenTransferred => {
                WithdrawalTokenTransferOutput::TokenTransferred
            }
            WithdrawalTokenTransferResult::Rejected { reason } => {
                WithdrawalTokenTransferOutput::Rejected { reason }
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
    use appletheia::domain::{Aggregate, AggregateVersion, EventPayload, UniqueKey, UniqueValue};
    use banking_iam_domain::UserId;
    use banking_ledger_domain::account::AccountId;
    use banking_ledger_domain::core::{CurrencyAmount, TokenAccountOwnerAddress};
    use banking_ledger_domain::currency::{
        Currency, CurrencyDecimals, CurrencyDefinition, CurrencyId, CurrencyName, CurrencyOwner,
        CurrencySymbol, MintAccount, MintAccountAddress, PoolTokenAccountAddress,
    };
    use banking_ledger_domain::withdrawal::{
        Withdrawal, WithdrawalEventPayload, WithdrawalId, WithdrawalRequest, WithdrawalStatus,
        WithdrawalTokenTransferRejectionReason,
    };
    use thiserror::Error;
    use uuid::Uuid;

    use super::{
        WithdrawalTokenTransferCommand, WithdrawalTokenTransferCommandHandler,
        WithdrawalTokenTransferCommandHandlerError, WithdrawalTokenTransferOutput,
    };
    use crate::mint::{
        PoolTokenTransferExecutor, PoolTokenTransferExecutorError, PoolTokenTransferRequest,
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
    struct TestWithdrawalRepository {
        item: Arc<Mutex<Option<Withdrawal>>>,
        saved: Arc<Mutex<Option<Withdrawal>>>,
    }

    impl TestWithdrawalRepository {
        fn new(withdrawal: Withdrawal) -> Self {
            Self {
                item: Arc::new(Mutex::new(Some(withdrawal))),
                saved: Arc::new(Mutex::new(None)),
            }
        }
    }

    impl Repository<Withdrawal> for TestWithdrawalRepository {
        type Uow = TestUow;

        async fn read(
            &self,
            _uow: &mut Self::Uow,
            id: WithdrawalId,
        ) -> Result<Withdrawal, RepositoryError<Withdrawal>> {
            self.item
                .lock()
                .expect("lock")
                .clone()
                .ok_or_else(|| RepositoryError::NotFound {
                    aggregate_type: Withdrawal::TYPE,
                    aggregate_id: id,
                })
        }

        async fn read_at_version(
            &self,
            _uow: &mut Self::Uow,
            id: WithdrawalId,
            _at: AggregateVersion,
        ) -> Result<Withdrawal, RepositoryError<Withdrawal>> {
            self.read(_uow, id).await
        }

        async fn find_by_unique_value(
            &self,
            _uow: &mut Self::Uow,
            _unique_key: UniqueKey,
            _unique_value: &UniqueValue,
        ) -> Result<Option<Withdrawal>, RepositoryError<Withdrawal>> {
            Ok(None)
        }

        async fn save(
            &self,
            _uow: &mut Self::Uow,
            _request_context: &RequestContext,
            aggregate: &mut Withdrawal,
        ) -> Result<(), RepositoryError<Withdrawal>> {
            *self.item.lock().expect("lock") = Some(aggregate.clone());
            *self.saved.lock().expect("lock") = Some(aggregate.clone());
            Ok(())
        }
    }

    #[derive(Clone)]
    struct TestCurrencyRepository {
        item: Arc<Mutex<Option<Currency>>>,
    }

    impl TestCurrencyRepository {
        fn new(currency: Currency) -> Self {
            Self {
                item: Arc::new(Mutex::new(Some(currency))),
            }
        }
    }

    impl Repository<Currency> for TestCurrencyRepository {
        type Uow = TestUow;

        async fn read(
            &self,
            _uow: &mut Self::Uow,
            id: CurrencyId,
        ) -> Result<Currency, RepositoryError<Currency>> {
            self.item
                .lock()
                .expect("lock")
                .clone()
                .ok_or_else(|| RepositoryError::NotFound {
                    aggregate_type: Currency::TYPE,
                    aggregate_id: id,
                })
        }

        async fn read_at_version(
            &self,
            _uow: &mut Self::Uow,
            id: CurrencyId,
            _at: AggregateVersion,
        ) -> Result<Currency, RepositoryError<Currency>> {
            self.read(_uow, id).await
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
            *self.item.lock().expect("lock") = Some(aggregate.clone());
            Ok(())
        }
    }

    #[derive(Clone)]
    struct TestPoolTokenTransferExecutor {
        outcome: TestPoolTokenTransferOutcome,
        request: Arc<Mutex<Option<PoolTokenTransferRequest>>>,
    }

    impl TestPoolTokenTransferExecutor {
        fn new(outcome: TestPoolTokenTransferOutcome) -> Self {
            Self {
                outcome,
                request: Arc::new(Mutex::new(None)),
            }
        }
    }

    impl PoolTokenTransferExecutor for TestPoolTokenTransferExecutor {
        async fn execute(
            &self,
            request: PoolTokenTransferRequest,
        ) -> Result<(), PoolTokenTransferExecutorError> {
            *self.request.lock().expect("lock") = Some(request);
            match self.outcome {
                TestPoolTokenTransferOutcome::Transferred => Ok(()),
                TestPoolTokenTransferOutcome::BackendFailed => Err(
                    PoolTokenTransferExecutorError::Backend(Box::new(TestBackendError)),
                ),
            }
        }
    }

    #[derive(Clone, Copy)]
    enum TestPoolTokenTransferOutcome {
        Transferred,
        BackendFailed,
    }

    #[derive(Debug, Error)]
    #[error("test backend error")]
    struct TestBackendError;

    fn request_context() -> RequestContext {
        RequestContext::new(
            CorrelationId::from(Uuid::now_v7()),
            MessageId::new(),
            Principal::System,
        )
        .expect("request context should be valid")
    }

    fn token_account_owner_address() -> TokenAccountOwnerAddress {
        TokenAccountOwnerAddress::try_from("11111111111111111111111111111111")
            .expect("token account owner address should be valid")
    }

    fn mint_account() -> MintAccount {
        MintAccount::new(
            MintAccountAddress::try_from("Mint111111111111111111111111111111111111")
                .expect("mint account address should be valid"),
            PoolTokenAccountAddress::try_from("Pool111111111111111111111111111111111111")
                .expect("pool token account address should be valid"),
        )
    }

    fn provisioned_currency() -> Currency {
        let mut currency = Currency::new();
        currency
            .define(CurrencyDefinition {
                owner: CurrencyOwner::from(UserId::new()),
                symbol: CurrencySymbol::try_from("usdc").expect("symbol should be valid"),
                name: CurrencyName::try_from("USD Coin").expect("name should be valid"),
                decimals: CurrencyDecimals::new(6),
                description: None,
                image: None,
            })
            .expect("currency should define");
        currency
            .provision(mint_account())
            .expect("currency should provision");
        currency.core_mut().clear_uncommitted_events();
        currency
    }

    fn unprovisioned_currency() -> Currency {
        let mut currency = Currency::new();
        currency
            .define(CurrencyDefinition {
                owner: CurrencyOwner::from(UserId::new()),
                symbol: CurrencySymbol::try_from("usdc").expect("symbol should be valid"),
                name: CurrencyName::try_from("USD Coin").expect("name should be valid"),
                decimals: CurrencyDecimals::new(6),
                description: None,
                image: None,
            })
            .expect("currency should define");
        currency.core_mut().clear_uncommitted_events();
        currency
    }

    fn requested_withdrawal(currency_id: CurrencyId) -> Withdrawal {
        let mut withdrawal = Withdrawal::new();
        withdrawal
            .request(WithdrawalRequest {
                account_id: AccountId::new(),
                currency_id,
                token_account_owner_address: token_account_owner_address(),
                amount: CurrencyAmount::new(100),
            })
            .expect("withdrawal should be requested");
        withdrawal.core_mut().clear_uncommitted_events();
        withdrawal
    }

    #[tokio::test]
    async fn handle_records_successful_token_transfer() {
        let currency = provisioned_currency();
        let currency_id = currency.aggregate_id();
        let withdrawal = requested_withdrawal(currency_id);
        let withdrawal_id = withdrawal.aggregate_id();
        let withdrawal_repository = TestWithdrawalRepository::new(withdrawal);
        let pool_token_transfer_executor =
            TestPoolTokenTransferExecutor::new(TestPoolTokenTransferOutcome::Transferred);
        let handler = WithdrawalTokenTransferCommandHandler::new(
            withdrawal_repository.clone(),
            TestCurrencyRepository::new(currency),
            pool_token_transfer_executor,
        );
        let mut uow = TestUow;

        let handled = handler
            .handle(
                &mut uow,
                &request_context(),
                &WithdrawalTokenTransferCommand { withdrawal_id },
            )
            .await
            .expect("token transfer should succeed");

        assert_eq!(
            handled.into_output(),
            WithdrawalTokenTransferOutput::TokenTransferred
        );
        let saved = withdrawal_repository
            .saved
            .lock()
            .expect("lock")
            .clone()
            .expect("withdrawal should be saved");
        assert_eq!(
            saved.status().expect("status should exist"),
            &WithdrawalStatus::TokenTransferred
        );
        assert_eq!(
            saved.uncommitted_events()[0].payload().name(),
            WithdrawalEventPayload::TOKEN_TRANSFERRED
        );
    }

    #[tokio::test]
    async fn handle_records_rejection_when_currency_is_unprovisioned() {
        let currency = unprovisioned_currency();
        let currency_id = currency.aggregate_id();
        let withdrawal = requested_withdrawal(currency_id);
        let withdrawal_id = withdrawal.aggregate_id();
        let withdrawal_repository = TestWithdrawalRepository::new(withdrawal);
        let handler = WithdrawalTokenTransferCommandHandler::new(
            withdrawal_repository.clone(),
            TestCurrencyRepository::new(currency),
            TestPoolTokenTransferExecutor::new(TestPoolTokenTransferOutcome::Transferred),
        );
        let mut uow = TestUow;

        let handled = handler
            .handle(
                &mut uow,
                &request_context(),
                &WithdrawalTokenTransferCommand { withdrawal_id },
            )
            .await
            .expect("unprovisioned currency should reject the token transfer");

        let reason = WithdrawalTokenTransferRejectionReason::CurrencyUnprovisioned;
        assert_eq!(
            handled.into_output(),
            WithdrawalTokenTransferOutput::Rejected { reason }
        );
        let saved = withdrawal_repository
            .saved
            .lock()
            .expect("lock")
            .clone()
            .expect("rejected withdrawal should be saved");
        assert_eq!(
            saved.status().expect("status should exist"),
            &WithdrawalStatus::Pending
        );
        assert_eq!(
            saved.uncommitted_events()[0].payload(),
            &WithdrawalEventPayload::TokenTransferRejected { reason }
        );
    }

    #[tokio::test]
    async fn handle_keeps_backend_error_retryable_without_saving_withdrawal() {
        let currency = provisioned_currency();
        let currency_id = currency.aggregate_id();
        let withdrawal = requested_withdrawal(currency_id);
        let withdrawal_id = withdrawal.aggregate_id();
        let withdrawal_repository = TestWithdrawalRepository::new(withdrawal);
        let pool_token_transfer_executor =
            TestPoolTokenTransferExecutor::new(TestPoolTokenTransferOutcome::BackendFailed);
        let handler = WithdrawalTokenTransferCommandHandler::new(
            withdrawal_repository.clone(),
            TestCurrencyRepository::new(currency),
            pool_token_transfer_executor,
        );
        let mut uow = TestUow;

        let error = handler
            .handle(
                &mut uow,
                &request_context(),
                &WithdrawalTokenTransferCommand { withdrawal_id },
            )
            .await
            .expect_err("backend failure should stay retryable");

        assert!(matches!(
            error,
            WithdrawalTokenTransferCommandHandlerError::PoolTokenTransferExecutor(
                PoolTokenTransferExecutorError::Backend(_)
            )
        ));
        assert!(withdrawal_repository.saved.lock().expect("lock").is_none());
    }
}
