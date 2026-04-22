use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use super::ObjectContentType;

#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ObjectContentTypes(BTreeSet<ObjectContentType>);

impl ObjectContentTypes {
    pub fn contains(&self, value: &ObjectContentType) -> bool {
        self.0.contains(value)
    }

    pub fn iter(&self) -> impl Iterator<Item = &ObjectContentType> {
        self.0.iter()
    }
}

impl<const N: usize> From<[ObjectContentType; N]> for ObjectContentTypes {
    fn from(value: [ObjectContentType; N]) -> Self {
        Self(value.into_iter().collect())
    }
}

impl FromIterator<ObjectContentType> for ObjectContentTypes {
    fn from_iter<T: IntoIterator<Item = ObjectContentType>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}

#[cfg(test)]
mod tests {
    use super::ObjectContentTypes;
    use crate::object_storage::ObjectContentType;

    #[test]
    fn from_iter_dedupes_content_types() {
        let content_types = ObjectContentTypes::from_iter([
            ObjectContentType::png(),
            ObjectContentType::png(),
            ObjectContentType::webp(),
        ]);

        let values: Vec<&str> = content_types
            .iter()
            .map(ObjectContentType::as_str)
            .collect();

        assert_eq!(values, vec!["image/png", "image/webp"]);
    }
}
