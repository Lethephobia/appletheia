use appletheia::relations;
use banking_iam_domain::OrganizationInvitation;

use super::{
    OrganizationInvitationCancelerRelation, OrganizationInvitationInviteeRelation,
    OrganizationInvitationOrganizationRelation,
};

/// Defines static authorization relations for `OrganizationInvitation`.
#[relations(
    aggregate = OrganizationInvitation,
    relations = [
        OrganizationInvitationOrganizationRelation,
        OrganizationInvitationInviteeRelation,
        OrganizationInvitationCancelerRelation
    ]
)]
pub struct OrganizationInvitationRelations;

#[cfg(test)]
mod tests {
    use appletheia::application::authorization::{
        Relation, RelationNameOwned, Relations, UsersetExpr,
    };

    use super::{
        OrganizationInvitationCancelerRelation, OrganizationInvitationInviteeRelation,
        OrganizationInvitationOrganizationRelation, OrganizationInvitationRelations,
    };

    #[test]
    fn organization_invitation_relations_define_expected_expressions() {
        let definition = OrganizationInvitationRelations.build();
        let organization =
            RelationNameOwned::from(OrganizationInvitationOrganizationRelation::NAME);
        let invitee = RelationNameOwned::from(OrganizationInvitationInviteeRelation::NAME);
        let canceler = RelationNameOwned::from(OrganizationInvitationCancelerRelation::NAME);

        assert_eq!(definition.expr_for(&organization), Some(&UsersetExpr::This));
        assert_eq!(definition.expr_for(&invitee), Some(&UsersetExpr::This));
        assert_eq!(
            definition.expr_for(&canceler),
            Some(&UsersetExpr::TupleToUserset {
                tupleset_relation: organization.clone(),
                computed_relation: RelationNameOwned::from(
                    crate::OrganizationInviterRelation::NAME,
                ),
            })
        );
    }
}
