use appletheia::application::authorization::{Relation, RelationName, RelationRef, UsersetExpr};
use appletheia::domain::Aggregate;

use super::{Organization, OrganizationOwnerRelation};

/// Allows organization owners to update organization-owned currency definitions.
pub struct OrganizationCurrencyDefinitionUpdaterRelation;

impl Relation for OrganizationCurrencyDefinitionUpdaterRelation {
    const REF: RelationRef = RelationRef::new(
        Organization::TYPE,
        RelationName::new("currency_definition_updater"),
    );

    const EXPR: UsersetExpr = UsersetExpr::Union(&[
        UsersetExpr::This,
        UsersetExpr::ComputedUserset {
            relation: OrganizationOwnerRelation::REF,
        },
    ]);
}
