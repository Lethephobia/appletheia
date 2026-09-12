use appletheia::application::authorization::{Relation, RelationName, RelationRef, UsersetExpr};
use appletheia::domain::Aggregate;
use banking_iam_application::OrganizationFinanceManagerRelation;

use super::{WalletBookmark, WalletBookmarkOwnerRelation};

/// Allows owners to remove a wallet bookmark.
pub struct WalletBookmarkRemoverRelation;

impl Relation for WalletBookmarkRemoverRelation {
    const REF: RelationRef = RelationRef::new(WalletBookmark::TYPE, RelationName::new("remover"));

    fn expr(&self) -> UsersetExpr {
        UsersetExpr::Union(vec![
            UsersetExpr::ComputedUserset {
                relation: WalletBookmarkOwnerRelation::REF.into(),
            },
            UsersetExpr::TupleToUserset {
                tupleset_relation: WalletBookmarkOwnerRelation::REF.into(),
                computed_userset: OrganizationFinanceManagerRelation::REF.into(),
            },
        ])
    }
}
