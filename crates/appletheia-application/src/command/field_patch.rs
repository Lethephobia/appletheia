use serde::de::DeserializeOwned;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// Represents a patch for a single command field.
///
/// `Unchanged` means the field was not provided by the caller. `Set` carries the
/// field value, including `Option<T>` when the caller wants to clear a nullable
/// field.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
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

impl<T> Serialize for FieldPatch<T>
where
    T: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Unchanged => serializer.serialize_none(),
            Self::Set(value) => value.serialize(serializer),
        }
    }
}

impl<'de, T> Deserialize<'de> for FieldPatch<T>
where
    T: DeserializeOwned,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        T::deserialize(deserializer).map(Self::Set)
    }
}

#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};

    use super::FieldPatch;

    #[derive(Debug, Default, Deserialize, Serialize, PartialEq)]
    struct PatchCarrier {
        #[serde(default, skip_serializing_if = "FieldPatch::is_unchanged")]
        value: FieldPatch<Option<String>>,
    }

    #[test]
    fn default_is_unchanged() {
        assert_eq!(FieldPatch::<u32>::default(), FieldPatch::Unchanged);
    }

    #[test]
    fn serializes_set_value_as_inner_value() {
        let patch = FieldPatch::Set(Some("hello".to_owned()));
        let value = serde_json::to_value(patch).expect("patch should serialize");

        assert_eq!(value, serde_json::json!("hello"));
    }

    #[test]
    fn serializes_unchanged_field_as_missing_field_when_skipped() {
        let carrier = PatchCarrier::default();
        let value = serde_json::to_value(carrier).expect("carrier should serialize");

        assert_eq!(value, serde_json::json!({}));
    }

    #[test]
    fn deserializes_missing_field_as_unchanged() {
        let carrier = serde_json::from_value::<PatchCarrier>(serde_json::json!({}))
            .expect("missing field should deserialize");

        assert_eq!(carrier.value, FieldPatch::Unchanged);
    }

    #[test]
    fn deserializes_null_field_as_clear_value() {
        let carrier = serde_json::from_value::<PatchCarrier>(serde_json::json!({
            "value": null
        }))
        .expect("null field should deserialize");

        assert_eq!(carrier.value, FieldPatch::Set(None));
    }

    #[test]
    fn deserializes_value_field_as_set_value() {
        let carrier = serde_json::from_value::<PatchCarrier>(serde_json::json!({
            "value": "hello"
        }))
        .expect("value field should deserialize");

        assert_eq!(carrier.value, FieldPatch::Set(Some("hello".to_owned())));
    }
}
