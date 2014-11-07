// Copyright 2014 Jonathan Eyolfson

use std::ptr;

use libc::{c_int, c_void, uint32_t};

use raw;
use raw::types::objects;
use raw::types::listeners;

pub const WL_DISPLAY_SYNC: uint32_t = 0;
pub const WL_DISPLAY_GET_REGISTRY: uint32_t = 1;

#[inline(always)]
pub unsafe fn wl_display_add_listener(
    wl_display: *mut objects::wl_display,
    listener: *const listeners::wl_display_listener,
    data: *mut c_void
) -> c_int {
    raw::wl_proxy_add_listener(
        wl_display as *mut objects::wl_proxy,
        listener as *mut extern fn(),
        data
    )
}

#[inline(always)]
pub unsafe fn wl_display_set_user_data(
    wl_display: *mut objects::wl_display,
    user_data: *mut c_void
) {
    raw::wl_proxy_set_user_data(
        wl_display as *mut objects::wl_proxy,
        user_data
    )
}

#[inline(always)]
pub unsafe fn wl_display_get_user_data(
    wl_display: *mut objects::wl_display
) -> *mut c_void {
    raw::wl_proxy_get_user_data(wl_display as *mut objects::wl_proxy)
}

#[inline(always)]
pub unsafe fn wl_display_sync(
    wl_display: *mut objects::wl_display
) -> *mut objects::wl_callback {
    let callback = raw::wl_proxy_marshal_constructor(
        wl_display as *mut objects::wl_proxy,
        WL_DISPLAY_SYNC,
        &raw::wl_callback_interface,
        ptr::null_mut::<c_void>()
    );
    callback as *mut objects::wl_callback
}

#[inline(always)]
pub unsafe fn wl_display_get_registry(
    wl_display: *mut objects::wl_display
) -> *mut objects::wl_registry {
    let registry = raw::wl_proxy_marshal_constructor(
        wl_display as *mut objects::wl_proxy,
        WL_DISPLAY_GET_REGISTRY,
        &raw::wl_registry_interface,
        ptr::null_mut::<c_void>()
    );
    registry as *mut objects::wl_registry
}
