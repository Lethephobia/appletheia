use appletheia::application::authorization::{
    Relation, RelationName, RelationNameOwned, UsersetExpr,
};

use crate::OrganizationOwnerRelation;

/// Allows organization owners to define currency definitions.
pub struct OrganizationCurrencyDefinitionDefinerRelation;

impl Relation for OrganizationCurrencyDefinitionDefinerRelation {
    const NAME: RelationName = RelationName::new("currency_definition_definer");

    fn expr(&self) -> UsersetExpr {
        UsersetExpr::ComputedUserset {
            relation: RelationNameOwned::from(OrganizationOwnerRelation::NAME),
        }
    }
}
