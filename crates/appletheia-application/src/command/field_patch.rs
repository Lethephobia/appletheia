use serde::{Deserialize, Serialize};

/// Represents a patch for a single command field.
///
/// `Unchanged` means the field was not provided by the caller. The enum is
/// serialized using an adjacently tagged representation.
///
/// `Set` carries the field value, including `Option<T>` when the caller wants
/// to clear a nullable field.
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum FieldPatch<T> {
    /// Leaves the field unchanged.
    #[default]
    Unchanged,

    /// Replaces the field with the provided value.
    Set(T),
}

impl<T> FieldPatch<T> {
    /// Returns whether the field is unchanged.
    pub const fn is_unchanged(&self) -> bool {
        matches!(self, Self::Unchanged)
    }

    /// Returns whether the field is set.
    pub const fn is_set(&self) -> bool {
        matches!(self, Self::Set(_))
    }

    /// Converts the patch into an optional value.
    pub fn into_option(self) -> Option<T> {
        match self {
            Self::Unchanged => None,
            Self::Set(value) => Some(value),
        }
    }

    /// Creates a patch that sets the field.
    pub fn set(value: T) -> Self {
        Self::Set(value)
    }
}

impl<T> From<T> for FieldPatch<T> {
    fn from(value: T) -> Self {
        Self::Set(value)
    }
}

#[cfg(test)]
mod tests {
    use super::FieldPatch;

    #[test]
    fn default_is_unchanged() {
        assert_eq!(FieldPatch::<u32>::default(), FieldPatch::Unchanged);
    }

    #[test]
    fn serializes_unchanged_value_as_enum_variant() {
        let patch = FieldPatch::<String>::Unchanged;
        let value = serde_json::to_value(patch).expect("patch should serialize");

        assert_eq!(value, serde_json::json!({ "type": "unchanged" }));
    }

    #[test]
    fn serializes_set_value_as_enum_variant_object() {
        let patch = FieldPatch::Set(Some("hello".to_owned()));
        let value = serde_json::to_value(patch).expect("patch should serialize");

        assert_eq!(
            value,
            serde_json::json!({
                "type": "set",
                "data": "hello"
            })
        );
    }

    #[test]
    fn deserializes_unchanged_variant_as_unchanged() {
        let patch = serde_json::from_value::<FieldPatch<String>>(serde_json::json!({
            "type": "unchanged"
        }))
        .expect("unchanged variant should deserialize");

        assert_eq!(patch, FieldPatch::Unchanged);
    }

    #[test]
    fn deserializes_null_field_as_clear_value() {
        let patch = serde_json::from_value::<FieldPatch<Option<String>>>(serde_json::json!({
            "type": "set",
            "data": null
        }))
        .expect("null field should deserialize");

        assert_eq!(patch, FieldPatch::Set(None));
    }

    #[test]
    fn deserializes_value_field_as_set_value() {
        let patch = serde_json::from_value::<FieldPatch<Option<String>>>(serde_json::json!({
            "type": "set",
            "data": "hello"
        }))
        .expect("value field should deserialize");

        assert_eq!(patch, FieldPatch::Set(Some("hello".to_owned())));
    }
}
