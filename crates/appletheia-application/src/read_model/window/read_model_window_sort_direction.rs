use serde::{Deserialize, Serialize};

/// Direction used to order read model results.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReadModelWindowSortDirection {
    Asc,
    Desc,
}
