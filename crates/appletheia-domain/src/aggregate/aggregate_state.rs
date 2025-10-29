use std::{error::Error, fmt::Debug, hash::Hash};

use serde::Serialize;

use serde::de::DeserializeOwned;

use super::AggregateId;

pub trait AggregateState:
    Clone + Debug + Eq + Hash + Serialize + DeserializeOwned + Send + Sync + 'static
{
    type Id: AggregateId;
    type Error: Error + From<serde_json::Error> + Send + Sync + 'static;

    fn id(&self) -> Self::Id;

    fn try_from_json_value(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value).map_err(serde_json::Error::into)
    }

    fn to_json_value(&self) -> Result<serde_json::Value, Self::Error> {
        serde_json::to_value(self).map_err(Self::Error::from)
    }
}
