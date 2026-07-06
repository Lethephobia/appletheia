use appletheia::application::authorization::{Relation, RelationName, RelationRef, UsersetExpr};
use appletheia::domain::Aggregate;

use super::WalletBookmark;

/// Allows the owning subject itself.
pub struct WalletBookmarkOwnerRelation;

impl Relation for WalletBookmarkOwnerRelation {
    const REF: RelationRef = RelationRef::new(WalletBookmark::TYPE, RelationName::new("owner"));

    const EXPR: UsersetExpr = UsersetExpr::This;
}
