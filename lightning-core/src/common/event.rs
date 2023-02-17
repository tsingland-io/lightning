use std::sync::atomic::{AtomicBool, Ordering};
use ulid::Ulid;
use crate::topic::Topic;
use crate::utils::{get_current_timestamp_millis, ID};

pub struct EventBuilder {
    id: Option<Ulid>,
    topic: Option<u64>,
    timestamp: Option<u64>,
    data: *mut (),
    drop: unsafe fn(*mut ()),
}

impl EventBuilder {
    pub fn new<T>(data: T) -> Self{
        let data = Box::into_raw(Box::new(data)) as *mut ();

        Self{
            id: None,
            topic: None,
            timestamp: None,
            data,
            drop: |v| unsafe {
                let _ = Box::from_raw(v);
            },
        }
    }

    pub fn id(mut self, id: Ulid) -> Self {
        self.id = Some(id);
        self
    }

    pub fn with_topic(mut self, topic: u64) -> Self {
        self.topic = Some(topic);
        self
    }

    pub fn with_timestamp(mut self, timestamp: u64) -> Self {
        self.timestamp = Some(timestamp);
        self
    }

    pub fn build(self) -> Event{
        Event{
            id: self.id.unwrap_or(Ulid::new()),
            topic: self.topic.unwrap_or(0),
            is_cancelled: AtomicBool::new(false),
            timestamp: self.timestamp.unwrap_or(get_current_timestamp_millis()),
            data: self.data,
            drop: self.drop,
        }
    }
}

pub struct Event {
    id: Ulid,
    topic: u64,
    is_cancelled: AtomicBool,
    timestamp: u64,
    data: *mut (),
    drop: unsafe fn(*mut ()),
}

unsafe impl Sync for Event {}
unsafe impl Send for Event {}

impl Event {
    pub fn id(&self) -> Ulid{
        self.id
    }

    pub fn timestamp(&self) -> u64 {
        self.timestamp
    }

    pub fn topic(&self) -> Topic {
        self.topic
    }

    pub fn is_cancelled(&self) -> bool {
        self.is_cancelled.load(Ordering::Relaxed)
    }

    pub fn cancel(&self) {
        self.is_cancelled.store(true, Ordering::Relaxed)
    }

    pub fn data(&self) -> *const (){
        self.data
    }
}

impl Event {
    pub unsafe fn downcast_ref<T: 'static>(&self) -> &T{
        &*(self.data as *const T)
    }

}

impl Drop for Event {
    fn drop(&mut self) {
        unsafe {
            (self.drop)(self.data)
        }
    }
}