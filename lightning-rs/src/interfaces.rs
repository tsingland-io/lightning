use std::any::Any;
use im::HashMap;
use rust_decimal::Decimal;
use ulid::Ulid;

fn format_code(exchange: &str, symbol: &str) -> String {
    format!("{}:{}", exchange, symbol)
}

pub trait AsEvent {
    fn id(&self) -> Ulid;
    fn event_type(&self) -> u64;
    fn is_cancelled(&self) -> bool;
    fn cancel(&self);
}

pub trait AsBus {
    /// 发布事件, 使用调用栈即时入栈执行
    fn publish(&self, event: Box<dyn AsEvent + Sync + Send>);
    /// 订阅事件
    fn subscribe(&self, event_type: u64, callback: i64, priority: i64);
}

/// 抽象出运行时接口
pub trait AsRuntime {
    /// 运行时ID
    fn id(&self) -> Ulid;
    /// 获取运行时持有的账户
    fn get_account(&self, name: &str) -> Box<dyn AsAccount + Send + Sync>;
    /// 获取指定标的信息
    fn get_instrument(&self, code: &str) -> Box<dyn AsInstrument + Send + Sync>;
    /// 获取标的最新价格
    fn get_price(&self, code: &str) -> Option<f64>;

    /// 发布外部事件, 执行外部推送, 这里也可以使用nng实现
    fn publish_external(&self);

    /// 发布事件, 使用调用栈即时入栈执行
    fn publish(&self, event: Box<dyn AsEvent + Sync + Send>);
    /// 订阅事件
    fn subscribe(&self, event_type: u64, subscription: i64);
}

pub trait AsContext {

}

pub trait AsAccount {
    fn id(&self) -> Ulid;
    fn account_type(&self) -> u64;
    fn name(&self) -> &str;
    fn positions(&self) -> Vec<Box<dyn AsPosition + Send + Sync>>;
    fn get_position(&self, code: &str, direction: u8) -> Box<dyn AsPosition + Send + Sync>;
    fn available_cash(&self) -> f64;
    fn frozen_cash(&self) -> f64;
    fn cash_type(&self) -> &str;
    fn submit_order(&self);
    fn cancel_order(&self);
    fn open_orders(&self);
    fn on_trade(&mut self);

}

pub trait AsPosition {
    fn symbol(&self) -> &str;
    fn exchange(&self) -> &str;
    fn code(&self) -> String {
        format_code(self.exchange(), self.symbol())
    }
    fn position_type(&self) -> u64;
    fn direction(&self) -> u8;
    fn total_amount(&self) -> f64;
    fn available_amount(&self) -> f64;
    fn frozen_amount(&self) -> f64;
    fn average_price(&self) -> f64;
    fn on_trade(&mut self) -> f64;
}

pub trait AsInstrument {
    fn symbol(&self) -> &str;
    fn exchange(&self) -> &str;
    fn code(&self) -> String {
        format_code(self.exchange(), self.symbol())
    }
    fn tick_size(&self) -> Decimal;
    fn lot_size(&self) -> Decimal;
}

pub trait AsOrder {
    fn id(&self) -> Ulid;
    fn account_id(&self) -> Ulid;
    fn symbol(&self) -> &str;
    fn exchange(&self) -> &str;
    fn code(&self) -> String {
        format_code(self.exchange(), self.symbol())
    }
    /// 挂单价格
    fn price(&self) -> f64;
    /// 成交均价
    fn average_price(&self) -> f64;
    /// 订单状态
    fn status(&self) -> u8;
    /// 下单数量
    fn amount(&self) -> f64;
    /// 已成交数量
    fn filled_amount(&self) -> f64;

    /// 成交单处理回调
    fn on_trade(&mut self) -> f64;

    /// 成交单创建时间
    fn updated_at(&self) -> i64;
    /// 成交单创建时间
    fn created_at(&self) -> i64;
}

pub trait AsTrade {
    fn id(&self) -> Ulid;
    fn account_id(&self) -> Ulid;
    fn order_id(&self) -> Ulid;
    fn symbol(&self) -> &str;
    fn exchange(&self) -> &str;
    fn code(&self) -> String {
        format_code(self.exchange(), self.symbol())
    }
    /// 成交价格
    fn price(&self) -> f64;
    /// 成交数量
    fn amount(&self) -> f64;
    /// 交易手续费
    fn commission(&self) -> f64;
    /// 成交单创建时间
    fn created_at(&self) -> i64;
}

#[cfg(test)]
mod interface_tests{
    use rust_decimal::Decimal;
    use ulid::Ulid;
    use crate::models::interfaces::{AsAccount, AsBus, AsEvent, AsInstrument, AsOrder, AsPosition, AsRuntime, AsTrade};

    struct EventBus {}

    impl AsBus for EventBus {
        fn publish(&self, event: Box<dyn AsEvent + Sync + Send>) {
        }

        fn subscribe(&self, event_type: u64, callback: i64, priority: i64) {
            todo!()
        }
    }

    struct Account{}

    impl AsAccount for Account {
        fn id(&self) -> Ulid {
            todo!()
        }

        fn account_type(&self) -> u64 {
            todo!()
        }

        fn name(&self) -> &str {
            todo!()
        }

        fn positions(&self) -> Vec<Box<dyn AsPosition + Send + Sync>> {
            todo!()
        }

        fn get_position(&self, code: &str, direction: u8) -> Box<dyn AsPosition + Send + Sync> {
            todo!()
        }

        fn available_cash(&self) -> f64 {
            todo!()
        }

        fn frozen_cash(&self) -> f64 {
            todo!()
        }

        fn cash_type(&self) -> &str {
            todo!()
        }

        fn submit_order(&self) {
            todo!()
        }

        fn cancel_order(&self) {
            todo!()
        }

        fn open_orders(&self) {
            todo!()
        }

        fn on_trade(&mut self) {
            todo!()
        }
    }

    struct Position{}

    impl AsPosition for Position {
        fn symbol(&self) -> &str {
            todo!()
        }

        fn exchange(&self) -> &str {
            todo!()
        }

        fn position_type(&self) -> u64 {
            todo!()
        }

        fn direction(&self) -> u8 {
            todo!()
        }

        fn total_amount(&self) -> f64 {
            todo!()
        }

        fn available_amount(&self) -> f64 {
            todo!()
        }

        fn frozen_amount(&self) -> f64 {
            todo!()
        }

        fn average_price(&self) -> f64 {
            todo!()
        }

        fn on_trade(&mut self) -> f64 {
            todo!()
        }
    }

    struct Instrument{}

    impl AsInstrument for Instrument {
        fn symbol(&self) -> &str {
            todo!()
        }

        fn exchange(&self) -> &str {
            todo!()
        }

        fn tick_size(&self) -> Decimal {
            todo!()
        }

        fn lot_size(&self) -> Decimal {
            todo!()
        }
    }

    struct Trade{}

    impl AsTrade for Trade {
        fn id(&self) -> Ulid {
            todo!()
        }

        fn account_id(&self) -> Ulid {
            todo!()
        }

        fn order_id(&self) -> Ulid {
            todo!()
        }

        fn symbol(&self) -> &str {
            todo!()
        }

        fn exchange(&self) -> &str {
            todo!()
        }

        fn price(&self) -> f64 {
            todo!()
        }

        fn amount(&self) -> f64 {
            todo!()
        }

        fn commission(&self) -> f64 {
            todo!()
        }

        fn created_at(&self) -> i64 {
            todo!()
        }
    }

    struct Order {}

    impl AsOrder for Order {
        fn id(&self) -> Ulid {
            todo!()
        }

        fn account_id(&self) -> Ulid {
            todo!()
        }

        fn symbol(&self) -> &str {
            todo!()
        }

        fn exchange(&self) -> &str {
            todo!()
        }

        fn price(&self) -> f64 {
            todo!()
        }

        fn average_price(&self) -> f64 {
            todo!()
        }

        fn status(&self) -> u8 {
            todo!()
        }

        fn amount(&self) -> f64 {
            todo!()
        }

        fn filled_amount(&self) -> f64 {
            todo!()
        }

        fn on_trade(&mut self) -> f64 {
            todo!()
        }

        fn updated_at(&self) -> i64 {
            todo!()
        }

        fn created_at(&self) -> i64 {
            todo!()
        }
    }

    struct Runtime {}

    impl Runtime {
        fn run(&self, bus: Box<dyn AsBus>){
            // 1. 从外部事件源获取外部事件
            // for event in self.nng{
            //      // 可以通过调整优先级订阅event的方式来实现事件的pre和post处理
            //      bus.publish(event)
            // }
            // self.nng.stop()
            // self.eventbus.clear()
        }
    }

    impl AsRuntime for Runtime {
        fn id(&self) -> Ulid {
            todo!()
        }

        fn get_account(&self, name: &str) -> Box<dyn AsAccount + Send + Sync> {
            todo!()
        }

        fn get_instrument(&self, code: &str) -> Box<dyn AsInstrument + Send + Sync> {
            todo!()
        }

        fn get_price(&self, code: &str) -> Option<f64> {
            todo!()
        }

        fn publish_external(&self) {
            todo!()
        }

        fn publish(&self, event: Box<dyn AsEvent + Sync + Send>) {
            todo!()
        }

        fn subscribe(&self, event_type: u64, subscription: i64) {
            todo!()
        }
    }

    trait Plugin{
        fn name(&self) -> &str;
        fn version(&self) -> &str;
    }

    trait PluginBuilder{
        fn build(self, runtime: Box<dyn AsRuntime + Sync + Send>) -> Box<dyn Plugin>;
    }

    struct PluginManager{}

    impl PluginManager {
        fn load(&mut self, plugin: i64){

        }

        fn install(&mut self) {

        }

        fn uninstall(&mut self) {

        }
    }

    struct Config {}
    #[test]
    fn logic() {
        // 1. 初始化配置文件
        let mut config = Config{};
        // 2. 初始化运行时
        let mut rtm = Runtime{};
        // 3. 初始化插件管理器
        let mut pm = PluginManager{};
        // 4. 初始化插件
        // plugin_builder.build(rtm);
        // pm.load(111);
        // 5. 运行时开始监听外部时间并启动执行逻辑
        // 运行结束后,清理所有订阅回调
        rtm.run();
        // 6. 卸载所有插件
        drop(pm);
        // 7. 卸载运行时
        drop(rtm);

    }
}