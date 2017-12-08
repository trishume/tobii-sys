// #![feature(link_args)]
extern crate tobii_sys;
use tobii_sys::*;
use std::ptr;
use std::mem;
// use std::os::raw;
use std::ffi::CStr;

// #[link_args = "-F /Library/Application\\ Support/Tobii/"]
// extern {}

unsafe extern "C"
fn custom_log_fn(_log_context: *mut ::std::os::raw::c_void, level: LogLevel, text: *const ::std::os::raw::c_char) {
    let s = CStr::from_ptr(text);
    println!("LOG {}: {}", level, s.to_str().unwrap());
}

fn main() {
    unsafe{
        let custom_log = CustomLog {
            log_context: ptr::null_mut(),
            log_func: Some(custom_log_fn)
        };

        println!("Initializing API!");
        let mut api: *mut Api = mem::zeroed();
        let error = tobii_api_create( &mut api as *mut *mut Api, ptr::null_mut(), &custom_log as *const _);
        if error != TOBII_ERROR_NO_ERROR {
            println!("Failed to initialize API");
            return;
        }
    }
}
