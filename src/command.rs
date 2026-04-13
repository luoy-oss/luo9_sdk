use libc::{c_char, c_int};
use std::ffi::{CStr, CString};

#[repr(C)]
pub struct CommandHandle {
    _private: [u8; 0],
}

unsafe extern "C" {
    unsafe fn luo9_command_create(
        msg: *const c_char,
        cmd_name: *const c_char,
        mode: c_int,
        prefix_char: c_char,
    ) -> *mut CommandHandle;

    unsafe fn luo9_command_free(handle: *mut CommandHandle);
    unsafe fn luo9_command_get_name(handle: *const CommandHandle) -> *mut c_char;
    unsafe fn luo9_command_get_args_raw(handle: *const CommandHandle) -> *mut c_char;
    unsafe fn luo9_command_has_args(handle: *const CommandHandle) -> c_int;
    unsafe fn luo9_command_args_count(handle: *const CommandHandle) -> c_int;
    unsafe fn luo9_command_get_arg(handle: *const CommandHandle, index: c_int) -> *mut c_char;
    unsafe fn luo9_free_string(ptr: *mut c_char);
}

#[derive(Debug, Clone, Copy)]
pub enum PrefixMode {
    Required(char),
    Optional(char),
    None,
}

#[derive(Debug)]
pub struct Command {
    handle: *mut CommandHandle,
}

impl Command {
    pub fn parse(
        msg: &str,
        cmd_name: &str,
        mode: PrefixMode,
    ) -> Option<Self> {
        let msg_c: CString = CString::new(msg).ok()?;
        let cmd_c: CString = CString::new(cmd_name).ok()?;

        let (mode_val, prefix) = match mode {
            PrefixMode::Required(c) => (0, c as c_char),
            PrefixMode::Optional(c) => (1, c as c_char),
            PrefixMode::None => (2, 0),
        };

        let handle = unsafe {
            luo9_command_create(
                msg_c.as_ptr(),
                cmd_c.as_ptr(),
                mode_val,
                prefix,
            )
        };

        if handle.is_null() {
            None
        } else {
            Some(Self { handle })
        }
    }

    pub fn name(&self) -> String {
        unsafe {
            let ptr = luo9_command_get_name(self.handle);
            let s = CStr::from_ptr(ptr).to_string_lossy().to_string();
            luo9_free_string(ptr);
            s
        }
    }

    pub fn args_raw(&self) -> String {
        unsafe {
            let ptr = luo9_command_get_args_raw(self.handle);
            let s = CStr::from_ptr(ptr).to_string_lossy().to_string();
            luo9_free_string(ptr);
            s
        }
    }

    pub fn has_args(&self) -> bool {
        unsafe { luo9_command_has_args(self.handle) == 1 }
    }

    pub fn args_count(&self) -> usize {
        unsafe { luo9_command_args_count(self.handle) as usize }
    }

    pub fn arg_at(&self, index: usize) -> Option<String> {
        unsafe {
            let ptr = luo9_command_get_arg(self.handle, index as c_int);
            if ptr.is_null() {
                None
            } else {
                let s = CStr::from_ptr(ptr).to_string_lossy().to_string();
                luo9_free_string(ptr);
                Some(s)
            }
        }
    }
}

impl Drop for Command {
    fn drop(&mut self) {
        unsafe {
            luo9_command_free(self.handle);
        }
    }
}


#[cfg(test)]
mod tests{
    use super::*;
   
    #[test]
    fn test_optional_prefix_echo() {
        let cmd = Command::parse("/echo hello world", "echo", PrefixMode::Optional('/')).unwrap();
        assert_eq!(cmd.name(), "echo");
        assert_eq!(cmd.args_raw(), " hello world");
        assert_eq!(cmd.has_args(), true);
        assert_eq!(cmd.args_count(), 2);
        assert_eq!(cmd.arg_at(0), Some("hello".to_string()));
        assert_eq!(cmd.arg_at(1), Some("world".to_string()));
    }


}