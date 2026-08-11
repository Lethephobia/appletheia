use appletheia::application::authorization::{
    AuthorizationPlan, PrincipalRequirement, Relation, RelationshipRequirement,
};
use appletheia::application::command::CommandHandler;
use appletheia::application::repository::Repository;
use appletheia::application::request_context::RequestContext;
use appletheia::domain::Aggregate;
use banking_ledger_domain::account::Account;
use banking_ledger_domain::currency::{Currency, CurrencyStatus};
use banking_ledger_domain::withdrawal::{
    Withdrawal, WithdrawalRequest, WithdrawalRequestRejectionReason, WithdrawalRequestResult,
};

use crate::authorization::AccountWithdrawalRequesterRelation;
use crate::mint::{
    TokenAccountOwnerAddressValidationResult, TokenAccountOwnerAddressValidator,
    TokenAccountOwnerAddressValidatorError,
};

use super::{
    WithdrawalRequestCommand, WithdrawalRequestCommandHandlerError, WithdrawalRequestOutput,
};

/// Handles `WithdrawalRequestCommand`.
pub struct WithdrawalRequestCommandHandler<AR, CR, TAOV, WR>
where
    AR: Repository<Account, Uow = WR::Uow>,
    CR: Repository<Currency, Uow = WR::Uow>,
    TAOV: TokenAccountOwnerAddressValidator,
    WR: Repository<Withdrawal>,
{
    account_repository: AR,
    currency_repository: CR,
    token_account_owner_address_validator: TAOV,
    withdrawal_repository: WR,
}

impl<AR, CR, TAOV, WR> WithdrawalRequestCommandHandler<AR, CR, TAOV, WR>
where
    AR: Repository<Account, Uow = WR::Uow>,
    CR: Repository<Currency, Uow = WR::Uow>,
    TAOV: TokenAccountOwnerAddressValidator,
    WR: Repository<Withdrawal>,
{
    pub fn new(
        account_repository: AR,
        currency_repository: CR,
        token_account_owner_address_validator: TAOV,
        withdrawal_repository: WR,
    ) -> Self {
        Self {
            account_repository,
            currency_repository,
            token_account_owner_address_validator,
            withdrawal_repository,
        }
    }
}

impl<AR, CR, TAOV, WR> CommandHandler for WithdrawalRequestCommandHandler<AR, CR, TAOV, WR>
where
    AR: Repository<Account, Uow = WR::Uow>,
    CR: Repository<Currency, Uow = WR::Uow>,
    TAOV: TokenAccountOwnerAddressValidator,
    WR: Repository<Withdrawal>,
{
    type Command = WithdrawalRequestCommand;
    type Output = WithdrawalRequestOutput;
    type Error = WithdrawalRequestCommandHandlerError;
    type Uow = WR::Uow;

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

        match self
            .token_account_owner_address_validator
            .validate(&command.token_account_owner_address)
            .await
        {
            Ok(TokenAccountOwnerAddressValidationResult::Valid) => {}
            Ok(TokenAccountOwnerAddressValidationResult::Invalid) => {
                let reason = WithdrawalRequestRejectionReason::InvalidTokenAccountOwnerAddress;
                let output = WithdrawalRequestOutput::Rejected {
                    withdrawal_id,
                    reason,
                };
                return Ok(output);
            }
            Err(error @ TokenAccountOwnerAddressValidatorError::Backend(_)) => {
                return Err(error.into());
            }
        }

        let account = self
            .account_repository
            .read(uow, command.account_id)
            .await?;
        let account_id = command.account_id;
        let currency_id = *account.currency_id()?;
        let token_account_owner_address = command.token_account_owner_address.clone();
        let amount = command.amount;
        let request = WithdrawalRequest {
            account_id,
            currency_id,
            token_account_owner_address,
            amount,
        };

        let currency = self.currency_repository.read(uow, currency_id).await?;

        if matches!(
            currency.status()?,
            CurrencyStatus::Provisioning | CurrencyStatus::ProvisioningFailed
        ) {
            let reason = WithdrawalRequestRejectionReason::CurrencyUnprovisioned;
            withdrawal.reject_request(request, reason)?;
            let output = WithdrawalRequestOutput::Rejected {
                withdrawal_id,
                reason,
            };
            self.withdrawal_repository
                .save(uow, request_context, &mut withdrawal)
                .await?;
            return Ok(output);
        }

        if !currency.is_active()? {
            let reason = WithdrawalRequestRejectionReason::CurrencyInactive;
            withdrawal.reject_request(request, reason)?;
            let output = WithdrawalRequestOutput::Rejected {
                withdrawal_id,
                reason,
            };
            self.withdrawal_repository
                .save(uow, request_context, &mut withdrawal)
                .await?;
            return Ok(output);
        }

        let result = withdrawal.request(request)?;

        self.withdrawal_repository
            .save(uow, request_context, &mut withdrawal)
            .await?;

        let output = match result {
            WithdrawalRequestResult::Requested => {
                WithdrawalRequestOutput::Requested { withdrawal_id }
            }
            WithdrawalRequestResult::Rejected { reason } => WithdrawalRequestOutput::Rejected {
                withdrawal_id,
                reason,
            },
        };

        Ok(output)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::sync::{Arc, Mutex};

    use appletheia::application::authorization::{
        AggregateRef, AuthorizationPlan, PrincipalRequirement, Relation, RelationshipRequirement,
    };
    use appletheia::application::command::CommandHandler;
    use appletheia::application::repository::{Repository, RepositoryError};
    use appletheia::application::request_context::{
        CorrelationId, MessageId, Principal, RequestContext,
    };
    use appletheia::application::unit_of_work::{UnitOfWork, UnitOfWorkError};
    use appletheia::domain::{Aggregate, AggregateVersion, UniqueKey, UniqueValue};
    use banking_iam_domain::{User, UserId};
    use banking_ledger_domain::account::{
        Account, AccountId, AccountName, AccountOpening, AccountOwner,
    };
    use banking_ledger_domain::core::{CurrencyAmount, TokenAccountOwnerAddress};
    use banking_ledger_domain::currency::{
        Currency, CurrencyDecimals, CurrencyDefinition, CurrencyId, CurrencyName, CurrencyOwner,
        CurrencySymbol, MintAccount, MintAccountAddress, PoolTokenAccountAddress,
    };
    use banking_ledger_domain::withdrawal::{
        Withdrawal, WithdrawalId, WithdrawalRequestRejectionReason, WithdrawalStatus,
    };
    use uuid::Uuid;

    use crate::authorization::AccountWithdrawalRequesterRelation;
    use crate::mint::{
        TokenAccountOwnerAddressValidationResult, TokenAccountOwnerAddressValidator,
        TokenAccountOwnerAddressValidatorError,
    };

    use super::{
        WithdrawalRequestCommand, WithdrawalRequestCommandHandler,
        WithdrawalRequestCommandHandlerError, WithdrawalRequestOutput,
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

    #[derive(Clone, Copy)]
    struct PassingValidator;

    impl TokenAccountOwnerAddressValidator for PassingValidator {
        async fn validate(
            &self,
            _address: &TokenAccountOwnerAddress,
        ) -> Result<TokenAccountOwnerAddressValidationResult, TokenAccountOwnerAddressValidatorError>
        {
            Ok(TokenAccountOwnerAddressValidationResult::Valid)
        }
    }

    #[derive(Clone, Copy)]
    struct RejectingValidator;

    impl TokenAccountOwnerAddressValidator for RejectingValidator {
        async fn validate(
            &self,
            _address: &TokenAccountOwnerAddress,
        ) -> Result<TokenAccountOwnerAddressValidationResult, TokenAccountOwnerAddressValidatorError>
        {
            Ok(TokenAccountOwnerAddressValidationResult::Invalid)
        }
    }

    #[derive(Clone, Default)]
    struct TestAccountRepository {
        items: Arc<Mutex<HashMap<AccountId, Account>>>,
    }

    impl TestAccountRepository {
        fn insert(&self, aggregate: Account) {
            let id = aggregate.aggregate_id();
            self.items.lock().expect("lock").insert(id, aggregate);
        }
    }

    impl Repository<Account> for TestAccountRepository {
        type Uow = TestUow;

        async fn read(
            &self,
            _uow: &mut Self::Uow,
            id: AccountId,
        ) -> Result<Account, RepositoryError<Account>> {
            self.items
                .lock()
                .expect("lock")
                .get(&id)
                .cloned()
                .ok_or_else(|| RepositoryError::NotFound {
                    aggregate_type: Account::TYPE,
                    aggregate_id: id,
                })
        }

        async fn read_at_version(
            &self,
            _uow: &mut Self::Uow,
            id: AccountId,
            _at: AggregateVersion,
        ) -> Result<Account, RepositoryError<Account>> {
            self.items
                .lock()
                .expect("lock")
                .get(&id)
                .cloned()
                .ok_or_else(|| RepositoryError::NotFound {
                    aggregate_type: Account::TYPE,
                    aggregate_id: id,
                })
        }

        async fn find_by_unique_value(
            &self,
            _uow: &mut Self::Uow,
            _unique_key: UniqueKey,
            _unique_value: &UniqueValue,
        ) -> Result<Option<Account>, RepositoryError<Account>> {
            Ok(None)
        }

        async fn save(
            &self,
            _uow: &mut Self::Uow,
            _request_context: &RequestContext,
            aggregate: &mut Account,
        ) -> Result<(), RepositoryError<Account>> {
            let id = aggregate.aggregate_id();
            self.items
                .lock()
                .expect("lock")
                .insert(id, aggregate.clone());
            Ok(())
        }
    }

    #[derive(Clone, Default)]
    struct TestCurrencyRepository {
        items: Arc<Mutex<HashMap<CurrencyId, Currency>>>,
    }

    impl TestCurrencyRepository {
        fn insert(&self, aggregate: Currency) {
            let id = aggregate.aggregate_id();
            self.items.lock().expect("lock").insert(id, aggregate);
        }
    }

    impl Repository<Currency> for TestCurrencyRepository {
        type Uow = TestUow;

        async fn read(
            &self,
            _uow: &mut Self::Uow,
            id: CurrencyId,
        ) -> Result<Currency, RepositoryError<Currency>> {
            self.items
                .lock()
                .expect("lock")
                .get(&id)
                .cloned()
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
            self.items
                .lock()
                .expect("lock")
                .get(&id)
                .cloned()
                .ok_or_else(|| RepositoryError::NotFound {
                    aggregate_type: Currency::TYPE,
                    aggregate_id: id,
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
            let id = aggregate.aggregate_id();
            self.items
                .lock()
                .expect("lock")
                .insert(id, aggregate.clone());
            Ok(())
        }
    }

    #[derive(Clone, Default)]
    struct TestWithdrawalRepository {
        saved: Arc<Mutex<Option<Withdrawal>>>,
    }

    impl Repository<Withdrawal> for TestWithdrawalRepository {
        type Uow = TestUow;

        async fn read(
            &self,
            _uow: &mut Self::Uow,
            _id: WithdrawalId,
        ) -> Result<Withdrawal, RepositoryError<Withdrawal>> {
            self.saved
                .lock()
                .expect("lock")
                .clone()
                .ok_or_else(|| RepositoryError::NotFound {
                    aggregate_type: Withdrawal::TYPE,
                    aggregate_id: _id,
                })
        }

        async fn read_at_version(
            &self,
            _uow: &mut Self::Uow,
            _id: WithdrawalId,
            _at: AggregateVersion,
        ) -> Result<Withdrawal, RepositoryError<Withdrawal>> {
            self.saved
                .lock()
                .expect("lock")
                .clone()
                .ok_or_else(|| RepositoryError::NotFound {
                    aggregate_type: Withdrawal::TYPE,
                    aggregate_id: _id,
                })
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
            *self.saved.lock().expect("lock") = Some(aggregate.clone());
            Ok(())
        }
    }

    fn request_context(user_id: UserId) -> RequestContext {
        let subject = AggregateRef::from_id::<User>(user_id);
        RequestContext::new(
            CorrelationId::from(Uuid::now_v7()),
            MessageId::new(),
            Principal::Authenticated { subject },
        )
        .expect("request context should be valid")
    }

    fn account_name() -> AccountName {
        AccountName::try_from("main").expect("account name should be valid")
    }

    fn token_account_owner_address() -> TokenAccountOwnerAddress {
        TokenAccountOwnerAddress::try_from("11111111111111111111111111111111")
            .expect("wallet bookmark address should be valid")
    }

    fn mint_account() -> MintAccount {
        MintAccount::new(
            MintAccountAddress::try_from("Mint111111111111111111111111111111111111")
                .expect("mint account address should be valid"),
            PoolTokenAccountAddress::try_from("Pool111111111111111111111111111111111111")
                .expect("pool token account address should be valid"),
        )
    }

    fn opened_account(owner: AccountOwner, currency_id: CurrencyId) -> Account {
        let mut account = Account::new();
        account
            .open(AccountOpening {
                owner,
                name: account_name(),
                currency_id,
            })
            .expect("account should open");
        account.core_mut().clear_uncommitted_events();
        account
    }

    fn provisioned_currency(owner: CurrencyOwner) -> Currency {
        let mut currency = Currency::new();
        currency
            .define(CurrencyDefinition {
                owner,
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

    #[test]
    fn authorization_plan_requires_withdrawal_requester_relationship() {
        let handler = WithdrawalRequestCommandHandler::new(
            TestAccountRepository::default(),
            TestCurrencyRepository::default(),
            PassingValidator,
            TestWithdrawalRepository::default(),
        );
        let command = WithdrawalRequestCommand {
            account_id: AccountId::new(),
            token_account_owner_address: token_account_owner_address(),
            amount: CurrencyAmount::new(10),
        };

        let plan = handler
            .authorization_plan(&command)
            .expect("authorization plan should build");

        assert_eq!(
            plan,
            AuthorizationPlan::OnlyPrincipals(vec![
                PrincipalRequirement::AuthenticatedWithRelationship(
                    RelationshipRequirement::check::<Account>(
                        command.account_id,
                        AccountWithdrawalRequesterRelation::REF,
                    ),
                ),
            ])
        );
    }

    #[tokio::test]
    async fn handle_requests_withdrawal_with_token_account_owner_address() {
        let user_id = UserId::new();
        let account_owner = AccountOwner::from(user_id);
        let currency_owner = CurrencyOwner::from(user_id);
        let token_account_owner_address = token_account_owner_address();
        let currency = provisioned_currency(currency_owner);
        let currency_id = currency.aggregate_id();
        let account = opened_account(account_owner, currency_id);
        let account_id = account.aggregate_id();
        let account_repository = TestAccountRepository::default();
        account_repository.insert(account);
        let currency_repository = TestCurrencyRepository::default();
        currency_repository.insert(currency);
        let withdrawal_repository = TestWithdrawalRepository::default();
        let handler = WithdrawalRequestCommandHandler::new(
            account_repository,
            currency_repository,
            PassingValidator,
            withdrawal_repository.clone(),
        );
        let mut uow = TestUow;

        let handled = handler
            .handle(
                &mut uow,
                &request_context(user_id),
                &WithdrawalRequestCommand {
                    account_id,
                    token_account_owner_address: token_account_owner_address.clone(),
                    amount: CurrencyAmount::new(10),
                },
            )
            .await
            .expect("request should succeed");

        let output = handled;
        let WithdrawalRequestOutput::Requested { withdrawal_id } = output else {
            panic!("expected requested output");
        };
        let saved = withdrawal_repository
            .saved
            .lock()
            .expect("lock")
            .clone()
            .expect("withdrawal should be saved");

        assert_eq!(saved.aggregate_id(), withdrawal_id);
        assert_eq!(
            saved.account_id().expect("account id should exist"),
            &account_id
        );
        assert_eq!(
            saved
                .token_account_owner_address()
                .expect("token account owner address should exist"),
            &token_account_owner_address
        );
        assert_eq!(
            saved.status().expect("status should exist"),
            &WithdrawalStatus::Pending
        );
    }

    #[tokio::test]
    async fn handle_rejects_invalid_address_without_saving_withdrawal() {
        let user_id = UserId::new();
        let currency_id = CurrencyId::new();
        let account = opened_account(AccountOwner::from(user_id), currency_id);
        let account_id = account.aggregate_id();
        let account_repository = TestAccountRepository::default();
        account_repository.insert(account);
        let withdrawal_repository = TestWithdrawalRepository::default();
        let handler = WithdrawalRequestCommandHandler::new(
            account_repository,
            TestCurrencyRepository::default(),
            RejectingValidator,
            withdrawal_repository.clone(),
        );

        let handled = handler
            .handle(
                &mut TestUow,
                &request_context(user_id),
                &WithdrawalRequestCommand {
                    account_id,
                    token_account_owner_address: token_account_owner_address(),
                    amount: CurrencyAmount::new(10),
                },
            )
            .await
            .expect("invalid address should be handled as a rejection");

        let output = handled;
        let WithdrawalRequestOutput::Rejected { reason, .. } = output else {
            panic!("expected rejected output");
        };
        assert_eq!(
            reason,
            WithdrawalRequestRejectionReason::InvalidTokenAccountOwnerAddress
        );
        assert!(withdrawal_repository.saved.lock().expect("lock").is_none());
    }

    #[tokio::test]
    async fn handle_errors_when_account_is_missing() {
        let handler = WithdrawalRequestCommandHandler::new(
            TestAccountRepository::default(),
            TestCurrencyRepository::default(),
            PassingValidator,
            TestWithdrawalRepository::default(),
        );
        let mut uow = TestUow;
        let error = handler
            .handle(
                &mut uow,
                &request_context(UserId::new()),
                &WithdrawalRequestCommand {
                    account_id: AccountId::new(),
                    token_account_owner_address: token_account_owner_address(),
                    amount: CurrencyAmount::new(10),
                },
            )
            .await
            .expect_err("missing account should error");

        assert!(matches!(
            error,
            WithdrawalRequestCommandHandlerError::AccountRepository(
                RepositoryError::NotFound { .. }
            )
        ));
    }
}
