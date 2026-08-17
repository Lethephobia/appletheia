use std::cmp::Ordering;
use std::marker::PhantomData;

use serde::{Serialize, de::DeserializeOwned};

use crate::read_model::{ReadModelPart, pagination::SortDirection};

use super::{
    ReadModelListCoverage, ReadModelListCriteria, ReadModelListMatcher, ReadModelListQuery,
    ReadModelListSortKey,
};

/// Matches client-facing list-item parts from application-defined criteria and ordering.
pub struct DefaultReadModelListMatcher<Q> {
    query: PhantomData<fn() -> Q>,
}

impl<Q> DefaultReadModelListMatcher<Q> {
    /// Creates a matcher whose part and ordering are defined by `Q`.
    pub const fn new() -> Self {
        Self { query: PhantomData }
    }
}

impl<Q> ReadModelListMatcher for DefaultReadModelListMatcher<Q>
where
    Q: ReadModelListQuery,
    <Q::Criteria as ReadModelListCriteria>::Candidate: ReadModelPart,
    <Q::SortKey as ReadModelListSortKey>::Cursor:
        Serialize + DeserializeOwned + Send + Sync + 'static,
{
    type Part = <Q::Criteria as ReadModelListCriteria>::Candidate;
    type Query = Q;
    type Cursor = <Q::SortKey as ReadModelListSortKey>::Cursor;

    fn includes(
        &self,
        query: &Self::Query,
        coverage: &ReadModelListCoverage<Self::Cursor>,
        part: &Self::Part,
    ) -> bool {
        if !query.criteria().matches(part) {
            return false;
        }

        match coverage {
            ReadModelListCoverage::Empty => false,
            ReadModelListCoverage::Complete => true,
            ReadModelListCoverage::Through { cursor } => {
                let ordering = query.sort().key.compare_to_cursor(part, cursor);

                match query.sort().direction {
                    SortDirection::Asc => ordering != Ordering::Greater,
                    SortDirection::Desc => ordering != Ordering::Less,
                }
            }
        }
    }
}

impl<Q> Default for DefaultReadModelListMatcher<Q> {
    fn default() -> Self {
        Self::new()
    }
}
