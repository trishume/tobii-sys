use super::*;
use std::os::raw;
use std::thread;
use std::time;
use std::ffi::CStr;
use std::ptr;

pub struct PtrWrapper<T> {
    ptr: *mut T,
    destroy_fn: unsafe extern "C" fn(ptr: *mut T) -> Status,
}

impl<T> PtrWrapper<T> {
    pub unsafe fn new(ptr: *mut T, destroy_fn: unsafe extern "C" fn(ptr: *mut T) -> Status) -> PtrWrapper<T> {
        PtrWrapper { ptr, destroy_fn }
    }

    pub fn ptr(&self) -> *mut T {
        self.ptr
    }
}

impl<T> Drop for PtrWrapper<T> {
    fn drop(&mut self) {
        let destroy_fn = self.destroy_fn;
        let status = unsafe { destroy_fn(self.ptr) };
        assert_eq!(status, TOBII_ERROR_NO_ERROR);
    }
}

#[derive(Debug,Clone,Copy)]
pub enum TobiiError {
    Internal,
    InsufficientLicense,
    NotSupported,
    NotAvailable,
    ConnectionFailed,
    TimedOut,
    AllocationFailed,
    InvalidParameter,
    CalibrationAlreadyStarted,
    CalibrationNotStarted,
    AlreadySubscribed,
    NotSubscribed,
    OperationFailed,
    ConflictingApiInstances,
    CalibrationBusy,
    CallbackInProgress,
    Unknown(Status),
}

pub fn status_to_result(status: Status) -> Result<(),TobiiError> {
    match status {
        TOBII_ERROR_NO_ERROR => Ok(()),
        TOBII_ERROR_INTERNAL => Err(TobiiError::Internal),
        TOBII_ERROR_INSUFFICIENT_LICENSE => Err(TobiiError::InsufficientLicense),
        TOBII_ERROR_NOT_SUPPORTED => Err(TobiiError::NotSupported),
        TOBII_ERROR_NOT_AVAILABLE => Err(TobiiError::NotAvailable),
        TOBII_ERROR_CONNECTION_FAILED => Err(TobiiError::ConnectionFailed),
        TOBII_ERROR_TIMED_OUT => Err(TobiiError::TimedOut),
        TOBII_ERROR_ALLOCATION_FAILED => Err(TobiiError::AllocationFailed),
        TOBII_ERROR_INVALID_PARAMETER => Err(TobiiError::InvalidParameter),
        TOBII_ERROR_CALIBRATION_ALREADY_STARTED => Err(TobiiError::CalibrationAlreadyStarted),
        TOBII_ERROR_CALIBRATION_NOT_STARTED => Err(TobiiError::CalibrationNotStarted),
        TOBII_ERROR_ALREADY_SUBSCRIBED => Err(TobiiError::AlreadySubscribed),
        TOBII_ERROR_NOT_SUBSCRIBED => Err(TobiiError::NotSubscribed),
        TOBII_ERROR_OPERATION_FAILED => Err(TobiiError::OperationFailed),
        TOBII_ERROR_CONFLICTING_API_INSTANCES => Err(TobiiError::ConflictingApiInstances),
        TOBII_ERROR_CALIBRATION_BUSY => Err(TobiiError::CalibrationBusy),
        TOBII_ERROR_CALLBACK_IN_PROGRESS => Err(TobiiError::CallbackInProgress),
        _ => Err(TobiiError::Unknown(status))
    }
}

pub unsafe fn list_devices(api: *mut Api) -> Result<Vec<String>, TobiiError> {
    unsafe extern "C" fn callback(url: *const raw::c_char, user_data: *mut raw::c_void) {
        let list = &mut *(user_data as *mut Vec<String>);
        let s = CStr::from_ptr(url);
        list.push(s.to_str().unwrap().to_string());
    }

    let mut list: Vec<String> = Vec::new();
    let list_ptr = &mut list as *mut Vec<String>;
    let status = tobii_enumerate_local_device_urls(api, Some(callback), list_ptr as *mut raw::c_void);
    status_to_result(status)?;
    Ok(list)
}

pub unsafe fn reconnect(device: *mut Device) -> Status {
    for _i in 0..40 {
        let status = tobii_device_reconnect(device);
        if status != TOBII_ERROR_CONNECTION_FAILED { return status; }
        thread::sleep(time::Duration::from_millis(250));
    }
    return TOBII_ERROR_CONNECTION_FAILED;
}

pub unsafe fn wait_for_device_callbacks(device: *mut Device) -> Status {
    let ptr_ptr_dev: *const *mut Device = (&device) as *const *mut Device;
    tobii_wait_for_callbacks(1, ptr_ptr_dev)
}
