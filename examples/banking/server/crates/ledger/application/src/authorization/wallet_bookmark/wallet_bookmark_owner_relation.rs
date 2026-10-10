use appletheia::application::aggregate::AggregateRef;
use appletheia::application::authorization::{
    Relation, RelationName, RelationRef, RelationshipEntries, RelationshipEntry,
    RelationshipSubject, UsersetExpr,
};
use appletheia::domain::Aggregate;
use banking_iam_application::authorization::OrganizationOwnerRelation;
use banking_iam_domain::{Organization, User};
use banking_ledger_domain::wallet_bookmark::{WalletBookmark, WalletBookmarkOwner};

use super::WalletBookmarkOwnerDerivationHandlerError;

/// Allows the owning user or the owner of the owning organization.
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
                        entries.insert_entry(RelationshipEntry::from_parts(
                            AggregateRef::from_id::<WalletBookmark>(aggregate.aggregate_id()),
                            RelationshipSubject::aggregate_set::<Organization>(
                                id,
                                OrganizationOwnerRelation::REF,
                            ),
                        ))
                    }
                };
                Ok(entries)
            },
        )
    }
}
