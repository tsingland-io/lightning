use dashmap::DashMap;
use crate::common::Event;
use crate::common::Subscription;

pub struct Bus {
    subscriptions: DashMap<u64, Vec<Subscription>>,
}

impl Bus {
    pub fn new() -> Self{
        Self{
            subscriptions: DashMap::new(),
        }
    }

    pub fn subscribe(&self, topic: u64, callback: impl Fn(&Event) + 'static, priority: Option<i64>) {
        println!("注册topic: {}", topic);
        let subscription = Subscription::new(topic, callback, priority.unwrap_or(100));
        if let Some(mut subscriptions) = self.subscriptions.get_mut(&topic) {
            subscriptions.value_mut().push(subscription);
        } else {
            self.subscriptions.insert(topic, vec![subscription]);
        }
    }

    pub fn publish(&self, event: &Event) {
        if let Some(subscriptions) = self.subscriptions.get(&event.topic()){
            for subscription in subscriptions.value() {
                subscription.on_event(event)
            }
        }
    }
}

