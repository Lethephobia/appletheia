use appletheia::application::authorization::{
    AuthorizationPlan, PrincipalRequirement, Relation, RelationshipRequirement,
};
use appletheia::application::command::CommandHandler;
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

/// Handles `WalletBookmarkRegisterCommand`.
pub struct WalletBookmarkRegisterCommandHandler<WBR>
where
    WBR: Repository<WalletBookmark>,
{
    wallet_bookmark_repository: WBR,
}

impl<WBR> WalletBookmarkRegisterCommandHandler<WBR>
where
    WBR: Repository<WalletBookmark>,
{
    pub fn new(wallet_bookmark_repository: WBR) -> Self {
        Self {
            wallet_bookmark_repository,
        }
    }
}

impl<WBR> CommandHandler for WalletBookmarkRegisterCommandHandler<WBR>
where
    WBR: Repository<WalletBookmark>,
{
    type Command = WalletBookmarkRegisterCommand;
    type Output = WalletBookmarkRegisterOutput;
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
    ) -> Result<Self::Output, Self::Error> {
        let mut wallet_bookmark = WalletBookmark::new();
        let wallet_bookmark_id = wallet_bookmark.aggregate_id();
        let result = wallet_bookmark.register(WalletBookmarkRegistration {
            owner: command.owner,
            display_name: command.display_name.clone(),
            description: command.description.clone(),
            token_owner_address: command.token_owner_address,
        })?;

        self.wallet_bookmark_repository
            .save(uow, request_context, &mut wallet_bookmark)
            .await?;

        Ok(match result {
            WalletBookmarkRegisterResult::Registered => {
                WalletBookmarkRegisterOutput::Registered { wallet_bookmark_id }
            }
            WalletBookmarkRegisterResult::Rejected { reason } => {
                WalletBookmarkRegisterOutput::Rejected {
                    wallet_bookmark_id,
                    reason,
                }
            }
        })
    }
}
