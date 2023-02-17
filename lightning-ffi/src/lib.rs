
mod utils;
mod globals;
mod private;
mod public;
mod common;

/* 范式

/// 获取标的符号, 使用该方法将会取得obj的所有权，并在调用后销毁obj，
/// 若不需要进行销毁，则可以使用Box::into_raw再次释放obj
#[no_mangle]
pub extern "C" fn Instrument_get_symbol(obj: Option<Box<Instrument>>) -> *mut c_char {
    if let Some(instrument) = obj{
        let result = CString::new(instrument.symbol.clone()).unwrap().into_raw();
        Box::into_raw(instrument);
        result
    } else {
        panic!("第一个参数必须为合法的实例")
    }
}

/// 获取标的符号, 使用该方法将会取得obj的引用，不会主动销毁obj，
/// 若需要进行销毁，则可以使用Box::from_raw进行销毁
#[no_mangle]
pub extern "C" fn Instrument_get_symbol(obj: *mut Instrument) -> *mut c_char {
    if let Some(instrument) = obj{
        let result = CString::new(instrument.symbol.clone()).unwrap().into_raw();
        Box::into_raw(instrument);
        result
    } else {
        panic!("第一个参数必须为合法的实例")
    }

    unsafe {
        Box::from_raw(ptr);
    }
}


*/