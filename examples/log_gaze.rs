extern crate tobii_sys;
use tobii_sys::*;
use std::ptr;
use std::mem;
use std::os::raw;
use std::ffi::{CStr, CString};

use tobii_sys::helpers::{self, PtrWrapper, status_to_result, TobiiError};

unsafe extern "C"
fn custom_log_fn(_log_context: *mut ::std::os::raw::c_void, level: LogLevel, text: *const raw::c_char) {
    if level > TOBII_LOG_LEVEL_WARN { return; }
    let s = CStr::from_ptr(text);
    println!("LOG {}: {}", level, s.to_str().unwrap());
}

unsafe extern "C"
fn gaze_callback(gaze_point: *const GazePoint, _user_data: *mut ::std::os::raw::c_void) {
    let pt = &*gaze_point;
    println!("GAZE {}: {}, {}", pt.timestamp_us, pt.position_xy[0], pt.position_xy[1]);
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

        let devices = helpers::list_devices(api.ptr())?;
        println!("{:?}", devices);

        if devices.len() < 1 {
            println!("No devices");
            return Ok(());
        }

        let url_c_string = CString::new(devices[0].clone()).unwrap();
        let url_c = url_c_string.as_c_str();
        let mut device_ptr: *mut Device = mem::zeroed();
        let status = tobii_device_create(api.ptr(), url_c.as_ptr(), &mut device_ptr as *mut *mut Device);
        status_to_result(status)?;
        let device = PtrWrapper::new(device_ptr, tobii_device_destroy);

        let status = tobii_gaze_point_subscribe(device.ptr(), Some(gaze_callback), ptr::null_mut());
        let _subscription = PtrWrapper::new(device.ptr(), tobii_gaze_point_unsubscribe);
        status_to_result(status)?;
        for _i in 1..1000 {
            let status = tobii_wait_for_callbacks(device.ptr());
            match status_to_result(status) {
                Err(TobiiError::TimedOut) => continue,
                Err(TobiiError::ConnectionFailed) => {
                    status_to_result(helpers::reconnect(device.ptr()))?;
                    continue;
                },
                Err(e) => return Err(e),
                Ok(()) => (),
            }

            let status = tobii_process_callbacks(device.ptr());
            if status == TOBII_ERROR_CONNECTION_FAILED {
                status_to_result(helpers::reconnect(device.ptr()))?;
                continue;
            }
            status_to_result(status)?;
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
