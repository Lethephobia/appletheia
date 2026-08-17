use std::sync::Arc;

use super::{
    ReadModelFragment, ReadModelPart, ReadModelPartName, ReadModelPartPath, ReadModelPartPathError,
    ReadModelPartTree,
};

/// Resolves a part's complete replacement path from a read model's part tree.
#[derive(Clone)]
pub struct ReadModelPartPathResolver {
    parts: Arc<[ReadModelPartTree]>,
}

impl ReadModelPartPathResolver {
    /// Creates a resolver from one read model's type-level part tree.
    pub fn new(parts: Vec<ReadModelPartTree>) -> Self {
        Self {
            parts: parts.into(),
        }
    }

    /// Resolves the complete replacement path for a materialized part.
    pub fn try_for_part<P>(&self, part: &P) -> Result<ReadModelPartPath, ReadModelPartPathError>
    where
        P: ReadModelPart,
    {
        self.try_for_key::<P>(&part.key())
    }

    /// Resolves the complete replacement path for a part's source-fragment key.
    pub fn try_for_key<P>(
        &self,
        key: &<P::SourceFragment as ReadModelFragment>::Key,
    ) -> Result<ReadModelPartPath, ReadModelPartPathError>
    where
        P: ReadModelPart,
    {
        self.try_for_route_key::<P::SourceFragment, P>(key)
    }

    /// Resolves a part path using the source key of an explicit multi-output route.
    pub fn try_for_route_key<F, P>(
        &self,
        key: &F::Key,
    ) -> Result<ReadModelPartPath, ReadModelPartPathError>
    where
        F: ReadModelFragment,
        P: ReadModelPart,
    {
        let serialized_key =
            serde_json::to_value(key).map_err(ReadModelPartPathError::SerializeKey)?;
        self.resolve(P::NAME, &serialized_key)
            .ok_or(ReadModelPartPathError::UndeclaredPart { part_name: P::NAME })
    }

    fn resolve(
        &self,
        part_name: ReadModelPartName,
        key: &serde_json::Value,
    ) -> Option<ReadModelPartPath> {
        Self::resolve_in(&self.parts, part_name, key, None)
    }

    fn resolve_in(
        parts: &[ReadModelPartTree],
        part_name: ReadModelPartName,
        key: &serde_json::Value,
        parent_path: Option<ReadModelPartPath>,
    ) -> Option<ReadModelPartPath> {
        for part in parts {
            let relative_path = part.relative_path(key);
            let path = parent_path
                .clone()
                .map_or(relative_path.clone(), |parent| parent.append(relative_path));
            if part.part_name == part_name {
                return Some(path);
            }
            if let Some(nested_path) = Self::resolve_in(&part.children, part_name, key, Some(path))
            {
                return Some(nested_path);
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};
    use uuid::Uuid;

    use super::*;
    use crate::read_model::{
        ReadModelFragmentName, ReadModelObservation, ReadModelObservationSource,
        ReadModelPartPathSegment,
    };

    #[derive(Clone, Debug, Deserialize, Serialize)]
    struct TestFragment {
        id: Uuid,
    }

    impl ReadModelObservationSource for TestFragment {
        fn observations(&self) -> Vec<ReadModelObservation> {
            Vec::new()
        }
    }

    impl ReadModelFragment for TestFragment {
        const NAME: ReadModelFragmentName = ReadModelFragmentName::new("test_fragment");

        type Key = Uuid;

        fn key(&self) -> Self::Key {
            self.id
        }
    }

    #[derive(Deserialize, Serialize)]
    struct ParentPart {
        id: Uuid,
    }

    impl From<TestFragment> for ParentPart {
        fn from(fragment: TestFragment) -> Self {
            Self { id: fragment.id }
        }
    }

    impl ReadModelObservationSource for ParentPart {
        fn observations(&self) -> Vec<ReadModelObservation> {
            Vec::new()
        }
    }

    impl ReadModelPart for ParentPart {
        const NAME: ReadModelPartName = ReadModelPartName::new("parent");

        type SourceFragment = TestFragment;

        fn key(&self) -> <Self::SourceFragment as ReadModelFragment>::Key {
            self.id
        }

        fn parts(_part: Option<&Self>) -> Vec<ReadModelPartTree> {
            vec![ReadModelPartTree::field_with_explicit_route::<ChildPart>(
                "organization",
                None,
            )]
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    struct ChildFragment {
        id: Uuid,
    }

    impl ReadModelObservationSource for ChildFragment {
        fn observations(&self) -> Vec<ReadModelObservation> {
            Vec::new()
        }
    }

    impl ReadModelFragment for ChildFragment {
        const NAME: ReadModelFragmentName = ReadModelFragmentName::new("child_fragment");

        type Key = Uuid;

        fn key(&self) -> Self::Key {
            self.id
        }
    }

    #[derive(Deserialize, Serialize)]
    struct ChildPart {
        id: Uuid,
    }

    impl From<ChildFragment> for ChildPart {
        fn from(fragment: ChildFragment) -> Self {
            Self { id: fragment.id }
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
            self.id
        }
    }

    #[test]
    fn resolves_a_child_path_by_appending_each_relative_tree_location() {
        let id = Uuid::now_v7();
        let parts = vec![ReadModelPartTree::collection_at_with_explicit_route::<
            ParentPart,
        >(&["items"], None)];
        let resolver = ReadModelPartPathResolver::new(parts);

        let path = resolver
            .try_for_key::<ChildPart>(&id)
            .expect("declared child path should resolve");

        assert_eq!(
            path.segments(),
            &[
                ReadModelPartPathSegment::Attribute("items".to_owned()),
                ReadModelPartPathSegment::Key(
                    serde_json::to_value(id).expect("key should serialize"),
                ),
                ReadModelPartPathSegment::Attribute("organization".to_owned()),
            ]
        );
    }
}
