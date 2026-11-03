use appletheia::application::read_model::{ReadModel, ReadModelType};
use banking_iam_domain::{UserIdentityProvider, UserIdentitySubject};
use banking_shared_kernel_domain::contact::Email;

use super::{
    UserIdentityReadModelAttributeKey, UserIdentityReadModelAttributeValue,
    UserIdentityReadModelFilter, UserIdentityReadModelId, UserIdentityReadModelRelationship,
    UserIdentityReadModelRelationshipKey,
};

#[derive(Clone, Debug)]
pub struct UserIdentityReadModel {
    pub id: UserIdentityReadModelId,
    pub provider: UserIdentityProvider,
    pub subject: UserIdentitySubject,
    pub email: Option<Email>,
}

impl ReadModel for UserIdentityReadModel {
    type Id = UserIdentityReadModelId;
    type AttributeKey = UserIdentityReadModelAttributeKey;
    type AttributeValue = UserIdentityReadModelAttributeValue;
    type RelationshipKey = UserIdentityReadModelRelationshipKey;
    type Relationship = UserIdentityReadModelRelationship;
    type Filter = UserIdentityReadModelFilter;

    const TYPE: ReadModelType = ReadModelType::new("user_identity");

    fn id(&self) -> &Self::Id {
        &self.id
    }

    fn attribute(&self, key: Self::AttributeKey) -> Self::AttributeValue {
        match key {
            UserIdentityReadModelAttributeKey::Provider => {
                UserIdentityReadModelAttributeValue::Provider(self.provider.clone())
            }
            UserIdentityReadModelAttributeKey::Subject => {
                UserIdentityReadModelAttributeValue::Subject(self.subject.clone())
            }
            UserIdentityReadModelAttributeKey::Email => {
                UserIdentityReadModelAttributeValue::Email(self.email.clone())
            }
        }
    }

    fn relationship(&self, key: Self::RelationshipKey) -> Self::Relationship {
        match key {}
    }
}
