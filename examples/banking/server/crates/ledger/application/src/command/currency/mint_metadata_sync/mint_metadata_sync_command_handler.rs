use crate::mint::{MintMetadataUpdateRequest, MintMetadataUpdater};
use appletheia::application::authorization::{AuthorizationPlan, PrincipalRequirement};
use appletheia::application::command::{CommandHandled, CommandHandler};
use appletheia::application::repository::Repository;
use appletheia::application::request_context::RequestContext;
use banking_ledger_domain::currency::{Currency, MintMetadataSyncRejectionReason};

use super::{MintMetadataSyncCommand, MintMetadataSyncCommandHandlerError, MintMetadataSyncOutput};

/// Handles `MintMetadataSyncCommand`.
pub struct MintMetadataSyncCommandHandler<CR, MAMU>
where
    CR: Repository<Currency>,
    MAMU: MintMetadataUpdater,
{
    currency_repository: CR,
    mint_metadata_updater: MAMU,
}

impl<CR, MAMU> MintMetadataSyncCommandHandler<CR, MAMU>
where
    CR: Repository<Currency>,
    MAMU: MintMetadataUpdater,
{
    pub fn new(currency_repository: CR, mint_metadata_updater: MAMU) -> Self {
        Self {
            currency_repository,
            mint_metadata_updater,
        }
    }
}

impl<CR, MAMU> CommandHandler for MintMetadataSyncCommandHandler<CR, MAMU>
where
    CR: Repository<Currency>,
    MAMU: MintMetadataUpdater,
{
    type Command = MintMetadataSyncCommand;
    type Output = MintMetadataSyncOutput;
    type ReplayOutput = MintMetadataSyncOutput;
    type Error = MintMetadataSyncCommandHandlerError;
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
            let reason = MintMetadataSyncRejectionReason::NotProvisioned;
            currency.reject_mint_metadata_sync(reason)?;

            self.currency_repository
                .save(uow, request_context, &mut currency)
                .await?;

            return Ok(CommandHandled::same(MintMetadataSyncOutput::Rejected {
                reason,
            }));
        }

        self.mint_metadata_updater
            .update(MintMetadataUpdateRequest::new(
                command.currency_id,
                currency.name()?.clone(),
                currency.symbol()?.clone(),
                currency.description()?.cloned(),
                currency.image()?.cloned(),
            ))
            .await?;
        currency.record_mint_metadata_synced()?;

        self.currency_repository
            .save(uow, request_context, &mut currency)
            .await?;

        Ok(CommandHandled::same(MintMetadataSyncOutput::Synced))
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
        Currency, CurrencyDecimals, CurrencyEventPayload, CurrencyId, CurrencyImageObjectName,
        CurrencyImageRef, CurrencyImageUrl, CurrencyName, CurrencyOwner, CurrencySymbol,
        MintAccount, MintAccountAddress, MintMetadataSyncRejectionReason, PoolTokenAccountAddress,
    };
    use uuid::Uuid;

    use super::{MintMetadataSyncCommand, MintMetadataSyncCommandHandler, MintMetadataSyncOutput};
    use crate::mint::{MintMetadataUpdateRequest, MintMetadataUpdater, MintMetadataUpdaterError};

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
        saved_currency: Arc<Mutex<Option<Currency>>>,
    }

    impl TestCurrencyRepository {
        fn new(currency: Currency) -> Self {
            Self {
                currency: Arc::new(Mutex::new(Some(currency))),
                saved_currency: Arc::new(Mutex::new(None)),
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
            let aggregate = aggregate.clone();
            *self.saved_currency.lock().expect("lock") = Some(aggregate.clone());
            *self.currency.lock().expect("lock") = Some(aggregate);
            Ok(())
        }
    }

    #[derive(Clone)]
    struct TestMintMetadataUpdater {
        calls: Arc<Mutex<usize>>,
        request: Arc<Mutex<Option<MintMetadataUpdateRequest>>>,
    }

    impl TestMintMetadataUpdater {
        fn new() -> Self {
            Self {
                calls: Arc::new(Mutex::new(0)),
                request: Arc::new(Mutex::new(None)),
            }
        }
    }

    impl MintMetadataUpdater for TestMintMetadataUpdater {
        async fn update(
            &self,
            request: MintMetadataUpdateRequest,
        ) -> Result<(), MintMetadataUpdaterError> {
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

    fn defined_currency_with_mint_account() -> Currency {
        defined_currency(None, true)
    }

    fn defined_currency(image: Option<CurrencyImageRef>, with_mint_account: bool) -> Currency {
        let mut currency = Currency::new();
        currency
            .define(
                CurrencyOwner::User(UserId::new()),
                CurrencySymbol::try_from("usdc").expect("symbol should be valid"),
                CurrencyName::try_from("USD Coin").expect("name should be valid"),
                CurrencyDecimals::new(6),
                Some(
                    banking_ledger_domain::currency::CurrencyDescription::try_from(
                        "Stablecoin backed by USD",
                    )
                    .expect("description should be valid"),
                ),
                image,
            )
            .expect("currency should be defined");
        if with_mint_account {
            currency
                .provision(mint_account())
                .expect("currency should be provisioned");
        }
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

    fn handler(
        repository: TestCurrencyRepository,
        updater: TestMintMetadataUpdater,
    ) -> MintMetadataSyncCommandHandler<TestCurrencyRepository, TestMintMetadataUpdater> {
        MintMetadataSyncCommandHandler::new(repository, updater)
    }

    #[tokio::test]
    async fn handle_rejects_when_mint_account_is_not_recorded() {
        let currency = defined_currency(None, false);
        let currency_id = currency.aggregate_id().expect("currency id should exist");
        let repository = TestCurrencyRepository::new(currency);
        let saved_currency = repository.saved_currency.clone();
        let updater = TestMintMetadataUpdater::new();
        let updater_calls = updater.calls.clone();
        let handler = handler(repository, updater);
        let mut uow = TestUow;

        let handled = handler
            .handle(
                &mut uow,
                &request_context(),
                &MintMetadataSyncCommand { currency_id },
            )
            .await
            .expect("command should be handled");

        assert_eq!(
            handled.into_output(),
            MintMetadataSyncOutput::Rejected {
                reason: MintMetadataSyncRejectionReason::NotProvisioned,
            }
        );
        assert_eq!(*updater_calls.lock().expect("lock"), 0);

        let saved = saved_currency
            .lock()
            .expect("lock")
            .clone()
            .expect("currency should be saved");
        assert_eq!(saved.uncommitted_events().len(), 1);
        assert_eq!(
            saved.uncommitted_events()[0].payload(),
            &CurrencyEventPayload::MintMetadataSyncRejected {
                reason: MintMetadataSyncRejectionReason::NotProvisioned,
            }
        );
    }

    #[tokio::test]
    async fn handle_updates_onchain_metadata() {
        let currency = defined_currency_with_mint_account();
        let currency_id = currency.aggregate_id().expect("currency id should exist");
        let repository = TestCurrencyRepository::new(currency);
        let saved_currency = repository.saved_currency.clone();
        let updater = TestMintMetadataUpdater::new();
        let update_request = updater.request.clone();
        let updater_calls = updater.calls.clone();
        let handler = handler(repository, updater);
        let mut uow = TestUow;

        let handled = handler
            .handle(
                &mut uow,
                &request_context(),
                &MintMetadataSyncCommand { currency_id },
            )
            .await
            .expect("command should be handled");

        assert_eq!(handled.into_output(), MintMetadataSyncOutput::Synced);
        assert_eq!(*updater_calls.lock().expect("lock"), 1);

        let update_request = update_request
            .lock()
            .expect("lock")
            .clone()
            .expect("update request should be captured");
        assert_eq!(
            update_request,
            MintMetadataUpdateRequest::new(
                currency_id,
                CurrencyName::try_from("USD Coin").expect("name should be valid"),
                CurrencySymbol::try_from("USDC").expect("symbol should be valid"),
                Some(
                    banking_ledger_domain::currency::CurrencyDescription::try_from(
                        "Stablecoin backed by USD",
                    )
                    .expect("description should be valid"),
                ),
                None,
            )
        );

        let saved = saved_currency
            .lock()
            .expect("lock")
            .clone()
            .expect("currency should be saved");
        assert_eq!(saved.uncommitted_events().len(), 1);
        assert_eq!(
            saved.uncommitted_events()[0].payload(),
            &CurrencyEventPayload::MintMetadataSynced
        );
    }

    #[tokio::test]
    async fn handle_passes_object_storage_image_to_metadata_updater() {
        let image = CurrencyImageRef::object_name(
            CurrencyImageObjectName::try_from(
                "currencies/00000000-0000-0000-0000-000000000001/images/00000000-0000-0000-0000-000000000002",
            )
            .expect("image object name should be valid"),
        );
        let currency = defined_currency(Some(image), true);
        let currency_id = currency.aggregate_id().expect("currency id should exist");
        let repository = TestCurrencyRepository::new(currency);
        let updater = TestMintMetadataUpdater::new();
        let update_request = updater.request.clone();
        let handler = handler(repository, updater);
        let mut uow = TestUow;

        handler
            .handle(
                &mut uow,
                &request_context(),
                &MintMetadataSyncCommand { currency_id },
            )
            .await
            .expect("command should be handled");

        let request = update_request
            .lock()
            .expect("lock")
            .clone()
            .expect("metadata update request should be captured");
        assert_eq!(
            request
                .image()
                .and_then(CurrencyImageRef::as_object_name)
                .map(|value| value.value()),
            Some(
                "currencies/00000000-0000-0000-0000-000000000001/images/00000000-0000-0000-0000-000000000002"
            )
        );
    }

    #[tokio::test]
    async fn handle_passes_external_image_to_metadata_updater() {
        let image = CurrencyImageRef::external_url(
            CurrencyImageUrl::try_from("https://cdn.example.com/currencies/usdc.png")
                .expect("image URL should be valid"),
        );
        let currency = defined_currency(Some(image), true);
        let currency_id = currency.aggregate_id().expect("currency id should exist");
        let repository = TestCurrencyRepository::new(currency);
        let updater = TestMintMetadataUpdater::new();
        let update_request = updater.request.clone();
        let handler = handler(repository, updater);
        let mut uow = TestUow;

        handler
            .handle(
                &mut uow,
                &request_context(),
                &MintMetadataSyncCommand { currency_id },
            )
            .await
            .expect("command should be handled");

        let request = update_request
            .lock()
            .expect("lock")
            .clone()
            .expect("metadata update request should be captured");
        assert_eq!(
            request
                .image()
                .and_then(CurrencyImageRef::as_external_url)
                .map(|value| value.value().as_str()),
            Some("https://cdn.example.com/currencies/usdc.png")
        );
    }
}
