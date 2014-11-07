// Copyright 2014 Jonathan Eyolfson

use libc::{c_int, c_void, uint32_t};

use raw;
use raw::types::listeners;
use raw::types::objects;

pub const WL_BUFFER_DESTROY: uint32_t = 0;

#[inline(always)]
pub unsafe fn wl_buffer_add_listener(
    wl_buffer: *mut objects::wl_buffer,
    listener: *const listeners::wl_buffer_listener,
    data: *mut c_void
) -> c_int {
    raw::wl_proxy_add_listener(
        wl_buffer as *mut objects::wl_proxy,
        listener as *mut extern fn(),
        data
    )
}

#[inline(always)]
pub unsafe fn wl_buffer_set_user_data(
    wl_buffer: *mut objects::wl_buffer,
    user_data: *mut c_void
) {
    raw::wl_proxy_set_user_data(
        wl_buffer as *mut objects::wl_proxy,
        user_data
    )
}

#[inline(always)]
pub unsafe fn wl_buffer_get_user_data(
    wl_buffer: *mut objects::wl_buffer
) -> *mut c_void {
    raw::wl_proxy_get_user_data(wl_buffer as *mut objects::wl_proxy)
}

#[inline(always)]
pub unsafe fn wl_buffer_destroy(
    wl_buffer: *mut objects::wl_buffer
) {
    raw::wl_proxy_marshal(
        wl_buffer as *mut objects::wl_proxy,
        WL_BUFFER_DESTROY
    );
    raw::wl_proxy_destroy(wl_buffer as *mut objects::wl_proxy)
}
