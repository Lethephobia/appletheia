use appletheia::application::authorization::{
    AuthorizationPlan, PrincipalRequirement, Relation, RelationshipRequirement,
};
use appletheia::application::command::CommandHandler;
use appletheia::application::repository::Repository;
use appletheia::application::request_context::RequestContext;
use banking_ledger_domain::wallet_bookmark::{WalletBookmark, WalletBookmarkRemoveResult};

use super::{
    WalletBookmarkRemoveCommand, WalletBookmarkRemoveCommandHandlerError,
    WalletBookmarkRemoveOutput,
};
use crate::authorization::WalletBookmarkRemoverRelation;

/// Handles `WalletBookmarkRemoveCommand`.
pub struct WalletBookmarkRemoveCommandHandler<WBR>
where
    WBR: Repository<WalletBookmark>,
{
    wallet_bookmark_repository: WBR,
}

impl<WBR> WalletBookmarkRemoveCommandHandler<WBR>
where
    WBR: Repository<WalletBookmark>,
{
    pub fn new(wallet_bookmark_repository: WBR) -> Self {
        Self {
            wallet_bookmark_repository,
        }
    }
}

impl<WBR> CommandHandler for WalletBookmarkRemoveCommandHandler<WBR>
where
    WBR: Repository<WalletBookmark>,
{
    type Command = WalletBookmarkRemoveCommand;
    type Output = WalletBookmarkRemoveOutput;
    type Error = WalletBookmarkRemoveCommandHandlerError;
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
                WalletBookmarkRemoverRelation::REF,
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

        let result = wallet_bookmark.remove()?;

        self.wallet_bookmark_repository
            .save(uow, request_context, &mut wallet_bookmark)
            .await?;

        let output = match result {
            WalletBookmarkRemoveResult::Removed => WalletBookmarkRemoveOutput::Removed,
            WalletBookmarkRemoveResult::Rejected { reason } => {
                WalletBookmarkRemoveOutput::Rejected { reason }
            }
        };

        Ok(output)
    }
}
