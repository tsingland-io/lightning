use ahash::RandomState;
use im::HashMap;
use ulid::Ulid;
use crate::models::private::account::Account;
use crate::models::private::order::Order;
use crate::models::public::instrument::Instrument;

pub struct Runtime {
    pub id: Ulid,
    /// 账户信息
    pub accounts: HashMap<Ulid, Account, RandomState>,
    /// 活跃订单
    pub orders: HashMap<Ulid, Order, RandomState>,
    /// 标的信息
    pub instruments: HashMap<String, Instrument, RandomState>,
    /// 规则
    pub rules: HashMap<Ulid, String, RandomState>,
}

pub struct RuntimeBuilder {
    id: Option<Ulid>,
    /// 账户信息
    accounts: HashMap<Ulid, Account, RandomState>,
    // /// 活跃持仓
    // positions: HashMap<Ulid, Order, RandomState>,
    // /// 活跃资产
    // assets: HashMap<Ulid, Order, RandomState>,
    // /// 活跃订单
    orders: HashMap<Ulid, Order, RandomState>,
    /// 标的信息
    instruments: HashMap<String, Instrument, RandomState>,
    /// 规则
    rules: HashMap<Ulid, String, RandomState>,
}

impl RuntimeBuilder {
    pub fn new() -> Self{
        Self{
            // 设置默认账户数量,优化性能
            id: None,
            accounts: HashMap::default(),
            // 订单信息列表
            orders: HashMap::default(),
            // 标的池
            instruments: HashMap::default(),
            // 规则池
            rules: HashMap::default(),
        }
    }

    pub fn build(self) -> Runtime{
        Runtime{
            id: self.id.unwrap_or(Ulid::new()),
            accounts: self.accounts,
            orders: self.orders,
            instruments: self.instruments,
            rules: self.rules,
        }
    }
}

impl Runtime {

    /// 注入账户信息
    pub fn set_account(&mut self, account: Account) -> Option<Account> {
        self.accounts.insert(account.id, account)
    }
    /// 获取账户列表
    pub fn get_accounts(&self) -> Vec<&Account>{
        self.accounts.values().collect()
    }

    /// 根据账户ID获取某个账户
    pub fn get_account(&self, account_id: Ulid) -> Option<&Account>{
        self.accounts.values().find(|v| v.id == account_id)
    }
    /// 根据账户名称获取某个账户
    pub fn get_account_by_name(&self, name: &str) -> Option<&Account>{
        self.accounts.values().find(|v| v.name == name)
    }

    pub fn submit_order(&mut self, order: Order) {

    }

    /// 获取活跃订单
    pub fn get_open_orders(&self, account_id: Option<Ulid>, code: Option<&str>) -> Vec<&Order>{
        self.orders.values().filter(|o| {
            // if o.status != ACTIVE{
            //     return false
            // }
            // if let Some(v) = account_id {
            //     if o.account_id != v{
            //         return false
            //     }
            // }
            // if let Some(v) = code {
            //     if o.exchange != v{
            //         return false
            //     }
            // }
            true
        }).collect()
    }

    /// 撤销订单
    pub fn cancel_order(&mut self, order: Order) -> bool{
        true
    }

    /// 获取指定标的
    pub fn get_instrument(&self, code: &str) -> Option<&Instrument>{
        self.instruments.get(code)
    }

    /// 获取标的池
    pub fn get_instruments(&self, symbol: Option<&str>, exchange: Option<&str>, underlying: Option<&str>, reversed: Option<bool>) -> Vec<&Instrument>{
        self.instruments.values().filter(|instrument| {
            if let Some(v) = symbol {
                if instrument.symbol != v{
                    return false
                }
            }
            if let Some(v) = exchange {
                if instrument.exchange != v{
                    return false
                }
            }
            if let Some(v) = underlying {
                if instrument.underlying != v{
                    return false
                }
            }
            if let Some(v) = reversed {
                if instrument.reversed != v{
                    return false
                }
            }
            true
        }).collect()
    }

    /// 更新标的池
    pub fn set_instrument(&mut self, instrument: Instrument) -> Option<Instrument>{
        self.instruments.insert(instrument.code(), instrument)
    }

    /// 获取价格
    pub fn get_price(&self){}

    pub fn set_price(&mut self) {}

}
