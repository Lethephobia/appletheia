use appletheia::relations;
use banking_ledger_domain::currency_definition::CurrencyDefinition;

use super::{
    CurrencyDefinitionActivatorRelation, CurrencyDefinitionDeactivatorRelation,
    CurrencyDefinitionOrganizationRelation, CurrencyDefinitionOwnerRelation,
    CurrencyDefinitionRemoverRelation, CurrencyDefinitionStatusManagerRelation,
    CurrencyDefinitionUpdaterRelation,
};

/// Defines static authorization relations for `CurrencyDefinition`.
#[relations(
    aggregate = CurrencyDefinition,
    relations = [
        CurrencyDefinitionOrganizationRelation,
        CurrencyDefinitionOwnerRelation,
        CurrencyDefinitionStatusManagerRelation,
        CurrencyDefinitionUpdaterRelation,
        CurrencyDefinitionActivatorRelation,
        CurrencyDefinitionDeactivatorRelation,
        CurrencyDefinitionRemoverRelation
    ]
)]
pub struct CurrencyDefinitionRelations;

#[cfg(test)]
mod tests {
    use appletheia::application::authorization::{
        Relation, RelationNameOwned, Relations, UsersetExpr,
    };
    use banking_iam_application::OrganizationOwnerRelation;

    use super::{
        CurrencyDefinitionActivatorRelation, CurrencyDefinitionDeactivatorRelation,
        CurrencyDefinitionOrganizationRelation, CurrencyDefinitionOwnerRelation,
        CurrencyDefinitionRelations, CurrencyDefinitionRemoverRelation,
        CurrencyDefinitionStatusManagerRelation, CurrencyDefinitionUpdaterRelation,
    };

    #[test]
    fn currency_definition_relations_define_expected_expressions() {
        let definition = CurrencyDefinitionRelations.build();
        let organization = RelationNameOwned::from(CurrencyDefinitionOrganizationRelation::NAME);
        let owner = RelationNameOwned::from(CurrencyDefinitionOwnerRelation::NAME);
        let organization_owner = RelationNameOwned::from(OrganizationOwnerRelation::NAME);
        let status_manager = RelationNameOwned::from(CurrencyDefinitionStatusManagerRelation::NAME);
        let updater = RelationNameOwned::from(CurrencyDefinitionUpdaterRelation::NAME);
        let activator = RelationNameOwned::from(CurrencyDefinitionActivatorRelation::NAME);
        let deactivator = RelationNameOwned::from(CurrencyDefinitionDeactivatorRelation::NAME);
        let remover = RelationNameOwned::from(CurrencyDefinitionRemoverRelation::NAME);

        assert_eq!(definition.expr_for(&organization), Some(&UsersetExpr::This));
        assert_eq!(
            definition.expr_for(&status_manager),
            Some(&UsersetExpr::Union(vec![
                UsersetExpr::This,
                UsersetExpr::ComputedUserset {
                    relation: owner.clone(),
                }
            ]))
        );
        assert_eq!(
            definition.expr_for(&owner),
            Some(&UsersetExpr::TupleToUserset {
                tupleset_relation: organization.clone(),
                computed_relation: organization_owner.clone(),
            })
        );
        assert_eq!(
            definition.expr_for(&updater),
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
                    relation: status_manager.clone(),
                },
            ]))
        );
        assert_eq!(
            definition.expr_for(&remover),
            Some(&UsersetExpr::Union(vec![
                UsersetExpr::This,
                UsersetExpr::ComputedUserset {
                    relation: status_manager,
                },
            ]))
        );
    }
}
