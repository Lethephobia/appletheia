use banking_iam_domain::User;

mod user_activator_relation;
mod user_bio_changer_relation;
mod user_deactivator_relation;
mod user_display_name_changer_relation;
mod user_owner_relation;
mod user_picture_changer_relation;
mod user_remover_relation;
mod user_username_changer_relation;

pub use user_activator_relation::UserActivatorRelation;
pub use user_bio_changer_relation::UserBioChangerRelation;
pub use user_deactivator_relation::UserDeactivatorRelation;
pub use user_display_name_changer_relation::UserDisplayNameChangerRelation;
pub use user_owner_relation::UserOwnerRelation;
pub use user_picture_changer_relation::UserPictureChangerRelation;
pub use user_remover_relation::UserRemoverRelation;
pub use user_username_changer_relation::UserUsernameChangerRelation;
