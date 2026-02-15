use std::fmt::{self, Display};

use crate::command::CommandName;
use crate::query::QueryName;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum AuthorizationAction {
    Command(CommandName),
    Query(QueryName),
}

impl Display for AuthorizationAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Command(name) => write!(f, "command:{}", name),
            Self::Query(name) => write!(f, "query:{}", name),
        }
    }
}
