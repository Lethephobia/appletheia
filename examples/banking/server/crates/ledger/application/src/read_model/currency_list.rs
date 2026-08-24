mod currency_list_reader;
mod currency_list_reader_error;

use appletheia::application::read_model::{
    ReadModel, ReadModelName, ReadModelObservation, ReadModelObservationSource,
    SerializedPartition, SerializedPartitionError,
};
use serde::Serialize;

use crate::projection::CurrencyFragment;

pub use currency_list_reader::CurrencyListReader;
pub use currency_list_reader_error::CurrencyListReaderError;

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct CurrencyList {
    pub items: Vec<CurrencyFragment>,
}

impl ReadModelObservationSource for CurrencyList {
    fn observations(&self) -> Vec<ReadModelObservation> {
        self.items.iter().map(|item| item.observation).collect()
    }
}

impl ReadModel for CurrencyList {
    const NAME: ReadModelName = ReadModelName::new("currency_list");

    fn partitions(&self) -> Result<Vec<SerializedPartition>, SerializedPartitionError> {
        self.items
            .iter()
            .map(|item| SerializedPartition::try_from_fragment_key::<CurrencyFragment>(&item.id))
            .collect()
    }
}
