use appletheia::application::authorization::{
    AuthorizationPlan, PrincipalRequirement, Relation, RelationshipRequirement,
};
use appletheia::application::command::CommandHandler;
use appletheia::application::repository::Repository;
use appletheia::application::request_context::RequestContext;
use banking_ledger_domain::wallet_bookmark::WalletBookmark;

use super::{
    WalletBookmarkDisplayNameSetCommand, WalletBookmarkDisplayNameSetCommandHandlerError,
    WalletBookmarkDisplayNameSetOutput,
};
use crate::authorization::WalletBookmarkUpdaterRelation;

pub struct WalletBookmarkDisplayNameSetCommandHandler<WBR>
where
    WBR: Repository<WalletBookmark>,
{
    wallet_bookmark_repository: WBR,
}

impl<WBR> WalletBookmarkDisplayNameSetCommandHandler<WBR>
where
    WBR: Repository<WalletBookmark>,
{
    pub fn new(wallet_bookmark_repository: WBR) -> Self {
        Self {
            wallet_bookmark_repository,
        }
    }
}

impl<WBR> CommandHandler for WalletBookmarkDisplayNameSetCommandHandler<WBR>
where
    WBR: Repository<WalletBookmark>,
{
    type Command = WalletBookmarkDisplayNameSetCommand;
    type Output = WalletBookmarkDisplayNameSetOutput;
    type Error = WalletBookmarkDisplayNameSetCommandHandlerError;
    type Uow = WBR::Uow;

    fn authorization_plan(
        &self,
        command: &Self::Command,
    ) -> Result<AuthorizationPlan, Self::Error> {
        Ok(AuthorizationPlan::OnlyPrincipals(vec![
            PrincipalRequirement::AuthenticatedWithRelationship(RelationshipRequirement::check::<
                WalletBookmark,
            >(
                command.wallet_bookmark_id,
                WalletBookmarkUpdaterRelation::REF,
            )),
        ]))
    }

    async fn handle(
        &self,
        uow: &mut Self::Uow,
        request_context: &RequestContext,
        command: &Self::Command,
    ) -> Result<Self::Output, Self::Error> {
        let mut wallet_bookmark = self
            .wallet_bookmark_repository
            .read(uow, command.wallet_bookmark_id)
            .await?;

        wallet_bookmark.set_display_name(command.display_name.clone())?;

        self.wallet_bookmark_repository
            .save(uow, request_context, &mut wallet_bookmark)
            .await?;

        Ok(WalletBookmarkDisplayNameSetOutput {})
    }
}
