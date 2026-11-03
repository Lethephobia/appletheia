use appletheia::application::read_model::ReadModelFieldFilter;

use super::{UserIdentityReadModelAttributeKey, UserIdentityReadModelRelationshipKey};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UserIdentityReadModelFilter {
    Private,
}

impl ReadModelFieldFilter for UserIdentityReadModelFilter {
    type AttributeKey = UserIdentityReadModelAttributeKey;
    type RelationshipKey = UserIdentityReadModelRelationshipKey;

    fn attribute_keys(&self) -> &[Self::AttributeKey] {
        match self {
            Self::Private => &[
                UserIdentityReadModelAttributeKey::Provider,
                UserIdentityReadModelAttributeKey::Subject,
                UserIdentityReadModelAttributeKey::Email,
            ],
        }
    }

    fn relationship_keys(&self) -> &[Self::RelationshipKey] {
        match self {
            Self::Private => &[],
        }
    }
}
