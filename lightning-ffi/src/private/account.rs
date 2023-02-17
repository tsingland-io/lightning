// use libc::c_char;
// use ulid::Ulid;
// use lightning_rs::models::Account;
// use crate::utils::copy_c_str;
//
// #[no_mangle]
// pub extern "C" fn Account_new(
//     name: *const c_char,
//     settlement_asset: *const c_char,
//
// ) -> Box<Account>{
//     let name_rust = copy_c_str(name).expect("无法转换Name");
//     let settlement_asset_rust = copy_c_str(settlement_asset).expect("无法转换SettlementAsset");
//     Box::new(Account::new(
//         Ulid::new(),
//         name_rust,
//         settlement_asset_rust
//     ))
// }

use std::ffi::CString;
use libc::c_char;
use lightning_core::{Account, Position};
use crate::utils::{Array, convert_raw, ref_c_str};

#[no_mangle]
pub extern "C" fn Account__name(obj: *mut Account) -> *const c_char{
    let account = convert_raw(obj);
    let name = account.name().to_string();
    let s = CString::new(name).expect("解析账户名称失败");
    s.into_raw()
}

#[no_mangle]
pub extern "C" fn Account__available_cash(obj: *mut Account) -> f64{
    let account = convert_raw(obj);
    account.available_cash()
}

#[no_mangle]
pub extern "C" fn Account__frozen_cash(obj: *mut Account) -> f64{
    let account = convert_raw(obj);
    account.frozen_cash()
}

#[no_mangle]
pub extern "C" fn Account__get_position(obj: *mut Account, code: *const c_char) -> Option<Box<Position>>{
    let account = convert_raw(obj);
    let code_str = ref_c_str(code).expect("标的代码不是合法的UTF8字符串");
    account.get_position(code_str).map(|pos| Box::new(pos.clone()))
}

/// 销毁标的实例
#[no_mangle]
pub extern "C" fn Account__drop(obj: Option<Box<Account>>) {
    if let Some(instance) = obj{
        drop(instance);
    } else {
        panic!("第一个参数必须为合法的实例")
    }
}

// #[no_mangle]
// pub extern "C" fn Account__get_positions(obj: *mut Account) -> Array<Box<Position>>{
//     let account = convert_raw(obj);
//     account.available_cash()
// }
