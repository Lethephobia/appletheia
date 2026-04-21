use appletheia::application::authorization::{Relation, RelationName, RelationRef, UsersetExpr};
use appletheia::domain::Aggregate;

use super::{Account, AccountStatusManagerRelation};

/// Allows status managers to thaw an account.
pub struct AccountThawerRelation;

impl Relation for AccountThawerRelation {
    const REF: RelationRef = RelationRef::new(Account::TYPE, RelationName::new("thawer"));

    const EXPR: UsersetExpr = UsersetExpr::ComputedUserset {
        relation: AccountStatusManagerRelation::REF,
    };
}
