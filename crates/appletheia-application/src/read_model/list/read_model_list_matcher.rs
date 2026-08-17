use serde::{Serialize, de::DeserializeOwned};

use crate::read_model::{ReadModelPart, ReadModelPartChange, ReadModelPartChangeError};

use super::{ReadModelListChangeDecision, ReadModelListCoverage};

/// Decides whether a read model part belongs to a client's loaded list range.
pub trait ReadModelListMatcher: Send + Sync {
    /// Defines the client-facing read model part that forms one list item.
    type Part: ReadModelPart;

    /// Defines criteria and ordering that identify one list query.
    type Query: Serialize + DeserializeOwned + Send + Sync + 'static;

    /// Defines the cursor used to describe the client's materialized range.
    type Cursor: Serialize + DeserializeOwned + Send + Sync + 'static;

    /// Returns `true` when the part matches both the query and loaded coverage.
    fn includes(
        &self,
        query: &Self::Query,
        coverage: &ReadModelListCoverage<Self::Cursor>,
        part: &Self::Part,
    ) -> bool;

    /// Classifies a client-facing part change using the current watch state.
    fn evaluate(
        &self,
        query: &Self::Query,
        coverage: &ReadModelListCoverage<Self::Cursor>,
        change: &ReadModelPartChange,
        part_is_watched: bool,
    ) -> Result<ReadModelListChangeDecision, ReadModelPartChangeError> {
        if let Some(part) = change.try_part::<Self::Part>()? {
            if self.includes(query, coverage, &part) {
                return Ok(ReadModelListChangeDecision::Included);
            }

            if part_is_watched {
                return Ok(ReadModelListChangeDecision::Invalidated);
            }

            return Ok(ReadModelListChangeDecision::Ignored);
        }

        if change.removes::<Self::Part>() && part_is_watched {
            return Ok(ReadModelListChangeDecision::Invalidated);
        }

        Ok(ReadModelListChangeDecision::Ignored)
    }
}
