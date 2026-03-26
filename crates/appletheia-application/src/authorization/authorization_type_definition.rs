use std::collections::HashMap;

use super::{Relation, RelationNameOwned, UsersetExpr};

/// Stores the relation definitions for a single authorization type.
///
/// Each relation name maps to the `UsersetExpr` used by the authorizer when it
/// evaluates access on aggregates of that type.
#[derive(Clone, Debug, Default)]
pub struct AuthorizationTypeDefinition {
    relations: HashMap<RelationNameOwned, UsersetExpr>,
}

impl AuthorizationTypeDefinition {
    /// Creates a type definition from pre-built relation expressions.
    pub fn new(relations: HashMap<RelationNameOwned, UsersetExpr>) -> Self {
        Self { relations }
    }

    /// Defines a relation using a dynamically provided name and userset expression.
    ///
    /// This is primarily intended for runtime configuration scenarios, such as
    /// loading authorization models from a database or configuration file.
    pub fn define_relation<R>(&mut self, relation: R, expr: UsersetExpr)
    where
        R: Into<RelationNameOwned>,
    {
        self.relations.insert(relation.into(), expr);
    }

    /// Defines a relation from a statically-typed `Relation`.
    ///
    /// This is primarily intended for in-memory, compile-time configuration of
    /// authorization models. Each relation is represented by its own type
    /// implementing `Relation`.
    pub fn define_static_relation<R>(&mut self, relation: R)
    where
        R: Relation,
    {
        self.relations
            .insert(RelationNameOwned::from(R::NAME), relation.expr());
    }

    /// Returns whether the given relation is defined for this type.
    pub fn is_defined(&self, relation: &RelationNameOwned) -> bool {
        self.relations.contains_key(relation)
    }

    /// Returns the userset expression registered for the given relation.
    pub fn expr_for(&self, relation: &RelationNameOwned) -> Option<&UsersetExpr> {
        self.relations.get(relation)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::AuthorizationTypeDefinition;
    use crate::authorization::{Relation, RelationName, RelationNameOwned, UsersetExpr};

    struct ViewerRelation;

    impl Relation for ViewerRelation {
        const NAME: RelationName = RelationName::new("viewer");

        fn expr(&self) -> UsersetExpr {
            UsersetExpr::This
        }
    }

    #[test]
    fn new_preserves_provided_relations() {
        let relation_name = RelationNameOwned::try_from("viewer").expect("valid relation");
        let expr = UsersetExpr::This;
        let definition = AuthorizationTypeDefinition::new(HashMap::from([(
            relation_name.clone(),
            expr.clone(),
        )]));

        assert!(definition.is_defined(&relation_name));
        assert_eq!(definition.expr_for(&relation_name), Some(&expr));
    }

    #[test]
    fn define_relation_registers_dynamic_relation() {
        let relation_name = RelationNameOwned::try_from("editor").expect("valid relation");
        let expr = UsersetExpr::ComputedUserset {
            relation: RelationNameOwned::try_from("viewer").expect("valid relation"),
        };
        let mut definition = AuthorizationTypeDefinition::default();

        definition.define_relation(relation_name.clone(), expr.clone());

        assert!(definition.is_defined(&relation_name));
        assert_eq!(definition.expr_for(&relation_name), Some(&expr));
    }

    #[test]
    fn define_static_relation_registers_relation_expr() {
        let relation_name = RelationNameOwned::from(ViewerRelation::NAME);
        let mut definition = AuthorizationTypeDefinition::default();

        definition.define_static_relation(ViewerRelation);

        assert!(definition.is_defined(&relation_name));
        assert_eq!(
            definition.expr_for(&relation_name),
            Some(&UsersetExpr::This)
        );
    }

    #[test]
    fn expr_for_returns_none_for_undefined_relation() {
        let definition = AuthorizationTypeDefinition::default();
        let relation_name = RelationNameOwned::try_from("viewer").expect("valid relation");

        assert!(!definition.is_defined(&relation_name));
        assert!(definition.expr_for(&relation_name).is_none());
    }
}
