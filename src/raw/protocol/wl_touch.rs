// Copyright 2014 Jonathan Eyolfson

use libc::{c_int, c_void, uint32_t};

use raw;
use raw::types::objects;
use raw::types::listeners;

pub const WL_TOUCH_RELEASE: uint32_t = 0;

#[inline(always)]
pub unsafe fn wl_touch_add_listener(
    wl_touch: *mut objects::wl_touch,
    listener: *const listeners::wl_touch_listener,
    data: *mut c_void
) -> c_int {
    raw::wl_proxy_add_listener(
        wl_touch as *mut objects::wl_proxy,
        listener as *mut extern fn(),
        data
    )
}

#[inline(always)]
pub unsafe fn wl_touch_set_user_data(
    wl_touch: *mut objects::wl_touch,
    user_data: *mut c_void
) {
    raw::wl_proxy_set_user_data(
        wl_touch as *mut objects::wl_proxy,
        user_data
    )
}

#[inline(always)]
pub unsafe fn wl_touch_get_user_data(
    wl_touch: *mut objects::wl_touch
) -> *mut c_void {
    raw::wl_proxy_get_user_data(wl_touch as *mut objects::wl_proxy)
}

#[inline(always)]
pub unsafe fn wl_touch_destroy(wl_touch: *mut objects::wl_touch) {
    raw::wl_proxy_destroy(wl_touch as *mut objects::wl_proxy)
}

#[inline(always)]
pub unsafe fn wl_touch_release(
    wl_touch: *mut objects::wl_touch
) {
    raw::wl_proxy_marshal(
        wl_touch as *mut objects::wl_proxy,
        WL_TOUCH_RELEASE
    );
    raw::wl_proxy_destroy(wl_touch as *mut objects::wl_proxy)
}
