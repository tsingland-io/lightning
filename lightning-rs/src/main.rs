use log::{info, LevelFilter};
use std::io::Write;
use lightning_core::{Bar, Instrument, RuntimeBuilder};
use lightning_core::Event;
use lightning_core::Runtime;
use lightning_core::topic;
use lightning_core::Account;

#[derive(Clone)]
pub struct Strategy {
    rtm: Runtime,
}

impl Strategy {
    pub fn new(runtime: Runtime) -> Self{
        Self{
            rtm: runtime
        }
    }

    pub unsafe fn on_bar(&self, bar: &Bar) {
        info!("策略获取到[{}]Bar数据: close={}", bar.code(), bar.close);
        info!("当前标的信息: {:?}", self.rtm.get_instrument(&bar.code()).unwrap());
        info!("账户可用资金: {:?}", self.rtm.get_account("默认").unwrap().available_cash());
        self.rtm.submit_order("默认", &bar.code(), 1, 1, bar.close, 1.0)
    }
    
    pub unsafe fn on_bar_event(&self, event: &Event) {
        let data = event.downcast_ref::<Bar>();
        self.on_bar(data);
    }

    pub unsafe fn on_instrument_update(&self, event: &Event) {
        let data = event.downcast_ref::<Instrument>();
        info!("标的更新: {}", data.code());
    }
}

impl Drop for Strategy {
    fn drop(&mut self) {
        info!("销毁策略")
    }
}

fn init_logger() {
    env_logger::builder()
        .format(|buf, record| {
            writeln!(
                buf,
                "{}:{} {} [{}] - {}",
                record.file().unwrap_or("unknown"),
                record.line().unwrap_or(0),
                chrono::Local::now().format("%Y-%m-%dT%H:%M:%S"),
                record.level(),
                record.args()
            )
        })
        .filter_level(LevelFilter::Debug).init();
}

fn main() {
    run()
}

fn run() {
    // init_logger();
    // 先构建运行时
    let rtm = RuntimeBuilder::new()
        .with_source_address("tcp://10.0.1.3:9988")
        .with_account(Account::new("默认", 100.0))
        .build();
    // 再构建事件总线
    let rtm_ref = rtm.clone();
    let strategy = Strategy{ rtm: rtm_ref };
    {
        let strategy = strategy.clone();
        rtm.bus().subscribe(topic::BAR, move |e| unsafe {
            strategy.on_bar_event(e);
        }, None);
    }
    {
        let strategy = strategy.clone();
        rtm.bus().subscribe(topic::UNIVERSE_CHANGED, move |e| unsafe {
            strategy.on_instrument_update(e);
        }, None);
    }
    rtm.bus().subscribe(2, |event| {info!("ID = 2, event = {}", event.id())}, None);
    rtm.run();
}
