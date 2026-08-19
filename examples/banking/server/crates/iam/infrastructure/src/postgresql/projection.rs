use banking_iam_application::{
    OrganizationFragmentProjector, OrganizationInvitationFragmentProjector,
    OrganizationJoinRequestFragmentProjector, OrganizationMembershipFragmentProjector,
    UserFragmentProjector, UserIdentityFragmentProjector,
};

mod organization_fragment;
mod organization_invitation_fragment;
mod organization_join_request_fragment;
mod organization_membership_fragment;
mod user_fragment;
mod user_identity_fragment;

pub use organization_fragment::PgOrganizationFragmentWriter;
pub use organization_invitation_fragment::PgOrganizationInvitationFragmentWriter;
pub use organization_join_request_fragment::PgOrganizationJoinRequestFragmentWriter;
pub use organization_membership_fragment::PgOrganizationMembershipFragmentWriter;
pub use user_fragment::PgUserFragmentWriter;
pub use user_identity_fragment::PgUserIdentityFragmentWriter;

/// PostgreSQL-backed organization fragment projector.
pub type PgOrganizationFragmentProjector =
    OrganizationFragmentProjector<PgOrganizationFragmentWriter>;

/// PostgreSQL-backed public user fragment projector.
pub type PgUserFragmentProjector = UserFragmentProjector<PgUserFragmentWriter>;

/// PostgreSQL-backed user identity fragment projector.
pub type PgUserIdentityFragmentProjector =
    UserIdentityFragmentProjector<PgUserIdentityFragmentWriter>;

/// PostgreSQL-backed organization membership fragment projector.
pub type PgOrganizationMembershipFragmentProjector =
    OrganizationMembershipFragmentProjector<PgOrganizationMembershipFragmentWriter>;

/// PostgreSQL-backed organization invitation fragment projector.
pub type PgOrganizationInvitationFragmentProjector =
    OrganizationInvitationFragmentProjector<PgOrganizationInvitationFragmentWriter>;

/// PostgreSQL-backed organization join request fragment projector.
pub type PgOrganizationJoinRequestFragmentProjector =
    OrganizationJoinRequestFragmentProjector<PgOrganizationJoinRequestFragmentWriter>;
