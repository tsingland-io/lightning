use std::ffi::c_void;
use lightning_core::{Bus, Event};
use crate::utils::convert_raw;

type CallbackFunctionType = unsafe extern "C" fn(*const c_void, *const Event);

/// 获取标的信息
#[no_mangle]
pub extern "C" fn Bus__subscribe(
    bus: *const Bus,
    topic: u64,
    env: *const c_void,
    callback: CallbackFunctionType,
    priority: i64,
) {
    let bus = convert_raw(bus);
    let env = env;
    let cb = move |event: &Event| unsafe {
        println!("外部事件回调");
        let e = event as *const Event;
        callback(env, e)
    };
    println!("订阅事件");
    bus.subscribe(topic, cb, Some(priority))
}

#[no_mangle]
pub extern "C" fn Bus__publish(
    bus: *const Bus,
    event: *const Event,
) {
    let bus = convert_raw(bus);
    let e = convert_raw(event);
    bus.publish(e)
}
