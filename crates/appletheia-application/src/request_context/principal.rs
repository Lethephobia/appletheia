use super::{SubjectRef, TenantId};

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum Principal {
    Unavailable,
    Anonymous,
    System,
    Authenticated {
        subject: SubjectRef,
        tenant_id: Option<TenantId>,
    },
}

impl Default for Principal {
    fn default() -> Self {
        Self::Unavailable
    }
}
