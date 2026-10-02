mod token_binding_error;
mod token_binding_event_payload;
mod token_binding_event_payload_error;
mod token_binding_id;
mod token_binding_state;
mod token_binding_state_error;
mod token_binding_status;

pub use token_binding_error::TokenBindingError;
pub use token_binding_event_payload::TokenBindingEventPayload;
pub use token_binding_event_payload_error::TokenBindingEventPayloadError;
pub use token_binding_id::TokenBindingId;
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

    pub fn define(
        &mut self,
        currency_id: CurrencyId,
        chain_network: ChainNetwork,
        token_address: TokenAddress,
        deposit_enabled: bool,
        withdrawal_enabled: bool,
    ) -> Result<(), TokenBindingError> {
        if self.state().is_some() {
            return Err(TokenBindingError::AlreadyDefined);
        }
        if !token_address.matches_network(chain_network) {
            return Err(TokenBindingError::ChainMismatch);
        }

        self.append_event(TokenBindingEventPayload::Defined {
            currency_id,
            chain_network,
            token_address,
            deposit_enabled,
            withdrawal_enabled,
        })?;
        Ok(())
    }

    pub fn change_deposit_enabled(&mut self, enabled: bool) -> Result<(), TokenBindingError> {
        let state = self.state_required()?;
        if state.status.is_removed() {
            return Err(TokenBindingError::Removed);
        }
        if state.deposit_enabled == enabled {
            return Err(if enabled {
                TokenBindingError::DepositAlreadyEnabled
            } else {
                TokenBindingError::DepositAlreadyDisabled
            });
        }

        self.append_event(TokenBindingEventPayload::DepositEnabledChanged { enabled })?;
        Ok(())
    }

    pub fn change_withdrawal_enabled(&mut self, enabled: bool) -> Result<(), TokenBindingError> {
        let state = self.state_required()?;
        if state.status.is_removed() {
            return Err(TokenBindingError::Removed);
        }
        if state.withdrawal_enabled == enabled {
            return Err(if enabled {
                TokenBindingError::WithdrawalAlreadyEnabled
            } else {
                TokenBindingError::WithdrawalAlreadyDisabled
            });
        }

        self.append_event(TokenBindingEventPayload::WithdrawalEnabledChanged { enabled })?;
        Ok(())
    }

    pub fn remove(&mut self) -> Result<(), TokenBindingError> {
        if self.state_required()?.status.is_removed() {
            return Err(TokenBindingError::Removed);
        }

        self.append_event(TokenBindingEventPayload::Removed)?;
        Ok(())
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

    use super::{TokenBinding, TokenBindingError, TokenBindingStatus};

    fn token_address() -> TokenAddress {
        TokenAddress::Ethereum(
            EvmTokenContractAddress::from_str("0x1111111111111111111111111111111111111111")
                .expect("token address should be valid"),
        )
    }

    #[test]
    fn defines_one_binding_as_an_independent_aggregate() {
        let mut token_binding = TokenBinding::new();
        let currency_id = CurrencyId::new();

        token_binding
            .define(
                currency_id,
                ChainNetwork::Ethereum,
                token_address(),
                true,
                false,
            )
            .expect("token binding definition should succeed");

        assert_eq!(
            token_binding.currency_id().expect("state should exist"),
            currency_id
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
    fn definition_rejects_already_defined_binding_without_recording_an_event() {
        let mut token_binding = TokenBinding::new();
        token_binding
            .define(
                CurrencyId::new(),
                ChainNetwork::Ethereum,
                token_address(),
                true,
                false,
            )
            .expect("binding should be defined");

        let error = token_binding
            .define(
                CurrencyId::new(),
                ChainNetwork::Ethereum,
                token_address(),
                true,
                false,
            )
            .expect_err("duplicate definition should fail");

        assert!(matches!(error, TokenBindingError::AlreadyDefined));
        assert_eq!(token_binding.uncommitted_events().len(), 1);
    }

    #[test]
    fn changes_deposit_and_withdrawal_enablement_independently() {
        let mut token_binding = TokenBinding::new();
        token_binding
            .define(
                CurrencyId::new(),
                ChainNetwork::Ethereum,
                token_address(),
                true,
                false,
            )
            .expect("token binding definition should succeed");

        token_binding
            .change_deposit_enabled(false)
            .expect("deposit enablement change should succeed");
        token_binding
            .change_withdrawal_enabled(true)
            .expect("withdrawal enablement change should succeed");
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
            .define(
                CurrencyId::new(),
                ChainNetwork::Ethereum,
                token_address(),
                true,
                false,
            )
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
            .define(
                CurrencyId::new(),
                ChainNetwork::Ethereum,
                token_address(),
                true,
                false,
            )
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
            .define(
                CurrencyId::new(),
                ChainNetwork::Ethereum,
                token_address(),
                true,
                false,
            )
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
