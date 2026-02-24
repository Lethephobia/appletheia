use super::relationship_eval_node_count::RelationshipEvalNodeCount;
use super::relationship_eval_scanned_relationship_count::RelationshipEvalScannedRelationshipCount;
use super::userset_expr_eval_depth::UsersetExprEvalDepth;

#[derive(Clone, Debug)]
pub struct RelationshipResolverConfig {
    pub max_depth: UsersetExprEvalDepth,
    pub max_node_count: RelationshipEvalNodeCount,
    pub max_scanned_relationship_count: RelationshipEvalScannedRelationshipCount,
}

impl Default for RelationshipResolverConfig {
    fn default() -> Self {
        Self {
            max_depth: UsersetExprEvalDepth::new(16),
            max_node_count: RelationshipEvalNodeCount::new(10_000),
            max_scanned_relationship_count: RelationshipEvalScannedRelationshipCount::new(100_000),
        }
    }
}

impl RelationshipResolverConfig {
    pub fn with_max_depth(mut self, max_depth: UsersetExprEvalDepth) -> Self {
        self.max_depth = max_depth;
        self
    }

    pub fn with_max_node_count(mut self, max_node_count: RelationshipEvalNodeCount) -> Self {
        self.max_node_count = max_node_count;
        self
    }

    pub fn with_max_scanned_relationship_count(
        mut self,
        max_scanned_relationship_count: RelationshipEvalScannedRelationshipCount,
    ) -> Self {
        self.max_scanned_relationship_count = max_scanned_relationship_count;
        self
    }
}
