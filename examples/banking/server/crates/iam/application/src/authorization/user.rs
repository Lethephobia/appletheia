use banking_iam_domain::User;

mod user_activator_relation;
mod user_deactivator_relation;
mod user_owner_relation;
mod user_profile_editor_relation;
mod user_remover_relation;
mod user_status_manager_relation;

pub use user_activator_relation::UserActivatorRelation;
pub use user_deactivator_relation::UserDeactivatorRelation;
pub use user_owner_relation::UserOwnerRelation;
pub use user_profile_editor_relation::UserProfileEditorRelation;
pub use user_remover_relation::UserRemoverRelation;
pub use user_status_manager_relation::UserStatusManagerRelation;
