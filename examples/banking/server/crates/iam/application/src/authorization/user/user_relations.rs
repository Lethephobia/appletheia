use appletheia::relations;
use banking_iam_domain::User;

use super::user_profile_editor_relation::UserProfileEditorRelation;

/// Defines static authorization relations for `User`.
#[relations(aggregate = User, relations = [UserProfileEditorRelation])]
pub struct UserRelations;
