use super::{SubjectRef, TenantId};

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ActorRef {
    Anonymous {
        tenant_id: Option<TenantId>,
    },
    System,
    Subject {
        subject: SubjectRef,
        tenant_id: Option<TenantId>,
    },
}
