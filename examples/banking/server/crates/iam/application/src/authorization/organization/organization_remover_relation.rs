use appletheia::application::authorization::{Relation, RelationName, RelationRef, UsersetExpr};
use appletheia::domain::Aggregate;

use super::{Organization, OrganizationOwnerRelation};

/// Allows owners to remove an organization.
pub struct OrganizationRemoverRelation;

impl Relation for OrganizationRemoverRelation {
    const REF: RelationRef = RelationRef::new(Organization::TYPE, RelationName::new("remover"));

    const EXPR: UsersetExpr = UsersetExpr::Union(&[
        UsersetExpr::This,
        UsersetExpr::ComputedUserset {
            relation: OrganizationOwnerRelation::REF,
        },
    ]);
}
