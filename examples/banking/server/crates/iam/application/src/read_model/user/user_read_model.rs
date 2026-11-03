use appletheia::application::read_model::{ReadModel, ReadModelType};
use banking_iam_domain::{UserBio, UserDisplayName, UserId, Username};

use super::{
    UserReadModelAttributeKey, UserReadModelAttributeValue, UserReadModelFilter,
    UserReadModelRelationship, UserReadModelRelationshipKey,
};

#[derive(Clone, Debug)]
pub struct UserReadModel {
    pub id: UserId,
    pub username: Option<Username>,
    pub display_name: Option<UserDisplayName>,
    pub bio: Option<UserBio>,
}

impl ReadModel for UserReadModel {
    type Id = UserId;
    type AttributeKey = UserReadModelAttributeKey;
    type AttributeValue = UserReadModelAttributeValue;
    type RelationshipKey = UserReadModelRelationshipKey;
    type Relationship = UserReadModelRelationship;
    type Filter = UserReadModelFilter;

    const TYPE: ReadModelType = ReadModelType::new("user");

    fn id(&self) -> &Self::Id {
        &self.id
    }

    fn attribute(&self, key: Self::AttributeKey) -> Self::AttributeValue {
        match key {
            UserReadModelAttributeKey::Username => {
                UserReadModelAttributeValue::Username(self.username.clone())
            }
            UserReadModelAttributeKey::DisplayName => {
                UserReadModelAttributeValue::DisplayName(self.display_name.clone())
            }
            UserReadModelAttributeKey::Bio => UserReadModelAttributeValue::Bio(self.bio.clone()),
        }
    }

    fn relationship(&self, key: Self::RelationshipKey) -> Self::Relationship {
        match key {}
    }
}

#[cfg(test)]
mod tests {
    use appletheia::application::read_model::ReadModelView;
    use banking_iam_domain::{UserBio, UserDisplayName, UserId, Username};
    use serde_json::json;

    use super::{UserReadModel, UserReadModelFilter};

    #[test]
    fn public_resource_contains_untagged_profile_attributes() {
        let id = UserId::new();
        let view = ReadModelView {
            read_model: UserReadModel {
                id,
                username: Some(Username::new("alice".to_owned()).unwrap()),
                display_name: Some(UserDisplayName::new("Alice".to_owned()).unwrap()),
                bio: Some(UserBio::new("Hello".to_owned()).unwrap()),
            },
            filter: UserReadModelFilter::Public,
        };

        let resource = view.try_into_resource().unwrap();

        assert_eq!(
            serde_json::to_value(resource).unwrap(),
            json!({
                "type": "user",
                "id": id.to_string(),
                "attributes": {
                    "username": "alice",
                    "display_name": "Alice",
                    "bio": "Hello"
                }
            })
        );
    }

    #[test]
    fn public_resource_preserves_unset_attributes_as_null() {
        let id = UserId::new();
        let view = ReadModelView {
            read_model: UserReadModel {
                id,
                username: None,
                display_name: None,
                bio: None,
            },
            filter: UserReadModelFilter::Public,
        };

        let resource = view.try_into_resource().unwrap();

        assert_eq!(
            serde_json::to_value(resource).unwrap(),
            json!({
                "type": "user",
                "id": id.to_string(),
                "attributes": {
                    "username": null,
                    "display_name": null,
                    "bio": null
                }
            })
        );
    }
}
