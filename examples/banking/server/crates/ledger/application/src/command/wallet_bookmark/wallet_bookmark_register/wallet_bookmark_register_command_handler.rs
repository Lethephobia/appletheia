use appletheia::application::authorization::{
    AuthorizationPlan, PrincipalRequirement, Relation, RelationshipRequirement,
};
use appletheia::application::command::{CommandHandled, CommandHandler};
use appletheia::application::repository::Repository;
use appletheia::application::request_context::RequestContext;
use appletheia::domain::Aggregate;
use banking_iam_application::authorization::{
    OrganizationFinanceManagerRelation, UserOwnerRelation,
};
use banking_iam_domain::{Organization, User};
use banking_ledger_domain::wallet_bookmark::{
    WalletBookmark, WalletBookmarkOwner, WalletBookmarkRegisterResult, WalletBookmarkRegistration,
};

use super::{
    WalletBookmarkRegisterCommand, WalletBookmarkRegisterCommandHandlerError,
    WalletBookmarkRegisterOutput,
};
use crate::mint::TokenAccountOwnerAddressValidator;

/// Handles `WalletBookmarkRegisterCommand`.
pub struct WalletBookmarkRegisterCommandHandler<WBR, TAOV>
where
    WBR: Repository<WalletBookmark>,
    TAOV: TokenAccountOwnerAddressValidator,
{
    wallet_bookmark_repository: WBR,
    token_account_owner_address_validator: TAOV,
}

impl<WBR, TAOV> WalletBookmarkRegisterCommandHandler<WBR, TAOV>
where
    WBR: Repository<WalletBookmark>,
    TAOV: TokenAccountOwnerAddressValidator,
{
    pub fn new(
        wallet_bookmark_repository: WBR,
        token_account_owner_address_validator: TAOV,
    ) -> Self {
        Self {
            wallet_bookmark_repository,
            token_account_owner_address_validator,
        }
    }
}

impl<WBR, TAOV> CommandHandler for WalletBookmarkRegisterCommandHandler<WBR, TAOV>
where
    WBR: Repository<WalletBookmark>,
    TAOV: TokenAccountOwnerAddressValidator,
{
    type Command = WalletBookmarkRegisterCommand;
    type Output = WalletBookmarkRegisterOutput;
    type ReplayOutput = WalletBookmarkRegisterOutput;
    type Error = WalletBookmarkRegisterCommandHandlerError;
    type Uow = WBR::Uow;

    fn authorization_plan(
        &self,
        command: &Self::Command,
    ) -> Result<AuthorizationPlan, Self::Error> {
        match command.owner {
            WalletBookmarkOwner::User(user_id) => Ok(AuthorizationPlan::OnlyPrincipals(vec![
                PrincipalRequirement::AuthenticatedWithRelationship(
                    RelationshipRequirement::check::<User>(user_id, UserOwnerRelation::REF),
                ),
            ])),
            WalletBookmarkOwner::Organization(organization_id) => {
                Ok(AuthorizationPlan::OnlyPrincipals(vec![
                    PrincipalRequirement::AuthenticatedWithRelationship(
                        RelationshipRequirement::check::<Organization>(
                            organization_id,
                            OrganizationFinanceManagerRelation::REF,
                        ),
                    ),
                ]))
            }
        }
    }

    async fn handle(
        &self,
        uow: &mut Self::Uow,
        request_context: &RequestContext,
        command: &Self::Command,
    ) -> Result<CommandHandled<Self::Output, Self::ReplayOutput>, Self::Error> {
        self.token_account_owner_address_validator
            .validate(&command.token_account_owner_address)
            .await?;

        let mut wallet_bookmark = WalletBookmark::new();
        let result = wallet_bookmark.register(WalletBookmarkRegistration {
            owner: command.owner,
            display_name: command.display_name.clone(),
            description: command.description.clone(),
            token_account_owner_address: command.token_account_owner_address.clone(),
        })?;

        self.wallet_bookmark_repository
            .save(uow, request_context, &mut wallet_bookmark)
            .await?;

        let output = match result {
            WalletBookmarkRegisterResult::Registered { wallet_bookmark_id } => {
                WalletBookmarkRegisterOutput::new(wallet_bookmark_id)
            }
        };

        Ok(CommandHandled::same(output))
    }
}

#[cfg(test)]
mod tests {
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
    use appletheia::domain::Aggregate;
    use banking_iam_application::authorization::{
        OrganizationFinanceManagerRelation, UserOwnerRelation,
    };
    use banking_iam_domain::{Organization, OrganizationId, User, UserId};
    use banking_ledger_domain::core::TokenAccountOwnerAddress;
    use banking_ledger_domain::wallet_bookmark::{
        WalletBookmark, WalletBookmarkDescription, WalletBookmarkDisplayName, WalletBookmarkId,
        WalletBookmarkOwner,
    };
    use uuid::Uuid;

    use super::{
        WalletBookmarkRegisterCommand, WalletBookmarkRegisterCommandHandler,
        WalletBookmarkRegisterCommandHandlerError, WalletBookmarkRegisterOutput,
    };
    use crate::mint::{TokenAccountOwnerAddressValidator, TokenAccountOwnerAddressValidatorError};

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
    struct TestWalletBookmarkRepository {
        wallet_bookmark: Arc<Mutex<Option<WalletBookmark>>>,
    }

    impl Repository<WalletBookmark> for TestWalletBookmarkRepository {
        type Uow = TestUow;

        async fn read(
            &self,
            _uow: &mut Self::Uow,
            _id: WalletBookmarkId,
        ) -> Result<WalletBookmark, RepositoryError<WalletBookmark>> {
            self.wallet_bookmark
                .lock()
                .expect("lock")
                .clone()
                .ok_or_else(|| RepositoryError::NotFound {
                    aggregate_type: WalletBookmark::TYPE,
                    aggregate_id: _id,
                })
        }

        async fn read_at_version(
            &self,
            _uow: &mut Self::Uow,
            _id: WalletBookmarkId,
            _at: appletheia::domain::AggregateVersion,
        ) -> Result<WalletBookmark, RepositoryError<WalletBookmark>> {
            self.wallet_bookmark
                .lock()
                .expect("lock")
                .clone()
                .ok_or_else(|| RepositoryError::NotFound {
                    aggregate_type: WalletBookmark::TYPE,
                    aggregate_id: _id,
                })
        }

        async fn find_by_unique_value(
            &self,
            _uow: &mut Self::Uow,
            _unique_key: appletheia::domain::UniqueKey,
            _unique_value: &appletheia::domain::UniqueValue,
        ) -> Result<Option<WalletBookmark>, RepositoryError<WalletBookmark>> {
            Ok(None)
        }

        async fn save(
            &self,
            _uow: &mut Self::Uow,
            _request_context: &RequestContext,
            aggregate: &mut WalletBookmark,
        ) -> Result<(), RepositoryError<WalletBookmark>> {
            *self.wallet_bookmark.lock().expect("lock") = Some(aggregate.clone());
            Ok(())
        }
    }

    #[derive(Clone, Copy, Default)]
    struct PassingValidator;

    impl TokenAccountOwnerAddressValidator for PassingValidator {
        async fn validate(
            &self,
            _address: &TokenAccountOwnerAddress,
        ) -> Result<(), TokenAccountOwnerAddressValidatorError> {
            Ok(())
        }
    }

    #[derive(Clone, Copy, Default)]
    struct RejectingValidator;

    impl TokenAccountOwnerAddressValidator for RejectingValidator {
        async fn validate(
            &self,
            _address: &TokenAccountOwnerAddress,
        ) -> Result<(), TokenAccountOwnerAddressValidatorError> {
            Err(TokenAccountOwnerAddressValidatorError::InvalidAddress)
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

    fn token_account_owner_address() -> TokenAccountOwnerAddress {
        TokenAccountOwnerAddress::try_from("wallet-123").expect("address should be valid")
    }

    fn wallet_bookmark_display_name() -> WalletBookmarkDisplayName {
        WalletBookmarkDisplayName::try_from("Main wallet").expect("display name should be valid")
    }

    fn wallet_bookmark_description() -> WalletBookmarkDescription {
        WalletBookmarkDescription::try_from("Personal main wallet")
            .expect("description should be valid")
    }

    #[test]
    fn authorization_plan_requires_target_user_owner() {
        let handler = WalletBookmarkRegisterCommandHandler::new(
            TestWalletBookmarkRepository::default(),
            PassingValidator,
        );
        let user_id = UserId::new();

        let plan = handler
            .authorization_plan(&WalletBookmarkRegisterCommand {
                owner: WalletBookmarkOwner::User(user_id),
                display_name: Some(wallet_bookmark_display_name()),
                description: Some(wallet_bookmark_description()),
                token_account_owner_address: token_account_owner_address(),
            })
            .expect("authorization plan should build");

        assert_eq!(
            plan,
            AuthorizationPlan::OnlyPrincipals(vec![
                PrincipalRequirement::AuthenticatedWithRelationship(
                    RelationshipRequirement::check::<User>(user_id, UserOwnerRelation::REF)
                ),
            ])
        );
    }

    #[test]
    fn authorization_plan_requires_organization_finance_manager() {
        let handler = WalletBookmarkRegisterCommandHandler::new(
            TestWalletBookmarkRepository::default(),
            PassingValidator,
        );
        let organization_id = OrganizationId::new();

        let plan = handler
            .authorization_plan(&WalletBookmarkRegisterCommand {
                owner: WalletBookmarkOwner::Organization(organization_id),
                display_name: Some(wallet_bookmark_display_name()),
                description: Some(wallet_bookmark_description()),
                token_account_owner_address: token_account_owner_address(),
            })
            .expect("authorization plan should build");

        assert_eq!(
            plan,
            AuthorizationPlan::OnlyPrincipals(vec![
                PrincipalRequirement::AuthenticatedWithRelationship(
                    RelationshipRequirement::check::<Organization>(
                        organization_id,
                        OrganizationFinanceManagerRelation::REF
                    )
                ),
            ])
        );
    }

    #[tokio::test]
    async fn handle_registers_wallet_bookmark() {
        let repository = TestWalletBookmarkRepository::default();
        let handler =
            WalletBookmarkRegisterCommandHandler::new(repository.clone(), PassingValidator);
        let user_id = UserId::new();
        let request_context = request_context(user_id);

        let handled = handler
            .handle(
                &mut TestUow,
                &request_context,
                &WalletBookmarkRegisterCommand {
                    owner: WalletBookmarkOwner::User(user_id),
                    display_name: Some(wallet_bookmark_display_name()),
                    description: Some(wallet_bookmark_description()),
                    token_account_owner_address: token_account_owner_address(),
                },
            )
            .await
            .expect("command should succeed");

        let output = handled.into_output();
        let aggregate = repository
            .wallet_bookmark
            .lock()
            .expect("lock")
            .clone()
            .expect("aggregate should be saved");

        assert_eq!(
            output,
            WalletBookmarkRegisterOutput::new(
                aggregate.aggregate_id().expect("aggregate id should exist")
            )
        );
    }

    #[tokio::test]
    async fn handle_returns_validation_error_when_address_is_invalid() {
        let repository = TestWalletBookmarkRepository::default();
        let handler =
            WalletBookmarkRegisterCommandHandler::new(repository.clone(), RejectingValidator);
        let user_id = UserId::new();
        let request_context = request_context(user_id);

        let error = handler
            .handle(
                &mut TestUow,
                &request_context,
                &WalletBookmarkRegisterCommand {
                    owner: WalletBookmarkOwner::User(user_id),
                    display_name: Some(wallet_bookmark_display_name()),
                    description: Some(wallet_bookmark_description()),
                    token_account_owner_address: token_account_owner_address(),
                },
            )
            .await
            .expect_err("command should fail");

        assert!(matches!(
            error,
            WalletBookmarkRegisterCommandHandlerError::TokenAccountOwnerAddressValidator(
                TokenAccountOwnerAddressValidatorError::InvalidAddress
            )
        ));
        assert!(repository.wallet_bookmark.lock().expect("lock").is_none());
    }
}
