use appletheia::application::authorization::{Relation, RelationName, RelationRef, UsersetExpr};
use appletheia::domain::Aggregate;

use super::{Organization, OrganizationOwnerRelation};

/// Allows organization owners to manage organization currency status.
pub struct OrganizationCurrencyStatusManagerRelation;

impl Relation for OrganizationCurrencyStatusManagerRelation {
    const REF: RelationRef = RelationRef::new(
        Organization::TYPE,
        RelationName::new("currency_status_manager"),
    );

    const EXPR: UsersetExpr = UsersetExpr::Union(&[
        UsersetExpr::This,
        UsersetExpr::ComputedUserset {
            relation: OrganizationOwnerRelation::REF,
        },
    ]);
}
