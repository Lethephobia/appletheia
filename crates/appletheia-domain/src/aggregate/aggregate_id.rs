use std::{error::Error, fmt::Debug, hash::Hash};

use serde::Serialize;
use serde::de::DeserializeOwned;

use uuid::Uuid;

pub trait AggregateId:
    Copy + Debug + Eq + Hash + Serialize + DeserializeOwned + Send + Sync + 'static
{
    type Error: Error + Send + Sync + 'static;

    fn value(self) -> Uuid;

    fn try_from_uuid(value: Uuid) -> Result<Self, Self::Error>;
}
