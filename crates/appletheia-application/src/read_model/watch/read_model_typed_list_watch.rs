use serde::de::DeserializeOwned;

use crate::read_model::list::{ReadModelListCoverage, ReadModelListWatch};

use super::ReadModelTypedListWatchError;

/// Holds a deserialized list query and the range materialized by one subscription.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReadModelTypedListWatch<Q, C> {
    pub query: Q,
    pub coverage: ReadModelListCoverage<C>,
}

impl<Q, C> TryFrom<&ReadModelListWatch> for ReadModelTypedListWatch<Q, C>
where
    Q: DeserializeOwned,
    C: DeserializeOwned,
{
    type Error = ReadModelTypedListWatchError;

    fn try_from(value: &ReadModelListWatch) -> Result<Self, Self::Error> {
        let query = value.query.try_into_typed::<Q>()?;
        let coverage = value
            .coverage
            .try_into_typed::<ReadModelListCoverage<C>>()?;

        Ok(Self { query, coverage })
    }
}
