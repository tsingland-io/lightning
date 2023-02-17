use ahash::HashMap;
use serde_json::Value;
use serde::{Serialize, Deserialize};
use crate::utils::format_code;

#[derive(Serialize, Deserialize, Debug)]
pub struct Bar{
    pub symbol: String,
    pub exchange: String,
    pub timestamp: u64,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    #[serde(default)]
    pub amount: f64,
    #[serde(default)]
    pub volume: f64,
    #[serde(default)]
    pub count: u64,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

impl Bar {
    pub fn code(&self) -> String{
        format_code(&self.exchange, &self.symbol)
    }
}