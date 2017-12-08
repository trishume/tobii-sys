use super::*;

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
        _ => Err(TobiiError::Unknown(status))
    }
}
