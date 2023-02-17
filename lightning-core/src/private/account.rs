use std::fmt::{Debug, Display, Formatter};
use std::sync::Arc;
use crossbeam::atomic::AtomicCell;
use dashmap::DashMap;
use crate::private::position::Position;

// pub trait AsAccount {
//     fn name(&self) -> &str;
//     fn available_cash(&self) -> f64;
//     fn frozen_cash(&self) -> f64;
//     fn get_position(&self, code: &str) -> Option<Position>;
//     fn get_positions(&self) -> Vec<Position>;
// }

struct AccountInner {
    name: String,
    available_cash: AtomicCell<f64>,
    frozen_cash: AtomicCell<f64>,
    positions: DashMap<String, Position>,
}

#[derive(Clone)]
pub struct Account(Arc<AccountInner>);

impl Debug for Account {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Account(name={})", self.name())
    }
}
impl Display for Account {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Account(name={})", self.name())
    }
}
impl Account {
    pub fn new(name: &str, init_cash: f64) -> Self{
        Self(
            Arc::new(
                AccountInner{
                    name: name.to_string(),
                    available_cash: AtomicCell::new(init_cash),
                    frozen_cash: AtomicCell::new(0.0),
                    positions: DashMap::new(),
                }
            )
        )
    }
}

impl Account {
    pub fn name(&self) -> &str{
        &self.0.name
    }
    pub fn available_cash(&self) -> f64{
        self.0.available_cash.load()
    }

    pub fn frozen_cash(&self) -> f64{
        self.0.frozen_cash.load()
    }

    pub fn get_position(&self, code: &str) -> Option<Position> {
        self.0.positions.get(code).map(|pos| pos.value().clone())
    }

    pub fn get_positions(&self) -> Vec<Position> {
        self.0.positions.iter().filter_map(|position| {
            return if position.total_amount() == 0. {
                None
            } else {
                Some(position.clone())
            }
        }).collect()
    }
}
