use std::collections::{HashMap, HashSet};

use super::relationship_memo_key::RelationshipMemoKey;

#[derive(Default)]
pub(super) struct RelationshipEvalState {
    pub(super) memo: HashMap<RelationshipMemoKey, bool>,
    pub(super) in_progress: HashSet<RelationshipMemoKey>,
    pub(super) nodes: usize,
    pub(super) relationships_scanned: usize,
}
