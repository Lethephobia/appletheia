use appletheia::relations;
use banking_iam_domain::OrganizationJoinRequest;

use super::{
    OrganizationJoinRequestApproverRelation, OrganizationJoinRequestCancelerRelation,
    OrganizationJoinRequestOrganizationRelation, OrganizationJoinRequestRejecterRelation,
    OrganizationJoinRequestRequesterRelation,
};

/// Defines static authorization relations for `OrganizationJoinRequest`.
#[relations(
    aggregate = OrganizationJoinRequest,
    relations = [
        OrganizationJoinRequestOrganizationRelation,
        OrganizationJoinRequestRequesterRelation,
        OrganizationJoinRequestCancelerRelation,
        OrganizationJoinRequestApproverRelation,
        OrganizationJoinRequestRejecterRelation,
    ]
)]
pub struct OrganizationJoinRequestRelations;

#[cfg(test)]
mod tests {
    use appletheia::application::authorization::{
        Relation, RelationNameOwned, Relations, UsersetExpr,
    };

    use super::{
        OrganizationJoinRequestApproverRelation, OrganizationJoinRequestCancelerRelation,
        OrganizationJoinRequestOrganizationRelation, OrganizationJoinRequestRejecterRelation,
        OrganizationJoinRequestRelations, OrganizationJoinRequestRequesterRelation,
    };

    #[test]
    fn organization_join_request_relations_define_expected_expressions() {
        let definition = OrganizationJoinRequestRelations.build();
        let organization =
            RelationNameOwned::from(OrganizationJoinRequestOrganizationRelation::NAME);
        let requester = RelationNameOwned::from(OrganizationJoinRequestRequesterRelation::NAME);
        let canceler = RelationNameOwned::from(OrganizationJoinRequestCancelerRelation::NAME);
        let approver = RelationNameOwned::from(OrganizationJoinRequestApproverRelation::NAME);
        let rejecter = RelationNameOwned::from(OrganizationJoinRequestRejecterRelation::NAME);

        assert_eq!(definition.expr_for(&organization), Some(&UsersetExpr::This));
        assert_eq!(definition.expr_for(&requester), Some(&UsersetExpr::This));
        assert_eq!(
            definition.expr_for(&canceler),
            Some(&UsersetExpr::ComputedUserset {
                relation: requester.clone(),
            })
        );
        assert_eq!(
            definition.expr_for(&approver),
            Some(&UsersetExpr::TupleToUserset {
                tupleset_relation: organization.clone(),
                computed_relation: RelationNameOwned::from(crate::OrganizationOwnerRelation::NAME,),
            })
        );
        assert_eq!(
            definition.expr_for(&rejecter),
            Some(&UsersetExpr::TupleToUserset {
                tupleset_relation: organization,
                computed_relation: RelationNameOwned::from(crate::OrganizationOwnerRelation::NAME,),
            })
        );
    }
}
