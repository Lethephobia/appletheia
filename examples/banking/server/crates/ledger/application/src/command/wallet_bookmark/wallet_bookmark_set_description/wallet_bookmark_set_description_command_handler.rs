use crate::authorization::WalletBookmarkManagerRelation;
use appletheia::application::authorization::{
    AuthorizationPlan, PrincipalRequirement, Relation, RelationshipRequirement,
};
use appletheia::application::command::CommandHandler;
use appletheia::application::repository::Repository;
use appletheia::application::request_context::RequestContext;
use banking_ledger_domain::wallet_bookmark::WalletBookmark;

use super::{
    WalletBookmarkSetDescriptionCommand, WalletBookmarkSetDescriptionCommandHandlerError,
    WalletBookmarkSetDescriptionOutput,
};

pub struct WalletBookmarkSetDescriptionCommandHandler<R>
where
    R: Repository,
{
    repository: R,
}

impl<R> WalletBookmarkSetDescriptionCommandHandler<R>
where
    R: Repository,
{
    pub fn new(repository: R) -> Self {
        Self { repository }
    }
}

impl<R> CommandHandler for WalletBookmarkSetDescriptionCommandHandler<R>
where
    R: Repository,
{
    type Command = WalletBookmarkSetDescriptionCommand;
    type Output = WalletBookmarkSetDescriptionOutput;
    type Error = WalletBookmarkSetDescriptionCommandHandlerError;
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
                WalletBookmarkManagerRelation::REF,
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

        wallet_bookmark.set_description(command.description.clone())?;

        self.repository
            .save(uow, request_context, &mut wallet_bookmark)
            .await?;

        Ok(WalletBookmarkSetDescriptionOutput {})
    }
}
