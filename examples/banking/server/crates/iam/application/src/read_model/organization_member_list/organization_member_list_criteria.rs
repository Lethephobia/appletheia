use banking_shared_kernel_application::read_model::SearchTerm;

/// Search criteria for organization member list reads.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct OrganizationMemberListCriteria {
    pub username_contains: Vec<SearchTerm>,
}
