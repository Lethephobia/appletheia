use appletheia::event_payload;

use crate::core::{ChainNetwork, TokenAddress};
use crate::currency::CurrencyId;

use super::TokenBindingEventPayloadError;

#[event_payload(error = TokenBindingEventPayloadError)]
pub enum TokenBindingEventPayload {
    Defined {
        currency_id: CurrencyId,
        chain_network: ChainNetwork,
        token_address: TokenAddress,
        deposit_enabled: bool,
        withdrawal_enabled: bool,
    },
    DepositEnabledChanged {
        enabled: bool,
    },
    WithdrawalEnabledChanged {
        enabled: bool,
    },
    Removed,
}
