use appletheia::application::authorization::{
    Relation, RelationName, RelationNameOwned, UsersetExpr,
};
use banking_iam_application::OrganizationOwnerRelation;

use super::CurrencyDefinitionOrganizationRelation;

/// Allows organization owners to access the currency definition.
pub struct CurrencyDefinitionOwnerRelation;

impl Relation for CurrencyDefinitionOwnerRelation {
    const NAME: RelationName = RelationName::new("owner");

    fn expr(&self) -> UsersetExpr {
        UsersetExpr::TupleToUserset {
            tupleset_relation: RelationNameOwned::from(
                CurrencyDefinitionOrganizationRelation::NAME,
            ),
            computed_relation: RelationNameOwned::from(OrganizationOwnerRelation::NAME),
        }
    }
}
