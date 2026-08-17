use super::{
    ReadModelFragment, ReadModelFragmentChangeError, ReadModelFragmentNameOwned,
    ReadModelPartition, SerializedPartition, SerializedReadModelFragment,
    SerializedReadModelFragmentChange,
};

/// A typed physical fragment replacement or removal produced by a projector.
///
/// This stays in the fragment's domain type until durable delivery requires serialization.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ReadModelFragmentChange<F>
where
    F: ReadModelFragment,
{
    /// Replaces the stored fragment with its complete current value.
    Changed(F),
    /// Removes the stored fragment identified by its physical key.
    Removed(F::Key),
}

impl<F> ReadModelFragmentChange<F>
where
    F: ReadModelFragment,
{
    /// Creates a typed replacement from a fragment value.
    pub fn try_from_fragment(fragment: &F) -> Result<Self, ReadModelFragmentChangeError>
    where
        F: Clone,
    {
        Ok(Self::Changed(fragment.clone()))
    }

    /// Creates a typed tombstone for one stored fragment key.
    pub fn try_removed(key: &F::Key) -> Result<Self, ReadModelFragmentChangeError>
    where
        F::Key: Clone,
    {
        Ok(Self::Removed(key.clone()))
    }

    /// Returns the source-fragment partition replaced or removed by this change.
    pub fn partition(&self) -> ReadModelPartition<F::Key> {
        match self {
            Self::Changed(fragment) => fragment.partition(),
            Self::Removed(key) => ReadModelPartition::new(key.clone()),
        }
    }

    /// Erases the fragment type at the durable-delivery boundary.
    pub fn try_into_serialized(
        self,
    ) -> Result<SerializedReadModelFragmentChange, ReadModelFragmentChangeError> {
        match self {
            Self::Changed(fragment) => {
                let partition = fragment.partition().try_into_serialized::<F>()?;
                let fragment_value = serde_json::to_value(fragment)
                    .map_err(ReadModelFragmentChangeError::SerializeFragment)?;
                Ok(SerializedReadModelFragmentChange::Changed {
                    fragment_name: ReadModelFragmentNameOwned::from(F::NAME),
                    partition,
                    fragment: SerializedReadModelFragment::try_from(fragment_value)?,
                })
            }
            Self::Removed(key) => Ok(SerializedReadModelFragmentChange::Removed {
                fragment_name: ReadModelFragmentNameOwned::from(F::NAME),
                partition: SerializedPartition::try_from_fragment_key::<F>(&key)?,
            }),
        }
    }
}
