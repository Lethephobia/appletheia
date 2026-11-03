use appletheia::application::read_model::ReadModelFieldFilter;

use super::{UserReadModelAttributeKey, UserReadModelRelationshipKey};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UserReadModelFilter {
    Public,
    Private,
}

impl ReadModelFieldFilter for UserReadModelFilter {
    type AttributeKey = UserReadModelAttributeKey;
    type RelationshipKey = UserReadModelRelationshipKey;

    fn attribute_keys(&self) -> &[Self::AttributeKey] {
        match self {
            Self::Public | Self::Private => &[
                UserReadModelAttributeKey::Username,
                UserReadModelAttributeKey::DisplayName,
                UserReadModelAttributeKey::Bio,
            ],
        }
    }

    fn relationship_keys(&self) -> &[Self::RelationshipKey] {
        match self {
            Self::Public => &[],
            Self::Private => &[UserReadModelRelationshipKey::Identities],
        }
    }
}
