use libc::c_char;
use std::ffi::CString;

pub struct Nbot;

impl Nbot {
    pub fn get_version() -> *const c_char {
        unsafe{
            luo9_version()
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