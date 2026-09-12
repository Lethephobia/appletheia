use super::WalletBookmarkOwnerDerivationHandlerError;

use appletheia::application::authorization::{
    Relation, RelationName, RelationRef, RelationshipEntries, UsersetExpr,
};
use appletheia::domain::Aggregate;
use banking_iam_domain::{Organization, User};
use banking_ledger_domain::wallet_bookmark::{WalletBookmark, WalletBookmarkOwner};

/// Allows the owning subject itself.
pub struct WalletBookmarkOwnerRelation;

impl Relation for WalletBookmarkOwnerRelation {
    const REF: RelationRef = RelationRef::new(WalletBookmark::TYPE, RelationName::new("owner"));

    fn expr(&self) -> UsersetExpr {
        UsersetExpr::this::<WalletBookmark, _, WalletBookmarkOwnerDerivationHandlerError>(
            |aggregate| {
                let mut entries = RelationshipEntries::new();
                match *aggregate.owner()? {
                    WalletBookmarkOwner::User(id) => {
                        entries.insert::<WalletBookmark, User>(aggregate.aggregate_id(), id)
                    }
                    WalletBookmarkOwner::Organization(id) => {
                        entries.insert::<WalletBookmark, Organization>(aggregate.aggregate_id(), id)
                    }
                };
                Ok(entries)
            },
        )
    }
}
