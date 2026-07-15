use appletheia::application::authorization::{
    Relation, Relationship, RelationshipChange, RelationshipStore, RelationshipSubject,
};
use banking_iam_domain::{Organization, User};
use banking_ledger_domain::wallet_bookmark::{
    WalletBookmark, WalletBookmarkId, WalletBookmarkOwner,
};

use super::{
    WalletBookmarkOwnerRelation, WalletBookmarkRelationshipUpdater,
    WalletBookmarkRelationshipUpdaterError,
};

pub struct DefaultWalletBookmarkRelationshipUpdater<RS>
where
    RS: RelationshipStore,
{
    relationship_store: RS,
}

impl<RS> DefaultWalletBookmarkRelationshipUpdater<RS>
where
    RS: RelationshipStore,
{
    pub fn new(relationship_store: RS) -> Self {
        Self { relationship_store }
    }

    fn owner_subject(owner: WalletBookmarkOwner) -> RelationshipSubject {
        match owner {
            WalletBookmarkOwner::User(user_id) => RelationshipSubject::aggregate::<User>(user_id),
            WalletBookmarkOwner::Organization(organization_id) => {
                RelationshipSubject::aggregate::<Organization>(organization_id)
            }
        }
    }
}

impl<RS> WalletBookmarkRelationshipUpdater for DefaultWalletBookmarkRelationshipUpdater<RS>
where
    RS: RelationshipStore,
{
    type Uow = RS::Uow;

    async fn upsert_owner(
        &self,
        uow: &mut Self::Uow,
        wallet_bookmark_id: WalletBookmarkId,
        owner: WalletBookmarkOwner,
    ) -> Result<(), WalletBookmarkRelationshipUpdaterError> {
        self.relationship_store
            .apply_changes(
                uow,
                &[RelationshipChange::Upsert(Relationship::new::<
                    WalletBookmark,
                >(
                    wallet_bookmark_id,
                    WalletBookmarkOwnerRelation::REF,
                    Self::owner_subject(owner),
                ))],
            )
            .await?;

        Ok(())
    }
}
