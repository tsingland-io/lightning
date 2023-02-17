use std::ops::Deref;
use std::sync::Arc;
use crossbeam::atomic::AtomicCell;
use ulid::Ulid;
use crate::utils::format_code;

struct OrderInner {
    id: Ulid,
    external_id: String,
    symbol: String,
    exchange: String,
    amount: f64,
    filled_amount: AtomicCell<f64>,
    price: f64,
    average_price: AtomicCell<f64>,
    status: AtomicCell<u8>,
}

#[derive(Clone)]
pub struct Order(Arc<OrderInner>);

impl Order {
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

    pub fn amount(&self) -> f64{
        self.0.amount
    }
    pub fn filled_amount(&self) -> f64{
        self.0.filled_amount.load()
    }
    pub fn price(&self) -> f64{
        self.0.price
    }
    pub fn average_price(&self) -> f64{
        self.0.average_price.load()
    }

    pub fn status(&self) -> u8{
        self.0.status.load()
    }
}
