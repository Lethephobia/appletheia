use super::WalletBookmarkOwnerRelation;
use crate::authorization::OrganizationFinanceManagerRelation;
use appletheia::application::authorization::{Relation, RelationName, RelationRef, UsersetExpr};
use appletheia::domain::Aggregate;
use banking_ledger_domain::wallet_bookmark::WalletBookmark;

pub struct WalletBookmarkManagerRelation;

impl Relation for WalletBookmarkManagerRelation {
    const REF: RelationRef = RelationRef::new(WalletBookmark::TYPE, RelationName::new("manager"));

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
