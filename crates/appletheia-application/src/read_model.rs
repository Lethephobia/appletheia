mod read_model_attribute_key;
mod read_model_attribute_key_error;
mod read_model_attribute_value;
mod read_model_attribute_value_error;
mod read_model_document;
mod read_model_document_error;
mod read_model_error;
mod read_model_field_filter;
mod read_model_id;
mod read_model_included;
mod read_model_included_error;
mod read_model_list_document;
mod read_model_list_document_error;
mod read_model_ref;
mod read_model_relationship;
mod read_model_relationship_data;
mod read_model_relationship_error;
mod read_model_relationship_key;
mod read_model_relationship_key_error;
mod read_model_resource;
mod read_model_resource_attribute_value;
mod read_model_resource_document;
mod read_model_resource_key;
mod read_model_resource_key_error;
mod read_model_resource_list_document;
mod read_model_resource_relationship;
mod read_model_type;

pub use read_model_attribute_key::*;
pub use read_model_attribute_key_error::*;
pub use read_model_attribute_value::*;
pub use read_model_attribute_value_error::*;
pub use read_model_document::*;
pub use read_model_document_error::*;
pub use read_model_error::*;
pub use read_model_field_filter::*;
pub use read_model_id::*;
pub use read_model_included::*;
pub use read_model_included_error::*;
pub use read_model_list_document::*;
pub use read_model_list_document_error::*;
pub use read_model_ref::*;
pub use read_model_relationship::*;
pub use read_model_relationship_data::*;
pub use read_model_relationship_error::*;
pub use read_model_relationship_key::*;
pub use read_model_relationship_key_error::*;
pub use read_model_resource::*;
pub use read_model_resource_attribute_value::*;
pub use read_model_resource_document::*;
pub use read_model_resource_key::*;
pub use read_model_resource_key_error::*;
pub use read_model_resource_list_document::*;
pub use read_model_resource_relationship::*;
pub use read_model_type::*;

use std::{collections::BTreeMap, fmt::Display};

/// A reusable resource whose typed fields are selected by key at the output boundary.
/// Implementations map each key to its corresponding value. Filters determine which
/// keys are requested; read models do not need to enumerate all available keys.
/// Each instance carries the filter selected by the query for its output.
pub trait ReadModel: Send + Sync {
    const TYPE: ReadModelType;

    type Id: Display + Send + Sync;
    type AttributeKey: ReadModelAttributeKey;
    type AttributeValue: ReadModelAttributeValue;
    type RelationshipKey: ReadModelRelationshipKey;
    type Relationship: ReadModelRelationship;
    type Filter: ReadModelFieldFilter<
            AttributeKey = Self::AttributeKey,
            RelationshipKey = Self::RelationshipKey,
        >;

    fn id(&self) -> &Self::Id;

    fn attribute(&self, key: Self::AttributeKey) -> Self::AttributeValue;

    fn relationship(&self, key: Self::RelationshipKey) -> Self::Relationship;

    fn filter(&self) -> &Self::Filter;

    fn try_to_resource(&self) -> Result<ReadModelResource, ReadModelError> {
        let filter = self.filter();
        let mut attributes = BTreeMap::new();
        for key in filter.attribute_keys() {
            let resource_key = key.try_to_resource_key()?;
            if attributes.contains_key(&resource_key) {
                return Err(ReadModelError::DuplicateField(resource_key));
            }
            let value = self.attribute(*key).try_to_resource_attribute_value()?;
            attributes.insert(resource_key, value);
        }

        let mut relationships = BTreeMap::new();
        for key in filter.relationship_keys() {
            let resource_key = key.try_to_resource_key()?;
            if attributes.contains_key(&resource_key) || relationships.contains_key(&resource_key) {
                return Err(ReadModelError::DuplicateField(resource_key));
            }
            let relationship = self.relationship(*key).try_to_resource_relationship()?;
            relationships.insert(resource_key, relationship);
        }

        Ok(ReadModelResource {
            read_model_type: Self::TYPE,
            id: ReadModelId::new(self.id().to_string()),
            attributes,
            relationships,
        })
    }
}

#[cfg(test)]
mod tests {
    use serde::Serialize;
    use serde_json::json;
    use uuid::Uuid;

    use super::*;

    #[derive(Clone, Copy, PartialEq, Eq)]
    enum ProfileAttributeKey {
        DisplayName,
        Picture,
        DisplayNameAlias,
        Reserved,
    }

    impl ReadModelAttributeKey for ProfileAttributeKey {
        fn try_to_resource_key(&self) -> Result<ReadModelResourceKey, ReadModelAttributeKeyError> {
            Ok(ReadModelResourceKey::new(
                match self {
                    Self::DisplayName | Self::DisplayNameAlias => "display_name",
                    Self::Reserved => "id",
                    Self::Picture => "picture",
                }
                .to_owned(),
            )?)
        }
    }

    enum ProfileAttributeValue {
        DisplayName(String),
        Picture(Option<String>),
    }

    impl ReadModelAttributeValue for ProfileAttributeValue {
        fn try_to_resource_attribute_value(
            &self,
        ) -> Result<ReadModelResourceAttributeValue, ReadModelAttributeValueError> {
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
        DisplayName,
    }

    impl ReadModelRelationshipKey for ProfileRelationshipKey {
        fn try_to_resource_key(
            &self,
        ) -> Result<ReadModelResourceKey, ReadModelRelationshipKeyError> {
            Ok(ReadModelResourceKey::new(
                match self {
                    Self::Owner => "owner",
                    Self::Authors => "authors",
                    Self::DisplayName => "display_name",
                }
                .to_owned(),
            )?)
        }
    }

    enum ProfileRelationship {
        Owner(Option<Uuid>),
        Authors(Vec<Uuid>),
    }

    impl ReadModelRelationship for ProfileRelationship {
        fn try_to_resource_relationship(
            &self,
        ) -> Result<ReadModelResourceRelationship, ReadModelRelationshipError> {
            Ok(ReadModelResourceRelationship {
                data: match self {
                    Self::Owner(id) => {
                        ReadModelRelationshipData::ToOne(id.map(ReadModelRef::new::<Profile>))
                    }
                    Self::Authors(ids) => ReadModelRelationshipData::ToMany(
                        ids.iter()
                            .map(|id| ReadModelRef::new::<Profile>(*id))
                            .collect(),
                    ),
                },
            })
        }
    }

    struct Profile {
        id: Uuid,
        display_name: String,
        picture: Option<String>,
        owner_id: Option<Uuid>,
        author_ids: Vec<Uuid>,
        filter: ProfileFilter,
    }

    impl ReadModel for Profile {
        const TYPE: ReadModelType = ReadModelType::new("profiles");

        type Id = Uuid;
        type AttributeKey = ProfileAttributeKey;
        type AttributeValue = ProfileAttributeValue;
        type RelationshipKey = ProfileRelationshipKey;
        type Relationship = ProfileRelationship;
        type Filter = ProfileFilter;

        fn id(&self) -> &Self::Id {
            &self.id
        }

        fn attribute(&self, key: Self::AttributeKey) -> Self::AttributeValue {
            match key {
                ProfileAttributeKey::DisplayName
                | ProfileAttributeKey::DisplayNameAlias
                | ProfileAttributeKey::Reserved => {
                    ProfileAttributeValue::DisplayName(self.display_name.clone())
                }
                ProfileAttributeKey::Picture => {
                    ProfileAttributeValue::Picture(self.picture.clone())
                }
            }
        }

        fn relationship(&self, key: Self::RelationshipKey) -> Self::Relationship {
            match key {
                ProfileRelationshipKey::Owner | ProfileRelationshipKey::DisplayName => {
                    ProfileRelationship::Owner(self.owner_id)
                }
                ProfileRelationshipKey::Authors => {
                    ProfileRelationship::Authors(self.author_ids.clone())
                }
            }
        }

        fn filter(&self) -> &Self::Filter {
            &self.filter
        }
    }

    enum ProfileFilter {
        Public,
        Selected {
            attributes: Vec<ProfileAttributeKey>,
            relationships: Vec<ProfileRelationshipKey>,
        },
    }

    impl ReadModelFieldFilter for ProfileFilter {
        type AttributeKey = ProfileAttributeKey;
        type RelationshipKey = ProfileRelationshipKey;

        fn attribute_keys(&self) -> &[Self::AttributeKey] {
            match self {
                Self::Public => &[ProfileAttributeKey::DisplayName],
                Self::Selected { attributes, .. } => attributes,
            }
        }

        fn relationship_keys(&self) -> &[Self::RelationshipKey] {
            match self {
                Self::Public => &[ProfileRelationshipKey::Owner],
                Self::Selected { relationships, .. } => relationships,
            }
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
            filter: ProfileFilter::Public,
        };
        let resource = profile.try_to_resource().unwrap();
        assert_eq!(
            serde_json::to_value(resource).unwrap(),
            json!({
                "type": "profiles",
                "id": Uuid::nil().to_string(),
                "attributes": {"display_name": "Alice"},
                "relationships": {
                    "owner": {"data": {"type": "profiles", "id": Uuid::from_u128(42).to_string()}}
                }
            })
        );
        assert_eq!(
            profile
                .attribute(ProfileAttributeKey::Picture)
                .try_to_resource_attribute_value()
                .unwrap()
                .value(),
            &json!(null)
        );
        assert_eq!(
            serde_json::to_value(
                profile
                    .relationship(ProfileRelationshipKey::Authors)
                    .try_to_resource_relationship()
                    .unwrap()
            )
            .unwrap(),
            json!({"data": [{"type": "profiles", "id": Uuid::from_u128(43).to_string()}]})
        );
    }

    #[test]
    fn absent_and_empty_relationships_preserve_their_cardinality() {
        assert_eq!(
            serde_json::to_value(
                ProfileRelationship::Owner(None)
                    .try_to_resource_relationship()
                    .unwrap()
            )
            .unwrap(),
            json!({"data": null})
        );
        assert_eq!(
            serde_json::to_value(
                ProfileRelationship::Authors(vec![])
                    .try_to_resource_relationship()
                    .unwrap()
            )
            .unwrap(),
            json!({"data": []})
        );
    }

    #[test]
    fn attribute_values_preserve_nested_objects_arrays_and_null() {
        for value in [
            json!({"width": 10, "height": 20}),
            json!([1, null]),
            json!(null),
        ] {
            let serialized = ReadModelResourceAttributeValue::from(value.clone());
            assert_eq!(serde_json::to_value(&serialized).unwrap(), value);
            let restored: ReadModelResourceAttributeValue =
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
    fn resource_omits_unselected_fields_but_preserves_selected_null() {
        let mut profile = Profile {
            id: Uuid::nil(),
            display_name: "Alice".to_owned(),
            picture: None,
            owner_id: None,
            author_ids: vec![],
            filter: ProfileFilter::Public,
        };
        profile.filter = ProfileFilter::Selected {
            attributes: vec![],
            relationships: vec![],
        };
        assert_eq!(
            serde_json::to_value(profile.try_to_resource().unwrap()).unwrap(),
            json!({"type": "profiles", "id": Uuid::nil().to_string()})
        );
        profile.filter = ProfileFilter::Selected {
            attributes: vec![ProfileAttributeKey::Picture],
            relationships: vec![],
        };
        assert_eq!(
            serde_json::to_value(profile.try_to_resource().unwrap()).unwrap(),
            json!({"type": "profiles", "id": Uuid::nil().to_string(), "attributes": {"picture": null}})
        );
    }

    #[test]
    fn resource_rejects_duplicate_output_names_and_propagates_invalid_keys() {
        let mut profile = Profile {
            id: Uuid::nil(),
            display_name: "Alice".to_owned(),
            picture: None,
            owner_id: None,
            author_ids: vec![],
            filter: ProfileFilter::Public,
        };
        for filter in [
            ProfileFilter::Selected {
                attributes: vec![
                    ProfileAttributeKey::DisplayName,
                    ProfileAttributeKey::DisplayNameAlias,
                ],
                relationships: vec![],
            },
            ProfileFilter::Selected {
                attributes: vec![ProfileAttributeKey::DisplayName],
                relationships: vec![ProfileRelationshipKey::DisplayName],
            },
            ProfileFilter::Selected {
                attributes: vec![],
                relationships: vec![ProfileRelationshipKey::Owner, ProfileRelationshipKey::Owner],
            },
        ] {
            profile.filter = filter;
            assert!(matches!(
                profile.try_to_resource(),
                Err(ReadModelError::DuplicateField(_))
            ));
        }
        profile.filter = ProfileFilter::Selected {
            attributes: vec![ProfileAttributeKey::Reserved],
            relationships: vec![],
        };
        assert!(matches!(
            profile.try_to_resource(),
            Err(ReadModelError::AttributeKey(
                ReadModelAttributeKeyError::ResourceKey(ReadModelResourceKeyError::ReservedName(_))
            ))
        ));
    }

    enum ProfileIncluded {
        Profile(Profile),
    }

    impl ReadModelIncluded for ProfileIncluded {
        fn try_to_resource(&self) -> Result<ReadModelResource, ReadModelIncludedError> {
            match self {
                Self::Profile(profile) => Ok(profile.try_to_resource()?),
            }
        }
    }

    #[test]
    fn document_preserves_typed_data_and_each_resources_filter() {
        let document = ReadModelDocument {
            data: Some(Profile {
                id: Uuid::nil(),
                display_name: "Alice".to_owned(),
                picture: None,
                owner_id: Some(Uuid::from_u128(42)),
                author_ids: vec![],
                filter: ProfileFilter::Public,
            }),
            included: vec![ProfileIncluded::Profile(Profile {
                id: Uuid::from_u128(42),
                display_name: "Owner".to_owned(),
                picture: None,
                owner_id: None,
                author_ids: vec![],
                filter: ProfileFilter::Selected {
                    attributes: vec![ProfileAttributeKey::Picture],
                    relationships: vec![],
                },
            })],
        };
        let resource_document = document.try_to_resource_document().unwrap();
        assert_eq!(document.data.as_ref().unwrap().display_name, "Alice");
        assert_eq!(
            serde_json::to_value(resource_document).unwrap(),
            json!({
                "data": {
                    "type": "profiles", "id": Uuid::nil().to_string(),
                    "attributes": {"display_name": "Alice"},
                    "relationships": {"owner": {"data": {"type": "profiles", "id": Uuid::from_u128(42).to_string()}}}
                },
                "included": [{
                    "type": "profiles", "id": Uuid::from_u128(42).to_string(),
                    "attributes": {"picture": null}
                }]
            })
        );
    }

    #[test]
    fn empty_documents_preserve_single_and_list_shapes() {
        let single: ReadModelDocument<Profile, ProfileIncluded> = ReadModelDocument {
            data: None,
            included: vec![],
        };
        let list: ReadModelListDocument<Profile, ProfileIncluded> = ReadModelListDocument {
            data: vec![],
            included: vec![],
        };
        assert_eq!(
            serde_json::to_value(single.try_to_resource_document().unwrap()).unwrap(),
            json!({"data": null})
        );
        assert_eq!(
            serde_json::to_value(list.try_to_resource_document().unwrap()).unwrap(),
            json!({"data": []})
        );
    }

    #[test]
    fn list_document_preserves_order_and_per_item_filters() {
        let document: ReadModelListDocument<Profile, ProfileIncluded> = ReadModelListDocument {
            data: vec![
                Profile {
                    id: Uuid::nil(),
                    display_name: "Alice".to_owned(),
                    picture: None,
                    owner_id: None,
                    author_ids: vec![],
                    filter: ProfileFilter::Public,
                },
                Profile {
                    id: Uuid::from_u128(42),
                    display_name: "Bob".to_owned(),
                    picture: None,
                    owner_id: None,
                    author_ids: vec![],
                    filter: ProfileFilter::Selected {
                        attributes: vec![ProfileAttributeKey::Picture],
                        relationships: vec![],
                    },
                },
            ],
            included: vec![],
        };
        let output = serde_json::to_value(document.try_to_resource_document().unwrap()).unwrap();
        assert_eq!(output["data"][0]["id"], Uuid::nil().to_string());
        assert_eq!(
            output["data"][0]["attributes"],
            json!({"display_name": "Alice"})
        );
        assert_eq!(output["data"][1]["id"], Uuid::from_u128(42).to_string());
        assert_eq!(output["data"][1]["attributes"], json!({"picture": null}));
        assert!(output.get("included").is_none());
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
