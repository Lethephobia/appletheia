pub mod aggregate_version_tag_id;
pub mod aggregate_version_tag_id_error;
pub mod aggregate_version_tag_name;
pub mod aggregate_version_tag_name_error;

pub use aggregate_version_tag_id::AggregateVersionTagId;
pub use aggregate_version_tag_id_error::AggregateVersionTagIdError;
pub use aggregate_version_tag_name::AggregateVersionTagName;
pub use aggregate_version_tag_name_error::AggregateVersionTagNameError;

use super::{AggregateId, AggregateVersion};

#[derive(Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct AggregateVersionTag<A: AggregateId> {
    id: AggregateVersionTagId,
    aggregate_id: A,
    aggregate_version: AggregateVersion,
    name: AggregateVersionTagName,
}

impl<A: AggregateId> AggregateVersionTag<A> {
    pub fn new(
        id: AggregateVersionTagId,
        aggregate_id: A,
        aggregate_version: AggregateVersion,
        name: AggregateVersionTagName,
    ) -> Self {
        Self {
            id,
            aggregate_id,
            aggregate_version,
            name,
        }
    }

    pub fn id(&self) -> AggregateVersionTagId {
        self.id
    }

    pub fn aggregate_id(&self) -> A {
        self.aggregate_id
    }

    pub fn aggregate_version(&self) -> AggregateVersion {
        self.aggregate_version
    }

    pub fn name(&self) -> &AggregateVersionTagName {
        &self.name
    }
}
