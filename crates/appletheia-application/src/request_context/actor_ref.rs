use crate::authorization::AggregateRef;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ActorRef {
    Anonymous,
    System,
    Subject { subject: AggregateRef },
}
