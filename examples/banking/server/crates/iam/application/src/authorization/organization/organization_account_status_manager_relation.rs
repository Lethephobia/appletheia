use appletheia::application::authorization::{Relation, RelationName, RelationRef, UsersetExpr};
use appletheia::domain::Aggregate;

use super::{Organization, OrganizationOwnerRelation};

/// Allows organization owners to manage organization account status.
pub struct OrganizationAccountStatusManagerRelation;

impl Relation for OrganizationAccountStatusManagerRelation {
    const REF: RelationRef = RelationRef::new(
        Organization::TYPE,
        RelationName::new("account_status_manager"),
    );

    const EXPR: UsersetExpr = UsersetExpr::Union(&[
        UsersetExpr::This,
        UsersetExpr::ComputedUserset {
            relation: OrganizationOwnerRelation::REF,
        },
    ]);
}
