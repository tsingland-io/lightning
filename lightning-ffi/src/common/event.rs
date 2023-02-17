use std::ffi::{c_char, c_void, CString};
use lightning_core::Event;
use lightning_core::topic::Topic;
use crate::utils::convert_raw;

#[no_mangle]
pub extern "C" fn Event__id(obj: *mut Event) -> *const c_char {
    let event = convert_raw(obj);
    let id = CString::new(event.id().to_string()).expect("解析事件ID失败");
    id.into_raw()
}
#[no_mangle]
pub extern "C" fn Event__timestamp(obj: *mut Event) -> u64 {
    let event = convert_raw(obj);
    event.timestamp()
}

#[no_mangle]
pub extern "C" fn Event__topic(obj: *mut Event) -> Topic {
    let event = convert_raw(obj);
    event.timestamp()
}

#[no_mangle]
pub extern "C" fn Event__is_cancelled(obj: *mut Event) -> bool {
    let event = convert_raw(obj);
    event.is_cancelled()
}

#[no_mangle]
pub extern "C" fn Event__cancel(obj: *mut Event) {
    let event = convert_raw(obj);
    event.cancel()
}

#[no_mangle]
pub extern "C" fn Event__data(obj: *mut Event) -> *const c_void {
    let event = convert_raw(obj);
    event.data() as *const c_void
}
