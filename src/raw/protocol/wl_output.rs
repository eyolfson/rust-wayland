// Copyright 2014 Jonathan Eyolfson

use libc::{c_int, c_void};

use raw;
use raw::types::objects;
use raw::types::listeners;

#[inline(always)]
pub unsafe fn wl_output_add_listener(
    wl_output: *mut objects::wl_output,
    listener: *const listeners::wl_output_listener,
    data: *mut c_void
) -> c_int {
    raw::wl_proxy_add_listener(
        wl_output as *mut objects::wl_proxy,
        listener as *mut extern fn(),
        data
    )
}

#[inline(always)]
pub unsafe fn wl_output_set_user_data(
    wl_output: *mut objects::wl_output,
    user_data: *mut c_void
) {
    raw::wl_proxy_set_user_data(
        wl_output as *mut objects::wl_proxy,
        user_data
    )
}

#[inline(always)]
pub unsafe fn wl_output_get_user_data(
    wl_output: *mut objects::wl_output
) -> *mut c_void {
    raw::wl_proxy_get_user_data(wl_output as *mut objects::wl_proxy)
}

#[inline(always)]
pub unsafe fn wl_output_destroy(wl_output: *mut objects::wl_output) {
    raw::wl_proxy_destroy(wl_output as *mut objects::wl_proxy)
}
