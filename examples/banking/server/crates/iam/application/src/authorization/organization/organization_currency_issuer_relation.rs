use appletheia::application::authorization::{Relation, RelationName, RelationRef, UsersetExpr};
use appletheia::domain::Aggregate;

use super::{Organization, OrganizationOwnerRelation};

/// Allows organization owners to issue supply for organization-owned currencies.
pub struct OrganizationCurrencyIssuerRelation;

impl Relation for OrganizationCurrencyIssuerRelation {
    const REF: RelationRef =
        RelationRef::new(Organization::TYPE, RelationName::new("currency_issuer"));

    const EXPR: UsersetExpr = UsersetExpr::Union(&[
        UsersetExpr::This,
        UsersetExpr::ComputedUserset {
            relation: OrganizationOwnerRelation::REF,
        },
    ]);
}
