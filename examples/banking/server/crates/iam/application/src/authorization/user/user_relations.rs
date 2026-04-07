use appletheia::relations;
use banking_iam_domain::User;

use super::{
    UserActivatorRelation, UserDeactivatorRelation, UserOwnerRelation, UserProfileEditorRelation,
    UserRemoverRelation, UserStatusManagerRelation,
};

/// Defines static authorization relations for `User`.
#[relations(
    aggregate = User,
    relations = [
        UserOwnerRelation,
        UserStatusManagerRelation,
        UserProfileEditorRelation,
        UserActivatorRelation,
        UserDeactivatorRelation,
        UserRemoverRelation
    ]
)]
pub struct UserRelations;

#[cfg(test)]
mod tests {
    use appletheia::application::authorization::{
        Relation, RelationNameOwned, Relations, UsersetExpr,
    };

    use super::{
        UserActivatorRelation, UserDeactivatorRelation, UserOwnerRelation,
        UserProfileEditorRelation, UserRelations, UserRemoverRelation, UserStatusManagerRelation,
    };

    #[test]
    fn user_relations_define_expected_expressions() {
        let definition = UserRelations.build();
        let owner = RelationNameOwned::from(UserOwnerRelation::NAME);
        let status_manager = RelationNameOwned::from(UserStatusManagerRelation::NAME);
        let profile_editor = RelationNameOwned::from(UserProfileEditorRelation::NAME);
        let activator = RelationNameOwned::from(UserActivatorRelation::NAME);
        let deactivator = RelationNameOwned::from(UserDeactivatorRelation::NAME);
        let remover = RelationNameOwned::from(UserRemoverRelation::NAME);

        assert_eq!(definition.expr_for(&owner), Some(&UsersetExpr::This));
        assert_eq!(
            definition.expr_for(&status_manager),
            Some(&UsersetExpr::This)
        );
        assert_eq!(
            definition.expr_for(&profile_editor),
            Some(&UsersetExpr::Union(vec![
                UsersetExpr::This,
                UsersetExpr::ComputedUserset {
                    relation: owner.clone(),
                },
            ]))
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
                    relation: owner.clone(),
                },
                UsersetExpr::ComputedUserset {
                    relation: status_manager.clone(),
                },
            ]))
        );
        assert_eq!(
            definition.expr_for(&remover),
            Some(&UsersetExpr::Union(vec![
                UsersetExpr::This,
                UsersetExpr::ComputedUserset { relation: owner },
                UsersetExpr::ComputedUserset {
                    relation: status_manager
                },
            ]))
        );
    }
}
