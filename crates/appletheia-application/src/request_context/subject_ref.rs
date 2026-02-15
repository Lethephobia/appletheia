use serde::{Deserialize, Serialize};

use super::{SubjectId, SubjectKind};

#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct SubjectRef {
    pub kind: SubjectKind,
    pub id: SubjectId,
}
