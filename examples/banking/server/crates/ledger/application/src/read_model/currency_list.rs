mod currency_list_reader;
mod currency_list_reader_error;

use appletheia::application::read_model::{ReadModel, ReadModelName};
use serde::Serialize;

use crate::projection::CurrencyFragment;

pub use currency_list_reader::CurrencyListReader;
pub use currency_list_reader_error::CurrencyListReaderError;

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct CurrencyList {
    pub items: Vec<CurrencyFragment>,
}

impl ReadModel for CurrencyList {
    const NAME: ReadModelName = ReadModelName::new("currency_list");
}
