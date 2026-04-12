use libc::c_char;
use std::ffi::CString;


pub mod command;
pub struct Bot;

impl Bot {
    pub fn get_version() -> String {
        unsafe {
            let ptr = luo9_version();
            if !ptr.is_null() {
                let c_string = CString::from_raw(ptr as *mut i8);
                c_string.into_string().unwrap_or_default()
            } else {
                String::new()
            }
        }
    }

    pub fn send_private_msg(user_id: u64, msg: CString) -> i64 {
        unsafe {
            luo9_send_private_msg(user_id, msg.as_ptr())
        }
    }
    pub fn send_group_msg(group_id: u64, msg: CString) -> i64 {
        unsafe {
            luo9_send_group_msg(group_id, msg.as_ptr())
        }
    }
}

unsafe extern "C" {
    /// 获取核心版本信息
    pub fn luo9_version() -> *const c_char;

    /// 发送群消息
    pub fn luo9_send_group_msg(group_id: u64, msg: *const c_char) -> i64;

    /// 发送私聊消息
    pub fn luo9_send_private_msg(user_id: u64, msg: *const c_char) -> i64;
}