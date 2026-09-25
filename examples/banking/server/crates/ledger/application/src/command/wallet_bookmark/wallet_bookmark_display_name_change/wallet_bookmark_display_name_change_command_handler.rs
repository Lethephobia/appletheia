use appletheia::application::authorization::{
    AuthorizationPlan, PrincipalRequirement, Relation, RelationshipRequirement,
};
use appletheia::application::command::CommandHandler;
use appletheia::application::repository::Repository;
use appletheia::application::request_context::RequestContext;
use banking_ledger_domain::wallet_bookmark::{
    WalletBookmark, WalletBookmarkDisplayNameChangeResult,
};

use super::{
    WalletBookmarkDisplayNameChangeCommand, WalletBookmarkDisplayNameChangeCommandHandlerError,
    WalletBookmarkDisplayNameChangeOutput,
};
use crate::authorization::WalletBookmarkUpdaterRelation;

/// Handles `WalletBookmarkDisplayNameChangeCommand`.
pub struct WalletBookmarkDisplayNameChangeCommandHandler<WBR>
where
    WBR: Repository<WalletBookmark>,
{
    wallet_bookmark_repository: WBR,
}

impl<WBR> WalletBookmarkDisplayNameChangeCommandHandler<WBR>
where
    WBR: Repository<WalletBookmark>,
{
    pub fn new(wallet_bookmark_repository: WBR) -> Self {
        Self {
            wallet_bookmark_repository,
        }
    }
}

impl<WBR> CommandHandler for WalletBookmarkDisplayNameChangeCommandHandler<WBR>
where
    WBR: Repository<WalletBookmark>,
{
    type Command = WalletBookmarkDisplayNameChangeCommand;
    type Output = WalletBookmarkDisplayNameChangeOutput;
    type Error = WalletBookmarkDisplayNameChangeCommandHandlerError;
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

        let result = wallet_bookmark.change_display_name(command.display_name.clone())?;

        self.wallet_bookmark_repository
            .save(uow, request_context, &mut wallet_bookmark)
            .await?;

        let output = match result {
            WalletBookmarkDisplayNameChangeResult::Changed => {
                WalletBookmarkDisplayNameChangeOutput::Changed
            }
            WalletBookmarkDisplayNameChangeResult::Rejected { reason } => {
                WalletBookmarkDisplayNameChangeOutput::Rejected { reason }
            }
        };

        Ok(output)
    }
}
