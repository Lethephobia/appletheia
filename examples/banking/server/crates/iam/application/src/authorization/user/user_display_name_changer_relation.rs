use appletheia::application::authorization::{Relation, RelationName, RelationRef, UsersetExpr};
use appletheia::domain::Aggregate;

use super::{User, UserOwnerRelation};

/// Defines the `display_name_changer` relation for `User`.
pub struct UserDisplayNameChangerRelation;

impl Relation for UserDisplayNameChangerRelation {
    const REF: RelationRef =
        RelationRef::new(User::TYPE, RelationName::new("display_name_changer"));

    const EXPR: UsersetExpr = UsersetExpr::ComputedUserset {
        relation: UserOwnerRelation::REF,
    };
}
