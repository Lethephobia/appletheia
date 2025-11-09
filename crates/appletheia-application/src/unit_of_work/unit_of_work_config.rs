use crate::command::CorrelationId;
use crate::event::CausationId;
use crate::request_context::RequestContext;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct UnitOfWorkConfig {
    pub correlation_id: CorrelationId,
    pub causation_id: CausationId,
    pub request_context: RequestContext,
}
