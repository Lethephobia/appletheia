use serde::{Deserialize, Serialize};

use super::{SerializedReadModelListCoverage, SerializedReadModelListQuery};

/// Carries an application-defined list query and the range materialized by a client.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ReadModelListWatch {
    pub query: SerializedReadModelListQuery,
    pub coverage: SerializedReadModelListCoverage,
}
