use std::collections::HashMap;

use super::{AuthorizationRelation, RelationNameOwned, UsersetExpr};

#[derive(Clone, Debug, Default)]
pub struct AuthorizationTypeDefinition {
    relations: HashMap<RelationNameOwned, UsersetExpr>,
}

impl AuthorizationTypeDefinition {
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

    /// Defines a relation from a statically-typed `AuthorizationRelation`.
    ///
    /// This is primarily intended for in-memory, compile-time configuration of
    /// authorization models. Each relation is represented by its own type
    /// implementing `AuthorizationRelation`.
    pub fn define_static_relation<R>(&mut self)
    where
        R: AuthorizationRelation,
    {
        self.relations
            .insert(RelationNameOwned::from(R::NAME), R::expr());
    }

    pub fn is_defined(&self, relation: &RelationNameOwned) -> bool {
        self.relations.contains_key(relation)
    }

    pub fn expr_for(&self, relation: &RelationNameOwned) -> Option<&UsersetExpr> {
        self.relations.get(relation)
    }
}
