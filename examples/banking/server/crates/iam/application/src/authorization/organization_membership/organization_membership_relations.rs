use appletheia::relations;
use banking_iam_domain::OrganizationMembership;

use super::{
    OrganizationMembershipActivatorRelation, OrganizationMembershipDeactivatorRelation,
    OrganizationMembershipOrganizationRelation, OrganizationMembershipRemoverRelation,
    OrganizationMembershipStatusManagerRelation,
};

/// Defines static authorization relations for `OrganizationMembership`.
#[relations(
    aggregate = OrganizationMembership,
    relations = [
        OrganizationMembershipOrganizationRelation,
        OrganizationMembershipStatusManagerRelation,
        OrganizationMembershipActivatorRelation,
        OrganizationMembershipDeactivatorRelation,
        OrganizationMembershipRemoverRelation
    ]
)]
pub struct OrganizationMembershipRelations;

#[cfg(test)]
mod tests {
    use appletheia::application::authorization::{
        Relation, RelationNameOwned, Relations, UsersetExpr,
    };

    use super::{
        OrganizationMembershipActivatorRelation, OrganizationMembershipDeactivatorRelation,
        OrganizationMembershipOrganizationRelation, OrganizationMembershipRelations,
        OrganizationMembershipRemoverRelation, OrganizationMembershipStatusManagerRelation,
    };

    #[test]
    fn organization_membership_relations_define_expected_expressions() {
        let definition = OrganizationMembershipRelations.build();
        let organization =
            RelationNameOwned::from(OrganizationMembershipOrganizationRelation::NAME);
        let status_manager =
            RelationNameOwned::from(OrganizationMembershipStatusManagerRelation::NAME);
        let activator = RelationNameOwned::from(OrganizationMembershipActivatorRelation::NAME);
        let deactivator = RelationNameOwned::from(OrganizationMembershipDeactivatorRelation::NAME);
        let remover = RelationNameOwned::from(OrganizationMembershipRemoverRelation::NAME);

        assert_eq!(definition.expr_for(&organization), Some(&UsersetExpr::This));
        assert_eq!(
            definition.expr_for(&status_manager),
            Some(&UsersetExpr::TupleToUserset {
                tupleset_relation: organization.clone(),
                computed_relation: RelationNameOwned::from(crate::OrganizationOwnerRelation::NAME,),
            })
        );
        assert_eq!(
            definition.expr_for(&activator),
            Some(&UsersetExpr::Union(vec![
                UsersetExpr::This,
                UsersetExpr::ComputedUserset {
                    relation: status_manager.clone(),
                },
            ]))
        );
        assert_eq!(
            definition.expr_for(&deactivator),
            Some(&UsersetExpr::Union(vec![
                UsersetExpr::This,
                UsersetExpr::ComputedUserset {
                    relation: status_manager.clone(),
                },
            ]))
        );
        assert_eq!(
            definition.expr_for(&remover),
            Some(&UsersetExpr::Union(vec![
                UsersetExpr::This,
                UsersetExpr::ComputedUserset {
                    relation: status_manager
                },
            ]))
        );
    }
}
