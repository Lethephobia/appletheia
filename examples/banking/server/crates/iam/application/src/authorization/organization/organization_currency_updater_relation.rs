use appletheia::application::authorization::{Relation, RelationName, RelationRef, UsersetExpr};
use appletheia::domain::Aggregate;

use super::{Organization, OrganizationOwnerRelation};

/// Allows organization owners to update organization-owned currencies.
pub struct OrganizationCurrencyUpdaterRelation;

impl Relation for OrganizationCurrencyUpdaterRelation {
    const REF: RelationRef =
        RelationRef::new(Organization::TYPE, RelationName::new("currency_updater"));

    const EXPR: UsersetExpr = UsersetExpr::Union(&[
        UsersetExpr::This,
        UsersetExpr::ComputedUserset {
            relation: OrganizationOwnerRelation::REF,
        },
    ]);
}
