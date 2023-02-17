use std::ffi::{c_char, CString};
use lightning_core::Instrument;
use crate::utils::convert_raw;

// use std::ffi::CString;
// use libc::c_char;
// use lightning_rs::models::Instrument;
// use crate::utils::{convert_raw, copy_c_str};
//
// /// 创建新标的对象
// #[no_mangle]
// pub extern "C" fn Instrument_new(
//     symbol: *const c_char,
//     exchange: *const c_char,
//     underlying: *const c_char,
//     reversed: bool,
//     lot_size: *const c_char,
//     tick_size: *const c_char,
//     raw: *const c_char,
// ) -> Box<Instrument> {
//     let symbol_rust = copy_c_str(symbol).expect("无法转换Symbol");
//     let exchange_rust = copy_c_str(exchange).expect("无法转换Symbol");
//     let underlying_rust = copy_c_str(underlying).expect("无法转换Abbr");
//     let lot_size_rust = copy_c_str(lot_size).expect("无法转换LotSize");
//     let tick_size_rust = copy_c_str(tick_size).expect("无法转换TickSize");
//     let raw_rust = copy_c_str(raw).expect("无法转换Raw");
//     Box::new(Instrument::new(
//         symbol_rust,
//         exchange_rust,
//         underlying_rust,
//         reversed,
//         lot_size_rust,
//         tick_size_rust,
//         raw_rust
//     ))
// }
//
/// 获取标的符号
#[no_mangle]
pub extern "C" fn Instrument__symbol(obj: *const Instrument) -> *mut c_char {
    let instrument = convert_raw(obj);
    CString::new(instrument.symbol.clone()).unwrap().into_raw()
}

/// 获取标的所在交易所
#[no_mangle]
pub extern "C" fn Instrument__exchange(obj: *const Instrument) -> *mut c_char {
    let instrument = convert_raw(obj);
    CString::new(instrument.exchange.clone()).unwrap().into_raw()
}

/// 获取标的锚定标的代码(code)
#[no_mangle]
pub extern "C" fn Instrument__underlying(obj: *const Instrument) -> *mut c_char {
    let instrument = convert_raw(obj);
    CString::new(instrument.underlying.clone()).unwrap().into_raw()
}
//
// /// 获取标的其他信息
// #[no_mangle]
// pub extern "C" fn Instrument_get_extra(obj: *const Instrument) -> *mut c_char {
//     let instrument = convert_raw(obj);
//     CString::new(instrument.extra.clone()).unwrap().into_raw()
// }
//
/// 标的是否为反转合约
#[no_mangle]
pub extern "C" fn Instrument__is_reversed(obj: *const Instrument) -> bool {
    let instrument = convert_raw(obj);
    instrument.is_reversed
}

/// 获取标的每手大小
#[no_mangle]
pub extern "C" fn Instrument__lot_size(obj: *const Instrument) -> *mut c_char {
    let instrument = convert_raw(obj);
    CString::new(instrument.lot_size.to_string()).unwrap().into_raw()
}

/// 获取标的每跳大小
#[no_mangle]
pub extern "C" fn Instrument__tick_size(obj: *const Instrument) -> *mut c_char {
    let instrument = convert_raw(obj);
    CString::new(instrument.tick_size.to_string()).unwrap().into_raw()
}


/// 裁剪价格
#[no_mangle]
pub extern "C" fn Instrument_trim_price(obj: *const Instrument, v: f64) -> *mut c_char {
    let instrument = convert_raw(obj);
    CString::new(instrument.trim_price(v).to_string()).unwrap().into_raw()
}


/// 裁剪数量
#[no_mangle]
pub extern "C" fn Instrument_trim_amount(obj: *const Instrument, v: f64) -> *mut c_char {
    let instrument = convert_raw(obj);
    println!("获取到数据: {}", v);
    CString::new(instrument.trim_amount(v).to_string()).unwrap().into_raw()
}

/// 获取标的的唯一代码
#[no_mangle]
pub extern "C" fn Instrument__code(obj: *const Instrument) -> *mut c_char {
    let instrument = convert_raw(obj);
    CString::new(instrument.code()).unwrap().into_raw()
}

/// 销毁标的实例
#[no_mangle]
pub extern "C" fn Instrument__drop(obj: Option<Box<Instrument>>) {
    if let Some(instance) = obj{
        drop(instance);
    } else {
        panic!("第一个参数必须为合法的实例")
    }
}
