use std::cmp::Ordering;
use ulid::Ulid;
use crate::Event;

pub struct Subscription {
    /// 订阅实例ID
    pub id: Ulid,
    /// 订阅实例优先级
    pub priority: i64,
    /// 订阅实例回调函数
    callback: Box<dyn Fn(&Event) + 'static>,
    /// 订阅主题
    pub topic: u64,
}



impl Subscription  {
    pub fn new(topic: u64, f: impl Fn(&Event) + 'static, priority: i64) -> Self{
        let callback =  Box::new(f);
        Self{
            id: Ulid::new(),
            priority,
            callback,
            topic,
        }
    }

    pub fn on_event(&self, event: &Event) {
        (self.callback)(event)
    }
}

impl PartialEq<Self> for Subscription {
    fn eq(&self, other: &Self) -> bool {
        self.priority.eq(&other.priority)
    }
}

impl PartialOrd for Subscription {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.priority.partial_cmp(&other.priority)
    }
}

impl Ord for Subscription {
    fn cmp(&self, other: &Self) -> Ordering {
        self.priority.cmp(&other.priority)
    }
}

impl Eq for Subscription {}
