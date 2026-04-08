use appletheia::application::authorization::{Relation, RelationName, RelationRef, UsersetExpr};
use appletheia::domain::Aggregate;

use super::{Organization, OrganizationOwnerRelation};

/// Allows organization owners to manage organization currency-definition status.
pub struct OrganizationCurrencyDefinitionStatusManagerRelation;

impl Relation for OrganizationCurrencyDefinitionStatusManagerRelation {
    const REF: RelationRef = RelationRef::new(
        Organization::TYPE,
        RelationName::new("currency_definition_status_manager"),
    );

    const EXPR: UsersetExpr = UsersetExpr::Union(&[
        UsersetExpr::This,
        UsersetExpr::ComputedUserset {
            relation: OrganizationOwnerRelation::REF,
        },
    ]);
}
