use appletheia::application::read_model::{
    ReadModelFragment, ReadModelFragmentName, ReadModelObservation, ReadModelObservationSource,
};
use banking_ledger_domain::core::{CurrencyCode, CurrencyDecimals};
use banking_ledger_domain::currency::{CurrencyDescription, CurrencyId, CurrencyStatus};
use banking_ledger_domain::currency_registrar::CurrencyRegistrarId;
use serde::{Deserialize, Serialize};

use super::CurrencyTokenBindingFragment;

/// Event-backed Currency projection used by settlement and account queries.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct CurrencyFragment {
    pub id: CurrencyId,
    pub currency_registrar_id: CurrencyRegistrarId,
    pub code: CurrencyCode,
    pub decimals: CurrencyDecimals,
    pub description: Option<CurrencyDescription>,
    pub status: CurrencyStatus,
    pub token_bindings: Vec<CurrencyTokenBindingFragment>,
    pub observation: ReadModelObservation,
}

impl ReadModelObservationSource for CurrencyFragment {
    fn observations(&self) -> Vec<ReadModelObservation> {
        vec![self.observation]
    }
}

impl ReadModelFragment for CurrencyFragment {
    const NAME: ReadModelFragmentName = ReadModelFragmentName::new("currency_fragment");

    type Key = CurrencyId;

    fn key(&self) -> Self::Key {
        self.id
    }
}
