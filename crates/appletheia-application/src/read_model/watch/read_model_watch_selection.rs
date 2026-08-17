use crate::read_model::{
    ReadModel, ReadModelNameOwned, SerializedPartition,
    read_model_part_tree_selection::ReadModelPartTreeSelection,
};

use super::{ReadModelWatchPartitionDependencies, ReadModelWatchSelectionError};

/// Contains the serialized fragment selection derived from one read model snapshot.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReadModelWatchSelection {
    pub read_model_name: ReadModelNameOwned,
    pub partitions: Vec<SerializedPartition>,
    pub partition_dependencies: Vec<ReadModelWatchPartitionDependencies>,
}

impl ReadModelWatchSelection {
    /// Serializes the roots and complete dependency graph declared by `read_model`.
    pub fn try_from_read_model<R>(read_model: &R) -> Result<Self, ReadModelWatchSelectionError>
    where
        R: ReadModel,
    {
        let tree_selection =
            ReadModelPartTreeSelection::try_from_roots(R::parts(Some(read_model)))?;
        let partitions = tree_selection.roots;
        let partition_dependencies = tree_selection
            .dependencies
            .into_iter()
            .map(
                |(partition, referenced_partitions)| ReadModelWatchPartitionDependencies {
                    partition,
                    referenced_partitions,
                },
            )
            .collect();

        Ok(Self {
            read_model_name: ReadModelNameOwned::from(R::NAME),
            partitions,
            partition_dependencies,
        })
    }
}

#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};

    use crate::read_model::{
        ReadModelFragment, ReadModelFragmentName, ReadModelName, ReadModelObservation,
        ReadModelObservationSource, ReadModelPart, ReadModelPartName, ReadModelPartTree,
    };

    use super::*;

    #[derive(Clone, Deserialize, Serialize)]
    struct RootFragment(u32);

    impl ReadModelObservationSource for RootFragment {
        fn observations(&self) -> Vec<ReadModelObservation> {
            Vec::new()
        }
    }

    impl ReadModelFragment for RootFragment {
        const NAME: ReadModelFragmentName = ReadModelFragmentName::new("root");
        type Key = u32;

        fn key(&self) -> Self::Key {
            self.0
        }
    }

    #[derive(Deserialize, Serialize)]
    struct ChildPart(u32);

    impl From<ChildFragment> for ChildPart {
        fn from(fragment: ChildFragment) -> Self {
            Self(fragment.0)
        }
    }

    impl ReadModelObservationSource for ChildPart {
        fn observations(&self) -> Vec<ReadModelObservation> {
            Vec::new()
        }
    }

    impl ReadModelPart for ChildPart {
        const NAME: ReadModelPartName = ReadModelPartName::new("child");
        type SourceFragment = ChildFragment;

        fn key(&self) -> <Self::SourceFragment as ReadModelFragment>::Key {
            self.0
        }
    }

    #[derive(Clone, Deserialize, Serialize)]
    struct ChildFragment(u32);

    impl ReadModelObservationSource for ChildFragment {
        fn observations(&self) -> Vec<ReadModelObservation> {
            Vec::new()
        }
    }

    impl ReadModelFragment for ChildFragment {
        const NAME: ReadModelFragmentName = ReadModelFragmentName::new("child");
        type Key = u32;

        fn key(&self) -> Self::Key {
            self.0
        }
    }

    #[derive(Deserialize, Serialize)]
    struct RootPart {
        id: u32,
        child: ChildPart,
    }

    impl From<RootFragment> for RootPart {
        fn from(fragment: RootFragment) -> Self {
            Self {
                id: fragment.0,
                child: ChildPart(fragment.0),
            }
        }
    }

    impl ReadModelObservationSource for RootPart {
        fn observations(&self) -> Vec<ReadModelObservation> {
            Vec::new()
        }
    }

    impl ReadModelPart for RootPart {
        const NAME: ReadModelPartName = ReadModelPartName::new("root");
        type SourceFragment = RootFragment;

        fn key(&self) -> <Self::SourceFragment as ReadModelFragment>::Key {
            self.id
        }

        fn parts(part: Option<&Self>) -> Vec<ReadModelPartTree> {
            vec![ReadModelPartTree::field_with_explicit_route::<ChildPart>(
                "child",
                part.map(|root| &root.child),
            )]
        }
    }

    struct TestReadModel {
        root: RootPart,
    }

    impl ReadModelObservationSource for TestReadModel {
        fn observations(&self) -> Vec<ReadModelObservation> {
            Vec::new()
        }
    }

    impl ReadModel for TestReadModel {
        const NAME: ReadModelName = ReadModelName::new("test_read_model");

        fn parts(read_model: Option<&Self>) -> Vec<ReadModelPartTree> {
            vec![ReadModelPartTree::field_with_explicit_route::<RootPart>(
                "root",
                read_model.map(|read_model| &read_model.root),
            )]
        }
    }

    #[test]
    fn serializes_roots_and_dependencies_from_one_snapshot() {
        let read_model = TestReadModel {
            root: RootPart {
                id: 1,
                child: ChildPart(2),
            },
        };
        let selection = ReadModelWatchSelection::try_from_read_model(&read_model)
            .expect("read model selection should serialize");

        assert_eq!(selection.read_model_name.value(), "test_read_model");
        assert_eq!(
            selection.partitions[0].value(),
            &serde_json::json!({ "fragment_name": "root", "key": 1 })
        );
        assert_eq!(
            selection.partition_dependencies[0].referenced_partitions[0].value(),
            &serde_json::json!({ "fragment_name": "child", "key": 2 })
        );
    }

    #[test]
    fn serializes_an_absent_optional_snapshot_as_an_empty_selection() {
        let read_model = None::<TestReadModel>;
        let selection = ReadModelWatchSelection::try_from_read_model(&read_model)
            .expect("absent read model selection should serialize");

        assert_eq!(selection.read_model_name.value(), "test_read_model");
        assert!(selection.partitions.is_empty());
        assert!(selection.partition_dependencies.is_empty());
    }
}
