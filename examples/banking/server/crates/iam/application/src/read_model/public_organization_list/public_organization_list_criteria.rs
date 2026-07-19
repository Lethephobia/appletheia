/// Search criteria for public organization list reads.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct PublicOrganizationListCriteria {
    pub handle_contains: Vec<String>,
}
