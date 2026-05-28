use appletheia::application::authorization::{
    AuthorizationPlan, PrincipalRequirement, Relation, RelationshipRequirement,
};
use appletheia::application::command::{CommandHandled, CommandHandler};
use appletheia::application::repository::Repository;
use appletheia::application::request_context::RequestContext;
use banking_iam_application::authorization::{
    OrganizationFinanceManagerRelation, UserOwnerRelation,
};
use banking_iam_domain::{Organization, User};
use banking_ledger_domain::payout_destination::{
    PayoutDestination, PayoutDestinationOwner, PayoutDestinationRegisterResult,
    PayoutDestinationRegistration,
};

use super::{
    PayoutDestinationRegisterCommand, PayoutDestinationRegisterCommandHandlerError,
    PayoutDestinationRegisterOutput,
};
use crate::mint::{TokenAccountOwnerAddress, TokenAccountOwnerAddressValidator};

/// Handles `PayoutDestinationRegisterCommand`.
pub struct PayoutDestinationRegisterCommandHandler<PDR, PDAV>
where
    PDR: Repository<PayoutDestination>,
    PDAV: TokenAccountOwnerAddressValidator,
{
    payout_destination_repository: PDR,
    token_account_owner_address_validator: PDAV,
}

impl<PDR, PDAV> PayoutDestinationRegisterCommandHandler<PDR, PDAV>
where
    PDR: Repository<PayoutDestination>,
    PDAV: TokenAccountOwnerAddressValidator,
{
    pub fn new(
        payout_destination_repository: PDR,
        token_account_owner_address_validator: PDAV,
    ) -> Self {
        Self {
            payout_destination_repository,
            token_account_owner_address_validator,
        }
    }
}

impl<PDR, PDAV> CommandHandler for PayoutDestinationRegisterCommandHandler<PDR, PDAV>
where
    PDR: Repository<PayoutDestination>,
    PDAV: TokenAccountOwnerAddressValidator,
{
    type Command = PayoutDestinationRegisterCommand;
    type Output = PayoutDestinationRegisterOutput;
    type ReplayOutput = PayoutDestinationRegisterOutput;
    type Error = PayoutDestinationRegisterCommandHandlerError;
    type Uow = PDR::Uow;

    fn authorization_plan(
        &self,
        command: &Self::Command,
    ) -> Result<AuthorizationPlan, Self::Error> {
        match command.owner {
            PayoutDestinationOwner::User(user_id) => Ok(AuthorizationPlan::OnlyPrincipals(vec![
                PrincipalRequirement::AuthenticatedWithRelationship(
                    RelationshipRequirement::check::<User>(user_id, UserOwnerRelation::REF),
                ),
            ])),
            PayoutDestinationOwner::Organization(organization_id) => {
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
            .validate(&TokenAccountOwnerAddress::from(
                command.token_account_owner_address.clone(),
            ))
            .await?;

        let mut payout_destination = PayoutDestination::default();
        let result = payout_destination.register(PayoutDestinationRegistration {
            owner: command.owner,
            token_account_owner_address: command.token_account_owner_address.clone(),
        })?;

        self.payout_destination_repository
            .save(uow, request_context, &mut payout_destination)
            .await?;

        let output = match result {
            PayoutDestinationRegisterResult::Registered {
                payout_destination_id,
            } => PayoutDestinationRegisterOutput::new(payout_destination_id),
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
    use banking_ledger_domain::payout_destination::{
        PayoutDestination, PayoutDestinationId, PayoutDestinationOwner,
        PayoutDestinationTokenAccountOwnerAddress,
    };
    use uuid::Uuid;

    use crate::mint::{
        TokenAccountOwnerAddress, TokenAccountOwnerAddressValidator,
        TokenAccountOwnerAddressValidatorError,
    };

    use super::{
        PayoutDestinationRegisterCommand, PayoutDestinationRegisterCommandHandler,
        PayoutDestinationRegisterCommandHandlerError, PayoutDestinationRegisterOutput,
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

    #[derive(Clone, Default)]
    struct TestPayoutDestinationRepository {
        payout_destination: Arc<Mutex<Option<PayoutDestination>>>,
    }

    impl Repository<PayoutDestination> for TestPayoutDestinationRepository {
        type Uow = TestUow;

        async fn find(
            &self,
            _uow: &mut Self::Uow,
            _id: PayoutDestinationId,
        ) -> Result<Option<PayoutDestination>, RepositoryError<PayoutDestination>> {
            Ok(self.payout_destination.lock().expect("lock").clone())
        }

        async fn find_at_version(
            &self,
            _uow: &mut Self::Uow,
            _id: PayoutDestinationId,
            _at: Option<appletheia::domain::AggregateVersion>,
        ) -> Result<Option<PayoutDestination>, RepositoryError<PayoutDestination>> {
            Ok(self.payout_destination.lock().expect("lock").clone())
        }

        async fn find_by_unique_value(
            &self,
            _uow: &mut Self::Uow,
            _unique_key: appletheia::domain::UniqueKey,
            _unique_value: &appletheia::domain::UniqueValue,
        ) -> Result<Option<PayoutDestination>, RepositoryError<PayoutDestination>> {
            Ok(None)
        }

        async fn save(
            &self,
            _uow: &mut Self::Uow,
            _request_context: &RequestContext,
            aggregate: &mut PayoutDestination,
        ) -> Result<(), RepositoryError<PayoutDestination>> {
            *self.payout_destination.lock().expect("lock") = Some(aggregate.clone());
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

    fn token_account_owner_address() -> PayoutDestinationTokenAccountOwnerAddress {
        PayoutDestinationTokenAccountOwnerAddress::try_from("wallet-123")
            .expect("address should be valid")
    }

    #[test]
    fn authorization_plan_requires_target_user_owner() {
        let handler = PayoutDestinationRegisterCommandHandler::new(
            TestPayoutDestinationRepository::default(),
            PassingValidator,
        );
        let user_id = UserId::new();

        let plan = handler
            .authorization_plan(&PayoutDestinationRegisterCommand {
                owner: PayoutDestinationOwner::User(user_id),
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
        let handler = PayoutDestinationRegisterCommandHandler::new(
            TestPayoutDestinationRepository::default(),
            PassingValidator,
        );
        let organization_id = OrganizationId::new();

        let plan = handler
            .authorization_plan(&PayoutDestinationRegisterCommand {
                owner: PayoutDestinationOwner::Organization(organization_id),
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
    async fn handle_registers_payout_destination() {
        let repository = TestPayoutDestinationRepository::default();
        let handler =
            PayoutDestinationRegisterCommandHandler::new(repository.clone(), PassingValidator);
        let user_id = UserId::new();
        let request_context = request_context(user_id);

        let handled = handler
            .handle(
                &mut TestUow,
                &request_context,
                &PayoutDestinationRegisterCommand {
                    owner: PayoutDestinationOwner::User(user_id),
                    token_account_owner_address: token_account_owner_address(),
                },
            )
            .await
            .expect("command should succeed");

        let output = handled.into_output();
        let aggregate = repository
            .payout_destination
            .lock()
            .expect("lock")
            .clone()
            .expect("aggregate should be saved");

        assert_eq!(
            output,
            PayoutDestinationRegisterOutput::new(
                aggregate.aggregate_id().expect("aggregate id should exist")
            )
        );
    }

    #[tokio::test]
    async fn handle_returns_validation_error_when_address_is_invalid() {
        let repository = TestPayoutDestinationRepository::default();
        let handler =
            PayoutDestinationRegisterCommandHandler::new(repository.clone(), RejectingValidator);
        let user_id = UserId::new();
        let request_context = request_context(user_id);

        let error = handler
            .handle(
                &mut TestUow,
                &request_context,
                &PayoutDestinationRegisterCommand {
                    owner: PayoutDestinationOwner::User(user_id),
                    token_account_owner_address: token_account_owner_address(),
                },
            )
            .await
            .expect_err("command should fail");

        assert!(matches!(
            error,
            PayoutDestinationRegisterCommandHandlerError::TokenAccountOwnerAddressValidator(
                TokenAccountOwnerAddressValidatorError::InvalidAddress
            )
        ));
        assert!(
            repository
                .payout_destination
                .lock()
                .expect("lock")
                .is_none()
        );
    }
}
