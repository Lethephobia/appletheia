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
use banking_ledger_domain::wallet_bookmark::{WalletBookmark, WalletBookmarkOwner};

use super::{
    WalletBookmarkRegisterCommand, WalletBookmarkRegisterCommandHandlerError,
    WalletBookmarkRegisterOutput,
};

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
        wallet_bookmark.register(command.owner, command.token_owner_address)?;
        if let Some(display_name) = &command.display_name {
            wallet_bookmark.set_display_name(Some(display_name.clone()))?;
        }
        if let Some(description) = &command.description {
            wallet_bookmark.set_description(Some(description.clone()))?;
        }

        self.wallet_bookmark_repository
            .save(uow, request_context, &mut wallet_bookmark)
            .await?;

        Ok(WalletBookmarkRegisterOutput { wallet_bookmark_id })
    }
}
