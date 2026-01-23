use std::fmt::Display;
use std::hash::Hash;
use std::str::FromStr;

use serde::{Serialize, de::DeserializeOwned};

pub trait CommandName:
    Copy + Display + FromStr + Serialize + DeserializeOwned + Eq + Hash + Send + Sync + 'static
{
}
