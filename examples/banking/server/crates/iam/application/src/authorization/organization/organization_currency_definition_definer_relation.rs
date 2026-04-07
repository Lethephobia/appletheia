use appletheia::application::authorization::{Relation, RelationName, RelationRef, UsersetExpr};
use appletheia::domain::Aggregate;

use super::{Organization, OrganizationOwnerRelation};

/// Allows organization owners to define currency definitions.
pub struct OrganizationCurrencyDefinitionDefinerRelation;

impl Relation for OrganizationCurrencyDefinitionDefinerRelation {
    const REF: RelationRef = RelationRef::new(
        Organization::TYPE,
        RelationName::new("currency_definition_definer"),
    );

    const EXPR: UsersetExpr = UsersetExpr::ComputedUserset {
        relation: OrganizationOwnerRelation::REF,
    };
}
