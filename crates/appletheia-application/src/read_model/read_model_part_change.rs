use serde::{Deserialize, Serialize};

use super::{
    ReadModelFragment, ReadModelPart, ReadModelPartChangeError, ReadModelPartNameOwned,
    ReadModelPartPath, ReadModelPartPathResolver, SerializedPartition,
    SerializedReadModelFragmentChange, SerializedReadModelPart,
};

/// Describes one complete read model part replacement or removal.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum ReadModelPartChange {
    Changed {
        source_partition: SerializedPartition,
        part_name: ReadModelPartNameOwned,
        path: ReadModelPartPath,
        part: SerializedReadModelPart,
        #[serde(default)]
        audience_partitions: Vec<SerializedPartition>,
        #[serde(default)]
        referenced_partitions: Vec<SerializedPartition>,
    },
    Removed {
        source_partition: SerializedPartition,
        part_name: ReadModelPartNameOwned,
        path: ReadModelPartPath,
        #[serde(default)]
        audience_partitions: Vec<SerializedPartition>,
    },
}

impl ReadModelPartChange {
    /// Maps one source-fragment replacement or removal to at most one delivered part change.
    ///
    /// The complete replacement path is resolved from the target read model's part tree.
    pub fn map_one<F, P>(
        change: &SerializedReadModelFragmentChange,
        path_resolver: ReadModelPartPathResolver,
        changed_audiences: impl FnOnce(&F) -> Result<Vec<SerializedPartition>, ReadModelPartChangeError>,
        removed_audiences: impl FnOnce(
            &F::Key,
        )
            -> Result<Vec<SerializedPartition>, ReadModelPartChangeError>,
        references: impl FnOnce(&F) -> Result<Vec<SerializedPartition>, ReadModelPartChangeError>,
    ) -> Result<Vec<Self>, ReadModelPartChangeError>
    where
        F: ReadModelFragment + Clone,
        P: ReadModelPart<SourceFragment = F>,
    {
        if let Some(fragment) = change.try_fragment::<F>()? {
            let audiences = changed_audiences(&fragment)?;
            let referenced_partitions = references(&fragment)?;
            let part = P::from(fragment.clone());
            let replacement_path = path_resolver.try_for_part(&part)?;
            let part_change = Self::try_changed(
                &fragment,
                &part,
                replacement_path,
                audiences,
                referenced_partitions,
            )?;

            return Ok(vec![part_change]);
        }
        let Some(fragment_key) = change.try_removed_key::<F>()? else {
            return Ok(Vec::new());
        };
        let audiences = removed_audiences(&fragment_key)?;
        let replacement_path = path_resolver.try_for_key::<P>(&fragment_key)?;
        let part_change = Self::try_removed::<P>(&fragment_key, replacement_path, audiences)?;

        Ok(vec![part_change])
    }

    /// Creates a complete client-facing part replacement from a materialized source fragment.
    pub fn try_changed<F, P>(
        source_fragment: &F,
        part: &P,
        path: ReadModelPartPath,
        audience_partitions: Vec<SerializedPartition>,
        referenced_partitions: Vec<SerializedPartition>,
    ) -> Result<Self, ReadModelPartChangeError>
    where
        F: ReadModelFragment,
        P: ReadModelPart,
    {
        let part_value =
            serde_json::to_value(part).map_err(ReadModelPartChangeError::SerializePart)?;

        Ok(Self::Changed {
            source_partition: source_fragment.partition().try_into_serialized::<F>()?,
            part_name: ReadModelPartNameOwned::from(P::NAME),
            path,
            part: SerializedReadModelPart::try_from(part_value)?,
            audience_partitions,
            referenced_partitions,
        })
    }

    /// Creates a client-facing part tombstone from a removed source fragment key.
    pub fn try_removed<P>(
        key: &<P::SourceFragment as ReadModelFragment>::Key,
        path: ReadModelPartPath,
        audience_partitions: Vec<SerializedPartition>,
    ) -> Result<Self, ReadModelPartChangeError>
    where
        P: ReadModelPart,
    {
        Self::try_removed_from_fragment::<P::SourceFragment, P>(key, path, audience_partitions)
    }

    /// Creates a part tombstone caused by an explicit multi-output source fragment route.
    pub fn try_removed_from_fragment<F, P>(
        source_key: &F::Key,
        path: ReadModelPartPath,
        audience_partitions: Vec<SerializedPartition>,
    ) -> Result<Self, ReadModelPartChangeError>
    where
        F: ReadModelFragment,
        P: ReadModelPart,
    {
        Ok(Self::Removed {
            source_partition: SerializedPartition::try_from_fragment_key::<F>(source_key)?,
            part_name: ReadModelPartNameOwned::from(P::NAME),
            path,
            audience_partitions,
        })
    }

    /// Returns the location replaced or removed by this change.
    pub fn path(&self) -> &ReadModelPartPath {
        match self {
            Self::Changed { path, .. } | Self::Removed { path, .. } => path,
        }
    }

    /// Returns the source-fragment partition affected by this change.
    pub fn source_partition(&self) -> &SerializedPartition {
        match self {
            Self::Changed {
                source_partition, ..
            }
            | Self::Removed {
                source_partition, ..
            } => source_partition,
        }
    }

    /// Returns stable partitions whose subscriptions should receive this change.
    pub fn audience_partitions(&self) -> &[SerializedPartition] {
        match self {
            Self::Changed {
                audience_partitions,
                ..
            }
            | Self::Removed {
                audience_partitions,
                ..
            } => audience_partitions,
        }
    }

    /// Returns further fragment partitions required to materialize this part.
    pub fn referenced_partitions(&self) -> &[SerializedPartition] {
        match self {
            Self::Changed {
                referenced_partitions,
                ..
            } => referenced_partitions,
            Self::Removed { .. } => &[],
        }
    }

    /// Reports whether the source partition was removed.
    pub fn removes_partition(&self) -> bool {
        matches!(self, Self::Removed { .. })
    }

    /// Deserializes a replacement when it belongs to `P`.
    pub fn try_part<P>(&self) -> Result<Option<P>, ReadModelPartChangeError>
    where
        P: ReadModelPart,
    {
        let Self::Changed {
            part_name, part, ..
        } = self
        else {
            return Ok(None);
        };
        if part_name.value() != P::NAME.value() {
            return Ok(None);
        }

        let deserialized_part = serde_json::from_value::<P>(part.value().clone())
            .map_err(ReadModelPartChangeError::DeserializePart)?;

        Ok(Some(deserialized_part))
    }

    /// Reports whether this change removes a part of type `P`.
    pub fn removes<P>(&self) -> bool
    where
        P: ReadModelPart,
    {
        matches!(
            self,
            Self::Removed { part_name, .. } if part_name.value() == P::NAME.value()
        )
    }
}
