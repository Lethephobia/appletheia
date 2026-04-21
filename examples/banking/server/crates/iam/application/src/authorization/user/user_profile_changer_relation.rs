use appletheia::application::authorization::{Relation, RelationName, RelationRef, UsersetExpr};
use appletheia::domain::Aggregate;

use super::{User, UserOwnerRelation};

/// Defines the `profile_changer` relation for `User`.
pub struct UserProfileChangerRelation;

impl Relation for UserProfileChangerRelation {
    const REF: RelationRef = RelationRef::new(User::TYPE, RelationName::new("profile_changer"));

    const EXPR: UsersetExpr = UsersetExpr::ComputedUserset {
        relation: UserOwnerRelation::REF,
    };
}
