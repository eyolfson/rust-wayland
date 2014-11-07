// Copyright 2014 Jonathan Eyolfson

use libc::{c_int, c_void, uint32_t};

use raw;
use raw::types::objects;
use raw::types::listeners;

pub const WL_KEYBOARD_RELEASE: uint32_t = 0;

#[inline(always)]
pub unsafe fn wl_keyboard_add_listener(
    wl_keyboard: *mut objects::wl_keyboard,
    listener: *const listeners::wl_keyboard_listener,
    data: *mut c_void
) -> c_int {
    raw::wl_proxy_add_listener(
        wl_keyboard as *mut objects::wl_proxy,
        listener as *mut extern fn(),
        data
    )
}

#[inline(always)]
pub unsafe fn wl_keyboard_set_user_data(
    wl_keyboard: *mut objects::wl_keyboard,
    user_data: *mut c_void
) {
    raw::wl_proxy_set_user_data(
        wl_keyboard as *mut objects::wl_proxy,
        user_data
    )
}

#[inline(always)]
pub unsafe fn wl_keyboard_get_user_data(
    wl_keyboard: *mut objects::wl_keyboard
) -> *mut c_void {
    raw::wl_proxy_get_user_data(wl_keyboard as *mut objects::wl_proxy)
}

#[inline(always)]
pub unsafe fn wl_keyboard_destroy(wl_keyboard: *mut objects::wl_keyboard) {
    raw::wl_proxy_destroy(wl_keyboard as *mut objects::wl_proxy)
}

#[inline(always)]
pub unsafe fn wl_keyboard_release(
    wl_keyboard: *mut objects::wl_keyboard
) {
    raw::wl_proxy_marshal(
        wl_keyboard as *mut objects::wl_proxy,
        WL_KEYBOARD_RELEASE
    );
    raw::wl_proxy_destroy(wl_keyboard as *mut objects::wl_proxy)
}
