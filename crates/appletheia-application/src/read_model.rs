mod read_model_attribute_key;
mod read_model_attribute_value;
mod read_model_attribute_value_error;
mod read_model_field_filter;
mod read_model_id;
mod read_model_ref;
mod read_model_relationship_data;
mod read_model_relationship_key;
mod read_model_relationship_value;
mod read_model_relationship_value_error;
mod read_model_type;
mod serialized_read_model_attribute_value;

pub use read_model_attribute_key::*;
pub use read_model_attribute_value::*;
pub use read_model_attribute_value_error::*;
pub use read_model_field_filter::*;
pub use read_model_id::*;
pub use read_model_ref::*;
pub use read_model_relationship_data::*;
pub use read_model_relationship_key::*;
pub use read_model_relationship_value::*;
pub use read_model_relationship_value_error::*;
pub use read_model_type::*;
pub use serialized_read_model_attribute_value::*;

use std::fmt::Display;

/// A reusable resource whose typed fields are selected by key at the output boundary.
/// Implementations map each key to its corresponding value. Filters determine which
/// keys are requested; read models do not need to enumerate all available keys.
pub trait ReadModel: Send + Sync {
    const TYPE: ReadModelType;

    type Id: Display + Send + Sync;
    type AttributeKey: ReadModelAttributeKey;
    type AttributeValue: ReadModelAttributeValue;
    type RelationshipKey: ReadModelRelationshipKey;
    type RelationshipValue: ReadModelRelationshipValue;

    fn id(&self) -> &Self::Id;

    fn attribute(&self, key: Self::AttributeKey) -> Self::AttributeValue;

    fn relationship(&self, key: Self::RelationshipKey) -> Self::RelationshipValue;
}

#[cfg(test)]
mod tests {
    use std::fmt;

    use serde::Serialize;
    use serde_json::json;
    use uuid::Uuid;

    use super::*;

    #[derive(Clone, Copy, PartialEq, Eq)]
    enum ProfileAttributeKey {
        DisplayName,
        Picture,
    }

    impl fmt::Display for ProfileAttributeKey {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.write_str(match self {
                Self::DisplayName => "display_name",
                Self::Picture => "picture",
            })
        }
    }

    impl ReadModelAttributeKey for ProfileAttributeKey {}

    enum ProfileAttributeValue {
        DisplayName(String),
        Picture(Option<String>),
    }

    impl ReadModelAttributeValue for ProfileAttributeValue {
        fn try_to_value(
            &self,
        ) -> Result<SerializedReadModelAttributeValue, ReadModelAttributeValueError> {
            Ok(match self {
                Self::DisplayName(value) => serde_json::to_value(value)?.into(),
                Self::Picture(value) => serde_json::to_value(value)?.into(),
            })
        }
    }

    #[derive(Clone, Copy, PartialEq, Eq)]
    enum ProfileRelationshipKey {
        Owner,
        Authors,
    }

    impl fmt::Display for ProfileRelationshipKey {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.write_str(match self {
                Self::Owner => "owner",
                Self::Authors => "authors",
            })
        }
    }

    impl ReadModelRelationshipKey for ProfileRelationshipKey {}

    enum ProfileRelationshipValue {
        Owner(Option<Uuid>),
        Authors(Vec<Uuid>),
    }

    impl ReadModelRelationshipValue for ProfileRelationshipValue {
        fn try_to_data(
            &self,
        ) -> Result<ReadModelRelationshipData, ReadModelRelationshipValueError> {
            Ok(match self {
                Self::Owner(id) => {
                    ReadModelRelationshipData::ToOne(id.map(ReadModelRef::new::<Profile>))
                }
                Self::Authors(ids) => ReadModelRelationshipData::ToMany(
                    ids.iter()
                        .map(|id| ReadModelRef::new::<Profile>(*id))
                        .collect(),
                ),
            })
        }
    }

    struct Profile {
        id: Uuid,
        display_name: String,
        picture: Option<String>,
        owner_id: Option<Uuid>,
        author_ids: Vec<Uuid>,
    }

    impl ReadModel for Profile {
        const TYPE: ReadModelType = ReadModelType::new("profiles");

        type Id = Uuid;
        type AttributeKey = ProfileAttributeKey;
        type AttributeValue = ProfileAttributeValue;
        type RelationshipKey = ProfileRelationshipKey;
        type RelationshipValue = ProfileRelationshipValue;

        fn id(&self) -> &Self::Id {
            &self.id
        }

        fn attribute(&self, key: Self::AttributeKey) -> Self::AttributeValue {
            match key {
                ProfileAttributeKey::DisplayName => {
                    ProfileAttributeValue::DisplayName(self.display_name.clone())
                }
                ProfileAttributeKey::Picture => {
                    ProfileAttributeValue::Picture(self.picture.clone())
                }
            }
        }

        fn relationship(&self, key: Self::RelationshipKey) -> Self::RelationshipValue {
            match key {
                ProfileRelationshipKey::Owner => ProfileRelationshipValue::Owner(self.owner_id),
                ProfileRelationshipKey::Authors => {
                    ProfileRelationshipValue::Authors(self.author_ids.clone())
                }
            }
        }
    }

    enum ProfileFilter {
        Public,
    }

    impl ReadModelFieldFilter<Profile> for ProfileFilter {
        fn attribute_keys(&self) -> &[ProfileAttributeKey] {
            &[ProfileAttributeKey::DisplayName]
        }

        fn relationship_keys(&self) -> &[ProfileRelationshipKey] {
            &[ProfileRelationshipKey::Owner]
        }
    }

    #[test]
    fn filters_select_typed_fields_without_enumerating_all_keys() {
        let profile = Profile {
            id: Uuid::nil(),
            display_name: "Alice".to_owned(),
            picture: None,
            owner_id: Some(Uuid::from_u128(42)),
            author_ids: vec![Uuid::from_u128(43)],
        };
        let filter = ProfileFilter::Public;
        let attributes = filter
            .attribute_keys()
            .iter()
            .map(|key| {
                (
                    key.to_string(),
                    profile.attribute(*key).try_to_value().unwrap(),
                )
            })
            .collect::<std::collections::BTreeMap<_, _>>();
        assert_eq!(
            serde_json::to_value(attributes).unwrap(),
            json!({"display_name": "Alice"})
        );
        assert_eq!(
            profile
                .attribute(ProfileAttributeKey::Picture)
                .try_to_value()
                .unwrap()
                .value(),
            &json!(null)
        );
        let relationships = filter
            .relationship_keys()
            .iter()
            .map(|key| {
                (
                    key.to_string(),
                    profile.relationship(*key).try_to_data().unwrap(),
                )
            })
            .collect::<std::collections::BTreeMap<_, _>>();
        assert_eq!(
            serde_json::to_value(relationships).unwrap(),
            json!({"owner": {"type": "profiles", "id": Uuid::from_u128(42).to_string()}})
        );
        assert_eq!(
            serde_json::to_value(
                profile
                    .relationship(ProfileRelationshipKey::Authors)
                    .try_to_data()
                    .unwrap()
            )
            .unwrap(),
            json!([{"type": "profiles", "id": Uuid::from_u128(43).to_string()}])
        );
        assert_eq!(
            serde_json::to_value(ReadModelRef::new::<Profile>(*profile.id())).unwrap(),
            json!({"type": "profiles", "id": Uuid::nil().to_string()})
        );
    }

    #[test]
    fn absent_and_empty_relationships_preserve_their_cardinality() {
        assert_eq!(
            serde_json::to_value(ProfileRelationshipValue::Owner(None).try_to_data().unwrap())
                .unwrap(),
            json!(null)
        );
        assert_eq!(
            serde_json::to_value(
                ProfileRelationshipValue::Authors(vec![])
                    .try_to_data()
                    .unwrap()
            )
            .unwrap(),
            json!([])
        );
    }

    #[test]
    fn attribute_values_preserve_nested_objects_arrays_and_null() {
        for value in [
            json!({"width": 10, "height": 20}),
            json!([1, null]),
            json!(null),
        ] {
            let serialized = SerializedReadModelAttributeValue::from(value.clone());
            assert_eq!(serde_json::to_value(&serialized).unwrap(), value);
            let restored: SerializedReadModelAttributeValue =
                serde_json::from_value(value.clone()).unwrap();
            assert_eq!(restored.value(), &value);
        }
    }

    #[test]
    fn attribute_serialization_errors_are_preserved() {
        struct Unserializable;

        impl Serialize for Unserializable {
            fn serialize<S>(&self, _serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                Err(serde::ser::Error::custom("cannot serialize attribute"))
            }
        }

        let error = serde_json::to_value(Unserializable).unwrap_err();
        let attribute_error = ReadModelAttributeValueError::from(error);
        assert!(matches!(
            attribute_error,
            ReadModelAttributeValueError::Json(_)
        ));
    }

    #[test]
    fn read_model_types_follow_member_name_rules() {
        assert_eq!(
            serde_json::to_value(ReadModelType::new("user-identities")).unwrap(),
            json!("user-identities")
        );
        for name in ["", "_name", "name-", "a.b", "a:b", "a/b"] {
            assert!(std::panic::catch_unwind(|| ReadModelType::new(name)).is_err());
        }
    }
}
