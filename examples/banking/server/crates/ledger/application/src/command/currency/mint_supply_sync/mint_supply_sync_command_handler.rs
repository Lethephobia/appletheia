use crate::mint::{MintSupplySyncRequest, MintSupplySynchronizer};
use appletheia::application::authorization::{AuthorizationPlan, PrincipalRequirement};
use appletheia::application::command::{CommandHandled, CommandHandler};
use appletheia::application::repository::Repository;
use appletheia::application::request_context::RequestContext;
use banking_ledger_domain::currency::{Currency, MintSupplySyncRejectionReason};

use super::{MintSupplySyncCommand, MintSupplySyncCommandHandlerError, MintSupplySyncOutput};

/// Handles `MintSupplySyncCommand`.
pub struct MintSupplySyncCommandHandler<CR, MSS>
where
    CR: Repository<Currency>,
    MSS: MintSupplySynchronizer,
{
    currency_repository: CR,
    mint_supply_synchronizer: MSS,
}

impl<CR, MSS> MintSupplySyncCommandHandler<CR, MSS>
where
    CR: Repository<Currency>,
    MSS: MintSupplySynchronizer,
{
    pub fn new(currency_repository: CR, mint_supply_synchronizer: MSS) -> Self {
        Self {
            currency_repository,
            mint_supply_synchronizer,
        }
    }
}

impl<CR, MSS> CommandHandler for MintSupplySyncCommandHandler<CR, MSS>
where
    CR: Repository<Currency>,
    MSS: MintSupplySynchronizer,
{
    type Command = MintSupplySyncCommand;
    type Output = MintSupplySyncOutput;
    type ReplayOutput = MintSupplySyncOutput;
    type Error = MintSupplySyncCommandHandlerError;
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
        let mut currency = self
            .currency_repository
            .read(uow, command.currency_id)
            .await?;

        if currency.mint_account()?.is_none() {
            let reason = MintSupplySyncRejectionReason::NotProvisioned;
            currency.reject_mint_supply_sync(reason)?;

            self.currency_repository
                .save(uow, request_context, &mut currency)
                .await?;

            return Ok(CommandHandled::same(MintSupplySyncOutput::Rejected {
                reason,
            }));
        }

        let target_supply = currency.target_supply()?;
        self.mint_supply_synchronizer
            .sync_supply(MintSupplySyncRequest::new(
                command.currency_id,
                *currency.decimals()?,
                target_supply,
            ))
            .await?;

        currency.record_mint_supply_synced(target_supply)?;
        self.currency_repository
            .save(uow, request_context, &mut currency)
            .await?;

        Ok(CommandHandled::same(MintSupplySyncOutput::Synced))
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
    use banking_ledger_domain::core::CurrencyAmount;
    use banking_ledger_domain::currency::{
        Currency, CurrencyDecimals, CurrencyDefinition, CurrencyEventPayload, CurrencyId,
        CurrencyName, CurrencyOwner, CurrencySymbol, MintAccount, MintAccountAddress,
        MintSupplySyncRejectionReason, PoolTokenAccountAddress,
    };
    use uuid::Uuid;

    use super::{MintSupplySyncCommand, MintSupplySyncCommandHandler, MintSupplySyncOutput};
    use crate::mint::{MintSupplySyncRequest, MintSupplySynchronizer, MintSupplySynchronizerError};

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
            _at: AggregateVersion,
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
            Ok(())
        }
    }

    #[derive(Clone, Default)]
    struct TestMintSupplySynchronizer {
        calls: Arc<Mutex<usize>>,
        request: Arc<Mutex<Option<MintSupplySyncRequest>>>,
    }

    impl MintSupplySynchronizer for TestMintSupplySynchronizer {
        async fn sync_supply(
            &self,
            request: MintSupplySyncRequest,
        ) -> Result<(), MintSupplySynchronizerError> {
            *self.calls.lock().expect("lock") += 1;
            *self.request.lock().expect("lock") = Some(request);
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
        let mut currency = Currency::new();
        currency
            .define(CurrencyDefinition {
                owner: CurrencyOwner::User(UserId::new()),
                symbol: CurrencySymbol::try_from("usdc").expect("symbol should be valid"),
                name: CurrencyName::try_from("USD Coin").expect("name should be valid"),
                decimals: CurrencyDecimals::new(6),
                description: None,
                image: None,
            })
            .expect("currency should be defined");
        currency.core_mut().clear_uncommitted_events();
        currency
    }

    fn mint_account() -> MintAccount {
        MintAccount::new(
            MintAccountAddress::try_from("Mint111111111111111111111111111111111111")
                .expect("mint account address should be valid"),
            PoolTokenAccountAddress::try_from("Pool111111111111111111111111111111111111")
                .expect("pool account address should be valid"),
        )
    }

    #[tokio::test]
    async fn handle_syncs_supply_and_records_event() {
        let mut currency = defined_currency();
        let currency_id = currency.aggregate_id();
        currency
            .provision(mint_account())
            .expect("currency should be provisioned");
        currency
            .reserve_supply(CurrencyAmount::new(100))
            .expect("supply reserve should succeed");
        currency.core_mut().clear_uncommitted_events();
        let repository = TestCurrencyRepository::new(currency);
        let synchronizer = TestMintSupplySynchronizer::default();
        let request_log = synchronizer.request.clone();
        let handler = MintSupplySyncCommandHandler::new(repository.clone(), synchronizer);
        let mut uow = TestUow;

        let handled = handler
            .handle(
                &mut uow,
                &request_context(),
                &MintSupplySyncCommand { currency_id },
            )
            .await
            .expect("command should be handled");

        assert_eq!(handled.into_output(), MintSupplySyncOutput::Synced);
        assert_eq!(
            request_log.lock().expect("lock").clone(),
            Some(MintSupplySyncRequest::new(
                currency_id,
                CurrencyDecimals::new(6),
                CurrencyAmount::new(100),
            ))
        );

        let saved = repository
            .saved
            .lock()
            .expect("lock")
            .clone()
            .expect("currency should be saved");
        assert_eq!(
            saved.uncommitted_events()[0].payload(),
            &CurrencyEventPayload::MintSupplySynced {
                supply: CurrencyAmount::new(100),
            }
        );
    }

    #[tokio::test]
    async fn handle_rejects_when_mint_account_is_missing() {
        let mut currency = defined_currency();
        let currency_id = currency.aggregate_id();
        currency.core_mut().clear_uncommitted_events();
        let repository = TestCurrencyRepository::new(currency);
        let handler = MintSupplySyncCommandHandler::new(
            repository.clone(),
            TestMintSupplySynchronizer::default(),
        );
        let mut uow = TestUow;

        let handled = handler
            .handle(
                &mut uow,
                &request_context(),
                &MintSupplySyncCommand { currency_id },
            )
            .await
            .expect("command should record a rejection until mint account exists");

        let reason = MintSupplySyncRejectionReason::NotProvisioned;
        assert_eq!(
            handled.into_output(),
            MintSupplySyncOutput::Rejected { reason }
        );
        let saved = repository
            .saved
            .lock()
            .expect("lock")
            .clone()
            .expect("rejected sync should be saved");
        assert_eq!(
            saved.uncommitted_events()[0].payload(),
            &CurrencyEventPayload::MintSupplySyncRejected { reason }
        );
    }
}
