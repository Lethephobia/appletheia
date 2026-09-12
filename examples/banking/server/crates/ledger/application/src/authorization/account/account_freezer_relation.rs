use appletheia::application::authorization::{Relation, RelationName, RelationRef, UsersetExpr};
use appletheia::domain::Aggregate;

use super::{Account, AccountStatusManagerRelation};

/// Allows status managers to freeze an account.
pub struct AccountFreezerRelation;

impl Relation for AccountFreezerRelation {
    const REF: RelationRef = RelationRef::new(Account::TYPE, RelationName::new("freezer"));

    fn expr(&self) -> UsersetExpr {
        UsersetExpr::ComputedUserset {
            relation: AccountStatusManagerRelation::REF.into(),
        }
    }
}
