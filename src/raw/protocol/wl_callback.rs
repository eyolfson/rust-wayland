// Copyright 2014 Jonathan Eyolfson

use libc::{c_int, c_void};

use raw;
use raw::types::objects;
use raw::types::listeners;

#[inline(always)]
pub unsafe fn wl_callback_add_listener(
    wl_callback: *mut objects::wl_callback,
    listener: *const listeners::wl_callback_listener,
    data: *mut c_void
) -> c_int {
    raw::wl_proxy_add_listener(
        wl_callback as *mut objects::wl_proxy,
        listener as *mut extern fn(),
        data
    )
}

#[inline(always)]
pub unsafe fn wl_callback_set_user_data(
    wl_callback: *mut objects::wl_callback,
    user_data: *mut c_void
) {
    raw::wl_proxy_set_user_data(
        wl_callback as *mut objects::wl_proxy,
        user_data
    )
}

#[inline(always)]
pub unsafe fn wl_callback_get_user_data(
    wl_callback: *mut objects::wl_callback
) -> *mut c_void {
    raw::wl_proxy_get_user_data(wl_callback as *mut objects::wl_proxy)
}

#[inline(always)]
pub unsafe fn wl_callback_destroy(wl_callback: *mut objects::wl_callback) {
    raw::wl_proxy_destroy(wl_callback as *mut objects::wl_proxy)
}
