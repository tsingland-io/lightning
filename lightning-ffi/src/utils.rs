use std::ffi::{CStr, CString, IntoStringError};
use std::ptr;
use std::str::Utf8Error;
use libc::c_char;

/// 从指针转换到对应实例的引用
pub fn convert_raw<T>(obj: *const T) -> &'static T{
    unsafe {
        assert!(!obj.is_null());
        &*obj
    }
}

/// 从指针转换到对应实例的可变引用
pub fn convert_raw_mut<T>(obj: *mut T) -> &'static mut T{
    unsafe {
        assert!(!obj.is_null());
        &mut *obj
    }
}

/// 从C字符串拷贝到Rust字符串，按值拷贝
pub fn copy_c_str(msg: *const c_char) -> Result<String, IntoStringError>{
    // 获取`C`字符串的内容长度
    let msg_len = unsafe {libc::strlen(msg)};
    // 计算`C`字符串总长度（内容长度 + `NUL`位）
    let msg_len_with_zero = msg_len + 1;
    // 预划出一段连续的字节数组缓冲区
    let mut msg_data = Vec::with_capacity(msg_len_with_zero);
    unsafe { // 内存复制和填充`C`字符串的内容段。
        ptr::copy_nonoverlapping(msg as *mut u8, msg_data.as_mut_ptr(), msg_len);
        // 需要手动设置一下容器长度
        msg_data.set_len(msg_len);
    }; // 设置字节数组的长度和给最后一位`NUL`初始化`\0`。
    msg_data.resize(msg_len_with_zero, 0);
    // 字节数组 -> `rust - C`字符串
    let msg_cstr: CString = CString::from_vec_with_nul(msg_data.into()).unwrap();
    // println!("{:?}", msg_cstr);
    // `rust - C`字符串 -> `rust`字符串
    msg_cstr.into_string()
}

/// 从C字符串引用到Rust字符串
pub fn ref_c_str(msg: *const c_char) -> Result<&'static str, Utf8Error>{
    unsafe {
        CStr::from_ptr(msg).to_str()
    }
}

/// 释放掉由Rust创建的结构体指针
pub fn drop_row_pointer<T>(obj: *mut T) {
    unsafe {
        let _ = Box::from_raw(obj);
    }
}


#[repr(C)]
pub struct Array<T>{
    pub data: *mut T,
    pub length: usize,
}

impl<T> From<Vec<T>> for Array<T> {
    fn from(v: Vec<T>) -> Self {
        let length = v.len();
        let mut buf = v.into_boxed_slice();
        let data = buf.as_mut_ptr();
        std::mem::forget(buf);
        Self{
            data,
            length,
        }
    }
}

impl<T> Drop for Array<T> {
    fn drop(&mut self) {
        println!("销毁Array");
    }
}


// #[repr(C)]
// pub struct Object {
//     say_hello: unsafe fn(*mut c_void),
//     drop: unsafe fn(*mut c_void),
//     instance: String,
// }

// unsafe fn say_hello(this: *mut c_void) {
//     let obj = &mut *(this as *mut Object);
//     let data = &obj.instance;
//     println!("{}", data)
// }

// unsafe fn drop_object(this: *mut c_void) {
//     let obj = this as *mut Object;
//     let mut obj_owner = Box::from_raw(obj);
//     let instance = Box::from_raw(&mut obj_owner.instance as *mut String);
// }

// #[no_mangle]
// pub unsafe extern "C" fn get_object() -> *mut Object {
//     let instance = "测试数据".to_string();
//     let obj = Box::new(Object{
//         instance,
//         say_hello,
//         drop: drop_object,
//     });
//     Box::into_raw(obj)
// }
