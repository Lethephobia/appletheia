use appletheia::application::authorization::{
    AuthorizationPlan, PrincipalRequirement, Relation, RelationshipRequirement,
};
use appletheia::application::command::{CommandHandled, CommandHandler};
use appletheia::application::repository::Repository;
use appletheia::application::request_context::RequestContext;
use banking_ledger_domain::account::Account;
use banking_ledger_domain::currency::Currency;
use banking_ledger_domain::payout_destination::{
    PayoutDestination, PayoutDestinationOwner, PayoutDestinationStatus,
};
use banking_ledger_domain::withdrawal::{
    Withdrawal, WithdrawalRequestRejectionReason, WithdrawalRequestResult,
};

use crate::authorization::AccountWithdrawalRequesterRelation;

use super::{
    WithdrawalRequestCommand, WithdrawalRequestCommandHandlerError, WithdrawalRequestOutput,
};

/// Handles `WithdrawalRequestCommand`.
pub struct WithdrawalRequestCommandHandler<AR, CR, PDR, WR>
where
    AR: Repository<Account, Uow = WR::Uow>,
    CR: Repository<Currency, Uow = WR::Uow>,
    PDR: Repository<PayoutDestination, Uow = WR::Uow>,
    WR: Repository<Withdrawal>,
{
    account_repository: AR,
    currency_repository: CR,
    payout_destination_repository: PDR,
    withdrawal_repository: WR,
}

impl<AR, CR, PDR, WR> WithdrawalRequestCommandHandler<AR, CR, PDR, WR>
where
    AR: Repository<Account, Uow = WR::Uow>,
    CR: Repository<Currency, Uow = WR::Uow>,
    PDR: Repository<PayoutDestination, Uow = WR::Uow>,
    WR: Repository<Withdrawal>,
{
    pub fn new(
        account_repository: AR,
        currency_repository: CR,
        payout_destination_repository: PDR,
        withdrawal_repository: WR,
    ) -> Self {
        Self {
            account_repository,
            currency_repository,
            payout_destination_repository,
            withdrawal_repository,
        }
    }
}

impl<AR, CR, PDR, WR> CommandHandler for WithdrawalRequestCommandHandler<AR, CR, PDR, WR>
where
    AR: Repository<Account, Uow = WR::Uow>,
    CR: Repository<Currency, Uow = WR::Uow>,
    PDR: Repository<PayoutDestination, Uow = WR::Uow>,
    WR: Repository<Withdrawal>,
{
    type Command = WithdrawalRequestCommand;
    type Output = WithdrawalRequestOutput;
    type ReplayOutput = WithdrawalRequestOutput;
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
    ) -> Result<CommandHandled<Self::Output, Self::ReplayOutput>, Self::Error> {
        let Some(account) = self
            .account_repository
            .find(uow, command.account_id)
            .await?
        else {
            return Err(WithdrawalRequestCommandHandlerError::AccountNotFound);
        };
        let Some(payout_destination) = self
            .payout_destination_repository
            .find(uow, command.payout_destination_id)
            .await?
        else {
            return Err(WithdrawalRequestCommandHandlerError::PayoutDestinationNotFound);
        };
        let Some(currency) = self
            .currency_repository
            .find(uow, *account.currency_id()?)
            .await?
        else {
            return Err(WithdrawalRequestCommandHandlerError::CurrencyNotFound);
        };

        let mut withdrawal = Withdrawal::default();
        let account_id = command.account_id;
        let currency_id = *account.currency_id()?;
        let payout_destination_id = command.payout_destination_id;
        let amount = command.amount;

        if PayoutDestinationOwner::from(account.owner()?) != *payout_destination.owner()? {
            let reason = WithdrawalRequestRejectionReason::PayoutDestinationOwnerMismatch;
            let output = WithdrawalRequestOutput::Rejected {
                withdrawal_id: withdrawal.reject_request(
                    account_id,
                    currency_id,
                    payout_destination_id,
                    amount,
                    reason,
                )?,
                reason,
            };
            self.withdrawal_repository
                .save(uow, request_context, &mut withdrawal)
                .await?;
            return Ok(CommandHandled::same(output));
        }

        if payout_destination.status()? != &PayoutDestinationStatus::Active {
            let reason = WithdrawalRequestRejectionReason::PayoutDestinationRemoved;
            let output = WithdrawalRequestOutput::Rejected {
                withdrawal_id: withdrawal.reject_request(
                    account_id,
                    currency_id,
                    payout_destination_id,
                    amount,
                    reason,
                )?,
                reason,
            };
            self.withdrawal_repository
                .save(uow, request_context, &mut withdrawal)
                .await?;
            return Ok(CommandHandled::same(output));
        }

        if !currency.is_active()? {
            let reason = WithdrawalRequestRejectionReason::CurrencyInactive;
            let output = WithdrawalRequestOutput::Rejected {
                withdrawal_id: withdrawal.reject_request(
                    account_id,
                    currency_id,
                    payout_destination_id,
                    amount,
                    reason,
                )?,
                reason,
            };
            self.withdrawal_repository
                .save(uow, request_context, &mut withdrawal)
                .await?;
            return Ok(CommandHandled::same(output));
        }

        if currency.mint_account()?.is_none() {
            let reason = WithdrawalRequestRejectionReason::CurrencyUnprovisioned;
            let output = WithdrawalRequestOutput::Rejected {
                withdrawal_id: withdrawal.reject_request(
                    account_id,
                    currency_id,
                    payout_destination_id,
                    amount,
                    reason,
                )?,
                reason,
            };
            self.withdrawal_repository
                .save(uow, request_context, &mut withdrawal)
                .await?;
            return Ok(CommandHandled::same(output));
        }

        let result = withdrawal.request(account_id, currency_id, payout_destination_id, amount)?;

        self.withdrawal_repository
            .save(uow, request_context, &mut withdrawal)
            .await?;

        let output = match result {
            WithdrawalRequestResult::Requested { withdrawal_id } => {
                WithdrawalRequestOutput::Requested { withdrawal_id }
            }
            WithdrawalRequestResult::Rejected {
                withdrawal_id,
                reason,
            } => WithdrawalRequestOutput::Rejected {
                withdrawal_id,
                reason,
            },
        };

        Ok(CommandHandled::same(output))
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
    use banking_ledger_domain::account::{Account, AccountId, AccountName, AccountOwner};
    use banking_ledger_domain::core::CurrencyAmount;
    use banking_ledger_domain::currency::{
        Currency, CurrencyDecimals, CurrencyId, CurrencyMintAccount, CurrencyMintAccountAddress,
        CurrencyName, CurrencyOwner, CurrencyPoolTokenAccountAddress, CurrencySymbol,
        CurrencyTokenProgramId,
    };
    use banking_ledger_domain::payout_destination::{
        PayoutDestination, PayoutDestinationId, PayoutDestinationOwner,
        PayoutDestinationTokenAccountOwnerAddress,
    };
    use banking_ledger_domain::withdrawal::{
        Withdrawal, WithdrawalId, WithdrawalRequestRejectionReason, WithdrawalStatus,
    };
    use uuid::Uuid;

    use crate::authorization::AccountWithdrawalRequesterRelation;

    use super::{
        WithdrawalRequestCommand, WithdrawalRequestCommandHandler,
        WithdrawalRequestCommandHandlerError, WithdrawalRequestOutput,
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

    #[derive(Clone, Default)]
    struct TestAccountRepository {
        items: Arc<Mutex<HashMap<AccountId, Account>>>,
    }

    impl TestAccountRepository {
        fn insert(&self, aggregate: Account) {
            let id = aggregate.aggregate_id().expect("account id should exist");
            self.items.lock().expect("lock").insert(id, aggregate);
        }
    }

    impl Repository<Account> for TestAccountRepository {
        type Uow = TestUow;

        async fn find(
            &self,
            _uow: &mut Self::Uow,
            id: AccountId,
        ) -> Result<Option<Account>, RepositoryError<Account>> {
            Ok(self.items.lock().expect("lock").get(&id).cloned())
        }

        async fn find_at_version(
            &self,
            _uow: &mut Self::Uow,
            id: AccountId,
            _at: Option<AggregateVersion>,
        ) -> Result<Option<Account>, RepositoryError<Account>> {
            Ok(self.items.lock().expect("lock").get(&id).cloned())
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
            let id = aggregate.aggregate_id().expect("account id should exist");
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
            let id = aggregate.aggregate_id().expect("currency id should exist");
            self.items.lock().expect("lock").insert(id, aggregate);
        }
    }

    impl Repository<Currency> for TestCurrencyRepository {
        type Uow = TestUow;

        async fn find(
            &self,
            _uow: &mut Self::Uow,
            id: CurrencyId,
        ) -> Result<Option<Currency>, RepositoryError<Currency>> {
            Ok(self.items.lock().expect("lock").get(&id).cloned())
        }

        async fn find_at_version(
            &self,
            _uow: &mut Self::Uow,
            id: CurrencyId,
            _at: Option<AggregateVersion>,
        ) -> Result<Option<Currency>, RepositoryError<Currency>> {
            Ok(self.items.lock().expect("lock").get(&id).cloned())
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
            let id = aggregate.aggregate_id().expect("currency id should exist");
            self.items
                .lock()
                .expect("lock")
                .insert(id, aggregate.clone());
            Ok(())
        }
    }

    #[derive(Clone, Default)]
    struct TestPayoutDestinationRepository {
        items: Arc<Mutex<HashMap<PayoutDestinationId, PayoutDestination>>>,
    }

    impl TestPayoutDestinationRepository {
        fn insert(&self, aggregate: PayoutDestination) {
            let id = aggregate
                .aggregate_id()
                .expect("payout destination id should exist");
            self.items.lock().expect("lock").insert(id, aggregate);
        }
    }

    impl Repository<PayoutDestination> for TestPayoutDestinationRepository {
        type Uow = TestUow;

        async fn find(
            &self,
            _uow: &mut Self::Uow,
            id: PayoutDestinationId,
        ) -> Result<Option<PayoutDestination>, RepositoryError<PayoutDestination>> {
            Ok(self.items.lock().expect("lock").get(&id).cloned())
        }

        async fn find_at_version(
            &self,
            _uow: &mut Self::Uow,
            id: PayoutDestinationId,
            _at: Option<AggregateVersion>,
        ) -> Result<Option<PayoutDestination>, RepositoryError<PayoutDestination>> {
            Ok(self.items.lock().expect("lock").get(&id).cloned())
        }

        async fn find_by_unique_value(
            &self,
            _uow: &mut Self::Uow,
            _unique_key: UniqueKey,
            _unique_value: &UniqueValue,
        ) -> Result<Option<PayoutDestination>, RepositoryError<PayoutDestination>> {
            Ok(None)
        }

        async fn save(
            &self,
            _uow: &mut Self::Uow,
            _request_context: &RequestContext,
            aggregate: &mut PayoutDestination,
        ) -> Result<(), RepositoryError<PayoutDestination>> {
            let id = aggregate
                .aggregate_id()
                .expect("payout destination id should exist");
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

        async fn find(
            &self,
            _uow: &mut Self::Uow,
            _id: WithdrawalId,
        ) -> Result<Option<Withdrawal>, RepositoryError<Withdrawal>> {
            Ok(self.saved.lock().expect("lock").clone())
        }

        async fn find_at_version(
            &self,
            _uow: &mut Self::Uow,
            _id: WithdrawalId,
            _at: Option<AggregateVersion>,
        ) -> Result<Option<Withdrawal>, RepositoryError<Withdrawal>> {
            Ok(self.saved.lock().expect("lock").clone())
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

    fn payout_destination_token_account_owner_address() -> PayoutDestinationTokenAccountOwnerAddress
    {
        PayoutDestinationTokenAccountOwnerAddress::try_from("11111111111111111111111111111111")
            .expect("payout destination address should be valid")
    }

    fn mint_account() -> CurrencyMintAccount {
        CurrencyMintAccount::new(
            CurrencyMintAccountAddress::try_from("Mint111111111111111111111111111111111111")
                .expect("mint account address should be valid"),
            CurrencyPoolTokenAccountAddress::try_from("Pool111111111111111111111111111111111111")
                .expect("pool token account address should be valid"),
            CurrencyTokenProgramId::try_from(TOKEN_PROGRAM_ID)
                .expect("token program ID should be valid"),
        )
    }

    fn opened_account(owner: AccountOwner, currency_id: CurrencyId) -> Account {
        let mut account = Account::default();
        account
            .open(owner, account_name(), currency_id)
            .expect("account should open");
        account.core_mut().clear_uncommitted_events();
        account
    }

    fn provisioned_currency(owner: CurrencyOwner) -> Currency {
        let mut currency = Currency::default();
        currency
            .define(
                owner,
                CurrencySymbol::try_from("usdc").expect("symbol should be valid"),
                CurrencyName::try_from("USD Coin").expect("name should be valid"),
                CurrencyDecimals::new(6),
                None,
                None,
            )
            .expect("currency should define");
        currency
            .provision(mint_account())
            .expect("currency should provision");
        currency.core_mut().clear_uncommitted_events();
        currency
    }

    fn registered_payout_destination(owner: PayoutDestinationOwner) -> PayoutDestination {
        let mut payout_destination = PayoutDestination::default();
        payout_destination
            .register(owner, payout_destination_token_account_owner_address())
            .expect("payout destination should register");
        payout_destination.core_mut().clear_uncommitted_events();
        payout_destination
    }

    #[test]
    fn authorization_plan_requires_withdrawal_requester_relationship() {
        let handler = WithdrawalRequestCommandHandler::new(
            TestAccountRepository::default(),
            TestCurrencyRepository::default(),
            TestPayoutDestinationRepository::default(),
            TestWithdrawalRepository::default(),
        );
        let command = WithdrawalRequestCommand {
            account_id: AccountId::new(),
            payout_destination_id: PayoutDestinationId::new(),
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
    async fn handle_requests_withdrawal_when_account_and_destination_match() {
        let user_id = UserId::new();
        let account_owner = AccountOwner::from(user_id);
        let currency_owner = CurrencyOwner::from(user_id);
        let payout_destination_owner = PayoutDestinationOwner::from(user_id);
        let currency = provisioned_currency(currency_owner);
        let currency_id = currency.aggregate_id().expect("currency id should exist");
        let account = opened_account(account_owner, currency_id);
        let account_id = account.aggregate_id().expect("account id should exist");
        let payout_destination = registered_payout_destination(payout_destination_owner);
        let payout_destination_id = payout_destination
            .aggregate_id()
            .expect("payout destination id should exist");
        let account_repository = TestAccountRepository::default();
        account_repository.insert(account);
        let currency_repository = TestCurrencyRepository::default();
        currency_repository.insert(currency);
        let payout_destination_repository = TestPayoutDestinationRepository::default();
        payout_destination_repository.insert(payout_destination);
        let withdrawal_repository = TestWithdrawalRepository::default();
        let handler = WithdrawalRequestCommandHandler::new(
            account_repository,
            currency_repository,
            payout_destination_repository,
            withdrawal_repository.clone(),
        );
        let mut uow = TestUow;

        let handled = handler
            .handle(
                &mut uow,
                &request_context(user_id),
                &WithdrawalRequestCommand {
                    account_id,
                    payout_destination_id,
                    amount: CurrencyAmount::new(10),
                },
            )
            .await
            .expect("request should succeed");

        let output = handled.into_output();
        let WithdrawalRequestOutput::Requested { withdrawal_id } = output else {
            panic!("expected requested output");
        };
        let saved = withdrawal_repository
            .saved
            .lock()
            .expect("lock")
            .clone()
            .expect("withdrawal should be saved");

        assert_eq!(saved.aggregate_id(), Some(withdrawal_id));
        assert_eq!(
            saved.account_id().expect("account id should exist"),
            &account_id
        );
        assert_eq!(
            saved
                .payout_destination_id()
                .expect("payout destination id should exist"),
            &payout_destination_id
        );
        assert_eq!(
            saved.status().expect("status should exist"),
            &WithdrawalStatus::Pending
        );
    }

    #[tokio::test]
    async fn handle_rejects_when_payout_destination_owner_differs() {
        let user_id = UserId::new();
        let another_user_id = UserId::new();
        let currency = provisioned_currency(CurrencyOwner::from(user_id));
        let currency_id = currency.aggregate_id().expect("currency id should exist");
        let account = opened_account(AccountOwner::from(user_id), currency_id);
        let account_id = account.aggregate_id().expect("account id should exist");
        let payout_destination =
            registered_payout_destination(PayoutDestinationOwner::from(another_user_id));
        let payout_destination_id = payout_destination
            .aggregate_id()
            .expect("payout destination id should exist");
        let account_repository = TestAccountRepository::default();
        account_repository.insert(account);
        let currency_repository = TestCurrencyRepository::default();
        currency_repository.insert(currency);
        let payout_destination_repository = TestPayoutDestinationRepository::default();
        payout_destination_repository.insert(payout_destination);
        let withdrawal_repository = TestWithdrawalRepository::default();
        let handler = WithdrawalRequestCommandHandler::new(
            account_repository,
            currency_repository,
            payout_destination_repository,
            withdrawal_repository.clone(),
        );
        let mut uow = TestUow;

        let handled = handler
            .handle(
                &mut uow,
                &request_context(user_id),
                &WithdrawalRequestCommand {
                    account_id,
                    payout_destination_id,
                    amount: CurrencyAmount::new(10),
                },
            )
            .await
            .expect("request should return a rejection");

        assert_eq!(
            handled.into_output(),
            WithdrawalRequestOutput::Rejected {
                withdrawal_id: withdrawal_repository
                    .saved
                    .lock()
                    .expect("lock")
                    .as_ref()
                    .and_then(|withdrawal| withdrawal.aggregate_id())
                    .expect("withdrawal id should exist"),
                reason: WithdrawalRequestRejectionReason::PayoutDestinationOwnerMismatch,
            }
        );
        assert_eq!(
            withdrawal_repository
                .saved
                .lock()
                .expect("lock")
                .as_ref()
                .expect("withdrawal should be saved")
                .status()
                .expect("status should exist"),
            &WithdrawalStatus::Rejected
        );
    }

    #[tokio::test]
    async fn handle_errors_when_account_is_missing() {
        let handler = WithdrawalRequestCommandHandler::new(
            TestAccountRepository::default(),
            TestCurrencyRepository::default(),
            TestPayoutDestinationRepository::default(),
            TestWithdrawalRepository::default(),
        );
        let mut uow = TestUow;
        let error = handler
            .handle(
                &mut uow,
                &request_context(UserId::new()),
                &WithdrawalRequestCommand {
                    account_id: AccountId::new(),
                    payout_destination_id: PayoutDestinationId::new(),
                    amount: CurrencyAmount::new(10),
                },
            )
            .await
            .expect_err("missing account should error");

        assert!(matches!(
            error,
            WithdrawalRequestCommandHandlerError::AccountNotFound
        ));
    }
}
