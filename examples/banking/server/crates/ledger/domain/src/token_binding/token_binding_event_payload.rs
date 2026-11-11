use appletheia::event_payload;

use crate::currency::CurrencyId;
use crate::onchain::{ChainNetwork, TokenAddress};

use super::TokenBindingEventPayloadError;

#[event_payload(error = TokenBindingEventPayloadError)]
pub enum TokenBindingEventPayload {
    Bound {
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
    Unbound,
}
