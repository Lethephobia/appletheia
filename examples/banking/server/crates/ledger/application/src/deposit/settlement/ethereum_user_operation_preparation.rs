use super::EvmUserOperationRequest;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EthereumUserOperationPreparation {
    user_operation_request: EvmUserOperationRequest,
}

impl EthereumUserOperationPreparation {
    pub const fn new(user_operation_request: EvmUserOperationRequest) -> Self {
        Self {
            user_operation_request,
        }
    }

    pub const fn user_operation_request(&self) -> &EvmUserOperationRequest {
        &self.user_operation_request
    }

    pub fn into_user_operation_request(self) -> EvmUserOperationRequest {
        self.user_operation_request
    }
}
