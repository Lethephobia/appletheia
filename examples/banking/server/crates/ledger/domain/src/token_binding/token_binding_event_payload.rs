use appletheia::event_payload;

use crate::core::{ChainNetwork, TokenAddress};
use crate::currency::CurrencyId;

use super::{
    TokenBindingDefineRejectionReason, TokenBindingDefinition,
    TokenBindingEnablementChangeRejectionReason, TokenBindingEventPayloadError,
    TokenBindingRemoveRejectionReason,
};

#[event_payload(error = TokenBindingEventPayloadError)]
pub enum TokenBindingEventPayload {
    Defined {
        currency_id: CurrencyId,
        chain_network: ChainNetwork,
        token_address: TokenAddress,
        deposit_enabled: bool,
        withdrawal_enabled: bool,
    },
    DefinitionRejected {
        definition: TokenBindingDefinition,
        reason: TokenBindingDefineRejectionReason,
    },
    DepositEnabledChanged {
        enabled: bool,
    },
    DepositEnabledChangeRejected {
        enabled: bool,
        reason: TokenBindingEnablementChangeRejectionReason,
    },
    WithdrawalEnabledChanged {
        enabled: bool,
    },
    WithdrawalEnabledChangeRejected {
        enabled: bool,
        reason: TokenBindingEnablementChangeRejectionReason,
    },
    Removed,
    RemovalRejected {
        reason: TokenBindingRemoveRejectionReason,
    },
}
