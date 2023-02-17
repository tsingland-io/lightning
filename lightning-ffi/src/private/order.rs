use std::ffi::{c_char, CString};
use lightning_core::Order;
use crate::utils::convert_raw;

#[no_mangle]
pub extern "C" fn Order__symbol(obj: *mut Order) -> *const c_char{
    let order = convert_raw(obj);
    let symbol = order.symbol().to_string();
    let s = CString::new(symbol).expect("解析账户名称失败");
    s.into_raw()
}

#[no_mangle]
pub extern "C" fn Order__exchange(obj: *mut Order) -> *const c_char{
    let order = convert_raw(obj);
    let exchange = order.exchange().to_string();
    let s = CString::new(exchange).expect("解析账户名称失败");
    s.into_raw()
}

#[no_mangle]
pub extern "C" fn Order__code(obj: *mut Order) -> *const c_char{
    let order = convert_raw(obj);
    let s = CString::new(order.code()).expect("解析账户名称失败");
    s.into_raw()
}

#[no_mangle]
pub extern "C" fn Order__amount(obj: *mut Order) -> f64 {
    let order = convert_raw(obj);
    order.amount()
}

#[no_mangle]
pub extern "C" fn Order_filled_amount(obj: *mut Order) -> f64 {
    let order = convert_raw(obj);
    order.filled_amount()
}

#[no_mangle]
pub extern "C" fn Order__price(obj: *mut Order) -> f64 {
    let order = convert_raw(obj);
    order.price()
}

#[no_mangle]
pub extern "C" fn Order__average_price(obj: *mut Order) -> f64 {
    let order = convert_raw(obj);
    order.average_price()
}

#[no_mangle]
pub extern "C" fn Order__status(obj: *mut Order) -> u8 {
    let order = convert_raw(obj);
    order.status()
}

/// 销毁标的实例
#[no_mangle]
pub extern "C" fn Order__drop(obj: Option<Box<Order>>) {
    if let Some(instance) = obj{
        drop(instance);
    } else {
        panic!("第一个参数必须为合法的实例")
    }
}
