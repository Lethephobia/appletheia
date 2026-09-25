use std::collections::HashMap;
use std::sync::Arc;

use appletheia_domain::Aggregate;

use super::{
    AuthorizationModel, AuthorizationModelError, Relation, RelationRefOwned, Relationship,
    RelationshipDerivation, RelationshipDeriver, RelationshipDeriverError, UsersetExpr,
};

#[derive(Clone, Debug, Default)]
pub struct InMemoryAuthorizationModel {
    exprs: HashMap<RelationRefOwned, Arc<UsersetExpr>>,
    relationship_derivation: RelationshipDerivation,
}

impl InMemoryAuthorizationModel {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn define_relation<R>(&mut self, relation: R)
    where
        R: Relation,
    {
        let relation_ref = RelationRefOwned::from(R::REF);
        let expr = relation.expr();
        self.relationship_derivation
            .define_relation(&relation_ref, &expr);
        self.exprs.insert(relation_ref, Arc::new(expr));
    }
}

impl RelationshipDeriver for InMemoryAuthorizationModel {
    fn derive<A: Aggregate>(
        &self,
        aggregate: &A,
    ) -> Result<Vec<Relationship>, RelationshipDeriverError> {
        Ok(self.relationship_derivation.derive(aggregate)?)
    }
}

impl AuthorizationModel for InMemoryAuthorizationModel {
    async fn expr_for(
        &self,
        relation: &RelationRefOwned,
    ) -> Result<Option<UsersetExpr>, AuthorizationModelError> {
        Ok(self.exprs.get(relation).map(|expr| (**expr).clone()))
    }
}
