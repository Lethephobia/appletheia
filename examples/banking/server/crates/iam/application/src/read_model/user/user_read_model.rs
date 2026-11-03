use appletheia::application::read_model::{ReadModel, ReadModelType};
use banking_iam_domain::{UserBio, UserDisplayName, UserId, Username};

use crate::read_model::UserIdentityReadModelId;

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
    pub identities: Vec<UserIdentityReadModelId>,
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
        match key {
            UserReadModelRelationshipKey::Identities => {
                UserReadModelRelationship::Identities(self.identities.clone())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use appletheia::application::read_model::ReadModelView;
    use banking_iam_domain::{UserBio, UserDisplayName, UserId, Username};
    use serde_json::json;

    use super::{UserReadModel, UserReadModelFilter};

    #[test]
    fn identities_are_private_and_link_to_the_identity_resource() {
        use crate::read_model::{
            UserIdentityReadModel, UserIdentityReadModelFilter, UserIdentityReadModelId,
        };
        use banking_iam_domain::{UserIdentityProvider, UserIdentitySubject};

        let provider = UserIdentityProvider::try_from("example").unwrap();
        let subject = UserIdentitySubject::try_from("alice").unwrap();
        let identity_id = UserIdentityReadModelId::new(&provider, &subject).unwrap();
        let user = UserReadModel {
            id: UserId::new(),
            username: None,
            display_name: None,
            bio: None,
            identities: vec![identity_id.clone()],
        };
        let public = ReadModelView {
            read_model: user.clone(),
            filter: UserReadModelFilter::Public,
        }
        .try_into_resource()
        .unwrap();
        let private = ReadModelView {
            read_model: user,
            filter: UserReadModelFilter::Private,
        }
        .try_into_resource()
        .unwrap();
        let identity = ReadModelView {
            read_model: UserIdentityReadModel {
                id: identity_id.clone(),
                provider,
                subject,
                email: None,
            },
            filter: UserIdentityReadModelFilter::Private,
        }
        .try_into_resource()
        .unwrap();

        let public_json = serde_json::to_value(public).unwrap();
        let private_json = serde_json::to_value(private).unwrap();
        let identity_json = serde_json::to_value(identity).unwrap();
        assert!(public_json.get("relationships").is_none());
        assert_eq!(private_json["attributes"], public_json["attributes"]);
        assert_eq!(
            private_json["relationships"],
            json!({
                "identities": {"data": [{"type": "user_identity", "id": identity_id.to_string()}]}
            })
        );
        assert_eq!(
            identity_json,
            json!({
                "type": "user_identity",
                "id": identity_id.to_string(),
                "attributes": {"provider": "example", "subject": "alice", "email": null}
            })
        );
    }

    #[test]
    fn private_resource_with_no_identities_has_empty_to_many_linkage() {
        let resource = ReadModelView {
            read_model: UserReadModel {
                id: UserId::new(),
                username: None,
                display_name: None,
                bio: None,
                identities: vec![],
            },
            filter: UserReadModelFilter::Private,
        }
        .try_into_resource()
        .unwrap();

        assert_eq!(
            serde_json::to_value(resource).unwrap()["relationships"],
            json!({"identities": {"data": []}})
        );
    }

    #[test]
    fn public_resource_contains_untagged_profile_attributes() {
        let id = UserId::new();
        let view = ReadModelView {
            read_model: UserReadModel {
                id,
                username: Some(Username::new("alice".to_owned()).unwrap()),
                display_name: Some(UserDisplayName::new("Alice".to_owned()).unwrap()),
                bio: Some(UserBio::new("Hello".to_owned()).unwrap()),
                identities: vec![],
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
                identities: vec![],
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
