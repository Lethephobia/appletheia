use appletheia::saga_step;

#[saga_step]
pub enum OrganizationJoinRequestSagaStep {
    CreateMembership,
}
