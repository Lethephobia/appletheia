use banking_shared_kernel_application::read_model::SearchTerm;

/// Search criteria for public organization list reads.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct PublicOrganizationListCriteria {
    pub handle_contains: Vec<SearchTerm>,
}
