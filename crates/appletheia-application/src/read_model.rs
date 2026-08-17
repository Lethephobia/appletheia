pub mod list;
pub mod pagination;
pub mod watch;

mod materialization_event_context;
mod read_model_fragment;
mod read_model_fragment_change;
mod read_model_fragment_change_envelope;
mod read_model_fragment_change_envelope_error;
mod read_model_fragment_change_error;
mod read_model_fragment_change_id;
mod read_model_fragment_change_id_error;
mod read_model_fragment_name;
mod read_model_fragment_name_owned;
mod read_model_fragment_name_owned_error;
mod read_model_name;
mod read_model_name_owned;
mod read_model_name_owned_error;
mod read_model_observation;
mod read_model_observation_source;
mod read_model_part;
mod read_model_part_change;
mod read_model_part_change_envelope;
mod read_model_part_change_envelope_error;
mod read_model_part_change_error;
mod read_model_part_change_route;
mod read_model_part_name;
mod read_model_part_name_owned;
mod read_model_part_name_owned_error;
mod read_model_part_path;
mod read_model_part_path_error;
mod read_model_part_path_resolver;
mod read_model_part_path_segment;
mod read_model_part_tree;
mod read_model_part_tree_mapper;
mod read_model_part_tree_selection;
mod read_model_part_tree_value;
mod read_model_partition;
mod registered_part_route;
mod registered_part_route_map;
mod serialized_partition;
mod serialized_partition_error;
mod serialized_read_model_fragment;
mod serialized_read_model_fragment_change;
mod serialized_read_model_fragment_error;
mod serialized_read_model_part;
mod serialized_read_model_part_error;

pub use materialization_event_context::MaterializationEventContext;
pub use read_model_fragment::ReadModelFragment;
pub use read_model_fragment_change::ReadModelFragmentChange;
pub use read_model_fragment_change_envelope::ReadModelFragmentChangeEnvelope;
pub use read_model_fragment_change_envelope_error::ReadModelFragmentChangeEnvelopeError;
pub use read_model_fragment_change_error::ReadModelFragmentChangeError;
pub use read_model_fragment_change_id::ReadModelFragmentChangeId;
pub use read_model_fragment_change_id_error::ReadModelFragmentChangeIdError;
pub use read_model_fragment_name::ReadModelFragmentName;
pub use read_model_fragment_name_owned::ReadModelFragmentNameOwned;
pub use read_model_fragment_name_owned_error::ReadModelFragmentNameOwnedError;
pub use read_model_name::ReadModelName;
pub use read_model_name_owned::ReadModelNameOwned;
pub use read_model_name_owned_error::ReadModelNameOwnedError;
pub use read_model_observation::ReadModelObservation;
pub use read_model_observation_source::ReadModelObservationSource;
pub use read_model_part::ReadModelPart;
pub use read_model_part_change::ReadModelPartChange;
pub use read_model_part_change_envelope::ReadModelPartChangeEnvelope;
pub use read_model_part_change_envelope_error::ReadModelPartChangeEnvelopeError;
pub use read_model_part_change_error::ReadModelPartChangeError;
pub use read_model_part_change_route::ReadModelPartChangeRoute;
pub use read_model_part_name::ReadModelPartName;
pub use read_model_part_name_owned::ReadModelPartNameOwned;
pub use read_model_part_name_owned_error::ReadModelPartNameOwnedError;
pub use read_model_part_path::ReadModelPartPath;
pub use read_model_part_path_error::ReadModelPartPathError;
pub use read_model_part_path_resolver::ReadModelPartPathResolver;
pub use read_model_part_path_segment::ReadModelPartPathSegment;
pub use read_model_part_tree::ReadModelPartTree;
pub use read_model_part_tree_mapper::ReadModelPartTreeMapper;
pub use read_model_partition::ReadModelPartition;
pub use serialized_partition::SerializedPartition;
pub use serialized_partition_error::SerializedPartitionError;
pub use serialized_read_model_fragment::SerializedReadModelFragment;
pub use serialized_read_model_fragment_change::SerializedReadModelFragmentChange;
pub use serialized_read_model_fragment_error::SerializedReadModelFragmentError;
pub use serialized_read_model_part::SerializedReadModelPart;
pub use serialized_read_model_part_error::SerializedReadModelPartError;

/// Defines a read model as a tree of independently delivered parts.
pub trait ReadModel: ReadModelObservationSource + Send + Sync {
    /// Identifies the read model's watch and storage stream.
    const NAME: ReadModelName;

    /// Declares routes that need application-specific mapping, audiences, or references.
    const PART_CHANGE_ROUTES: &'static [ReadModelPartChangeRoute] = &[];

    /// Returns the root parts and their attributes.
    ///
    /// Passing `None` returns the type-level tree used to register ordinary routes. Passing a
    /// snapshot returns the same tree with its materialized part values attached for watch
    /// selection.
    fn parts(read_model: Option<&Self>) -> Vec<ReadModelPartTree>;
}

/// Preserves a root read model's contract when a query can return no materialized snapshot.
impl<R> ReadModel for Option<R>
where
    R: ReadModel,
{
    const NAME: ReadModelName = R::NAME;
    const PART_CHANGE_ROUTES: &'static [ReadModelPartChangeRoute] = R::PART_CHANGE_ROUTES;

    fn parts(read_model: Option<&Self>) -> Vec<ReadModelPartTree> {
        R::parts(read_model.and_then(Option::as_ref))
    }
}
