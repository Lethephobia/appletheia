#[derive(Clone, Debug)]
pub struct RelationshipResolverConfig {
    pub max_depth: usize,
    pub max_nodes: usize,
    pub max_relationships_scanned: usize,
}

impl Default for RelationshipResolverConfig {
    fn default() -> Self {
        Self {
            max_depth: 16,
            max_nodes: 10_000,
            max_relationships_scanned: 100_000,
        }
    }
}

impl RelationshipResolverConfig {
    pub fn with_max_depth(mut self, max_depth: usize) -> Self {
        self.max_depth = max_depth;
        self
    }

    pub fn with_max_nodes(mut self, max_nodes: usize) -> Self {
        self.max_nodes = max_nodes;
        self
    }

    pub fn with_max_relationships_scanned(mut self, max_relationships_scanned: usize) -> Self {
        self.max_relationships_scanned = max_relationships_scanned;
        self
    }
}
