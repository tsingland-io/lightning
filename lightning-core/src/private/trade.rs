use std::sync::Arc;
use ulid::Ulid;
use crate::utils::format_code;

struct TradeInner {
    id: Ulid,
    external_id: String,
    symbol: String,
    exchange: String,
    amount: f64,
    price: f64,
}
#[derive(Clone)]
pub struct Trade(Arc<TradeInner>);

impl Trade {
    pub fn id(&self) -> Ulid{
        self.0.id
    }
    pub fn external_id(&self) -> &str{
        &self.0.external_id
    }
    pub fn symbol(&self) -> &str{
        &self.0.symbol
    }
    pub fn exchange(&self) -> &str{
        &self.0.exchange
    }
    pub fn code(&self) -> String{
        format_code(&self.0.exchange, &self.0.symbol)
    }
    pub fn price(&self) -> f64{
        self.0.price
    }
    pub fn amount(&self) -> f64{
        self.0.amount
    }
}
