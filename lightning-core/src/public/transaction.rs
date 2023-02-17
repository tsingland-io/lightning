use ahash::HashMap;
use serde_json::Value;
use serde::{Serialize, Deserialize};
use ulid::Ulid;
use crate::utils::format_code;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Transaction {
    #[serde(default = "Ulid::new")]
    pub id: Ulid,
    #[serde(default)]
    pub external_id: String,
    pub symbol: String,
    pub exchange: String,
    pub timestamp: u64,
    pub price: f64,
    pub amount: f64,
    pub side: u8,
    #[serde(default)]
    pub volume: f64,
    #[serde(default)]
    pub count: u64,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

impl Transaction {
    pub fn code(&self) -> String{
        format_code(&self.exchange, &self.symbol)
    }
}
