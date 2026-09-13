pub mod pagination;
pub mod watch;

mod materialization_event_context;
mod read_model_invalidated_partitions;

mod read_model_fragment;
mod read_model_fragment_name;
mod read_model_fragment_name_owned;
mod read_model_fragment_name_owned_error;
mod read_model_invalidation_envelope;
mod read_model_invalidation_envelope_error;
mod read_model_invalidation_id;
mod read_model_invalidation_id_error;
mod read_model_name;
mod read_model_name_owned;
mod read_model_name_owned_error;
mod read_model_observation;
mod read_model_observation_source;
mod read_model_partition;
mod serialized_partition;
mod serialized_partition_error;

pub use materialization_event_context::*;
pub use read_model_invalidated_partitions::*;

pub use read_model_fragment::*;
pub use read_model_fragment_name::*;
pub use read_model_fragment_name_owned::*;
pub use read_model_fragment_name_owned_error::*;
pub use read_model_invalidation_envelope::*;
pub use read_model_invalidation_envelope_error::*;
pub use read_model_invalidation_id::*;
pub use read_model_invalidation_id_error::*;
pub use read_model_name::*;
pub use read_model_name_owned::*;
pub use read_model_name_owned_error::*;
pub use read_model_observation::*;
pub use read_model_observation_source::*;
pub use read_model_partition::*;
pub use serialized_partition::*;
pub use serialized_partition_error::*;

/// Defines one complete query snapshot assembled from read-model fragments.
pub trait ReadModel: ReadModelObservationSource + serde::Serialize + Send + Sync {
    /// Identifies the read model's watch and storage stream.
    const NAME: ReadModelName;

    /// Returns the physical Fragment partitions contained in this snapshot.
    fn partitions(&self) -> Result<Vec<SerializedPartition>, SerializedPartitionError>;
}

/// Preserves a root read model's contract when a query can return no materialized snapshot.
impl<R> ReadModel for Option<R>
where
    R: ReadModel,
{
    const NAME: ReadModelName = R::NAME;
    fn partitions(&self) -> Result<Vec<SerializedPartition>, SerializedPartitionError> {
        self.as_ref()
            .map(ReadModel::partitions)
            .transpose()
            .map(Option::unwrap_or_default)
    }
}
