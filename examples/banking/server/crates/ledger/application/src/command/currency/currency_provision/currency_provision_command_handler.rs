use crate::mint::{MintProvisionRequest, MintProvisioner};
use appletheia::application::authorization::{AuthorizationPlan, PrincipalRequirement};
use appletheia::application::command::{CommandHandled, CommandHandler};
use appletheia::application::repository::Repository;
use appletheia::application::request_context::RequestContext;
use banking_ledger_domain::currency::{
    Currency, CurrencyProvisionRejectionReason, CurrencyProvisionResult,
};

use super::{
    CurrencyProvisionCommand, CurrencyProvisionCommandHandlerError, CurrencyProvisionOutput,
};

/// Handles `CurrencyProvisionCommand`.
pub struct CurrencyProvisionCommandHandler<CR, MP>
where
    CR: Repository<Currency>,
    MP: MintProvisioner,
{
    currency_repository: CR,
    mint_provisioner: MP,
}

impl<CR, MP> CurrencyProvisionCommandHandler<CR, MP>
where
    CR: Repository<Currency>,
    MP: MintProvisioner,
{
    pub fn new(currency_repository: CR, mint_provisioner: MP) -> Self {
        Self {
            currency_repository,
            mint_provisioner,
        }
    }
}

impl<CR, MP> CommandHandler for CurrencyProvisionCommandHandler<CR, MP>
where
    CR: Repository<Currency>,
    MP: MintProvisioner,
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
        let mut currency = self
            .currency_repository
            .read(uow, command.currency_id)
            .await?;

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

        let receipt = self
            .mint_provisioner
            .provision(MintProvisionRequest::new(
                command.currency_id,
                *currency.decimals()?,
                currency.name()?.clone(),
                currency.symbol()?.clone(),
                currency.description()?.cloned(),
                currency.image()?.cloned(),
            ))
            .await?;
        let result = currency.provision(receipt.into_mint_account())?;

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
    use appletheia::domain::{Aggregate, AggregateVersion, EventPayload, UniqueKey, UniqueValue};
    use banking_iam_domain::UserId;
    use banking_ledger_domain::currency::{
        Currency, CurrencyDecimals, CurrencyEventPayload, CurrencyId, CurrencyImageObjectName,
        CurrencyImageRef, CurrencyImageUrl, CurrencyName, CurrencyOwner,
        CurrencyProvisionRejectionReason, CurrencySymbol, MintAccount, MintAccountAddress,
        PoolTokenAccountAddress,
    };
    use uuid::Uuid;

    use super::{
        CurrencyProvisionCommand, CurrencyProvisionCommandHandler, CurrencyProvisionOutput,
    };
    use crate::mint::{
        MintProvisionReceipt, MintProvisionRequest, MintProvisioner, MintProvisionerError,
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
            *self.currency.lock().expect("lock") = Some(aggregate.clone());
            Ok(())
        }
    }

    #[derive(Clone)]
    struct TestMintProvisioner {
        calls: Arc<Mutex<usize>>,
        request: Arc<Mutex<Option<MintProvisionRequest>>>,
        receipt: MintProvisionReceipt,
    }

    impl TestMintProvisioner {
        fn new(receipt: MintProvisionReceipt) -> Self {
            Self {
                calls: Arc::new(Mutex::new(0)),
                request: Arc::new(Mutex::new(None)),
                receipt,
            }
        }
    }

    impl MintProvisioner for TestMintProvisioner {
        async fn provision(
            &self,
            request: MintProvisionRequest,
        ) -> Result<MintProvisionReceipt, MintProvisionerError> {
            *self.calls.lock().expect("lock") += 1;
            *self.request.lock().expect("lock") = Some(request);
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

    fn mint_account() -> MintAccount {
        MintAccount::new(
            MintAccountAddress::try_from("Mint111111111111111111111111111111111111")
                .expect("mint account address should be valid"),
            PoolTokenAccountAddress::try_from("Pool111111111111111111111111111111111111")
                .expect("pool account address should be valid"),
        )
    }

    fn receipt() -> MintProvisionReceipt {
        MintProvisionReceipt::new(mint_account())
    }

    fn handler(
        repository: TestCurrencyRepository,
        provisioner: TestMintProvisioner,
    ) -> CurrencyProvisionCommandHandler<TestCurrencyRepository, TestMintProvisioner> {
        CurrencyProvisionCommandHandler::new(repository, provisioner)
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
        let provisioner = TestMintProvisioner::new(receipt());
        let provisioner_calls = provisioner.calls.clone();
        let handler = handler(repository.clone(), provisioner);
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
        assert_eq!(*provisioner_calls.lock().expect("lock"), 0);
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
        let provisioner = TestMintProvisioner::new(receipt());
        let provisioner_calls = provisioner.calls.clone();
        let handler = handler(repository.clone(), provisioner);
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
        assert_eq!(*provisioner_calls.lock().expect("lock"), 0);

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
    async fn handle_provisions_mint_for_currency() {
        let currency = defined_currency();
        let currency_id = currency.aggregate_id().expect("currency id should exist");
        let repository = TestCurrencyRepository::new(currency);
        let provisioner = TestMintProvisioner::new(receipt());
        let provisioner_calls = provisioner.calls.clone();
        let provision_request = provisioner.request.clone();
        let handler = handler(repository.clone(), provisioner);
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
        assert_eq!(*provisioner_calls.lock().expect("lock"), 1);
        assert_eq!(
            provision_request
                .lock()
                .expect("lock")
                .clone()
                .expect("mint provision request should be captured")
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
    async fn handle_passes_object_storage_image_to_mint_provisioner() {
        let image = CurrencyImageRef::object_name(
            CurrencyImageObjectName::try_from(
                "currencies/00000000-0000-0000-0000-000000000001/images/00000000-0000-0000-0000-000000000002",
            )
            .expect("image object name should be valid"),
        );
        let currency = defined_currency_with_image(Some(image));
        let currency_id = currency.aggregate_id().expect("currency id should exist");
        let repository = TestCurrencyRepository::new(currency);
        let provisioner = TestMintProvisioner::new(receipt());
        let provision_request = provisioner.request.clone();
        let handler = handler(repository, provisioner);
        let mut uow = TestUow;

        handler
            .handle(
                &mut uow,
                &request_context(),
                &CurrencyProvisionCommand { currency_id },
            )
            .await
            .expect("command should be handled");

        let request = provision_request
            .lock()
            .expect("lock")
            .clone()
            .expect("mint provision request should be captured");
        assert_eq!(
            request.image().and_then(CurrencyImageRef::as_object_name),
            Some(
                &CurrencyImageObjectName::try_from(
                    "currencies/00000000-0000-0000-0000-000000000001/images/00000000-0000-0000-0000-000000000002",
                )
                .expect("image object name should be valid")
            )
        );
    }

    #[tokio::test]
    async fn handle_passes_external_image_to_mint_provisioner() {
        let image = CurrencyImageRef::external_url(
            CurrencyImageUrl::try_from("https://cdn.example.com/currencies/usdc.png")
                .expect("image URL should be valid"),
        );
        let currency = defined_currency_with_image(Some(image));
        let currency_id = currency.aggregate_id().expect("currency id should exist");
        let repository = TestCurrencyRepository::new(currency);
        let provisioner = TestMintProvisioner::new(receipt());
        let provision_request = provisioner.request.clone();
        let handler = handler(repository, provisioner);
        let mut uow = TestUow;

        handler
            .handle(
                &mut uow,
                &request_context(),
                &CurrencyProvisionCommand { currency_id },
            )
            .await
            .expect("command should be handled");

        let request = provision_request
            .lock()
            .expect("lock")
            .clone()
            .expect("mint provision request should be captured");
        assert_eq!(
            request
                .image()
                .and_then(CurrencyImageRef::as_external_url)
                .map(|value| value.value().as_str()),
            Some("https://cdn.example.com/currencies/usdc.png")
        );
    }
}
