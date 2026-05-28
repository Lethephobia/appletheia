use appletheia::application::authorization::{AuthorizationPlan, PrincipalRequirement};
use appletheia::application::command::{CommandHandled, CommandHandler};
use appletheia::application::repository::Repository;
use appletheia::application::request_context::RequestContext;
use banking_ledger_domain::currency::{
    Currency, CurrencyImageRef, CurrencyMintAccountMetadataSyncRejectionReason,
};

use super::{
    CurrencyMintAccountMetadataSyncCommand, CurrencyMintAccountMetadataSyncCommandHandlerConfig,
    CurrencyMintAccountMetadataSyncCommandHandlerError, CurrencyMintAccountMetadataSyncOutput,
};
use crate::mint::{
    MintAccountMetadata, MintAccountMetadataUpdateRequest, MintAccountMetadataUpdater,
    MintAccountSeed, MintMetadataDescription, MintMetadataDocument, MintMetadataImageUri,
    MintMetadataName, MintMetadataPublishRequest, MintMetadataPublisher, MintMetadataSymbol,
};

/// Handles `CurrencyMintAccountMetadataSyncCommand`.
pub struct CurrencyMintAccountMetadataSyncCommandHandler<CR, MMP, MAMU>
where
    CR: Repository<Currency>,
    MMP: MintMetadataPublisher,
    MAMU: MintAccountMetadataUpdater,
{
    currency_repository: CR,
    mint_metadata_publisher: MMP,
    mint_account_metadata_updater: MAMU,
    config: CurrencyMintAccountMetadataSyncCommandHandlerConfig,
}

impl<CR, MMP, MAMU> CurrencyMintAccountMetadataSyncCommandHandler<CR, MMP, MAMU>
where
    CR: Repository<Currency>,
    MMP: MintMetadataPublisher,
    MAMU: MintAccountMetadataUpdater,
{
    pub fn new(
        currency_repository: CR,
        mint_metadata_publisher: MMP,
        mint_account_metadata_updater: MAMU,
        config: CurrencyMintAccountMetadataSyncCommandHandlerConfig,
    ) -> Self {
        Self {
            currency_repository,
            mint_metadata_publisher,
            mint_account_metadata_updater,
            config,
        }
    }

    fn mint_metadata_image(
        &self,
        image: &CurrencyImageRef,
    ) -> Result<MintMetadataImageUri, CurrencyMintAccountMetadataSyncCommandHandlerError> {
        match image {
            CurrencyImageRef::ObjectName(object_name) => {
                Ok(self.config.image_public_base_url().resolve(object_name)?)
            }
            CurrencyImageRef::ExternalUrl(url) => Ok(MintMetadataImageUri::try_from(url)?),
        }
    }
}

impl<CR, MMP, MAMU> CommandHandler for CurrencyMintAccountMetadataSyncCommandHandler<CR, MMP, MAMU>
where
    CR: Repository<Currency>,
    MMP: MintMetadataPublisher,
    MAMU: MintAccountMetadataUpdater,
{
    type Command = CurrencyMintAccountMetadataSyncCommand;
    type Output = CurrencyMintAccountMetadataSyncOutput;
    type ReplayOutput = CurrencyMintAccountMetadataSyncOutput;
    type Error = CurrencyMintAccountMetadataSyncCommandHandlerError;
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
            return Err(CurrencyMintAccountMetadataSyncCommandHandlerError::CurrencyNotFound);
        };

        if currency.mint_account()?.is_none() {
            let reason = CurrencyMintAccountMetadataSyncRejectionReason::NotProvisioned;
            currency.reject_mint_account_metadata_sync(reason)?;

            self.currency_repository
                .save(uow, request_context, &mut currency)
                .await?;

            return Ok(CommandHandled::same(
                CurrencyMintAccountMetadataSyncOutput::Rejected { reason },
            ));
        }

        let seed = MintAccountSeed::try_from(command.currency_id)?;
        let metadata_name = MintMetadataName::from(currency.name()?);
        let metadata_symbol = MintMetadataSymbol::from(currency.symbol()?);
        let description = currency.description()?.map(MintMetadataDescription::from);
        let image = currency
            .image()?
            .map(|image| self.mint_metadata_image(image))
            .transpose()?;
        let document = MintMetadataDocument::new(
            metadata_name.clone(),
            metadata_symbol.clone(),
            description,
            image,
        );
        let metadata_uri = self
            .mint_metadata_publisher
            .publish(MintMetadataPublishRequest::new(seed.clone(), document))
            .await?;
        let metadata = MintAccountMetadata::new(metadata_name, metadata_symbol, metadata_uri);
        self.mint_account_metadata_updater
            .update(MintAccountMetadataUpdateRequest::new(seed, metadata))
            .await?;
        currency.record_mint_account_metadata_synced()?;

        self.currency_repository
            .save(uow, request_context, &mut currency)
            .await?;

        Ok(CommandHandled::same(
            CurrencyMintAccountMetadataSyncOutput::Synced,
        ))
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
        CurrencyImageRef, CurrencyImageUrl, CurrencyMintAccount, CurrencyMintAccountAddress,
        CurrencyMintAccountMetadataSyncRejectionReason, CurrencyName, CurrencyOwner,
        CurrencyPoolTokenAccountAddress, CurrencySymbol, CurrencyTokenProgramId,
    };
    use uuid::Uuid;

    use super::{
        CurrencyMintAccountMetadataSyncCommand, CurrencyMintAccountMetadataSyncCommandHandler,
        CurrencyMintAccountMetadataSyncCommandHandlerConfig, CurrencyMintAccountMetadataSyncOutput,
    };
    use crate::mint::{
        MintAccountMetadata, MintAccountMetadataUpdateRequest, MintAccountMetadataUpdater,
        MintAccountMetadataUpdaterError, MintAccountSeed, MintMetadataDescription,
        MintMetadataDocument, MintMetadataImagePublicBaseUrl, MintMetadataName,
        MintMetadataPublishRequest, MintMetadataPublisher, MintMetadataPublisherError,
        MintMetadataSymbol, MintMetadataUri,
    };

    const TOKEN_PROGRAM_ID: &str = "TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb";

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
            let aggregate = aggregate.clone();
            *self.saved_currency.lock().expect("lock") = Some(aggregate.clone());
            *self.currency.lock().expect("lock") = Some(aggregate);
            Ok(())
        }
    }

    #[derive(Clone)]
    struct TestMintMetadataPublisher {
        calls: Arc<Mutex<usize>>,
        request: Arc<Mutex<Option<MintMetadataPublishRequest>>>,
    }

    impl TestMintMetadataPublisher {
        fn new() -> Self {
            Self {
                calls: Arc::new(Mutex::new(0)),
                request: Arc::new(Mutex::new(None)),
            }
        }
    }

    impl MintMetadataPublisher for TestMintMetadataPublisher {
        async fn publish(
            &self,
            request: MintMetadataPublishRequest,
        ) -> Result<MintMetadataUri, MintMetadataPublisherError> {
            *self.calls.lock().expect("lock") += 1;
            *self.request.lock().expect("lock") = Some(request);
            Ok(MintMetadataUri::try_from(
                "https://metadata.example.com/currencies/test/mint/metadata.json",
            )
            .expect("metadata URI should be valid"))
        }
    }

    #[derive(Clone)]
    struct TestMintAccountMetadataUpdater {
        calls: Arc<Mutex<usize>>,
        request: Arc<Mutex<Option<MintAccountMetadataUpdateRequest>>>,
    }

    impl TestMintAccountMetadataUpdater {
        fn new() -> Self {
            Self {
                calls: Arc::new(Mutex::new(0)),
                request: Arc::new(Mutex::new(None)),
            }
        }
    }

    impl MintAccountMetadataUpdater for TestMintAccountMetadataUpdater {
        async fn update(
            &self,
            request: MintAccountMetadataUpdateRequest,
        ) -> Result<(), MintAccountMetadataUpdaterError> {
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
        let mut currency = Currency::default();
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

    fn mint_account() -> CurrencyMintAccount {
        CurrencyMintAccount::new(
            CurrencyMintAccountAddress::try_from("Mint111111111111111111111111111111111111")
                .expect("mint account address should be valid"),
            CurrencyPoolTokenAccountAddress::try_from("Pool111111111111111111111111111111111111")
                .expect("pool account address should be valid"),
            CurrencyTokenProgramId::try_from(TOKEN_PROGRAM_ID)
                .expect("token program ID should be valid"),
        )
    }

    fn config() -> CurrencyMintAccountMetadataSyncCommandHandlerConfig {
        CurrencyMintAccountMetadataSyncCommandHandlerConfig::new(
            MintMetadataImagePublicBaseUrl::try_from("https://assets.example.com/")
                .expect("image base URL should be valid"),
        )
    }

    fn handler(
        repository: TestCurrencyRepository,
        publisher: TestMintMetadataPublisher,
        updater: TestMintAccountMetadataUpdater,
    ) -> CurrencyMintAccountMetadataSyncCommandHandler<
        TestCurrencyRepository,
        TestMintMetadataPublisher,
        TestMintAccountMetadataUpdater,
    > {
        CurrencyMintAccountMetadataSyncCommandHandler::new(repository, publisher, updater, config())
    }

    #[tokio::test]
    async fn handle_rejects_when_mint_account_is_not_recorded() {
        let currency = defined_currency(None, false);
        let currency_id = currency.aggregate_id().expect("currency id should exist");
        let repository = TestCurrencyRepository::new(currency);
        let saved_currency = repository.saved_currency.clone();
        let publisher = TestMintMetadataPublisher::new();
        let publisher_calls = publisher.calls.clone();
        let updater = TestMintAccountMetadataUpdater::new();
        let updater_calls = updater.calls.clone();
        let handler = handler(repository, publisher, updater);
        let mut uow = TestUow;

        let handled = handler
            .handle(
                &mut uow,
                &request_context(),
                &CurrencyMintAccountMetadataSyncCommand { currency_id },
            )
            .await
            .expect("command should be handled");

        assert_eq!(
            handled.into_output(),
            CurrencyMintAccountMetadataSyncOutput::Rejected {
                reason: CurrencyMintAccountMetadataSyncRejectionReason::NotProvisioned,
            }
        );
        assert_eq!(*publisher_calls.lock().expect("lock"), 0);
        assert_eq!(*updater_calls.lock().expect("lock"), 0);

        let saved = saved_currency
            .lock()
            .expect("lock")
            .clone()
            .expect("currency should be saved");
        assert_eq!(saved.uncommitted_events().len(), 1);
        assert_eq!(
            saved.uncommitted_events()[0].payload(),
            &CurrencyEventPayload::MintAccountMetadataSyncRejected {
                reason: CurrencyMintAccountMetadataSyncRejectionReason::NotProvisioned,
            }
        );
    }

    #[tokio::test]
    async fn handle_republishes_metadata_and_updates_onchain_metadata() {
        let currency = defined_currency_with_mint_account();
        let currency_id = currency.aggregate_id().expect("currency id should exist");
        let repository = TestCurrencyRepository::new(currency);
        let saved_currency = repository.saved_currency.clone();
        let publisher = TestMintMetadataPublisher::new();
        let published_request = publisher.request.clone();
        let publisher_calls = publisher.calls.clone();
        let updater = TestMintAccountMetadataUpdater::new();
        let update_request = updater.request.clone();
        let updater_calls = updater.calls.clone();
        let handler = handler(repository, publisher, updater);
        let mut uow = TestUow;

        let handled = handler
            .handle(
                &mut uow,
                &request_context(),
                &CurrencyMintAccountMetadataSyncCommand { currency_id },
            )
            .await
            .expect("command should be handled");

        assert_eq!(
            handled.into_output(),
            CurrencyMintAccountMetadataSyncOutput::Synced
        );
        assert_eq!(*publisher_calls.lock().expect("lock"), 1);
        assert_eq!(*updater_calls.lock().expect("lock"), 1);

        let published_request = published_request
            .lock()
            .expect("lock")
            .clone()
            .expect("publish request should be captured");
        assert_eq!(
            published_request.document(),
            &MintMetadataDocument::new(
                MintMetadataName::try_from("USD Coin").expect("name should be valid"),
                MintMetadataSymbol::try_from("USDC").expect("symbol should be valid"),
                Some(
                    MintMetadataDescription::try_from("Stablecoin backed by USD")
                        .expect("description should be valid"),
                ),
                None,
            )
        );

        let update_request = update_request
            .lock()
            .expect("lock")
            .clone()
            .expect("update request should be captured");
        assert_eq!(
            update_request,
            MintAccountMetadataUpdateRequest::new(
                MintAccountSeed::try_from(currency_id).expect("seed should be valid"),
                MintAccountMetadata::new(
                    MintMetadataName::try_from("USD Coin").expect("name should be valid"),
                    MintMetadataSymbol::try_from("USDC").expect("symbol should be valid"),
                    MintMetadataUri::try_from(
                        "https://metadata.example.com/currencies/test/mint/metadata.json"
                    )
                    .expect("metadata URI should be valid"),
                ),
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
            &CurrencyEventPayload::MintAccountMetadataSynced
        );
    }

    #[tokio::test]
    async fn handle_resolves_object_storage_image_to_public_uri_before_syncing_metadata() {
        let image = CurrencyImageRef::object_name(
            CurrencyImageObjectName::try_from(
                "currencies/00000000-0000-0000-0000-000000000001/images/00000000-0000-0000-0000-000000000002",
            )
            .expect("image object name should be valid"),
        );
        let currency = defined_currency(Some(image), true);
        let currency_id = currency.aggregate_id().expect("currency id should exist");
        let repository = TestCurrencyRepository::new(currency);
        let publisher = TestMintMetadataPublisher::new();
        let published_request = publisher.request.clone();
        let updater = TestMintAccountMetadataUpdater::new();
        let handler = handler(repository, publisher, updater);
        let mut uow = TestUow;

        handler
            .handle(
                &mut uow,
                &request_context(),
                &CurrencyMintAccountMetadataSyncCommand { currency_id },
            )
            .await
            .expect("command should be handled");

        let request = published_request
            .lock()
            .expect("lock")
            .clone()
            .expect("metadata request should be captured");
        assert_eq!(
            request
                .document()
                .image()
                .map(|value| value.value().as_str()),
            Some(
                "https://assets.example.com/currencies/00000000-0000-0000-0000-000000000001/images/00000000-0000-0000-0000-000000000002"
            )
        );
    }

    #[tokio::test]
    async fn handle_preserves_external_image_uri_when_syncing_metadata() {
        let image = CurrencyImageRef::external_url(
            CurrencyImageUrl::try_from("https://cdn.example.com/currencies/usdc.png")
                .expect("image URL should be valid"),
        );
        let currency = defined_currency(Some(image), true);
        let currency_id = currency.aggregate_id().expect("currency id should exist");
        let repository = TestCurrencyRepository::new(currency);
        let publisher = TestMintMetadataPublisher::new();
        let published_request = publisher.request.clone();
        let updater = TestMintAccountMetadataUpdater::new();
        let handler = handler(repository, publisher, updater);
        let mut uow = TestUow;

        handler
            .handle(
                &mut uow,
                &request_context(),
                &CurrencyMintAccountMetadataSyncCommand { currency_id },
            )
            .await
            .expect("command should be handled");

        let request = published_request
            .lock()
            .expect("lock")
            .clone()
            .expect("metadata request should be captured");
        assert_eq!(
            request
                .document()
                .image()
                .map(|value| value.value().as_str()),
            Some("https://cdn.example.com/currencies/usdc.png")
        );
    }
}
