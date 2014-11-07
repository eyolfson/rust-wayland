// Copyright 2014 Jonathan Eyolfson

use libc::{c_int, c_void, int32_t, uint32_t};

use raw;
use raw::types::objects;
use raw::types::listeners;

pub const WL_POINTER_SET_CURSOR: uint32_t = 0;
pub const WL_POINTER_RELEASE: uint32_t = 1;

#[inline(always)]
pub unsafe fn wl_pointer_add_listener(
    wl_pointer: *mut objects::wl_pointer,
    listener: *const listeners::wl_pointer_listener,
    data: *mut c_void
) -> c_int {
    raw::wl_proxy_add_listener(
        wl_pointer as *mut objects::wl_proxy,
        listener as *mut extern fn(),
        data
    )
}

#[inline(always)]
pub unsafe fn wl_pointer_set_user_data(
    wl_pointer: *mut objects::wl_pointer,
    user_data: *mut c_void
) {
    raw::wl_proxy_set_user_data(
        wl_pointer as *mut objects::wl_proxy,
        user_data
    )
}

#[inline(always)]
pub unsafe fn wl_pointer_get_user_data(
    wl_pointer: *mut objects::wl_pointer
) -> *mut c_void {
    raw::wl_proxy_get_user_data(wl_pointer as *mut objects::wl_proxy)
}

#[inline(always)]
pub unsafe fn wl_pointer_destroy(wl_pointer: *mut objects::wl_pointer) {
    raw::wl_proxy_destroy(wl_pointer as *mut objects::wl_proxy)
}

#[inline(always)]
pub unsafe fn wl_pointer_set_cursor(
    wl_pointer: *mut objects::wl_pointer,
    serial: uint32_t,
    surface: *mut objects::wl_surface,
    hotspot_x: int32_t,
    hotspot_y: int32_t
) {
    raw::wl_proxy_marshal(
        wl_pointer as *mut objects::wl_proxy,
        WL_POINTER_SET_CURSOR,
        serial,
        surface,
        hotspot_x,
        hotspot_y
    )
}

#[inline(always)]
pub unsafe fn wl_pointer_release(
    wl_pointer: *mut objects::wl_pointer
) {
    raw::wl_proxy_marshal(
        wl_pointer as *mut objects::wl_proxy,
        WL_POINTER_RELEASE
    );
    raw::wl_proxy_destroy(wl_pointer as *mut objects::wl_proxy)
}
