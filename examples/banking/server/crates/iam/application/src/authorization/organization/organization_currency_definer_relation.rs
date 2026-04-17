use appletheia::application::authorization::{Relation, RelationName, RelationRef, UsersetExpr};
use appletheia::domain::Aggregate;

use super::{Organization, OrganizationOwnerRelation};

/// Allows organization owners to define currencies.
pub struct OrganizationCurrencyDefinerRelation;

impl Relation for OrganizationCurrencyDefinerRelation {
    const REF: RelationRef =
        RelationRef::new(Organization::TYPE, RelationName::new("currency_definer"));

    const EXPR: UsersetExpr = UsersetExpr::ComputedUserset {
        relation: OrganizationOwnerRelation::REF,
    };
}
