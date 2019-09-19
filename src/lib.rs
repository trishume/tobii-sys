//! Based on the stream engine headers from v1.2.1.305

/* automatically generated by rust-bindgen and then heavily cleaned up by @trishume */

pub mod helpers;

pub const TOBII_DEVICE_GENERATION_G5: ::std::os::raw::c_uint = 2;
pub const TOBII_DEVICE_GENERATION_IS3: ::std::os::raw::c_uint = 4;
pub const TOBII_DEVICE_GENERATION_IS4: ::std::os::raw::c_uint = 8;

pub const TOBII_ERROR_NO_ERROR: Status = 0;
pub const TOBII_ERROR_INTERNAL: Status = 1;
pub const TOBII_ERROR_INSUFFICIENT_LICENSE: Status = 2;
pub const TOBII_ERROR_NOT_SUPPORTED: Status = 3;
pub const TOBII_ERROR_NOT_AVAILABLE: Status = 4;
pub const TOBII_ERROR_CONNECTION_FAILED: Status = 5;
pub const TOBII_ERROR_TIMED_OUT: Status = 6;
pub const TOBII_ERROR_ALLOCATION_FAILED: Status = 7;
pub const TOBII_ERROR_INVALID_PARAMETER: Status = 8;
pub const TOBII_ERROR_CALIBRATION_ALREADY_STARTED: Status = 9;
pub const TOBII_ERROR_CALIBRATION_NOT_STARTED: Status = 10;
pub const TOBII_ERROR_ALREADY_SUBSCRIBED: Status = 11;
pub const TOBII_ERROR_NOT_SUBSCRIBED: Status = 12;
pub const TOBII_ERROR_OPERATION_FAILED: Status = 13;
pub const TOBII_ERROR_CONFLICTING_API_INSTANCES: Status = 14;
pub const TOBII_ERROR_CALIBRATION_BUSY: Status = 15;
pub const TOBII_ERROR_CALLBACK_IN_PROGRESS: Status = 16;

/// tobii_error_t
pub type Status = ::std::os::raw::c_uint;

/// tobii_device_t
#[repr(C)]
#[derive(Debug, Copy, Clone )]
pub struct Device {
    _unused: [u8; 0],
}

/// tobii_device_info_t
#[repr(C)]
#[derive(Copy, Clone )]
pub struct DeviceInfo {
    pub serial_number: [::std::os::raw::c_char; 128usize],
    pub model: [::std::os::raw::c_char; 64usize],
    pub generation: [::std::os::raw::c_char; 64usize],
    pub firmware_version: [::std::os::raw::c_char; 128usize],
}

/// tobii_version_t
#[repr(C)]
#[derive(Debug, Copy, Clone )]
pub struct Version {
    pub major: ::std::os::raw::c_int,
    pub minor: ::std::os::raw::c_int,
    pub revision: ::std::os::raw::c_int,
    pub build: ::std::os::raw::c_int,
}

pub const TOBII_LOG_LEVEL_ERROR: LogLevel = 0;
pub const TOBII_LOG_LEVEL_WARN: LogLevel = 1;
pub const TOBII_LOG_LEVEL_INFO: LogLevel = 2;
pub const TOBII_LOG_LEVEL_DEBUG: LogLevel = 3;
pub const TOBII_LOG_LEVEL_TRACE: LogLevel = 4;
pub type LogLevel = ::std::os::raw::c_uint;
pub type LogFn =
    ::std::option::Option<unsafe extern "C" fn(log_context: *mut ::std::os::raw::c_void,
                                               level: LogLevel,
                                               text: *const ::std::os::raw::c_char)>;

/// tobii_custom_log_t
#[repr(C)]
#[derive(Debug, Copy, Clone )]
pub struct CustomLog {
    pub log_context: *mut ::std::os::raw::c_void,
    pub log_func: LogFn,
}
pub type MallocFn =
    ::std::option::Option<unsafe extern "C" fn(mem_context: *mut ::std::os::raw::c_void,
                                               size: usize)
                                               -> *mut ::std::os::raw::c_void>;
pub type FreeFn =
    ::std::option::Option<unsafe extern "C" fn(mem_context: *mut ::std::os::raw::c_void,
                                               ptr: *mut ::std::os::raw::c_void)>;

#[repr(C)]
#[derive(Debug, Copy, Clone )]
pub struct CustomAlloc {
    pub mem_context: *mut ::std::os::raw::c_void,
    pub malloc_func: MallocFn,
    pub free_func: FreeFn,
}

/// tobii_api_t
#[repr(C)]
#[derive(Debug, Copy, Clone )]
pub struct Api {
    _unused: [u8; 0],
}
pub type DeviceUrlReceiver =
    ::std::option::Option<unsafe extern "C" fn(url: *const ::std::os::raw::c_char,
                                               user_data: *mut ::std::os::raw::c_void)>;

#[repr(C)]
#[derive(Debug, Copy, Clone )]
pub struct TrackBox {
    pub front_upper_right_xyz: [f32; 3],
    pub front_upper_left_xyz: [f32; 3],
    pub front_lower_left_xyz: [f32; 3],
    pub front_lower_right_xyz: [f32; 3],
    pub back_upper_right_xyz: [f32; 3],
    pub back_upper_left_xyz: [f32; 3],
    pub back_lower_left_xyz: [f32; 3],
    pub back_lower_right_xyz: [f32; 3],
}

pub const TOBII_STATE_POWER_SAVE_ACTIVE: State = 0;
pub const TOBII_STATE_REMOTE_WAKE_ACTIVE: State = 1;
pub const TOBII_STATE_DEVICE_PAUSED: State = 2;
pub const TOBII_STATE_EXCLUSIVE_MODE: State = 3;
pub type State = ::std::os::raw::c_uint;

pub const TOBII_STATE_BOOL_FALSE: StateBool = 0;
pub const TOBII_STATE_BOOL_TRUE: StateBool = 1;
pub type StateBool = ::std::os::raw::c_uint;

pub const TOBII_NOT_SUPPORTED: Supported = 0;
pub const TOBII_SUPPORTED: Supported = 1;
pub type Supported = ::std::os::raw::c_uint;

pub const TOBII_CAPABILITY_DISPLAY_AREA_WRITABLE: Capability = 0;
pub const TOBII_CAPABILITY_CALIBRATION_2D: Capability = 1;
pub const TOBII_CAPABILITY_CALIBRATION_3D: Capability = 2;
pub const TOBII_CAPABILITY_PERSISTENT_STORAGE: Capability = 3;
pub type Capability = ::std::os::raw::c_uint;

pub const TOBII_STREAM_GAZE_POINT: Stream = 0;
pub const TOBII_STREAM_GAZE_ORIGIN: Stream = 1;
pub const TOBII_STREAM_EYE_POSITION_NORMALIZED: Stream = 2;
pub const TOBII_STREAM_USER_PRESENCE: Stream = 3;
pub const TOBII_STREAM_HEAD_POSE: Stream = 4;
pub const TOBII_STREAM_WEARABLE: Stream = 5;
pub const TOBII_STREAM_GAZE_DATA: Stream = 6;
pub const TOBII_STREAM_DIGITAL_SYNCPORT: Stream = 7;
pub const TOBII_STREAM_DIAGNOSTICS_IMAGE: Stream = 8;
pub type Stream = ::std::os::raw::c_uint;

pub type DataReceiver =
    ::std::option::Option<unsafe extern "C" fn(data: *const ::std::os::raw::c_void,
                                               size: usize,
                                               user_data: *mut ::std::os::raw::c_void)>;

pub const TOBII_VALIDITY_INVALID: Validity = 0;
pub const TOBII_VALIDITY_VALID: Validity = 1;
pub type Validity = ::std::os::raw::c_uint;

#[repr(C)]
#[derive(Debug, Copy, Clone )]
pub struct DisplayArea {
    pub top_left_mm_xyz: [f32; 3],
    pub top_right_mm_xyz: [f32; 3],
    pub bottom_left_mm_xyz: [f32; 3],
}

// tobii_stream.h =========================

pub const TOBII_NOTIFICATION_TYPE_CALIBRATION_STATE_CHANGED : NotificationType = 0 ;
pub const TOBII_NOTIFICATION_TYPE_EXCLUSIVE_MODE_STATE_CHANGED : NotificationType = 1 ;
pub const TOBII_NOTIFICATION_TYPE_TRACK_BOX_CHANGED : NotificationType = 2 ;
pub const TOBII_NOTIFICATION_TYPE_DISPLAY_AREA_CHANGED : NotificationType = 3 ;
pub const TOBII_NOTIFICATION_TYPE_FRAMERATE_CHANGED : NotificationType = 4 ;
pub const TOBII_NOTIFICATION_TYPE_POWER_SAVE_STATE_CHANGED : NotificationType = 5 ;
pub const TOBII_NOTIFICATION_TYPE_DEVICE_PAUSED_STATE_CHANGED : NotificationType = 6 ;
pub type NotificationType = ::std::os::raw::c_uint;
pub const TOBII_NOTIFICATION_VALUE_TYPE_NONE : NotificationValueType = 0 ;
pub const TOBII_NOTIFICATION_VALUE_TYPE_FLOAT : NotificationValueType = 1 ;
pub const TOBII_NOTIFICATION_VALUE_TYPE_STATE : NotificationValueType = 2 ;
pub const TOBII_NOTIFICATION_VALUE_TYPE_DISPLAY_AREA : NotificationValueType = 3 ;
pub type NotificationValueType = ::std::os::raw::c_uint;

#[repr(C)]
#[derive(Copy, Clone)]
pub union NotificationUnion {
    float: f32,
    state: StateBool,
    display_area: DisplayArea,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Notification {
    pub type_: NotificationType,
    pub value_type: NotificationValueType,
    pub value: NotificationUnion,
}

pub type NotificationsCallbackFn =
    ::std::option::Option<unsafe extern "C" fn(notification: *const Notification,
                                               user_data: *mut ::std::os::raw::c_void)>;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct GazePoint {
    pub timestamp_us: i64,
    pub validity: Validity,
    pub position_xy: [f32; 2usize],
}
pub type GazePointFn =
    ::std::option::Option<unsafe extern "C" fn(gaze_point: *const GazePoint,
                                               user_data: *mut ::std::os::raw::c_void)>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct GazeOrigin {
    pub timestamp_us: i64,
    pub left_validity: Validity,
    pub left_xyz: [f32; 3usize],
    pub right_validity: Validity,
    pub right_xyz: [f32; 3usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct EyePositionNormalized {
    pub timestamp_us: i64,
    pub left_validity: Validity,
    pub left_xyz: [f32; 3usize],
    pub right_validity: Validity,
    pub right_xyz: [f32; 3usize],
}

pub const TOBII_USER_PRESENCE_STATUS_UNKNOWN : UserPresenceStatus = 0 ;
pub const TOBII_USER_PRESENCE_STATUS_AWAY : UserPresenceStatus = 1 ;
pub const TOBII_USER_PRESENCE_STATUS_PRESENT : UserPresenceStatus = 2 ;
pub type UserPresenceStatus = ::std::os::raw::c_uint;
pub type UserPresenceFn =
    ::std::option::Option<unsafe extern "C" fn(status: UserPresenceStatus,
                                               timestamp_us: i64,
                                               user_data: *mut ::std::os::raw::c_void)>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct HeadPose {
    pub timestamp_us: i64,
    pub position_validity: Validity,
    pub position_xyz: [f32; 3usize],
    pub rotation_validity_xyz: [Validity; 3usize],
    pub rotation_xyz: [f32; 3usize],
}
pub type HeadPoseFn =
    ::std::option::Option<unsafe extern "C" fn(head_pose: *const HeadPose,
                                               user_data: *mut ::std::os::raw::c_void)>;
pub type GazeOriginFn =
    ::std::option::Option<unsafe extern "C" fn(gaze_origin: *const GazeOrigin,
                                               user_data: *mut ::std::os::raw::c_void)>;
pub type EyePositionNormalizedFn = :: std :: option :: Option < unsafe extern "C" fn ( eye_position : * const EyePositionNormalized , user_data : * mut :: std :: os :: raw :: c_void ) > ;



#[link(name = "StreamEngineClientKit", kind = "framework")]
extern "C" {
    pub fn tobii_error_message(error: Status) -> *const ::std::os::raw::c_char;
    pub fn tobii_get_api_version(version: *mut Version) -> Status;
    pub fn tobii_api_create(api: *mut *mut Api,
                            custom_alloc: *const CustomAlloc,
                            custom_log: *const CustomLog)
                            -> Status;
    pub fn tobii_api_destroy(api: *mut Api) -> Status;
    pub fn tobii_system_clock(api: *mut Api, timestamp_us: *mut i64) -> Status;
    pub fn tobii_enumerate_local_device_urls(api: *mut Api,
                                             receiver: DeviceUrlReceiver,
                                             user_data: *mut ::std::os::raw::c_void)
                                             -> Status;
    pub fn tobii_enumerate_local_device_urls_ex(api: *mut Api,
                                                receiver: DeviceUrlReceiver,
                                                user_data: *mut ::std::os::raw::c_void,
                                                device_generations: u32)
                                                -> Status;
    pub fn tobii_device_create(api: *mut Api,
                               url: *const ::std::os::raw::c_char,
                               device: *mut *mut Device)
                               -> Status;
    pub fn tobii_device_destroy(device: *mut Device) -> Status;
    pub fn tobii_wait_for_callbacks(device_count: ::std::os::raw::c_int,
                                    devices: *const *mut Device)
                                    -> Status;
    pub fn tobii_device_process_callbacks(device: *mut Device) -> Status;
    pub fn tobii_clear_callback_buffers(device: *mut Device) -> Status;
    pub fn tobii_device_reconnect(device: *mut Device) -> Status;
    pub fn tobii_update_timesync(device: *mut Device) -> Status;
    pub fn tobii_get_device_info(device: *mut Device,
                                 device_info: *mut DeviceInfo)
                                 -> Status;
    pub fn tobii_get_track_box(device: *mut Device,
                               track_box: *mut TrackBox)
                               -> Status;
    pub fn tobii_get_state_bool(device: *mut Device,
                                state: State,
                                value: *mut StateBool)
                                -> Status;
    pub fn tobii_capability_supported(device: *mut Device,
                                      capability: Capability,
                                      supported: *mut Supported)
                                      -> Status;
    pub fn tobii_stream_supported(device: *mut Device,
                                  stream: Stream,
                                  supported: *mut Supported)
                                  -> Status;


    // tobii_stream.h =================================================================

    pub fn tobii_gaze_point_subscribe(device: *mut Device,
                                      callback: GazePointFn,
                                      user_data: *mut ::std::os::raw::c_void)
                                      -> Status;
    pub fn tobii_gaze_point_unsubscribe(device: *mut Device) -> Status;
    pub fn tobii_gaze_origin_subscribe(device: *mut Device,
                                       callback: GazeOriginFn,
                                       user_data: *mut ::std::os::raw::c_void)
                                       -> Status;
    pub fn tobii_gaze_origin_unsubscribe(device: *mut Device) -> Status;
    pub fn tobii_eye_position_normalized_subscribe(device: *mut Device,
                                       callback: EyePositionNormalizedFn,
                                       user_data: *mut ::std::os::raw::c_void)
                                       -> Status;
    pub fn tobii_eye_position_normalized_unsubscribe(device: *mut Device) -> Status;
    pub fn tobii_user_presence_subscribe(device: *mut Device,
                                         callback: UserPresenceFn,
                                         user_data: *mut ::std::os::raw::c_void)
                                         -> Status;
    pub fn tobii_user_presence_unsubscribe(device: *mut Device) -> Status;
    pub fn tobii_head_pose_subscribe(device: *mut Device,
                                     callback: HeadPoseFn,
                                     user_data: *mut ::std::os::raw::c_void)
                                     -> Status;
    pub fn tobii_head_pose_unsubscribe(device: *mut Device) -> Status;
    pub fn tobii_notifications_subscribe(device: *mut Device,
                                         callback: NotificationsCallbackFn,
                                         user_data: *mut ::std::os::raw::c_void)
                                         -> Status;
    pub fn tobii_notifications_unsubscribe(device: *mut Device) -> Status;
}
