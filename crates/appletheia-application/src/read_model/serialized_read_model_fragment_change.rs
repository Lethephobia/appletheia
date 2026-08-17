use serde::{Deserialize, Serialize};

use super::{
    ReadModelFragment, ReadModelFragmentChangeError, ReadModelFragmentNameOwned,
    SerializedPartition, SerializedReadModelFragment,
};

/// Describes one type-erased physical fragment replacement or removal on durable delivery.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum SerializedReadModelFragmentChange {
    Changed {
        fragment_name: ReadModelFragmentNameOwned,
        partition: SerializedPartition,
        fragment: SerializedReadModelFragment,
    },
    Removed {
        fragment_name: ReadModelFragmentNameOwned,
        partition: SerializedPartition,
    },
}

impl SerializedReadModelFragmentChange {
    /// Returns the physical fragment name.
    pub fn fragment_name(&self) -> &ReadModelFragmentNameOwned {
        match self {
            Self::Changed { fragment_name, .. } | Self::Removed { fragment_name, .. } => {
                fragment_name
            }
        }
    }

    /// Returns the source-fragment partition replaced or removed by this change.
    pub fn partition(&self) -> &SerializedPartition {
        match self {
            Self::Changed { partition, .. } | Self::Removed { partition, .. } => partition,
        }
    }

    /// Deserializes a complete replacement when it belongs to `F`.
    pub fn try_fragment<F>(&self) -> Result<Option<F>, ReadModelFragmentChangeError>
    where
        F: ReadModelFragment,
    {
        let Self::Changed {
            fragment_name,
            fragment,
            ..
        } = self
        else {
            return Ok(None);
        };
        if fragment_name.value() != F::NAME.value() {
            return Err(ReadModelFragmentChangeError::FragmentMismatch {
                expected: F::NAME.value().to_owned(),
                actual: fragment_name.value().to_owned(),
            });
        }

        serde_json::from_value(fragment.value().clone())
            .map(Some)
            .map_err(ReadModelFragmentChangeError::DeserializeFragment)
    }

    /// Deserializes a removed physical fragment key when this change belongs to `F`.
    pub fn try_removed_key<F>(&self) -> Result<Option<F::Key>, ReadModelFragmentChangeError>
    where
        F: ReadModelFragment,
    {
        let Self::Removed {
            fragment_name,
            partition,
        } = self
        else {
            return Ok(None);
        };
        if fragment_name.value() != F::NAME.value() {
            return Err(ReadModelFragmentChangeError::FragmentMismatch {
                expected: F::NAME.value().to_owned(),
                actual: fragment_name.value().to_owned(),
            });
        }

        partition
            .try_fragment_key::<F>()
            .map(Some)
            .map_err(Into::into)
    }

    /// Reports whether this change removes a fragment of type `F`.
    pub fn removes<F>(&self) -> bool
    where
        F: ReadModelFragment,
    {
        matches!(
            self,
            Self::Removed { fragment_name, .. } if fragment_name.value() == F::NAME.value()
        )
    }
}
