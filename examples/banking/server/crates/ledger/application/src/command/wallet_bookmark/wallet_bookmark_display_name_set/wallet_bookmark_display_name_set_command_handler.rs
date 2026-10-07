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

pub struct WalletBookmarkDisplayNameSetCommandHandler<R>
where
    R: Repository,
{
    repository: R,
}

impl<R> WalletBookmarkDisplayNameSetCommandHandler<R>
where
    R: Repository,
{
    pub fn new(repository: R) -> Self {
        Self { repository }
    }
}

impl<R> CommandHandler for WalletBookmarkDisplayNameSetCommandHandler<R>
where
    R: Repository,
{
    type Command = WalletBookmarkDisplayNameSetCommand;
    type Output = WalletBookmarkDisplayNameSetOutput;
    type Error = WalletBookmarkDisplayNameSetCommandHandlerError;
    type Uow = R::Uow;

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
            .repository
            .read::<WalletBookmark>(uow, command.wallet_bookmark_id)
            .await?;

        wallet_bookmark.set_display_name(command.display_name.clone())?;

        self.repository
            .save(uow, request_context, &mut wallet_bookmark)
            .await?;

        Ok(WalletBookmarkDisplayNameSetOutput {})
    }
}
