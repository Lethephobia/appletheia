use appletheia::application::authorization::{
    AuthorizationPlan, PrincipalRequirement, Relation, RelationshipRequirement,
};
use appletheia::application::command::{CommandHandled, CommandHandler};
use appletheia::application::repository::Repository;
use appletheia::application::request_context::RequestContext;
use banking_ledger_domain::wallet_bookmark::{
    WalletBookmark, WalletBookmarkDescriptionChangeResult,
};

use super::{
    WalletBookmarkDescriptionChangeCommand, WalletBookmarkDescriptionChangeCommandHandlerError,
    WalletBookmarkDescriptionChangeOutput,
};
use crate::authorization::WalletBookmarkUpdaterRelation;

/// Handles `WalletBookmarkDescriptionChangeCommand`.
pub struct WalletBookmarkDescriptionChangeCommandHandler<WBR>
where
    WBR: Repository<WalletBookmark>,
{
    wallet_bookmark_repository: WBR,
}

impl<WBR> WalletBookmarkDescriptionChangeCommandHandler<WBR>
where
    WBR: Repository<WalletBookmark>,
{
    pub fn new(wallet_bookmark_repository: WBR) -> Self {
        Self {
            wallet_bookmark_repository,
        }
    }
}

impl<WBR> CommandHandler for WalletBookmarkDescriptionChangeCommandHandler<WBR>
where
    WBR: Repository<WalletBookmark>,
{
    type Command = WalletBookmarkDescriptionChangeCommand;
    type Output = WalletBookmarkDescriptionChangeOutput;
    type ReplayOutput = WalletBookmarkDescriptionChangeOutput;
    type Error = WalletBookmarkDescriptionChangeCommandHandlerError;
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
    ) -> Result<CommandHandled<Self::Output, Self::ReplayOutput>, Self::Error> {
        let mut wallet_bookmark = self
            .wallet_bookmark_repository
            .read(uow, command.wallet_bookmark_id)
            .await?;

        let result = wallet_bookmark.change_description(command.description.clone())?;

        self.wallet_bookmark_repository
            .save(uow, request_context, &mut wallet_bookmark)
            .await?;

        let output = match result {
            WalletBookmarkDescriptionChangeResult::Changed => {
                WalletBookmarkDescriptionChangeOutput::Changed
            }
            WalletBookmarkDescriptionChangeResult::Rejected { reason } => {
                WalletBookmarkDescriptionChangeOutput::Rejected { reason }
            }
        };

        Ok(CommandHandled::same(output))
    }
}
