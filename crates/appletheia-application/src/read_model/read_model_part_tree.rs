use super::read_model_part_tree_value::ReadModelPartTreeValue;
use super::{
    ReadModelFragment, ReadModelFragmentName, ReadModelPart, ReadModelPartChange,
    ReadModelPartChangeError, ReadModelPartName, ReadModelPartPath, ReadModelPartPathResolver,
    ReadModelPartPathSegment, SerializedReadModelFragmentChange,
};

pub(super) type MapPartTreeFragmentChange =
    fn(
        &SerializedReadModelFragmentChange,
        ReadModelPartPathResolver,
    ) -> Result<Vec<ReadModelPartChange>, ReadModelPartChangeError>;

/// Describes one delivered part, its relative location, and its materialized values.
pub struct ReadModelPartTree {
    pub(super) fragment_name: ReadModelFragmentName,
    pub(super) part_name: ReadModelPartName,
    pub(super) attributes: Vec<&'static str>,
    pub(super) keyed: bool,
    pub(super) map: Option<MapPartTreeFragmentChange>,
    pub(super) children: Vec<Self>,
    pub(super) values: Vec<ReadModelPartTreeValue>,
}

impl ReadModelPartTree {
    /// Declares one directly mapped part stored at an object attribute.
    pub fn field<P>(attribute: &'static str, part: Option<&P>) -> Self
    where
        P: ReadModelPart,
        P::SourceFragment: Clone,
    {
        Self::field_at::<P>(&[attribute], part)
    }

    /// Declares one directly mapped part below a sequence of object attributes.
    pub fn field_at<P>(attributes: &[&'static str], part: Option<&P>) -> Self
    where
        P: ReadModelPart,
        P::SourceFragment: Clone,
    {
        Self::new::<P>(attributes, false, Some(Self::map_part::<P>), part)
    }

    /// Declares one explicitly mapped part stored at an object attribute.
    pub fn field_with_explicit_route<P>(attribute: &'static str, part: Option<&P>) -> Self
    where
        P: ReadModelPart,
    {
        Self::field_at_with_explicit_route::<P>(&[attribute], part)
    }

    /// Declares one explicitly mapped part below a sequence of object attributes.
    pub fn field_at_with_explicit_route<P>(attributes: &[&'static str], part: Option<&P>) -> Self
    where
        P: ReadModelPart,
    {
        Self::new::<P>(attributes, false, None, part)
    }

    /// Declares directly mapped parts stored in a keyed collection attribute.
    pub fn collection<P>(attribute: &'static str, parts: Option<&[P]>) -> Self
    where
        P: ReadModelPart,
        P::SourceFragment: Clone,
    {
        Self::collection_at::<P>(&[attribute], parts)
    }

    /// Declares directly mapped parts in a keyed collection below object attributes.
    pub fn collection_at<P>(attributes: &[&'static str], parts: Option<&[P]>) -> Self
    where
        P: ReadModelPart,
        P::SourceFragment: Clone,
    {
        Self::new::<P>(
            attributes,
            true,
            Some(Self::map_part::<P>),
            parts.into_iter().flatten(),
        )
    }

    /// Declares explicitly mapped parts stored in a keyed collection attribute.
    pub fn collection_with_explicit_route<P>(attribute: &'static str, parts: Option<&[P]>) -> Self
    where
        P: ReadModelPart,
    {
        Self::collection_at_with_explicit_route::<P>(&[attribute], parts)
    }

    /// Declares explicitly mapped parts in a keyed collection below object attributes.
    pub fn collection_at_with_explicit_route<P>(
        attributes: &[&'static str],
        parts: Option<&[P]>,
    ) -> Self
    where
        P: ReadModelPart,
    {
        Self::new::<P>(attributes, true, None, parts.into_iter().flatten())
    }

    fn new<'a, P>(
        attributes: &[&'static str],
        keyed: bool,
        map: Option<MapPartTreeFragmentChange>,
        parts: impl IntoIterator<Item = &'a P>,
    ) -> Self
    where
        P: ReadModelPart,
    {
        let values = parts.into_iter().map(ReadModelPartTreeValue::new).collect();
        let children = P::parts(None);
        assert!(
            children
                .iter()
                .all(|child| child.fragment_name != <P::SourceFragment as ReadModelFragment>::NAME),
            "a read model part cannot contain a child part with the same source fragment"
        );

        Self {
            fragment_name: <P::SourceFragment as ReadModelFragment>::NAME,
            part_name: P::NAME,
            attributes: attributes.to_vec(),
            keyed,
            map,
            children,
            values,
        }
    }

    pub(super) fn relative_path(&self, key: &serde_json::Value) -> ReadModelPartPath {
        let mut segments = self
            .attributes
            .iter()
            .map(|attribute| ReadModelPartPathSegment::Attribute((*attribute).to_owned()))
            .collect::<Vec<_>>();
        if self.keyed {
            segments.push(ReadModelPartPathSegment::Key(key.clone()));
        }

        ReadModelPartPath::new(segments)
    }

    fn map_part<P>(
        change: &SerializedReadModelFragmentChange,
        path_resolver: ReadModelPartPathResolver,
    ) -> Result<Vec<ReadModelPartChange>, ReadModelPartChangeError>
    where
        P: ReadModelPart,
        P::SourceFragment: Clone,
    {
        if let Some(fragment) = change.try_fragment::<P::SourceFragment>()? {
            let part = P::from(fragment.clone());
            let replacement_path = path_resolver.try_for_part(&part)?;
            return Ok(vec![ReadModelPartChange::try_changed(
                &fragment,
                &part,
                replacement_path,
                Vec::new(),
                Vec::new(),
            )?]);
        }
        let Some(fragment_key) = change.try_removed_key::<P::SourceFragment>()? else {
            return Ok(Vec::new());
        };
        let replacement_path = path_resolver.try_for_key::<P>(&fragment_key)?;

        Ok(vec![ReadModelPartChange::try_removed::<P>(
            &fragment_key,
            replacement_path,
            Vec::new(),
        )?])
    }
}
