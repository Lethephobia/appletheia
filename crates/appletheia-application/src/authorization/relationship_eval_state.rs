use std::collections::{HashMap, HashSet};

use super::relationship_memo_key::RelationshipMemoKey;

#[derive(Default)]
pub struct RelationshipEvalState {
    pub memo: HashMap<RelationshipMemoKey, bool>,
    pub in_progress: HashSet<RelationshipMemoKey>,
    pub nodes: usize,
    pub relationships_scanned: usize,
}
