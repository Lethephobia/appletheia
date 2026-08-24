use appletheia::event_payload;

use crate::core::{ChainNetwork, TokenAddress};
use crate::currency::CurrencyId;

use super::{
    TokenBindingDefineRejectionReason, TokenBindingDefinition, TokenBindingEventPayloadError,
    TokenBindingRemoveRejectionReason,
};

#[event_payload(error = TokenBindingEventPayloadError)]
pub enum TokenBindingEventPayload {
    Defined {
        currency_id: CurrencyId,
        chain_network: ChainNetwork,
        token_address: TokenAddress,
    },
    DefinitionRejected {
        definition: TokenBindingDefinition,
        reason: TokenBindingDefineRejectionReason,
    },
    Removed,
    RemovalRejected {
        reason: TokenBindingRemoveRejectionReason,
    },
}
