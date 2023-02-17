use std::fmt::{Debug, Display, Formatter};
use std::sync::Arc;
use crossbeam::atomic::AtomicCell;
use crate::utils::format_code;

struct PositionInner {
    symbol: String,
    exchange: String,
    available_amount: AtomicCell<f64>,
    frozen_amount: AtomicCell<f64>,
    average_price: AtomicCell<f64>,
}

#[derive(Clone)]
pub struct Position(Arc<PositionInner>);

impl Position{
    pub fn symbol(&self) -> &str{
        &self.0.symbol
    }

    pub fn exchange(&self) -> &str{
        &self.0.exchange
    }

    pub fn code(&self) -> String{
        format_code(self.exchange(), self.symbol())
    }
}

impl Position {
    pub fn new(symbol: String, exchange: String) -> Self{
        Self(
            Arc::new(
                PositionInner{
                    symbol,
                    exchange,
                    available_amount: AtomicCell::new(0.0),
                    frozen_amount: AtomicCell::new(0.0),
                    average_price: AtomicCell::new(0.0),
                }
            )
        )
    }

    pub fn on_trade(&self, _trade: String) -> f64{
        0.0
    }
    pub fn total_amount(&self) -> f64 {
        self.available_amount() + self.frozen_amount()
    }
    pub fn available_amount(&self) -> f64{
        self.0.available_amount.load()
    }

    pub fn frozen_amount(&self) -> f64{
        self.0.frozen_amount.load()
    }

    pub fn average_price(&self) -> f64{
        self.0.average_price.load()
    }
}

impl Debug for Position {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Position(code={})", self.code())
    }
}
impl Display for Position {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Position(code={})", self.code())
    }
}