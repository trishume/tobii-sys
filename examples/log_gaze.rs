extern crate tobii_sys;
use tobii_sys::*;
use std::ptr;
use std::mem;
use std::os::raw;
use std::ffi::CStr;

use tobii_sys::helpers::{PtrWrapper, status_to_result, TobiiError};

fn list_devices(api: *mut Api) -> Result<Vec<String>, TobiiError> {
    unsafe extern "C" fn callback(url: *const raw::c_char, user_data: *mut raw::c_void) {
        let list = &mut *(user_data as *mut Vec<String>);
        let s = CStr::from_ptr(url);
        list.push(s.to_str().unwrap().to_string());
    }

    unsafe {
        let mut list: Vec<String> = Vec::new();
        let list_ptr = &mut list as *mut Vec<String>;
        let status = tobii_enumerate_local_device_urls(api, Some(callback), list_ptr as *mut raw::c_void);
        status_to_result(status)?;
        Ok(list)
    }
}

unsafe extern "C"
fn custom_log_fn(_log_context: *mut ::std::os::raw::c_void, level: LogLevel, text: *const raw::c_char) {
    let s = CStr::from_ptr(text);
    println!("LOG {}: {}", level, s.to_str().unwrap());
}

fn run_demo() -> Result<(), TobiiError> {
    unsafe{
        let custom_log = CustomLog {
            log_context: ptr::null_mut(),
            log_func: Some(custom_log_fn)
        };

        println!("Initializing API!");
        let mut api_ptr: *mut Api = mem::zeroed();
        let status = tobii_api_create( &mut api_ptr as *mut *mut Api, ptr::null_mut(), &custom_log as *const _);
        status_to_result(status)?;
        let api = PtrWrapper::new(api_ptr, tobii_api_destroy);

        let devices = list_devices(api.ptr())?;
        println!("{:?}", devices);

        if devices.len() < 1 {
            println!("No devices");
            return Ok(());
        }

    }
    Ok(())
}

fn main() {
    match run_demo() {
        Ok(()) => (),
        Err(e) => {
            println!("Error: {:?}", e);
        }
    }
}
