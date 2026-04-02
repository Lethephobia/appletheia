use appletheia::relations;
use banking_iam_domain::Organization;

use super::OrganizationOwnerRelation;

/// Defines static authorization relations for `Organization`.
#[relations(
    aggregate = Organization,
    relations = [OrganizationOwnerRelation]
)]
pub struct OrganizationRelations;

#[cfg(test)]
mod tests {
    use appletheia::application::authorization::{
        Relation, RelationNameOwned, Relations, UsersetExpr,
    };

    use super::{OrganizationOwnerRelation, OrganizationRelations};

    #[test]
    fn organization_relations_define_expected_expressions() {
        let definition = OrganizationRelations.build();
        let owner = RelationNameOwned::from(OrganizationOwnerRelation::NAME);

        assert_eq!(definition.expr_for(&owner), Some(&UsersetExpr::This));
    }
}
