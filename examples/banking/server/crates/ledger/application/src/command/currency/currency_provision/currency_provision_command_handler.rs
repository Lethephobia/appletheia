use appletheia::application::authorization::{AuthorizationPlan, PrincipalRequirement};
use appletheia::application::command::{CommandHandled, CommandHandler};
use appletheia::application::repository::Repository;
use appletheia::application::request_context::RequestContext;
use banking_ledger_domain::currency::{
    Currency, CurrencyImageRef, CurrencyMintAccount, CurrencyProvisionRejectionReason,
    CurrencyProvisionResult,
};

use super::{
    CurrencyProvisionCommand, CurrencyProvisionCommandHandlerConfig,
    CurrencyProvisionCommandHandlerError, CurrencyProvisionOutput,
};
use crate::mint::{
    MintAccountCreateReceiptError, MintAccountCreateRequest, MintAccountCreator,
    MintAccountDecimals, MintAccountMetadata, MintAccountSeed, MintMetadataDescription,
    MintMetadataDocument, MintMetadataImageUri, MintMetadataName, MintMetadataPublishRequest,
    MintMetadataPublisher, MintMetadataSymbol,
};

/// Handles `CurrencyProvisionCommand`.
pub struct CurrencyProvisionCommandHandler<CR, MMP, MAC>
where
    CR: Repository<Currency>,
    MMP: MintMetadataPublisher,
    MAC: MintAccountCreator,
{
    currency_repository: CR,
    mint_metadata_publisher: MMP,
    mint_account_creator: MAC,
    config: CurrencyProvisionCommandHandlerConfig,
}

impl<CR, MMP, MAC> CurrencyProvisionCommandHandler<CR, MMP, MAC>
where
    CR: Repository<Currency>,
    MMP: MintMetadataPublisher,
    MAC: MintAccountCreator,
{
    pub fn new(
        currency_repository: CR,
        mint_metadata_publisher: MMP,
        mint_account_creator: MAC,
        config: CurrencyProvisionCommandHandlerConfig,
    ) -> Self {
        Self {
            currency_repository,
            mint_metadata_publisher,
            mint_account_creator,
            config,
        }
    }

    fn mint_metadata_image(
        &self,
        image: &CurrencyImageRef,
    ) -> Result<MintMetadataImageUri, CurrencyProvisionCommandHandlerError> {
        match image {
            CurrencyImageRef::ObjectName(object_name) => {
                Ok(self.config.image_public_base_url().resolve(object_name)?)
            }
            CurrencyImageRef::ExternalUrl(url) => Ok(MintMetadataImageUri::try_from(url)?),
        }
    }
}

impl<CR, MMP, MAC> CommandHandler for CurrencyProvisionCommandHandler<CR, MMP, MAC>
where
    CR: Repository<Currency>,
    MMP: MintMetadataPublisher,
    MAC: MintAccountCreator,
{
    type Command = CurrencyProvisionCommand;
    type Output = CurrencyProvisionOutput;
    type ReplayOutput = CurrencyProvisionOutput;
    type Error = CurrencyProvisionCommandHandlerError;
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
            return Err(CurrencyProvisionCommandHandlerError::CurrencyNotFound);
        };

        if currency.is_removed()? {
            let reason = CurrencyProvisionRejectionReason::Removed;
            currency.reject_provision(None, reason)?;
            self.currency_repository
                .save(uow, request_context, &mut currency)
                .await?;
            return Ok(CommandHandled::same(CurrencyProvisionOutput::Rejected {
                reason,
            }));
        }

        if currency.mint_account()?.is_some() {
            let reason = CurrencyProvisionRejectionReason::AlreadyProvisioned;
            currency.reject_provision(None, reason)?;
            self.currency_repository
                .save(uow, request_context, &mut currency)
                .await?;
            return Ok(CommandHandled::same(CurrencyProvisionOutput::Rejected {
                reason,
            }));
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
        let receipt = self
            .mint_account_creator
            .create_or_get(MintAccountCreateRequest::new(
                seed,
                MintAccountDecimals::from(currency.decimals()?),
                metadata,
            ))
            .await?;
        let mint_account = CurrencyMintAccount::new(
            receipt
                .address()
                .clone()
                .try_into()
                .map_err(MintAccountCreateReceiptError::from)?,
            receipt
                .pool_address()
                .clone()
                .try_into()
                .map_err(MintAccountCreateReceiptError::from)?,
            receipt
                .token_program_id()
                .clone()
                .try_into()
                .map_err(MintAccountCreateReceiptError::from)?,
        );
        let result = currency.provision(mint_account)?;

        self.currency_repository
            .save(uow, request_context, &mut currency)
            .await?;

        let output = match result {
            CurrencyProvisionResult::Provisioned { mint_account } => {
                CurrencyProvisionOutput::Provisioned { mint_account }
            }
            CurrencyProvisionResult::Rejected { reason } => {
                CurrencyProvisionOutput::Rejected { reason }
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
    use appletheia::domain::{
        Aggregate, AggregateId, AggregateVersion, EventPayload, UniqueKey, UniqueValue,
    };
    use banking_iam_domain::UserId;
    use banking_ledger_domain::currency::{
        Currency, CurrencyDecimals, CurrencyEventPayload, CurrencyId, CurrencyImageObjectName,
        CurrencyImageRef, CurrencyImageUrl, CurrencyMintAccount, CurrencyMintAccountAddress,
        CurrencyMintTokenProgramId, CurrencyName, CurrencyOwner, CurrencyPoolAccountAddress,
        CurrencyProvisionRejectionReason, CurrencySymbol,
    };
    use uuid::Uuid;

    use super::{
        CurrencyProvisionCommand, CurrencyProvisionCommandHandler,
        CurrencyProvisionCommandHandlerConfig, CurrencyProvisionOutput,
    };
    use crate::mint::{
        MintAccountAddress, MintAccountCreateReceipt, MintAccountCreateRequest, MintAccountCreator,
        MintAccountCreatorError, MintAccountSeed, MintMetadataImagePublicBaseUrl,
        MintMetadataPublishRequest, MintMetadataPublisher, MintMetadataPublisherError,
        MintMetadataUri, OnchainAccountAddress, TokenProgramId,
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
    struct TestMintAccountCreator {
        calls: Arc<Mutex<usize>>,
        receipt: MintAccountCreateReceipt,
    }

    impl TestMintAccountCreator {
        fn new(receipt: MintAccountCreateReceipt) -> Self {
            Self {
                calls: Arc::new(Mutex::new(0)),
                receipt,
            }
        }
    }

    impl MintAccountCreator for TestMintAccountCreator {
        async fn create_or_get(
            &self,
            _request: MintAccountCreateRequest,
        ) -> Result<MintAccountCreateReceipt, MintAccountCreatorError> {
            *self.calls.lock().expect("lock") += 1;
            Ok(self.receipt.clone())
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
        defined_currency_with_image(None)
    }

    fn defined_currency_with_image(image: Option<CurrencyImageRef>) -> Currency {
        let mut currency = Currency::default();
        currency
            .define(
                CurrencyOwner::User(UserId::new()),
                CurrencySymbol::try_from("usdc").expect("symbol should be valid"),
                CurrencyName::try_from("USD Coin").expect("name should be valid"),
                CurrencyDecimals::new(6),
                None,
                image,
            )
            .expect("currency should be defined");
        currency.core_mut().clear_uncommitted_events();
        currency
    }

    fn mint_account() -> CurrencyMintAccount {
        CurrencyMintAccount::new(
            CurrencyMintAccountAddress::try_from("Mint111111111111111111111111111111111111")
                .expect("mint account address should be valid"),
            CurrencyPoolAccountAddress::try_from("Pool111111111111111111111111111111111111")
                .expect("pool account address should be valid"),
            CurrencyMintTokenProgramId::try_from(TOKEN_PROGRAM_ID)
                .expect("token program ID should be valid"),
        )
    }

    fn receipt() -> MintAccountCreateReceipt {
        MintAccountCreateReceipt::new(
            MintAccountAddress::try_from("Mint111111111111111111111111111111111111")
                .expect("mint account address should be valid"),
            OnchainAccountAddress::try_from("Pool111111111111111111111111111111111111")
                .expect("pool account address should be valid"),
            TokenProgramId::try_from(TOKEN_PROGRAM_ID).expect("token program ID should be valid"),
        )
    }

    fn config() -> CurrencyProvisionCommandHandlerConfig {
        CurrencyProvisionCommandHandlerConfig::new(
            MintMetadataImagePublicBaseUrl::try_from("https://assets.example.com/")
                .expect("image base URL should be valid"),
        )
    }

    fn handler(
        repository: TestCurrencyRepository,
        publisher: TestMintMetadataPublisher,
        creator: TestMintAccountCreator,
    ) -> CurrencyProvisionCommandHandler<
        TestCurrencyRepository,
        TestMintMetadataPublisher,
        TestMintAccountCreator,
    > {
        CurrencyProvisionCommandHandler::new(repository, publisher, creator, config())
    }

    #[tokio::test]
    async fn handle_rejects_already_provisioned_without_external_side_effects() {
        let mut currency = defined_currency();
        let currency_id = currency.aggregate_id().expect("currency id should exist");
        currency
            .provision(mint_account())
            .expect("currency should be provisioned");
        currency.core_mut().clear_uncommitted_events();
        let repository = TestCurrencyRepository::new(currency);
        let publisher = TestMintMetadataPublisher::new();
        let publisher_calls = publisher.calls.clone();
        let creator = TestMintAccountCreator::new(receipt());
        let creator_calls = creator.calls.clone();
        let handler = handler(repository.clone(), publisher, creator);
        let mut uow = TestUow;

        let handled = handler
            .handle(
                &mut uow,
                &request_context(),
                &CurrencyProvisionCommand { currency_id },
            )
            .await
            .expect("command should be handled");

        assert_eq!(
            handled.into_output(),
            CurrencyProvisionOutput::Rejected {
                reason: CurrencyProvisionRejectionReason::AlreadyProvisioned
            }
        );
        assert_eq!(*publisher_calls.lock().expect("lock"), 0);
        assert_eq!(*creator_calls.lock().expect("lock"), 0);
        let saved = repository
            .saved
            .lock()
            .expect("lock")
            .clone()
            .expect("currency should be saved");
        assert_eq!(saved.uncommitted_events().len(), 1);
        assert_eq!(
            saved.uncommitted_events()[0].payload(),
            &CurrencyEventPayload::ProvisionRejected {
                mint_account: None,
                reason: CurrencyProvisionRejectionReason::AlreadyProvisioned,
            }
        );
    }

    #[tokio::test]
    async fn handle_rejects_removed_currency_without_external_side_effects() {
        let mut currency = defined_currency();
        let currency_id = currency.aggregate_id().expect("currency id should exist");
        currency.remove().expect("currency should be removed");
        currency.core_mut().clear_uncommitted_events();
        let repository = TestCurrencyRepository::new(currency);
        let publisher = TestMintMetadataPublisher::new();
        let publisher_calls = publisher.calls.clone();
        let creator = TestMintAccountCreator::new(receipt());
        let creator_calls = creator.calls.clone();
        let handler = handler(repository.clone(), publisher, creator);
        let mut uow = TestUow;

        let handled = handler
            .handle(
                &mut uow,
                &request_context(),
                &CurrencyProvisionCommand { currency_id },
            )
            .await
            .expect("command should be handled");

        assert_eq!(
            handled.into_output(),
            CurrencyProvisionOutput::Rejected {
                reason: CurrencyProvisionRejectionReason::Removed
            }
        );
        assert_eq!(*publisher_calls.lock().expect("lock"), 0);
        assert_eq!(*creator_calls.lock().expect("lock"), 0);

        let saved = repository
            .saved
            .lock()
            .expect("lock")
            .clone()
            .expect("currency should be saved");
        assert_eq!(saved.uncommitted_events().len(), 1);
        assert_eq!(
            saved.uncommitted_events()[0].payload(),
            &CurrencyEventPayload::ProvisionRejected {
                mint_account: None,
                reason: CurrencyProvisionRejectionReason::Removed,
            }
        );
    }

    #[tokio::test]
    async fn handle_publishes_metadata_creates_mint_and_provisions_currency() {
        let currency = defined_currency();
        let currency_id = currency.aggregate_id().expect("currency id should exist");
        let repository = TestCurrencyRepository::new(currency);
        let publisher = TestMintMetadataPublisher::new();
        let publisher_calls = publisher.calls.clone();
        let published_request = publisher.request.clone();
        let creator = TestMintAccountCreator::new(receipt());
        let creator_calls = creator.calls.clone();
        let handler = handler(repository.clone(), publisher, creator);
        let mut uow = TestUow;

        let handled = handler
            .handle(
                &mut uow,
                &request_context(),
                &CurrencyProvisionCommand { currency_id },
            )
            .await
            .expect("command should be handled");

        assert!(matches!(
            handled.into_output(),
            CurrencyProvisionOutput::Provisioned { .. }
        ));
        assert_eq!(*publisher_calls.lock().expect("lock"), 1);
        assert_eq!(*creator_calls.lock().expect("lock"), 1);
        assert_eq!(
            published_request
                .lock()
                .expect("lock")
                .clone()
                .expect("metadata request should be captured")
                .document()
                .image(),
            None
        );

        let saved = repository
            .saved
            .lock()
            .expect("lock")
            .clone()
            .expect("currency should be saved");
        assert_eq!(saved.uncommitted_events().len(), 1);
        assert_eq!(
            saved.uncommitted_events()[0].payload().name(),
            CurrencyEventPayload::PROVISIONED
        );
        assert!(
            saved
                .mint_account()
                .expect("mint account should be readable")
                .is_some()
        );
    }

    #[tokio::test]
    async fn handle_resolves_object_storage_image_to_public_uri_before_publishing_metadata() {
        let image = CurrencyImageRef::object_name(
            CurrencyImageObjectName::try_from(
                "currencies/00000000-0000-0000-0000-000000000001/images/00000000-0000-0000-0000-000000000002",
            )
            .expect("image object name should be valid"),
        );
        let currency = defined_currency_with_image(Some(image));
        let currency_id = currency.aggregate_id().expect("currency id should exist");
        let repository = TestCurrencyRepository::new(currency);
        let publisher = TestMintMetadataPublisher::new();
        let published_request = publisher.request.clone();
        let creator = TestMintAccountCreator::new(receipt());
        let handler = handler(repository, publisher, creator);
        let mut uow = TestUow;

        handler
            .handle(
                &mut uow,
                &request_context(),
                &CurrencyProvisionCommand { currency_id },
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
    async fn handle_preserves_external_image_uri_when_publishing_metadata() {
        let image = CurrencyImageRef::external_url(
            CurrencyImageUrl::try_from("https://cdn.example.com/currencies/usdc.png")
                .expect("image URL should be valid"),
        );
        let currency = defined_currency_with_image(Some(image));
        let currency_id = currency.aggregate_id().expect("currency id should exist");
        let repository = TestCurrencyRepository::new(currency);
        let publisher = TestMintMetadataPublisher::new();
        let published_request = publisher.request.clone();
        let creator = TestMintAccountCreator::new(receipt());
        let handler = handler(repository, publisher, creator);
        let mut uow = TestUow;

        handler
            .handle(
                &mut uow,
                &request_context(),
                &CurrencyProvisionCommand { currency_id },
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

    #[test]
    fn mint_account_seed_is_derived_from_currency_id() {
        let currency_id = CurrencyId::new();

        let seed = MintAccountSeed::try_from(currency_id).expect("seed should be valid");

        assert_eq!(seed.value(), currency_id.value().as_simple().to_string());
    }
}
