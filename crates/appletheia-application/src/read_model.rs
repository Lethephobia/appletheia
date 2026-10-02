mod read_model_attribute;
mod read_model_attribute_error;
mod read_model_attribute_key;
mod read_model_attribute_value;
mod read_model_field_filter;
mod read_model_id;
mod read_model_ref;
mod read_model_relationship;
mod read_model_relationship_data;
mod read_model_relationship_error;
mod read_model_relationship_key;
mod read_model_type;

pub use read_model_attribute::*;
pub use read_model_attribute_error::*;
pub use read_model_attribute_key::*;
pub use read_model_attribute_value::*;
pub use read_model_field_filter::*;
pub use read_model_id::*;
pub use read_model_ref::*;
pub use read_model_relationship::*;
pub use read_model_relationship_data::*;
pub use read_model_relationship_error::*;
pub use read_model_relationship_key::*;
pub use read_model_type::*;

use std::fmt::Display;

/// A reusable resource with complete typed fields, independent of a query document.
///
/// Implementations keep attribute and relationship keys unique in their shared
/// namespace and supply all fields required by their resource schema. The slices
/// do not enforce these collection-level invariants. Field filtering belongs at
/// the output boundary, after authorization.
pub trait ReadModel: Send + Sync {
    const TYPE: ReadModelType;

    type Id: Display + Send + Sync;
    type Attribute: ReadModelAttribute;
    type Relationship: ReadModelRelationship;

    /// Returns the resource identity shared with relationship references.
    fn id(&self) -> &Self::Id;

    fn attributes(&self) -> &[Self::Attribute];

    fn relationships(&self) -> &[Self::Relationship];
}

#[cfg(test)]
mod tests {
    use serde::Serialize;
    use serde_json::json;
    use uuid::Uuid;

    use super::*;

    enum ProfileAttribute {
        DisplayName(String),
        Picture(Option<String>),
    }

    impl ProfileAttribute {
        const DISPLAY_NAME: ReadModelAttributeKey = ReadModelAttributeKey::new("display_name");
        const PICTURE: ReadModelAttributeKey = ReadModelAttributeKey::new("picture");
    }

    impl ReadModelAttribute for ProfileAttribute {
        fn key(&self) -> ReadModelAttributeKey {
            match self {
                Self::DisplayName(_) => Self::DISPLAY_NAME,
                Self::Picture(_) => Self::PICTURE,
            }
        }

        fn try_to_value(&self) -> Result<ReadModelAttributeValue, ReadModelAttributeError> {
            Ok(match self {
                Self::DisplayName(value) => serde_json::to_value(value)?.into(),
                Self::Picture(value) => serde_json::to_value(value)?.into(),
            })
        }
    }

    enum ProfileRelationship {
        Owner(Uuid),
    }

    impl ProfileRelationship {
        const OWNER: ReadModelRelationshipKey = ReadModelRelationshipKey::new("owner");
    }

    impl ReadModelRelationship for ProfileRelationship {
        fn key(&self) -> ReadModelRelationshipKey {
            Self::OWNER
        }

        fn try_to_data(&self) -> Result<ReadModelRelationshipData, ReadModelRelationshipError> {
            let Self::Owner(id) = self;
            Ok(ReadModelRelationshipData::ToOne(Some(ReadModelRef::new::<
                Profile,
            >(*id))))
        }
    }

    struct Profile {
        id: Uuid,
        attributes: Vec<ProfileAttribute>,
        relationships: Vec<ProfileRelationship>,
    }

    impl ReadModel for Profile {
        const TYPE: ReadModelType = ReadModelType::new("profiles");

        type Id = Uuid;
        type Attribute = ProfileAttribute;
        type Relationship = ProfileRelationship;

        fn id(&self) -> &Self::Id {
            &self.id
        }

        fn attributes(&self) -> &[Self::Attribute] {
            &self.attributes
        }

        fn relationships(&self) -> &[Self::Relationship] {
            &self.relationships
        }
    }

    enum ProfileFilter {
        Public,
    }

    impl ReadModelFieldFilter for ProfileFilter {
        fn attribute_keys(&self) -> &[ReadModelAttributeKey] {
            &[ProfileAttribute::DISPLAY_NAME]
        }

        fn relationship_keys(&self) -> &[ReadModelRelationshipKey] {
            &[]
        }
    }

    #[test]
    fn typed_fields_can_be_selected_without_changing_resource_identity() {
        let profile = Profile {
            id: Uuid::nil(),
            attributes: vec![
                ProfileAttribute::DisplayName("Alice".to_owned()),
                ProfileAttribute::Picture(None),
            ],
            relationships: vec![ProfileRelationship::Owner(Uuid::nil())],
        };
        let filter = ProfileFilter::Public;
        let visible = profile
            .attributes()
            .iter()
            .filter(|attribute| filter.attribute_keys().contains(&attribute.key()))
            .map(|attribute| attribute.try_to_value().unwrap())
            .collect::<Vec<_>>();
        assert_eq!(serde_json::to_value(visible).unwrap(), json!(["Alice"]));
        assert_eq!(
            profile.attributes()[1].try_to_value().unwrap().value(),
            &json!(null)
        );
        assert!(
            !filter
                .relationship_keys()
                .contains(&profile.relationships()[0].key())
        );
        let identifier = ReadModelRef::new::<Profile>(*profile.id());
        assert_eq!(
            serde_json::to_value(identifier).unwrap(),
            json!({"type": "profiles", "id": "00000000-0000-0000-0000-000000000000"})
        );
    }

    #[test]
    fn relationship_data_serializes_without_enum_tags() {
        let owner = ProfileRelationship::Owner(Uuid::from_u128(42));
        let data = owner.try_to_data().unwrap();
        let identifier = json!({"type": "profiles", "id": "00000000-0000-0000-0000-00000000002a"});
        assert_eq!(serde_json::to_value(&data).unwrap(), identifier);
        assert_eq!(
            serde_json::to_value(ReadModelRelationshipData::ToOne(None)).unwrap(),
            json!(null)
        );
        assert_eq!(
            serde_json::to_value(ReadModelRelationshipData::ToMany(vec![])).unwrap(),
            json!([])
        );
        let ReadModelRelationshipData::ToOne(Some(resource)) = data else {
            panic!("expected owner");
        };
        assert_eq!(
            serde_json::to_value(ReadModelRelationshipData::ToMany(vec![resource])).unwrap(),
            json!([identifier])
        );
    }

    #[test]
    fn attribute_values_preserve_nested_objects_arrays_and_null() {
        for value in [
            json!({"width": 10, "height": 20}),
            json!([1, null]),
            json!(null),
        ] {
            let serialized = ReadModelAttributeValue::from(value.clone());
            assert_eq!(serde_json::to_value(&serialized).unwrap(), value);
            let restored: ReadModelAttributeValue = serde_json::from_value(value.clone()).unwrap();
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
        let attribute_error = ReadModelAttributeError::from(error);
        assert!(matches!(attribute_error, ReadModelAttributeError::Json(_)));
    }

    #[test]
    fn names_follow_json_api_member_rules_and_reserve_identity_fields() {
        const RESOURCE_TYPE: ReadModelType = ReadModelType::new("user-identities");
        assert_eq!(
            serde_json::to_value(RESOURCE_TYPE).unwrap(),
            json!("user-identities")
        );
        for name in ["display_name", "displayName", "表示名", "display name"] {
            assert_eq!(ReadModelAttributeKey::new(name).value(), name);
            assert_eq!(ReadModelRelationshipKey::new(name).value(), name);
        }
        for name in ["", "_name", "name-", "a.b", "a:b", "a/b"] {
            assert!(std::panic::catch_unwind(|| ReadModelType::new(name)).is_err());
            assert!(std::panic::catch_unwind(|| ReadModelAttributeKey::new(name)).is_err());
            assert!(std::panic::catch_unwind(|| ReadModelRelationshipKey::new(name)).is_err());
        }
        for name in ["id", "type"] {
            assert!(std::panic::catch_unwind(|| ReadModelAttributeKey::new(name)).is_err());
            assert!(std::panic::catch_unwind(|| ReadModelRelationshipKey::new(name)).is_err());
        }
    }
}
