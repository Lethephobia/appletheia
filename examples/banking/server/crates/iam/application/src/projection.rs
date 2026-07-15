mod user_private_info;
mod user_public_profile;

pub use user_private_info::{
    UserPrivateInfoProjector, UserPrivateInfoProjectorError, UserPrivateInfoProjectorSpec,
};
pub use user_public_profile::{
    UserPublicProfileProjector, UserPublicProfileProjectorError, UserPublicProfileProjectorSpec,
};
