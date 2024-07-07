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

#[no_mangle]
pub extern "C" fn fx_port_create(options: u32, out: *mut fx_handle_t) -> fx_status_t {
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
