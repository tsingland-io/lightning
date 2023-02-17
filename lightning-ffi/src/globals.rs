use std::ffi::CString;
use libc::c_char;


/// 释放从Rust端创建的字符串
#[no_mangle]
pub extern "C" fn rust_str_free(s: *mut c_char) {
    unsafe {
        if s.is_null() {
            return;
        }
        let sc = CString::from_raw(s);
        drop(sc);
        // println!("释放s: {:?}", sc)
    };
}
