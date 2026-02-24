use std::collections::{HashMap, HashSet};

use super::relationship_eval_node_count::RelationshipEvalNodeCount;
use super::relationship_eval_scanned_relationship_count::RelationshipEvalScannedRelationshipCount;
use super::relationship_memo_key::RelationshipMemoKey;

#[derive(Default)]
pub struct RelationshipEvalState {
    pub memo: HashMap<RelationshipMemoKey, bool>,
    pub in_progress: HashSet<RelationshipMemoKey>,
    pub node_count: RelationshipEvalNodeCount,
    pub scanned_relationship_count: RelationshipEvalScannedRelationshipCount,
}
