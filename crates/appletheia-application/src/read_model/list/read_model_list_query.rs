use serde::{Serialize, de::DeserializeOwned};

use crate::read_model::pagination::Sort;

use super::{ReadModelListCriteria, ReadModelListSortKey};

/// Exposes the criteria and ordering that identify one materialized list.
pub trait ReadModelListQuery: Serialize + DeserializeOwned + Send + Sync + 'static {
    /// Defines the application-owned membership predicate.
    type Criteria: ReadModelListCriteria;

    /// Defines the application-owned stable cursor ordering.
    type SortKey: ReadModelListSortKey<
        Candidate = <Self::Criteria as ReadModelListCriteria>::Candidate,
    >;

    /// Returns the membership criteria for this query.
    fn criteria(&self) -> &Self::Criteria;

    /// Returns the stable ordering for this query.
    fn sort(&self) -> &Sort<Self::SortKey>;
}
