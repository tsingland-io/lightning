use std::ffi::{c_char, CString};
use lightning_core::Bar;
use crate::utils::{convert_raw, ref_c_str};

#[no_mangle]
pub extern "C" fn Bar__symbol(obj: *mut Bar) -> *const c_char{
    let bar = convert_raw(obj);
    let symbol = bar.symbol.to_string();
    let s = CString::new(symbol).expect("解析账户名称失败");
    s.into_raw()
}

#[no_mangle]
pub extern "C" fn Bar__exchange(obj: *mut Bar) -> *const c_char{
    let bar = convert_raw(obj);
    let exchange = bar.exchange.to_string();
    let s = CString::new(exchange).expect("解析账户名称失败");
    s.into_raw()
}

#[no_mangle]
pub extern "C" fn Bar__code(obj: *mut Bar) -> *const c_char{
    let bar = convert_raw(obj);
    let s = CString::new(bar.code()).expect("解析账户名称失败");
    s.into_raw()
}

#[no_mangle]
pub extern "C" fn Bar__open(obj: *mut Bar) -> f64 {
    let bar = convert_raw(obj);
    bar.open
}

#[no_mangle]
pub extern "C" fn Bar__high(obj: *mut Bar) -> f64 {
    let bar = convert_raw(obj);
    bar.high
}

#[no_mangle]
pub extern "C" fn Bar__low(obj: *mut Bar) -> f64 {
    let bar = convert_raw(obj);
    bar.open
}

#[no_mangle]
pub extern "C" fn Bar__close(obj: *mut Bar) -> f64 {
    let bar = convert_raw(obj);
    bar.open
}

#[no_mangle]
pub extern "C" fn Bar__amount(obj: *mut Bar) -> f64 {
    let bar = convert_raw(obj);
    bar.amount
}

#[no_mangle]
pub extern "C" fn Bar__volume(obj: *mut Bar) -> f64 {
    let bar = convert_raw(obj);
    bar.volume
}

#[no_mangle]
pub extern "C" fn Bar__count(obj: *mut Bar) -> u64 {
    let bar = convert_raw(obj);
    bar.count
}

#[no_mangle]
pub extern "C" fn Bar__timestamp(obj: *mut Bar) -> u64 {
    let bar = convert_raw(obj);
    bar.timestamp
}

#[no_mangle]
pub extern "C" fn Bar__extra(obj: *mut Bar, key: *const c_char) -> *mut c_char {
    let bar = convert_raw(obj);
    let extra_key = ref_c_str(key).expect("extra key不是合法的UTF8字符串");
    let value = bar.extra.get(extra_key).map(|v|v.to_string()).unwrap_or("".to_string());
    CString::new(value).unwrap().into_raw()
}

