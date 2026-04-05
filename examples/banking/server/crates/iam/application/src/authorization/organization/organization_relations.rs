use appletheia::relations;
use banking_iam_domain::Organization;

use super::{
    OrganizationHandleEditorRelation, OrganizationMemberRelation, OrganizationOwnerRelation,
    OrganizationRemoverRelation, OrganizationRenamerRelation,
};

/// Defines static authorization relations for `Organization`.
#[relations(
    aggregate = Organization,
    relations = [
        OrganizationOwnerRelation,
        OrganizationMemberRelation,
        OrganizationRenamerRelation,
        OrganizationHandleEditorRelation,
        OrganizationRemoverRelation
    ]
)]
pub struct OrganizationRelations;

#[cfg(test)]
mod tests {
    use appletheia::application::authorization::{
        Relation, RelationNameOwned, Relations, UsersetExpr,
    };

    use super::{
        OrganizationHandleEditorRelation, OrganizationMemberRelation, OrganizationOwnerRelation,
        OrganizationRelations, OrganizationRemoverRelation, OrganizationRenamerRelation,
    };

    #[test]
    fn organization_relations_define_expected_expressions() {
        let definition = OrganizationRelations.build();
        let owner = RelationNameOwned::from(OrganizationOwnerRelation::NAME);
        let member = RelationNameOwned::from(OrganizationMemberRelation::NAME);
        let renamer = RelationNameOwned::from(OrganizationRenamerRelation::NAME);
        let handle_editor = RelationNameOwned::from(OrganizationHandleEditorRelation::NAME);
        let remover = RelationNameOwned::from(OrganizationRemoverRelation::NAME);

        assert_eq!(definition.expr_for(&owner), Some(&UsersetExpr::This));
        assert_eq!(
            definition.expr_for(&member),
            Some(&UsersetExpr::Union(vec![
                UsersetExpr::This,
                UsersetExpr::ComputedUserset {
                    relation: owner.clone(),
                },
            ]))
        );
        assert_eq!(
            definition.expr_for(&renamer),
            Some(&UsersetExpr::Union(vec![
                UsersetExpr::This,
                UsersetExpr::ComputedUserset {
                    relation: owner.clone(),
                },
            ]))
        );
        assert_eq!(
            definition.expr_for(&handle_editor),
            Some(&UsersetExpr::Union(vec![
                UsersetExpr::This,
                UsersetExpr::ComputedUserset {
                    relation: owner.clone(),
                },
            ]))
        );
        assert_eq!(
            definition.expr_for(&remover),
            Some(&UsersetExpr::Union(vec![
                UsersetExpr::This,
                UsersetExpr::ComputedUserset { relation: owner },
            ]))
        );
    }
}
