use std::fmt::{Debug, Display, Formatter};
use std::hash::{Hash, Hasher};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::Duration;
use ahash::{HashMap, HashMapExt};
use dashmap::DashMap;
use log::{info, warn};
use nng::{Error, Socket};
use signal_hook::consts::TERM_SIGNALS;
use ulid::Ulid;
use crate::utils::ID;
use crate::{Bus, Event, EventBuilder, Order, PublicData, topic};
use crate::Instrument;
use crate::logger::init_logger;
use crate::private::Account;


pub struct RuntimeBuilder {
    id: Option<Ulid>,
    source_address: Option<String>,
    accounts: DashMap<String, Account>,
}

impl RuntimeBuilder {
    pub fn new() -> Self{
        init_logger();
        Self{
            id: None,
            source_address: None,
            accounts: DashMap::new(),
        }
    }

    pub fn with_id(mut self, id: Ulid) -> Self{
        self.id = Some(id);
        self
    }

    pub fn with_account(mut self, account: Account) -> Self{
        self.accounts.insert(account.name().to_string(), account);
        self
    }

    pub fn with_source_address(mut self, source_address: &str) -> Self{
        self.source_address = Some(source_address.to_string());
        self
    }

    pub fn build(mut self) -> Runtime {
        if self.id.is_none(){
            self.id = Some(Ulid::new());
        }
        if self.source_address.is_none(){
            self.source_address = Some(format!("ipc://runtime/{}", self.id.unwrap().to_string()));
        }
        let inner = RuntimeInner{
            id: self.id.unwrap(),
            bus: Bus::new(),
            accounts: self.accounts,
            open_orders: DashMap::new(),
            instruments: DashMap::new(),
            prices: DashMap::new(),
            source_address: self.source_address.unwrap(),
            quit: Arc::new(AtomicBool::new(false)),
        };
        Runtime(Arc::new(inner))
    }
}

pub struct RuntimeInner {
    id: Ulid,
    bus: Bus,
    accounts: DashMap<String, Account>,
    open_orders: DashMap<Ulid, Order>,
    instruments: DashMap<String, Instrument>,
    prices: DashMap<String, f64>,
    source_address: String,
    quit: Arc<AtomicBool>,
}

#[derive(Clone)]
pub struct Runtime(Arc<RuntimeInner>);

unsafe impl Sync for Runtime {}

unsafe impl Send for Runtime {}

impl Display for Runtime {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Runtime(id={})", self.id())
    }
}

impl Debug for Runtime {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Runtime(id={})", self.id())
    }
}

impl Hash for Runtime {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.id.hash(state)
    }
}

impl Runtime {
    // 启动运行时
    pub fn run(&self) {
        for sig in TERM_SIGNALS {
            // When terminated by a second term signal, exit with exit code 1.
            // This will do nothing the first time (because term_now is false).
            signal_hook::flag::register_conditional_shutdown(*sig, 1, Arc::clone(&self.0.quit)).expect("无法配置信号钩子");
            // But this will "arm" the above for the second time, by setting it to true.
            // The order of registering these is important, if you put this one first, it will
            // first arm and then terminate ‒ all in the first round.
            signal_hook::flag::register(*sig, Arc::clone(&self.0.quit)).expect("无法配置信号钩子");
        }

        let socket = nng::Socket::new(nng::Protocol::Pull0).expect("无法创建Pull服务端");
        socket.listen(&self.0.source_address).expect("无法绑定到指定地址");

        let socket_clone = socket.clone();
        let runtime = self.clone();
        let background_thread = thread::spawn(move ||{
            runtime.run_background(socket_clone)
        });

        while !self.0.quit.load(Ordering::Relaxed){
            thread::sleep(Duration::from_millis(200));
        }
        socket.close();
        background_thread.join().expect("无法正常结束后台线程")
    }

    fn run_background(&self, socket: Socket) {
        println!("Lightning Data Source Listen on {}", self.0.source_address);
        while !self.0.quit.load(Ordering::Relaxed) {
            match socket.recv() {
                Ok(data) => {
                    self.on_external_event(&data)
                }
                Err(err) => {
                    match err {
                        Error::Closed => {
                            warn!("关闭监听通道, 即将退出");
                            break
                        }
                        _ => {
                            warn!("无法解析通道消息, {}", err.to_string());
                            break
                        }
                    }
                }
            }

        }
        println!("停止监听");
    }

    // 停止运行时
    pub fn quit(&self) {
        self.0.quit.store(true, Ordering::Relaxed)
    }
}

impl Runtime {
    /// 获取运行时ID
    pub fn id(&self) -> Ulid{
        self.0.id
    }
    pub fn bus(&self) -> &Bus{
        &self.0.bus
    }

}

impl Runtime {
    fn on_external_event(&self, data: &[u8]) {

        println!("发布事件");
        match serde_json::from_slice::<PublicData>(&data) {
            Ok(data) => {
                // info!("获取到新数据");
                match data {
                    PublicData::Bars(bars) => {
                        for bar in bars {
                            self.0.prices.insert(bar.code(), bar.close);
                            let event = EventBuilder::new(bar).with_topic(topic::BAR).build();
                            self.0.bus.publish(&event);
                        }
                    }
                    PublicData::Instruments(instruments) => {
                        for instrument in instruments {
                            self.0.instruments.insert(instrument.code(), instrument.clone());
                            let event = EventBuilder::new(instrument).with_topic(topic::UNIVERSE_CHANGED).build();
                            self.0.bus.publish(&event);
                        }
                    }
                    PublicData::Transactions(transactions) => {
                        for transaction in transactions {
                            self.0.prices.insert(transaction.code(), transaction.price);
                            let event = EventBuilder::new(transaction).with_topic(topic::TRANSACTION).build();
                            self.0.bus.publish(&event);
                            self.0.instruments.insert(instrument.code(), instrument.clone());
                            let event = EventBuilder::new(instrument).with_topic(topic::UNIVERSE_CHANGED).build();
                            self.0.bus.publish(&event);
                        }
                    }
                    PublicData::String(s) => {
                        info!("接收到字符串: {}", s)
                    }
                    PublicData::Binary(_s) => {}
                }
            }
            Err(err) => {
                warn!("{}: {}", err.to_string(), String::from_utf8(data.to_vec()).unwrap())
            }
        }
    }
}

impl Runtime {

    /// 获取标的信息
    pub fn get_account(&self, name: &str) -> Option<Account> {
        self.0.accounts.get(name).map(|v| v.value().clone())
    }

    /// 获取标的信息
    pub fn get_instrument(&self, code: &str) -> Option<Instrument> {
        self.0.instruments.get(code).map(|v| v.value().clone())
    }
    /// 获取标的信息
    pub fn get_price(&self, code: &str) -> Option<f64> {
        self.0.prices.get(code).map(|v| v.value().clone())
    }
}

impl Runtime{

    pub fn submit_order(
        &self,
        account_name: &str,
        code: &str,
        direction: u8,
        action: u8,
        price: f64,
        amount: f64,
    ) {
        // self.bus().publish(EventBuilder::new())
    }

    pub fn cancel_order(&self, order_id: Ulid) {

    }

    pub fn get_open_orders(&self, account_name: &str) {

    }
}

impl Drop for Runtime {
    fn drop(&mut self) {
        info!("销毁运行时")
    }
}
