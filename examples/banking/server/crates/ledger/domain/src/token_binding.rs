mod token_binding_define_rejection_reason;
mod token_binding_definition;
mod token_binding_enablement_change_rejection_reason;
mod token_binding_enablement_change_result;
mod token_binding_error;
mod token_binding_event_payload;
mod token_binding_event_payload_error;
mod token_binding_id;
mod token_binding_remove_rejection_reason;
mod token_binding_remove_result;
mod token_binding_state;
mod token_binding_state_error;
mod token_binding_status;

pub use token_binding_define_rejection_reason::TokenBindingDefineRejectionReason;
pub use token_binding_definition::TokenBindingDefinition;
pub use token_binding_enablement_change_rejection_reason::TokenBindingEnablementChangeRejectionReason;
pub use token_binding_enablement_change_result::TokenBindingEnablementChangeResult;
pub use token_binding_error::TokenBindingError;
pub use token_binding_event_payload::TokenBindingEventPayload;
pub use token_binding_event_payload_error::TokenBindingEventPayloadError;
pub use token_binding_id::TokenBindingId;
pub use token_binding_remove_rejection_reason::TokenBindingRemoveRejectionReason;
pub use token_binding_remove_result::TokenBindingRemoveResult;
pub use token_binding_state::TokenBindingState;
pub use token_binding_state_error::TokenBindingStateError;
pub use token_binding_status::TokenBindingStatus;

use appletheia::aggregate;
use appletheia::domain::{Aggregate, AggregateApply, AggregateCore};

use crate::core::{ChainNetwork, TokenAddress};
use crate::currency::CurrencyId;

/// Binds one Currency to an external token on one blockchain network.
#[aggregate(type = "token_binding", error = TokenBindingError)]
pub struct TokenBinding {
    core: AggregateCore<TokenBindingId, TokenBindingState, TokenBindingEventPayload>,
}

impl TokenBinding {
    pub fn currency_id(&self) -> Result<CurrencyId, TokenBindingError> {
        Ok(self.state_required()?.currency_id)
    }

    pub fn chain_network(&self) -> Result<ChainNetwork, TokenBindingError> {
        Ok(self.state_required()?.chain_network)
    }

    pub fn token_address(&self) -> Result<&TokenAddress, TokenBindingError> {
        Ok(&self.state_required()?.token_address)
    }

    pub fn is_deposit_enabled(&self) -> Result<bool, TokenBindingError> {
        Ok(self.state_required()?.deposit_enabled)
    }

    pub fn is_withdrawal_enabled(&self) -> Result<bool, TokenBindingError> {
        Ok(self.state_required()?.withdrawal_enabled)
    }

    pub fn status(&self) -> Result<TokenBindingStatus, TokenBindingError> {
        Ok(self.state_required()?.status)
    }

    pub fn is_active(&self) -> Result<bool, TokenBindingError> {
        Ok(self.state_required()?.status.is_active())
    }

    pub fn define(&mut self, definition: TokenBindingDefinition) -> Result<(), TokenBindingError> {
        if self.state().is_some() {
            return Err(TokenBindingError::AlreadyDefined);
        }
        if !definition
            .token_address
            .matches_network(definition.chain_network)
        {
            return Err(TokenBindingError::ChainMismatch);
        }

        self.append_event(TokenBindingEventPayload::Defined {
            currency_id: definition.currency_id,
            chain_network: definition.chain_network,
            token_address: definition.token_address,
            deposit_enabled: definition.deposit_enabled,
            withdrawal_enabled: definition.withdrawal_enabled,
        })?;
        Ok(())
    }

    pub fn reject_define(
        &mut self,
        _definition: TokenBindingDefinition,
        reason: TokenBindingDefineRejectionReason,
    ) -> Result<(), TokenBindingError> {
        Err(TokenBindingError::DefinitionRejected(reason))
    }

    pub fn change_deposit_enabled(
        &mut self,
        enabled: bool,
    ) -> Result<TokenBindingEnablementChangeResult, TokenBindingError> {
        let state = self.state_required()?;
        let reason = if state.status.is_removed() {
            Some(TokenBindingEnablementChangeRejectionReason::Removed)
        } else if state.deposit_enabled == enabled {
            Some(if enabled {
                TokenBindingEnablementChangeRejectionReason::AlreadyEnabled
            } else {
                TokenBindingEnablementChangeRejectionReason::AlreadyDisabled
            })
        } else {
            None
        };
        if let Some(reason) = reason {
            self.reject_change_deposit_enabled(enabled, reason)?;
            return Ok(TokenBindingEnablementChangeResult::Rejected { reason });
        }

        self.append_event(TokenBindingEventPayload::DepositEnabledChanged { enabled })?;
        Ok(TokenBindingEnablementChangeResult::Changed)
    }

    pub fn reject_change_deposit_enabled(
        &mut self,
        _enabled: bool,
        reason: TokenBindingEnablementChangeRejectionReason,
    ) -> Result<(), TokenBindingError> {
        Err(TokenBindingError::EnablementChangeRejected(reason))
    }

    pub fn change_withdrawal_enabled(
        &mut self,
        enabled: bool,
    ) -> Result<TokenBindingEnablementChangeResult, TokenBindingError> {
        let state = self.state_required()?;
        let reason = if state.status.is_removed() {
            Some(TokenBindingEnablementChangeRejectionReason::Removed)
        } else if state.withdrawal_enabled == enabled {
            Some(if enabled {
                TokenBindingEnablementChangeRejectionReason::AlreadyEnabled
            } else {
                TokenBindingEnablementChangeRejectionReason::AlreadyDisabled
            })
        } else {
            None
        };
        if let Some(reason) = reason {
            self.reject_change_withdrawal_enabled(enabled, reason)?;
            return Ok(TokenBindingEnablementChangeResult::Rejected { reason });
        }

        self.append_event(TokenBindingEventPayload::WithdrawalEnabledChanged { enabled })?;
        Ok(TokenBindingEnablementChangeResult::Changed)
    }

    pub fn reject_change_withdrawal_enabled(
        &mut self,
        _enabled: bool,
        reason: TokenBindingEnablementChangeRejectionReason,
    ) -> Result<(), TokenBindingError> {
        Err(TokenBindingError::EnablementChangeRejected(reason))
    }

    pub fn remove(&mut self) -> Result<TokenBindingRemoveResult, TokenBindingError> {
        if self.state_required()?.status.is_removed() {
            let reason = TokenBindingRemoveRejectionReason::AlreadyRemoved;
            self.reject_remove(reason)?;
            return Ok(TokenBindingRemoveResult::Rejected { reason });
        }

        self.append_event(TokenBindingEventPayload::Removed)?;
        Ok(TokenBindingRemoveResult::Removed)
    }

    pub fn reject_remove(
        &mut self,
        reason: TokenBindingRemoveRejectionReason,
    ) -> Result<(), TokenBindingError> {
        Err(TokenBindingError::RemovalRejected(reason))
    }
}

impl AggregateApply<TokenBindingEventPayload, TokenBindingError> for TokenBinding {
    fn apply(&mut self, payload: &TokenBindingEventPayload) -> Result<(), TokenBindingError> {
        match payload {
            TokenBindingEventPayload::Defined {
                currency_id,
                chain_network,
                token_address,
                deposit_enabled,
                withdrawal_enabled,
            } => self.set_state(Some(TokenBindingState {
                currency_id: *currency_id,
                chain_network: *chain_network,
                token_address: *token_address,
                deposit_enabled: *deposit_enabled,
                withdrawal_enabled: *withdrawal_enabled,
                status: TokenBindingStatus::Active,
            })),
            TokenBindingEventPayload::DepositEnabledChanged { enabled } => {
                self.state_required_mut()?.deposit_enabled = *enabled;
            }
            TokenBindingEventPayload::WithdrawalEnabledChanged { enabled } => {
                self.state_required_mut()?.withdrawal_enabled = *enabled;
            }
            TokenBindingEventPayload::Removed => {
                self.state_required_mut()?.status = TokenBindingStatus::Removed;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use appletheia::domain::Aggregate;

    use crate::core::{ChainNetwork, EvmTokenContractAddress, TokenAddress};
    use crate::currency::CurrencyId;

    use super::{
        TokenBinding, TokenBindingDefineRejectionReason, TokenBindingDefinition,
        TokenBindingEnablementChangeResult, TokenBindingStatus,
    };

    fn definition() -> TokenBindingDefinition {
        TokenBindingDefinition {
            currency_id: CurrencyId::new(),
            chain_network: ChainNetwork::Ethereum,
            token_address: TokenAddress::Ethereum(
                EvmTokenContractAddress::from_str("0x1111111111111111111111111111111111111111")
                    .expect("token address should be valid"),
            ),
            deposit_enabled: true,
            withdrawal_enabled: false,
        }
    }

    #[test]
    fn defines_one_binding_as_an_independent_aggregate() {
        let mut token_binding = TokenBinding::new();
        let definition = definition();

        token_binding
            .define(definition.clone())
            .expect("token binding definition should succeed");

        assert_eq!(
            token_binding.currency_id().expect("state should exist"),
            definition.currency_id
        );
        assert_eq!(
            token_binding.status().expect("state should exist"),
            TokenBindingStatus::Active
        );
        assert!(
            token_binding
                .is_deposit_enabled()
                .expect("state should exist")
        );
        assert!(
            !token_binding
                .is_withdrawal_enabled()
                .expect("state should exist")
        );
    }

    #[test]
    fn definition_rejection_returns_error_without_creating_binding_state() {
        let mut token_binding = TokenBinding::new();
        let definition = definition();

        token_binding
            .reject_define(
                definition.clone(),
                TokenBindingDefineRejectionReason::DuplicateToken,
            )
            .expect_err("definition should be rejected");

        assert!(token_binding.state().is_none());
        assert!(token_binding.uncommitted_events().is_empty());
    }

    #[test]
    fn changes_deposit_and_withdrawal_enablement_independently() {
        let mut token_binding = TokenBinding::new();
        token_binding
            .define(definition())
            .expect("token binding definition should succeed");

        assert_eq!(
            token_binding
                .change_deposit_enabled(false)
                .expect("deposit enablement change should succeed"),
            TokenBindingEnablementChangeResult::Changed
        );
        assert_eq!(
            token_binding
                .change_withdrawal_enabled(true)
                .expect("withdrawal enablement change should succeed"),
            TokenBindingEnablementChangeResult::Changed
        );
        assert!(
            !token_binding
                .is_deposit_enabled()
                .expect("state should exist")
        );
        assert!(
            token_binding
                .is_withdrawal_enabled()
                .expect("state should exist")
        );
    }

    #[test]
    fn unchanged_enablement_returns_errors() {
        let mut token_binding = TokenBinding::new();
        token_binding
            .define(definition())
            .expect("token binding definition should succeed");

        token_binding
            .change_deposit_enabled(true)
            .expect_err("unchanged deposit enablement should fail");
        token_binding
            .change_withdrawal_enabled(false)
            .expect_err("unchanged withdrawal enablement should fail");
    }

    #[test]
    fn enablement_changes_after_removal_return_errors() {
        let mut token_binding = TokenBinding::new();
        token_binding
            .define(definition())
            .expect("token binding definition should succeed");
        token_binding.remove().expect("removal should succeed");

        token_binding
            .change_deposit_enabled(false)
            .expect_err("removed binding should reject deposit enablement change");
        token_binding
            .change_withdrawal_enabled(true)
            .expect_err("removed binding should reject withdrawal enablement change");
    }

    #[test]
    fn repeated_removal_returns_error_without_appending_event() {
        let mut token_binding = TokenBinding::new();
        token_binding
            .define(definition())
            .expect("token binding definition should succeed");
        token_binding
            .remove()
            .expect("first removal should succeed");

        token_binding
            .remove()
            .expect_err("repeated removal should fail");
        assert_eq!(token_binding.uncommitted_events().len(), 2);
    }
}
