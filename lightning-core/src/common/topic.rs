
pub type Topic = u64;


// SCHEDULED_TASK_CREATE 创建调度任务
pub const SCHEDULED_TASK_CREATE: Topic = 100;
/// 系统保留事件号 0-10000;

/// 心跳事件
pub const HEART_BEAT: Topic = 1;

/// 系统初始化后触发
/// post_system_init()
pub const POST_SYSTEM_INIT: Topic = 2;

/// 策略执行完init函数后触发
/// post_user_init()
pub const POST_USER_INIT: Topic = 3;

/// 在实盘时，你可能需要在此事件后根据其他信息源对系统状态进行调整
pub const BEFORE_SYSTEM_RESTORED: Topic = 4;
pub const POST_SYSTEM_RESTORED: Topic = 5;


/// 在scheduler执行前触发
pub const PRE_SCHEDULED: Topic = 6;
/// 在scheduler执行前触发
pub const SCHEDULED: Topic = 7;
/// 在scheduler执行后触发
pub const POST_SCHEDULED: Topic = 8;

/// 执行持久化
pub const DO_PERSIST: Topic = 9;
/// 执行策略上下文重载
pub const DO_RESTORE: Topic = 10;

/// 策略暂停
pub const STRATEGY_PAUSED: Topic = 11;
/// 策略恢复
pub const STRATEGY_RESUMED: Topic = 12;

/// 在策略运行前执行
pub const BEFORE_STRATEGY_RUN: Topic = 13;
/// 在策略成功运行完成后
pub const POST_STRATEGY_RUN: Topic = 14;

/// 推送事件，支持推送到日志或外部消息工具
pub const NOTIFY: Topic = 15;


/// 策略证券池发生变化后触发
/// post_universe_changed(universe)
pub const PRE_UNIVERSE_CHANGED: Topic = 16;
/// 策略证券池发生变化后触发
/// post_universe_changed(universe)
pub const UNIVERSE_CHANGED: Topic = 17;
/// 策略证券池发生变化后触发
/// post_universe_changed(universe)
pub const POST_UNIVERSE_CHANGED: Topic = 18;

/// 执行before_trading函数前触发
/// pre_before_trading()
pub const PRE_BEFORE_TRADING: Topic = 19;
/// 该事件会触发策略的before_trading函数
/// before_trading()
pub const BEFORE_TRADING: Topic = 20;
/// 执行before_trading函数后触发
/// post_before_trading()
pub const POST_BEFORE_TRADING: Topic = 21;


/// 执行集合竞价事件前触发
pub const PRE_OPEN_AUCTION: Topic = 22;
/// 该事件会触发策略的open_auction函数
pub const OPEN_AUCTION: Topic = 23;
/// 执行集合竞价事件后触发
pub const POST_OPEN_AUCTION: Topic = 24;


/// 执行handle_bar函数前触发
pub const PRE_BAR: Topic = 25;
/// 该事件会触发策略的handle_bar函数
/// bar(bar_dict)
pub const BAR: Topic = 26;
/// 执行handle_bar函数后触发
pub const POST_BAR: Topic = 27;


/// 执行handle_tick函数前触发
pub const PRE_TICKER: Topic = 28;
/// 该事件会触发策略的handle_tick函数
/// tick(tick)
pub const TICKER: Topic = 29;
/// 执行handle_tick函数后触发
pub const POST_TICKER: Topic = 30;

/// 执行handle_tick函数前触发
pub const PRE_DEPTH: Topic = 31;
/// 该事件会触发策略的handle_tick函数
/// tick(tick)
pub const DEPTH: Topic = 32;
/// 执行handle_tick函数后触发
pub const POST_DEPTH: Topic = 33;


/// 执行handle_tick函数前触发
pub const PRE_TRANSACTION: Topic = 34;
/// 该事件会触发策略的handle_tick函数
/// tick(tick)
pub const TRANSACTION: Topic = 35;
/// 执行handle_tick函数后触发
pub const POST_TRANSACTION: Topic = 36;

/// 执行after_trading函数前触发
/// pre_after_trading()
pub const PRE_AFTER_TRADING: Topic = 37;
/// 该事件会触发策略的after_trading函数
/// after_trading()
pub const AFTER_TRADING: Topic = 38;
/// 执行after_trading函数后触发
/// post_after_trading()
pub const POST_AFTER_TRADING: Topic = 39;

/// 结算前触发该事件
/// pre_settlement()
pub const PRE_SETTLEMENT: Topic = 40;
/// 触发结算事件
/// settlement()
pub const SETTLEMENT: Topic = 41;
/// 结算后触发该事件
/// post_settlement()
pub const POST_SETTLEMENT: Topic = 42;

/// 创建订单
/// order_pending_new(account order)
pub const ORDER_PENDING_NEW: Topic = 43;
/// 创建订单成功
/// order_creation_pass(account order)
pub const ORDER_CREATION_PASS: Topic = 44;
/// 创建订单失败
/// order_creation_reject(account order)
pub const ORDER_CREATION_REJECT: Topic = 45;
/// 创建撤单
/// order_pending_cancel(account order)
pub const ORDER_PENDING_CANCEL: Topic = 46;
/// 撤销订单成功
/// order_cancellation_pass(account order)
pub const ORDER_CANCELLATION_PASS: Topic = 47;
/// 撤销订单失败
/// order_cancellation_reject(account order)
pub const ORDER_CANCELLATION_REJECT: Topic = 48;
/// 订单状态更新
/// order_unsolicited_update(account order)
pub const ORDER_UNSOLICITED_UPDATE: Topic = 49;

/// 成交事件
/// trade(account trade order)
pub const TRADE_FILLED: Topic = 50;

/// 尽快退出事件
/// immediately quit
pub const IMMEDIATELY_QUIT: Topic = 51;

/// 行情订阅
pub const SUBSCRIBE_BAR: Topic = 52;
pub const UNSUBSCRIBE_BAR: Topic = 53;
pub const SUBSCRIBE_TICKER: Topic = 54;
pub const UNSUBSCRIBE_TICKER: Topic = 55;
pub const SUBSCRIBE_DEPTH: Topic = 56;
pub const UNSUBSCRIBE_DEPTH: Topic = 57;
pub const SUBSCRIBE_TRANSACTION: Topic = 58;
pub const UNSUBSCRIBE_TRANSACTION: Topic = 59;