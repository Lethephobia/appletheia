use appletheia::application::authorization::{
    AuthorizationPlan, PrincipalRequirement, Relation, RelationshipRequirement,
};
use appletheia::application::command::{CommandHandled, CommandHandler};
use appletheia::application::repository::Repository;
use appletheia::application::request_context::RequestContext;
use appletheia::domain::Aggregate;
use banking_ledger_domain::account::Account;
use banking_ledger_domain::currency::Currency;
use banking_ledger_domain::deposit::{
    Deposit, DepositRequest, DepositRequestRejectionReason, DepositRequestResult,
};

use super::{
    DepositTokenTransferPrepareCommand, DepositTokenTransferPrepareCommandHandlerError,
    DepositTokenTransferPrepareOutput,
};
use crate::authorization::AccountDepositRequesterRelation;
use crate::mint::{
    TokenAccountOwnerAddressValidationResult, TokenAccountOwnerAddressValidator,
    TokenAccountOwnerAddressValidatorError, TokenDepositPrepareRequest, TokenDepositPreparer,
};

/// Handles `DepositTokenTransferPrepareCommand`.
pub struct DepositTokenTransferPrepareCommandHandler<DR, AR, CR, TAOV, PTDP>
where
    DR: Repository<Deposit>,
    AR: Repository<Account, Uow = DR::Uow>,
    CR: Repository<Currency, Uow = AR::Uow>,
    TAOV: TokenAccountOwnerAddressValidator,
    PTDP: TokenDepositPreparer,
{
    deposit_repository: DR,
    account_repository: AR,
    currency_repository: CR,
    token_account_owner_address_validator: TAOV,
    token_deposit_preparer: PTDP,
}

impl<DR, AR, CR, TAOV, PTDP> DepositTokenTransferPrepareCommandHandler<DR, AR, CR, TAOV, PTDP>
where
    DR: Repository<Deposit>,
    AR: Repository<Account, Uow = DR::Uow>,
    CR: Repository<Currency, Uow = AR::Uow>,
    TAOV: TokenAccountOwnerAddressValidator,
    PTDP: TokenDepositPreparer,
{
    pub fn new(
        deposit_repository: DR,
        account_repository: AR,
        currency_repository: CR,
        token_account_owner_address_validator: TAOV,
        token_deposit_preparer: PTDP,
    ) -> Self {
        Self {
            deposit_repository,
            account_repository,
            currency_repository,
            token_account_owner_address_validator,
            token_deposit_preparer,
        }
    }
}

impl<DR, AR, CR, TAOV, PTDP> CommandHandler
    for DepositTokenTransferPrepareCommandHandler<DR, AR, CR, TAOV, PTDP>
where
    DR: Repository<Deposit>,
    AR: Repository<Account, Uow = DR::Uow>,
    CR: Repository<Currency, Uow = AR::Uow>,
    TAOV: TokenAccountOwnerAddressValidator,
    PTDP: TokenDepositPreparer,
{
    type Command = DepositTokenTransferPrepareCommand;
    type Output = DepositTokenTransferPrepareOutput;
    type ReplayOutput = DepositTokenTransferPrepareOutput;
    type Error = DepositTokenTransferPrepareCommandHandlerError;
    type Uow = DR::Uow;

    fn authorization_plan(
        &self,
        command: &Self::Command,
    ) -> Result<AuthorizationPlan, Self::Error> {
        Ok(AuthorizationPlan::OnlyPrincipals(vec![
            PrincipalRequirement::AuthenticatedWithRelationship(RelationshipRequirement::check::<
                Account,
            >(
                command.account_id,
                AccountDepositRequesterRelation::REF,
            )),
        ]))
    }

    async fn handle(
        &self,
        uow: &mut Self::Uow,
        request_context: &RequestContext,
        command: &Self::Command,
    ) -> Result<CommandHandled<Self::Output, Self::ReplayOutput>, Self::Error> {
        let mut deposit = Deposit::new();
        let deposit_id = deposit.aggregate_id();

        match self
            .token_account_owner_address_validator
            .validate(&command.token_account_owner_address)
            .await
        {
            Ok(TokenAccountOwnerAddressValidationResult::Valid) => {}
            Ok(TokenAccountOwnerAddressValidationResult::Invalid) => {
                let reason = DepositRequestRejectionReason::InvalidTokenAccountOwnerAddress;
                return Ok(CommandHandled::same(
                    DepositTokenTransferPrepareOutput::Rejected { deposit_id, reason },
                ));
            }
            Err(error @ TokenAccountOwnerAddressValidatorError::Backend(_)) => {
                return Err(error.into());
            }
        }

        let account = self
            .account_repository
            .read(uow, command.account_id)
            .await?;
        let currency_id = *account.currency_id()?;
        let request = DepositRequest {
            account_id: command.account_id,
            currency_id,
            token_account_owner_address: command.token_account_owner_address.clone(),
            amount: command.amount,
        };

        let currency = self.currency_repository.read(uow, currency_id).await?;
        let Some(mint_account) = currency.mint_account()? else {
            let reason = DepositRequestRejectionReason::CurrencyUnprovisioned;
            deposit.reject_request(request, reason)?;
            self.deposit_repository
                .save(uow, request_context, &mut deposit)
                .await?;
            return Ok(CommandHandled::same(
                DepositTokenTransferPrepareOutput::Rejected { deposit_id, reason },
            ));
        };

        let result = deposit.request(request)?;
        match result {
            DepositRequestResult::Requested => {}
            DepositRequestResult::Rejected { reason } => {
                self.deposit_repository
                    .save(uow, request_context, &mut deposit)
                    .await?;
                return Ok(CommandHandled::same(
                    DepositTokenTransferPrepareOutput::Rejected { deposit_id, reason },
                ));
            }
        }

        let request = TokenDepositPrepareRequest::new(
            deposit_id,
            currency_id,
            mint_account.clone(),
            command.token_account_owner_address.clone(),
            command.amount,
        );
        let preparation = self.token_deposit_preparer.prepare(request).await?;
        self.deposit_repository
            .save(uow, request_context, &mut deposit)
            .await?;

        Ok(CommandHandled::same(
            DepositTokenTransferPrepareOutput::Prepared {
                deposit_id,
                preparation,
            },
        ))
    }
}

#[cfg(test)]
mod tests {
    use appletheia::application::authorization::AggregateRef;
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
    use banking_ledger_domain::currency::{Currency, CurrencyId};
    use banking_ledger_domain::deposit::{Deposit, DepositId, DepositRequestRejectionReason};
    use uuid::Uuid;

    use super::{
        DepositTokenTransferPrepareCommand, DepositTokenTransferPrepareCommandHandler,
        DepositTokenTransferPrepareOutput,
    };
    use crate::mint::{
        TokenAccountOwnerAddressValidationResult, TokenAccountOwnerAddressValidator,
        TokenAccountOwnerAddressValidatorError, TokenDepositPreparation,
        TokenDepositPrepareRequest, TokenDepositPreparer, TokenDepositPreparerError,
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
    struct TestAccountRepository {
        account: Account,
    }

    impl Repository<Account> for TestAccountRepository {
        type Uow = TestUow;

        async fn read(
            &self,
            _uow: &mut Self::Uow,
            _id: AccountId,
        ) -> Result<Account, RepositoryError<Account>> {
            Ok(self.account.clone())
        }

        async fn read_at_version(
            &self,
            _uow: &mut Self::Uow,
            _id: AccountId,
            _at: AggregateVersion,
        ) -> Result<Account, RepositoryError<Account>> {
            Ok(self.account.clone())
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
            _aggregate: &mut Account,
        ) -> Result<(), RepositoryError<Account>> {
            panic!("account should not be saved")
        }
    }

    #[derive(Clone, Copy)]
    struct TestDepositRepository;

    impl Repository<Deposit> for TestDepositRepository {
        type Uow = TestUow;

        async fn read(
            &self,
            _uow: &mut Self::Uow,
            _id: DepositId,
        ) -> Result<Deposit, RepositoryError<Deposit>> {
            panic!("deposit should not be read")
        }

        async fn read_at_version(
            &self,
            _uow: &mut Self::Uow,
            _id: DepositId,
            _at: AggregateVersion,
        ) -> Result<Deposit, RepositoryError<Deposit>> {
            panic!("deposit should not be read")
        }

        async fn find_by_unique_value(
            &self,
            _uow: &mut Self::Uow,
            _unique_key: UniqueKey,
            _unique_value: &UniqueValue,
        ) -> Result<Option<Deposit>, RepositoryError<Deposit>> {
            panic!("deposit should not be queried")
        }

        async fn save(
            &self,
            _uow: &mut Self::Uow,
            _request_context: &RequestContext,
            _aggregate: &mut Deposit,
        ) -> Result<(), RepositoryError<Deposit>> {
            panic!("invalid address rejection should not be saved")
        }
    }

    #[derive(Clone, Copy)]
    struct TestCurrencyRepository;

    impl Repository<Currency> for TestCurrencyRepository {
        type Uow = TestUow;

        async fn read(
            &self,
            _uow: &mut Self::Uow,
            _id: CurrencyId,
        ) -> Result<Currency, RepositoryError<Currency>> {
            panic!("currency should not be read after invalid address validation")
        }

        async fn read_at_version(
            &self,
            _uow: &mut Self::Uow,
            _id: CurrencyId,
            _at: AggregateVersion,
        ) -> Result<Currency, RepositoryError<Currency>> {
            panic!("currency should not be read after invalid address validation")
        }

        async fn find_by_unique_value(
            &self,
            _uow: &mut Self::Uow,
            _unique_key: UniqueKey,
            _unique_value: &UniqueValue,
        ) -> Result<Option<Currency>, RepositoryError<Currency>> {
            panic!("currency should not be queried after invalid address validation")
        }

        async fn save(
            &self,
            _uow: &mut Self::Uow,
            _request_context: &RequestContext,
            _aggregate: &mut Currency,
        ) -> Result<(), RepositoryError<Currency>> {
            panic!("currency should not be saved")
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

    #[derive(Clone, Copy)]
    struct TestTokenDepositPreparer;

    impl TokenDepositPreparer for TestTokenDepositPreparer {
        async fn prepare(
            &self,
            _request: TokenDepositPrepareRequest,
        ) -> Result<TokenDepositPreparation, TokenDepositPreparerError> {
            panic!("token deposit should not be prepared")
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

    fn opened_account(user_id: UserId, currency_id: CurrencyId) -> Account {
        let mut account = Account::new();
        account
            .open(AccountOpening {
                owner: AccountOwner::from(user_id),
                name: AccountName::try_from("main").expect("account name should be valid"),
                currency_id,
            })
            .expect("account should open");
        account
    }

    fn token_account_owner_address() -> TokenAccountOwnerAddress {
        TokenAccountOwnerAddress::try_from("11111111111111111111111111111111")
            .expect("token account owner address should be valid")
    }

    #[tokio::test]
    async fn handle_rejects_invalid_address_without_saving_a_deposit() {
        let user_id = UserId::new();
        let currency_id = CurrencyId::new();
        let account = opened_account(user_id, currency_id);
        let account_id = account.aggregate_id();
        let handler = DepositTokenTransferPrepareCommandHandler::new(
            TestDepositRepository,
            TestAccountRepository { account },
            TestCurrencyRepository,
            RejectingValidator,
            TestTokenDepositPreparer,
        );

        let handled = handler
            .handle(
                &mut TestUow,
                &request_context(user_id),
                &DepositTokenTransferPrepareCommand {
                    account_id,
                    token_account_owner_address: token_account_owner_address(),
                    amount: CurrencyAmount::new(10),
                },
            )
            .await
            .expect("invalid address should be handled as a rejection");

        let output = handled.into_output();
        let DepositTokenTransferPrepareOutput::Rejected { reason, .. } = output else {
            panic!("expected rejected output");
        };
        assert_eq!(
            reason,
            DepositRequestRejectionReason::InvalidTokenAccountOwnerAddress
        );
    }
}
