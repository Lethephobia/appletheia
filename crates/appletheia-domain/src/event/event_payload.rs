use std::{error::Error, fmt::Debug};

use serde::Serialize;
use serde::de::DeserializeOwned;

use super::EventName;

pub trait EventPayload:
    Clone + Debug + Eq + Serialize + DeserializeOwned + Send + Sync + 'static
{
    type Error: Error + From<serde_json::Error> + Send + Sync + 'static;

    fn name(&self) -> EventName;

    fn try_from_json_value(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value).map_err(serde_json::Error::into)
    }

    fn into_json_value(self) -> Result<serde_json::Value, Self::Error> {
        serde_json::to_value(self).map_err(serde_json::Error::into)
    }
}
