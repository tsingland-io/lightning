mod instrument;
mod bar;
mod transaction;

pub use instrument::Instrument;
pub use bar::Bar;
use serde::{Serialize, Deserialize};
use transaction::Transaction;

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type", content = "data", rename_all="snake_case")]
pub enum PublicData {
    Bars(Vec<Bar>),
    Instruments(Vec<Instrument>),
    Transactions(Vec<Transaction>),
    String(String),
    // base64过的代码
    Binary(String),
}