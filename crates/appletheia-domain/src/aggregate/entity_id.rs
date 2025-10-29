use std::{fmt::Debug, hash::Hash};

use serde::Serialize;
use serde::de::DeserializeOwned;

use crate::core::Id;

pub trait EntityId:
    Copy + Debug + Eq + Hash + Serialize + DeserializeOwned + Send + Sync + 'static
{
    fn value(self) -> Id;
}
