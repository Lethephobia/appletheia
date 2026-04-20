use appletheia::application::authorization::{Relation, RelationName, RelationRef, UsersetExpr};
use appletheia::domain::Aggregate;

use super::{Organization, OrganizationOwnerRelation};

/// Allows elevated finance managers and the owner.
pub struct OrganizationFinanceManagerRelation;

impl Relation for OrganizationFinanceManagerRelation {
    const REF: RelationRef =
        RelationRef::new(Organization::TYPE, RelationName::new("finance_manager"));

    const EXPR: UsersetExpr = UsersetExpr::Union(&[
        UsersetExpr::This,
        UsersetExpr::ComputedUserset {
            relation: OrganizationOwnerRelation::REF,
        },
    ]);
}
