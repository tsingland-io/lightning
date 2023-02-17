use std::ffi::{c_char, CString};
use lightning_core::Position;
use crate::utils::convert_raw;

#[no_mangle]
pub extern "C" fn Position__symbol(obj: *mut Position) -> *const c_char{
    let position = convert_raw(obj);
    let symbol = position.symbol().to_string();
    let s = CString::new(symbol).expect("解析账户名称失败");
    s.into_raw()
}

#[no_mangle]
pub extern "C" fn Position__exchange(obj: *mut Position) -> *const c_char{
    let position = convert_raw(obj);
    let exchange = position.exchange().to_string();
    let s = CString::new(exchange).expect("解析账户名称失败");
    s.into_raw()
}

#[no_mangle]
pub extern "C" fn Position__code(obj: *mut Position) -> *const c_char{
    let position = convert_raw(obj);
    let s = CString::new(position.code()).expect("解析账户名称失败");
    s.into_raw()
}

#[no_mangle]
pub extern "C" fn Position__total_amount(obj: *mut Position) -> f64{
    let position = convert_raw(obj);
    position.total_amount()
}

#[no_mangle]
pub extern "C" fn Position__available_amount(obj: *mut Position) -> f64{
    let position = convert_raw(obj);
    position.available_amount()
}

#[no_mangle]
pub extern "C" fn Position__frozen_amount(obj: *mut Position) -> f64{
    let position = convert_raw(obj);
    position.frozen_amount()
}

#[no_mangle]
pub extern "C" fn Position_average_price(obj: *mut Position) -> f64{
    let position = convert_raw(obj);
    position.average_price()
}

/// 销毁标的实例
#[no_mangle]
pub extern "C" fn Position__drop(obj: Option<Box<Position>>) {
    if let Some(instance) = obj{
        drop(instance);
    } else {
        panic!("第一个参数必须为合法的实例")
    }
}