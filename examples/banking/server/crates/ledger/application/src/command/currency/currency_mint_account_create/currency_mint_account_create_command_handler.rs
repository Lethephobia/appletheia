use appletheia::application::authorization::{AuthorizationPlan, PrincipalRequirement};
use appletheia::application::command::{CommandHandled, CommandHandler};
use appletheia::application::repository::Repository;
use appletheia::application::request_context::RequestContext;
use banking_ledger_domain::currency::{
    Currency, CurrencyImageRef, CurrencyMintAccount, CurrencyMintAccountRecordRejectionReason,
    CurrencyMintAccountRecordResult,
};

use super::{
    CurrencyMintAccountCreateCommand, CurrencyMintAccountCreateCommandHandlerConfig,
    CurrencyMintAccountCreateCommandHandlerError, CurrencyMintAccountCreateOutput,
};
use crate::onchain::{
    MintAccountCreateRequest, MintAccountCreator, MintAccountMetadata, MintAccountSeed,
    MintMetadataDescription, MintMetadataDocument, MintMetadataImage, MintMetadataImageObjectName,
    MintMetadataName, MintMetadataPublishRequest, MintMetadataPublisher, MintMetadataSymbol,
    MintMetadataUri,
};

/// Handles `CurrencyMintAccountCreateCommand`.
pub struct CurrencyMintAccountCreateCommandHandler<CR, MMP, MAC>
where
    CR: Repository<Currency>,
    MMP: MintMetadataPublisher,
    MAC: MintAccountCreator,
{
    currency_repository: CR,
    mint_metadata_publisher: MMP,
    mint_account_creator: MAC,
    config: CurrencyMintAccountCreateCommandHandlerConfig,
}

impl<CR, MMP, MAC> CurrencyMintAccountCreateCommandHandler<CR, MMP, MAC>
where
    CR: Repository<Currency>,
    MMP: MintMetadataPublisher,
    MAC: MintAccountCreator,
{
    pub fn new(
        currency_repository: CR,
        mint_metadata_publisher: MMP,
        mint_account_creator: MAC,
        config: CurrencyMintAccountCreateCommandHandlerConfig,
    ) -> Self {
        Self {
            currency_repository,
            mint_metadata_publisher,
            mint_account_creator,
            config,
        }
    }

    fn mint_metadata_image(
        image: Option<&CurrencyImageRef>,
    ) -> Result<Option<MintMetadataImage>, CurrencyMintAccountCreateCommandHandlerError> {
        match image {
            Some(CurrencyImageRef::ObjectName(object_name)) => Ok(Some(
                MintMetadataImage::object_name(MintMetadataImageObjectName::from(object_name)),
            )),
            Some(CurrencyImageRef::ExternalUrl(url)) => Ok(Some(MintMetadataImage::uri(
                MintMetadataUri::try_from(url.value().as_str())?,
            ))),
            None => Ok(None),
        }
    }

    fn mint_metadata_document(
        currency: &Currency,
    ) -> Result<MintMetadataDocument, CurrencyMintAccountCreateCommandHandlerError> {
        let name = MintMetadataName::from(currency.name()?);
        let symbol = MintMetadataSymbol::from(currency.symbol()?);
        let description = currency.description()?.map(MintMetadataDescription::from);
        let image = Self::mint_metadata_image(currency.image()?)?;

        Ok(MintMetadataDocument::new(name, symbol, description, image))
    }
}

impl<CR, MMP, MAC> CommandHandler for CurrencyMintAccountCreateCommandHandler<CR, MMP, MAC>
where
    CR: Repository<Currency>,
    MMP: MintMetadataPublisher,
    MAC: MintAccountCreator,
{
    type Command = CurrencyMintAccountCreateCommand;
    type Output = CurrencyMintAccountCreateOutput;
    type ReplayOutput = CurrencyMintAccountCreateOutput;
    type Error = CurrencyMintAccountCreateCommandHandlerError;
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
            return Err(CurrencyMintAccountCreateCommandHandlerError::CurrencyNotFound);
        };

        if currency.is_removed()? {
            let reason = CurrencyMintAccountRecordRejectionReason::Removed;
            currency.reject_record_mint_account(None, reason)?;
            self.currency_repository
                .save(uow, request_context, &mut currency)
                .await?;
            return Ok(CommandHandled::same(
                CurrencyMintAccountCreateOutput::Rejected { reason },
            ));
        }

        if currency.mint_account()?.is_some() {
            let reason = CurrencyMintAccountRecordRejectionReason::AlreadyRecorded;
            currency.reject_record_mint_account(None, reason)?;
            self.currency_repository
                .save(uow, request_context, &mut currency)
                .await?;
            return Ok(CommandHandled::same(
                CurrencyMintAccountCreateOutput::Rejected { reason },
            ));
        }

        let seed = MintAccountSeed::try_from(command.currency_id)?;
        let document = Self::mint_metadata_document(&currency)?;
        let metadata_name = document.name().clone();
        let metadata_symbol = document.symbol().clone();
        let metadata_uri = self
            .mint_metadata_publisher
            .publish(MintMetadataPublishRequest::new(seed.clone(), document))
            .await?;
        let metadata = MintAccountMetadata::new(metadata_name, metadata_symbol, metadata_uri);
        let receipt = self
            .mint_account_creator
            .create_or_get(MintAccountCreateRequest::new(
                seed,
                currency.decimals()?.value(),
                self.config.token_program_id().clone(),
                self.config.mint_authority().clone(),
                self.config.freeze_authority().cloned(),
                metadata,
            ))
            .await?;
        let mint_account = CurrencyMintAccount::try_from(receipt)?;
        let result = currency.record_mint_account(mint_account)?;

        self.currency_repository
            .save(uow, request_context, &mut currency)
            .await?;

        let output = match result {
            CurrencyMintAccountRecordResult::Recorded { mint_account } => {
                CurrencyMintAccountCreateOutput::Created { mint_account }
            }
            CurrencyMintAccountRecordResult::Rejected { reason } => {
                CurrencyMintAccountCreateOutput::Rejected { reason }
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
        Currency, CurrencyDecimals, CurrencyEventPayload, CurrencyId, CurrencyMintAccount,
        CurrencyMintAccountAddress, CurrencyMintAccountRecordRejectionReason,
        CurrencyMintTokenProgramId, CurrencyName, CurrencyOwner, CurrencySymbol,
    };
    use uuid::Uuid;

    use super::{
        CurrencyMintAccountCreateCommand, CurrencyMintAccountCreateCommandHandler,
        CurrencyMintAccountCreateCommandHandlerConfig, CurrencyMintAccountCreateOutput,
    };
    use crate::onchain::{
        MintAccountAddress, MintAccountCreateReceipt, MintAccountCreateRequest, MintAccountCreator,
        MintAccountCreatorError, MintAccountSeed, MintMetadataPublishRequest,
        MintMetadataPublisher, MintMetadataPublisherError, MintMetadataUri, OnchainAccountAddress,
        TokenProgramId,
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
    }

    impl TestMintMetadataPublisher {
        fn new() -> Self {
            Self {
                calls: Arc::new(Mutex::new(0)),
            }
        }
    }

    impl MintMetadataPublisher for TestMintMetadataPublisher {
        async fn publish(
            &self,
            _request: MintMetadataPublishRequest,
        ) -> Result<MintMetadataUri, MintMetadataPublisherError> {
            *self.calls.lock().expect("lock") += 1;
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
            CurrencyMintTokenProgramId::try_from(TOKEN_PROGRAM_ID)
                .expect("token program ID should be valid"),
        )
    }

    fn receipt() -> MintAccountCreateReceipt {
        MintAccountCreateReceipt::new(
            MintAccountAddress::try_from("Mint111111111111111111111111111111111111")
                .expect("mint account address should be valid"),
            TokenProgramId::try_from(TOKEN_PROGRAM_ID).expect("token program ID should be valid"),
        )
    }

    fn config() -> CurrencyMintAccountCreateCommandHandlerConfig {
        CurrencyMintAccountCreateCommandHandlerConfig::new(
            TokenProgramId::try_from(TOKEN_PROGRAM_ID).expect("token program ID should be valid"),
            OnchainAccountAddress::try_from("Authority111111111111111111111111111111")
                .expect("authority address should be valid"),
            None,
        )
    }

    fn handler(
        repository: TestCurrencyRepository,
        publisher: TestMintMetadataPublisher,
        creator: TestMintAccountCreator,
    ) -> CurrencyMintAccountCreateCommandHandler<
        TestCurrencyRepository,
        TestMintMetadataPublisher,
        TestMintAccountCreator,
    > {
        CurrencyMintAccountCreateCommandHandler::new(repository, publisher, creator, config())
    }

    #[tokio::test]
    async fn handle_rejects_already_recorded_without_external_side_effects() {
        let mut currency = defined_currency();
        let currency_id = currency.aggregate_id().expect("currency id should exist");
        currency
            .record_mint_account(mint_account())
            .expect("mint account should be recorded");
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
                &CurrencyMintAccountCreateCommand { currency_id },
            )
            .await
            .expect("command should be handled");

        assert_eq!(
            handled.into_output(),
            CurrencyMintAccountCreateOutput::Rejected {
                reason: CurrencyMintAccountRecordRejectionReason::AlreadyRecorded
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
            &CurrencyEventPayload::MintAccountRecordRejected {
                mint_account: None,
                reason: CurrencyMintAccountRecordRejectionReason::AlreadyRecorded,
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
                &CurrencyMintAccountCreateCommand { currency_id },
            )
            .await
            .expect("command should be handled");

        assert_eq!(
            handled.into_output(),
            CurrencyMintAccountCreateOutput::Rejected {
                reason: CurrencyMintAccountRecordRejectionReason::Removed
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
            &CurrencyEventPayload::MintAccountRecordRejected {
                mint_account: None,
                reason: CurrencyMintAccountRecordRejectionReason::Removed,
            }
        );
    }

    #[tokio::test]
    async fn handle_publishes_metadata_creates_mint_and_records_receipt() {
        let currency = defined_currency();
        let currency_id = currency.aggregate_id().expect("currency id should exist");
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
                &CurrencyMintAccountCreateCommand { currency_id },
            )
            .await
            .expect("command should be handled");

        assert!(matches!(
            handled.into_output(),
            CurrencyMintAccountCreateOutput::Created { .. }
        ));
        assert_eq!(*publisher_calls.lock().expect("lock"), 1);
        assert_eq!(*creator_calls.lock().expect("lock"), 1);

        let saved = repository
            .saved
            .lock()
            .expect("lock")
            .clone()
            .expect("currency should be saved");
        assert_eq!(saved.uncommitted_events().len(), 1);
        assert_eq!(
            saved.uncommitted_events()[0].payload().name(),
            CurrencyEventPayload::MINT_ACCOUNT_RECORDED
        );
        assert!(
            saved
                .mint_account()
                .expect("mint account should be readable")
                .is_some()
        );
    }

    #[test]
    fn mint_account_seed_is_derived_from_currency_id() {
        let currency_id = CurrencyId::new();

        let seed = MintAccountSeed::try_from(currency_id).expect("seed should be valid");

        assert_eq!(seed.value(), currency_id.value().as_simple().to_string());
    }
}
