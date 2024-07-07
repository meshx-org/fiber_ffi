macro_rules! multiconst {
    ($typename:ident, [$($(#[$attr:meta])* $rawname:ident = $value:expr;)*]) => {
        $(
            $(#[$attr])*
            pub const $rawname: $typename = $value;
        )*
    }
}

#[allow(non_camel_case_types)]
pub type fx_handle_t = u32;

#[allow(non_camel_case_types)]
pub type fx_status_t = i32;

#[allow(non_camel_case_types)]
pub type fx_obj_type_t = u32;

#[allow(non_camel_case_types)]
pub type fx_rights_t = u32;

#[allow(non_camel_case_types)]
pub type fx_signals_t = u32;

#[allow(non_camel_case_types)]
pub type fx_vaddr_t = u32;

#[allow(non_camel_case_types)]
/// Absolute time in nanoseconds (generally with respect to the monotonic clock)
pub type fx_time_t = i64;

#[repr(u32)]
#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum fx_packet_type_t {
    FX_PKT_TYPE_USER = 0,
    FX_PKT_TYPE_SIGNAL_ONE = 1,
    FX_PKT_TYPE_SIGNAL_REP = 2,
}

#[repr(C)]
#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct fx_port_packet_t {
    pub key: u64,
    pub packet_type: fx_packet_type_t,
    pub status: i32,
    pub union: [u8; 32],
}

#[repr(C)]
#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct fx_handle_info_t {
    pub handle: fx_handle_t,
    pub ty: fx_obj_type_t,
    pub rights: fx_rights_t,
    pub unused: u32,
}

multiconst!(fx_status_t, [
    FX_OK                         = 0;
]);

multiconst!(fx_obj_type_t, [
    FX_OBJ_TYPE_NONE                = 0;
    FX_OBJ_TYPE_PROCESS             = 1;
    FX_OBJ_TYPE_CHANNEL             = 4;
    FX_OBJ_TYPE_EVENT               = 5;
    FX_OBJ_TYPE_PORT                = 6;
    FX_OBJ_TYPE_JOB                 = 17;
]);

multiconst!(fx_time_t, [
    FX_TIME_INFINITE = 9223372036854775807;// std::i64::MAX;
    FX_TIME_INFINITE_PAST = -9223372036854775808; //std::i64::MIN;
]);

multiconst!(fx_signals_t, [
    FX_SIGNAL_NONE              = 0;
    FX_OBJECT_SIGNAL_ALL        = 0x00ffffff;
    FX_USER_SIGNAL_ALL          = 0xff000000;
    FX_OBJECT_SIGNAL_0          = 1 << 0;
    FX_OBJECT_SIGNAL_1          = 1 << 1;
    FX_OBJECT_SIGNAL_2          = 1 << 2;
    FX_OBJECT_SIGNAL_3          = 1 << 3;
    FX_OBJECT_SIGNAL_4          = 1 << 4;
    FX_OBJECT_SIGNAL_5          = 1 << 5;
    FX_OBJECT_SIGNAL_6          = 1 << 6;
    FX_OBJECT_SIGNAL_7          = 1 << 7;
    FX_OBJECT_SIGNAL_8          = 1 << 8;
    FX_OBJECT_SIGNAL_9          = 1 << 9;
    FX_OBJECT_SIGNAL_10         = 1 << 10;
    FX_OBJECT_SIGNAL_11         = 1 << 11;
    FX_OBJECT_SIGNAL_12         = 1 << 12;
    FX_OBJECT_SIGNAL_13         = 1 << 13;
    FX_OBJECT_SIGNAL_14         = 1 << 14;
    FX_OBJECT_SIGNAL_15         = 1 << 15;
    FX_OBJECT_SIGNAL_16         = 1 << 16;
    FX_OBJECT_SIGNAL_17         = 1 << 17;
    FX_OBJECT_SIGNAL_18         = 1 << 18;
    FX_OBJECT_SIGNAL_19         = 1 << 19;
    FX_OBJECT_SIGNAL_20         = 1 << 20;
    FX_OBJECT_SIGNAL_21         = 1 << 21;
    FX_OBJECT_SIGNAL_22         = 1 << 22;
     FX_OBJECT_HANDLE_CLOSED     = 1 << 23;
      FX_USER_SIGNAL_0            = 1 << 24;
    FX_USER_SIGNAL_1            = 1 << 25;
    FX_USER_SIGNAL_2            = 1 << 26;
    FX_USER_SIGNAL_3            = 1 << 27;
    FX_USER_SIGNAL_4            = 1 << 28;
    FX_USER_SIGNAL_5            = 1 << 29;
    FX_USER_SIGNAL_6            = 1 << 30;
    FX_USER_SIGNAL_7            = 1 << 31;
    FX_OBJECT_READABLE          = FX_OBJECT_SIGNAL_0;
    FX_OBJECT_WRITABLE          = FX_OBJECT_SIGNAL_1;
    FX_OBJECT_PEER_CLOSED       = FX_OBJECT_SIGNAL_2;
    // Cancelation (handle was closed while waiting with it)
    FX_SIGNAL_HANDLE_CLOSED     = FX_OBJECT_HANDLE_CLOSED;
    // Event
    FX_EVENT_SIGNALED           = FX_OBJECT_SIGNAL_3;
    // EventPair
    FX_EVENTPAIR_SIGNALED       = FX_OBJECT_SIGNAL_3;
    FX_EVENTPAIR_CLOSED         = FX_OBJECT_SIGNAL_2;
    // Task signals (process, job)
    FX_TASK_TERMINATED          = FX_OBJECT_SIGNAL_3;
    // Channel
    FX_CHANNEL_READABLE         = FX_OBJECT_SIGNAL_0;
    FX_CHANNEL_WRITABLE         = FX_OBJECT_SIGNAL_1;
    FX_CHANNEL_PEER_CLOSED      = FX_OBJECT_SIGNAL_2;
    // Clock
    FX_CLOCK_STARTED            = FX_OBJECT_SIGNAL_4;
    // Socket
    FX_SOCKET_READABLE            = FX_OBJECT_READABLE;
    FX_SOCKET_WRITABLE            = FX_OBJECT_WRITABLE;
    FX_SOCKET_PEER_CLOSED         = FX_OBJECT_PEER_CLOSED;
    FX_SOCKET_PEER_WRITE_DISABLED = FX_OBJECT_SIGNAL_4;
    FX_SOCKET_WRITE_DISABLED      = FX_OBJECT_SIGNAL_5;
    FX_SOCKET_READ_THRESHOLD      = FX_OBJECT_SIGNAL_10;
    FX_SOCKET_WRITE_THRESHOLD     = FX_OBJECT_SIGNAL_11;
    // Job
    FX_JOB_TERMINATED           = FX_OBJECT_SIGNAL_3;
    FX_JOB_NO_JOBS              = FX_OBJECT_SIGNAL_4;
    FX_JOB_NO_PROCESSES         = FX_OBJECT_SIGNAL_5;
    // Process
    FX_PROCESS_TERMINATED       = FX_OBJECT_SIGNAL_3;
    // Thread
    FX_THREAD_TERMINATED        = FX_OBJECT_SIGNAL_3;
    FX_THREAD_RUNNING           = FX_OBJECT_SIGNAL_4;
    FX_THREAD_SUSPENDED         = FX_OBJECT_SIGNAL_5;
    // Log
    FX_LOG_READABLE             = FX_OBJECT_READABLE;
    FX_LOG_WRITABLE             = FX_OBJECT_WRITABLE;
    // Timer
    FX_TIMER_SIGNALED           = FX_OBJECT_SIGNAL_3;
    // Vmo
    FX_VMO_ZERO_CHILDREN        = FX_OBJECT_SIGNAL_3;
]);

#[no_mangle]
pub extern "C" fn fx_port_create(options: u32, out: *mut fx_handle_t) -> fx_status_t {
    FX_OK
}

#[no_mangle]
pub extern "C" fn fx_port_queue(
    handle: fx_handle_t,
    packet: *const fx_port_packet_t,
) -> fx_status_t {
    FX_OK
}

#[no_mangle]
pub extern "C" fn fx_port_wait(
    handle: fx_handle_t,
    deadline: fx_time_t,
    packet: *mut fx_port_packet_t,
) -> fx_status_t {
    FX_OK
}

#[no_mangle]
pub extern "C" fn fx_port_cancel(
    handle: fx_handle_t,
    source: fx_handle_t,
    key: u64,
) -> fx_status_t {
    FX_OK
}

#[no_mangle]
pub extern "C" fn fx_handle_close(handle: fx_handle_t) -> fx_status_t {
    FX_OK
}

#[no_mangle]
pub extern "C" fn fx_handle_duplicate(
    handle: fx_handle_t,
    rights: fx_rights_t,
    out: *const fx_handle_t,
) -> fx_status_t {
    FX_OK
}

#[no_mangle]
pub extern "C" fn fx_handle_replace(
    handle: fx_handle_t,
    rights: fx_rights_t,
    out: *const fx_handle_t,
) -> fx_status_t {
    FX_OK
}

#[no_mangle]
pub extern "C" fn fx_channel_read(
    handle: fx_handle_t,
    options: u32,
    bytes: *mut u8,
    handles: *mut fx_handle_t,
    num_bytes: u32,
    num_handles: u32,
    actual_bytes: *mut u32,
    actual_handles: *mut u32,
) -> fx_status_t {
    FX_OK
}

#[no_mangle]
pub extern "C" fn fx_channel_read_etc(
    handle: fx_handle_t,
    options: u32,
    bytes: *mut u8,
    handles: *mut fx_handle_info_t,
    num_bytes: u32,
    num_handles: u32,
    actual_bytes: *mut u32,
    actual_handles: *mut u32,
) -> fx_status_t {
    FX_OK
}

#[no_mangle]
pub extern "C" fn fx_process_create(
    job: fx_handle_t,
    name: *const u8,
    name_size: usize,
    options: u32,
    proc_handle: *mut fx_handle_t,
    dv_handle: *mut fx_handle_t,
) -> fx_status_t {
    FX_OK
}

#[no_mangle]
pub extern "C" fn fx_process_start(
    handle: fx_handle_t,
    entry: fx_vaddr_t,
    arg1: fx_handle_t,
) -> fx_status_t {
    FX_OK
}

#[no_mangle]
pub extern "C" fn fx_process_exit(retcode: i64) -> fx_status_t {
    FX_OK
}

#[no_mangle]
pub extern "C" fn fx_object_wait_async(
    handle: fx_handle_t,
    port_handle: fx_handle_t,
    key: u64,
    signals: fx_signals_t,
    options: u32,
) -> fx_status_t {
    FX_OK
}
