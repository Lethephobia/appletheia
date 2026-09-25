use std::error::Error;
use std::sync::Arc;

use crate::aggregate::SerializedAggregate;

use super::RelationshipEntries;

pub(crate) type RelationshipDerivationHandler = Arc<
    dyn Fn(&SerializedAggregate) -> Result<RelationshipEntries, Box<dyn Error + Send + Sync>>
        + Send
        + Sync,
>;
