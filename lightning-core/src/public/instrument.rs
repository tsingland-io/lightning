use ahash::HashMap;
use rust_decimal::Decimal;
use serde_json::Value;
use serde::{Serialize, Deserialize};
use crate::utils::format_code;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Instrument{
    pub symbol: String,
    pub exchange: String,
    #[serde(default)]
    pub underlying: String,
    #[serde(default)]
    pub is_reversed: bool,
    #[serde(with = "rust_decimal::serde::str")]
    pub tick_size: Decimal,
    #[serde(with = "rust_decimal::serde::str")]
    pub lot_size: Decimal,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>
}

impl Instrument {
    pub fn code(&self) -> String{
        format_code(&self.exchange, &self.symbol)
    }

    // 裁剪价格
    pub fn trim_price(&self, price: f64) -> Decimal{
        let price_decimal = Decimal::try_from(price).expect("无法转换价格到十进制数");
        price_decimal - price_decimal % self.tick_size
    }

    // 裁剪数量
    pub fn trim_amount(&self, amount: f64) -> Decimal{
        let amount_decimal = Decimal::try_from(amount).expect("无法转换数量到十进制数");
        amount_decimal - amount_decimal % self.tick_size
    }
}
