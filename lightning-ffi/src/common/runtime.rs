use std::ffi::CString;
use std::str::FromStr;
use libc::c_char;
use ulid::Ulid;
use lightning_core::{Account, Bus, Instrument, Runtime, RuntimeBuilder};
use crate::utils::{convert_raw, convert_raw_mut, copy_c_str, ref_c_str};



#[no_mangle]
pub extern "C" fn RuntimeBuilder__new() -> Box<RuntimeBuilder> {
    Box::new(RuntimeBuilder::new())
}


#[no_mangle]
pub extern "C" fn RuntimeBuilder__with_id(builder: Box<RuntimeBuilder>, id: *const c_char) -> Box<RuntimeBuilder> {
    let id_str = ref_c_str(id).expect("运行时ID不是合法的UTF8字符串");
    let id = Ulid::from_str(id_str).expect("运行时ID不是合法的ULID");
    Box::new(builder.with_id(id))
}


#[no_mangle]
pub extern "C" fn RuntimeBuilder__with_account(builder: Box<RuntimeBuilder>, account_name: *const c_char, init_cash: f64) -> Box<RuntimeBuilder> {
    let name = copy_c_str(account_name).expect("运行时监听地址不是合法的UTF8字符串");
    Box::new(builder.with_account(Account::new(&name, init_cash)))
}


#[no_mangle]
pub extern "C" fn RuntimeBuilder__with_source_address(builder: Box<RuntimeBuilder>, source_address: *const c_char) -> Box<RuntimeBuilder> {
    let source_address = copy_c_str(source_address).expect("运行时监听地址不是合法的UTF8字符串");
    Box::new(builder.with_source_address(&source_address))
}

#[no_mangle]
pub extern "C" fn RuntimeBuilder__build(builder: Box<RuntimeBuilder>) -> Box<Runtime> {
    Box::new(builder.build())
}

#[no_mangle]
pub extern "C" fn Runtime__get_account(obj: *mut Runtime, account_name: *const c_char) -> Option<Box<Account>>{
    let runtime = convert_raw_mut(obj);
    let account_name_str = ref_c_str(account_name).expect("账户ID不是合法的UTF8字符串");
    // let name = Ulid::from_str(account_name_str).expect("账户ID不是合法的ULID");
    runtime.get_account(account_name_str).map(|v| Box::new(v))
}

#[no_mangle]
pub extern "C" fn Runtime__id(obj: *mut Runtime) ->*const c_char{
    let runtime = convert_raw(obj);
    let id = runtime.id().to_string();
    let s = CString::new(id).expect("解析运行时ID失败");
    s.into_raw()
}


#[no_mangle]
pub extern "C" fn Runtime__bus(obj: *mut Runtime) ->*const Bus{
    let runtime = convert_raw(obj);
    let bus = runtime.bus();
    bus as *const Bus
}

#[no_mangle]
pub extern "C" fn Runtime__run(obj: *mut Runtime) {
    let runtime = convert_raw(obj);
    runtime.run();
}

// 创建新标的对象
// https://stackoverflow.com/questions/39224904/how-to-expose-a-rust-vect-to-ffi
// #[no_mangle]
// pub extern "C" fn Runtime_get_accounts(obj: *mut Runtime) -> Array<Account> {
    // let runtime = convert_raw_mut(obj);
    // let accounts = runtime.get_accounts().into_iter().map(|v|  v.clone()).collect::<Vec<Account>>();
    // Array::from(accounts)
// }

/// 获取标的信息
#[no_mangle]
pub extern "C" fn Runtime__get_instrument(obj: *const Runtime, code: *const c_char) -> Option<Box<Instrument>>{
    let runtime = convert_raw(obj);
    let code_str = ref_c_str(code).expect("标的代码不是合法的UTF8字符串");
    runtime.get_instrument(code_str).map(|ins| Box::new(ins.clone()))
}

#[no_mangle]
pub extern "C" fn Runtime__submit_order(
    obj: *mut Runtime,
    account_name: *const c_char,
    code: *const c_char,
    direction: u8,
    action: u8,
    price: f64,
    amount: f64,
) {
    let runtime = convert_raw_mut(obj);
    let account_name_str = copy_c_str(account_name).expect("账户ID不是合法的UTF8字符串");
    let code_str = copy_c_str(code).expect("账户ID不是合法的UTF8字符串");
    runtime.submit_order(
        &account_name_str,
        &code_str,
        direction,
        action,
        price,
        amount,
    )
}

#[no_mangle]
pub extern "C" fn Runtime__cancel_order(obj: *mut Runtime, order_id: *const c_char) {
    let runtime = convert_raw_mut(obj);
    let order_id_str = ref_c_str(order_id).expect("账户ID不是合法的UTF8字符串");
    let id = Ulid::from_str(order_id_str).expect("账户ID不是合法的ULID");
    runtime.cancel_order(id)
}

#[no_mangle]
pub extern "C" fn Runtime__get_open_orders(obj: *mut Runtime, account_name: *const c_char) {
    let runtime = convert_raw_mut(obj);
    let account_name_str = ref_c_str(account_name).expect("标的代码不是合法的UTF8字符串");
    runtime.get_open_orders(account_name_str)
}

// 创建新标的对象
// https://stackoverflow.com/questions/39224904/how-to-expose-a-rust-vect-to-ffi
// #[no_mangle]
// pub extern "C" fn Runtime_get_instruments(obj: *mut Runtime) -> Array<Instrument> {
    // let runtime = convert_raw_mut(obj);
    // let instruments = runtime.get(
    //     None,
    //     None,
    //     None,
    //     None
    // ).into_iter().map(|v|  v.clone()).collect::<Vec<Instrument>>();
    // Array::from(instruments)
// }

// #[no_mangle]
// pub extern "C" fn free_instrument_array(obj: Array<Instrument>) {
    // let s = unsafe { std::slice::from_raw_parts_mut(obj.data, obj.length) };
    // let s = s.as_mut_ptr();
    // unsafe {
    //     Box::from_raw(s);
    // }
// }

// #[no_mangle]
// pub extern "C" fn Runtime_get_instruments_v1(obj: *mut Runtime, callback: extern "cdecl" fn (*mut Instrument)) {
    // let runtime = convert_raw_mut(obj);
    // let v = runtime.get_instruments(
    //     None,
    //     None,
    //     None,
    //     None
    // ).into_iter().map(|v|  Box::new(v.clone())).collect::<Vec<Box<Instrument>>>();
    // // let length = v.len();
    // // let mut buf = v.into_boxed_slice();
    // // let data = buf.as_mut_ptr();
    // for ins in v{
    //     callback(Box::into_raw(ins));
    // }
// }
