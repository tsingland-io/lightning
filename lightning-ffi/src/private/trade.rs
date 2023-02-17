use std::ffi::{c_char, CString};
use lightning_core::Trade;
use crate::utils::convert_raw;

#[no_mangle]
pub extern "C" fn Trade__id(obj: *mut Trade) -> *const c_char{
    let trade = convert_raw(obj);
    let id = trade.id().to_string();
    let s = CString::new(id).expect("解析成交单ID失败");
    s.into_raw()
}


#[no_mangle]
pub extern "C" fn Trade__external_id(obj: *mut Trade) -> *const c_char{
    let trade = convert_raw(obj);
    let external_id = trade.external_id().to_string();
    let s = CString::new(external_id).expect("解析账户名称失败");
    s.into_raw()
}

#[no_mangle]
pub extern "C" fn Trade__exchange(obj: *mut Trade) -> *const c_char{
    let trade = convert_raw(obj);
    let exchange = trade.exchange().to_string();
    let s = CString::new(exchange).expect("解析账户名称失败");
    s.into_raw()
}

#[no_mangle]
pub extern "C" fn Trade__code(obj: *mut Trade) -> *const c_char{
    let trade = convert_raw(obj);
    let s = CString::new(trade.code()).expect("解析账户名称失败");
    s.into_raw()
}

#[no_mangle]
pub extern "C" fn Trade__amount(obj: *mut Trade) -> f64 {
    let trade = convert_raw(obj);
    trade.amount()
}

#[no_mangle]
pub extern "C" fn Trade__price(obj: *mut Trade) -> f64 {
    let trade = convert_raw(obj);
    trade.price()
}

/// 销毁标的实例
#[no_mangle]
pub extern "C" fn Trade__drop(obj: Option<Box<Trade>>) {
    if let Some(instance) = obj{
        drop(instance);
    } else {
        panic!("第一个参数必须为合法的实例")
    }
}
