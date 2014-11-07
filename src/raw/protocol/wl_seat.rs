// Copyright 2014 Jonathan Eyolfson

use std::ptr;

use libc::{c_int, c_void, uint32_t};

use raw;
use raw::types::objects;
use raw::types::listeners;

pub const WL_SEAT_GET_POINTER: uint32_t = 0;
pub const WL_SEAT_GET_KEYBOARD: uint32_t = 1;
pub const WL_SEAT_GET_TOUCH: uint32_t = 2;

#[inline(always)]
pub unsafe fn wl_seat_add_listener(
    wl_seat: *mut objects::wl_seat,
    listener: *const listeners::wl_seat_listener,
    data: *mut c_void
) -> c_int {
    raw::wl_proxy_add_listener(
        wl_seat as *mut objects::wl_proxy,
        listener as *mut extern fn(),
        data
    )
}

#[inline(always)]
pub unsafe fn wl_seat_set_user_data(
    wl_seat: *mut objects::wl_seat,
    user_data: *mut c_void
) {
    raw::wl_proxy_set_user_data(
        wl_seat as *mut objects::wl_proxy,
        user_data
    )
}

#[inline(always)]
pub unsafe fn wl_seat_get_user_data(
    wl_seat: *mut objects::wl_seat
) -> *mut c_void {
    raw::wl_proxy_get_user_data(wl_seat as *mut objects::wl_proxy)
}

#[inline(always)]
pub unsafe fn wl_seat_destroy(wl_seat: *mut objects::wl_seat) {
    raw::wl_proxy_destroy(wl_seat as *mut objects::wl_proxy)
}

#[inline(always)]
pub unsafe fn wl_seat_get_pointer(
    wl_seat: *mut objects::wl_seat
) -> *mut objects::wl_pointer {
    let id = raw::wl_proxy_marshal_constructor(
        wl_seat as *mut objects::wl_proxy,
        WL_SEAT_GET_POINTER,
        &raw::wl_pointer_interface,
        ptr::null_mut::<c_void>()
    );
    id as *mut objects::wl_pointer
}

#[inline(always)]
pub unsafe fn wl_seat_get_keyboard(
    wl_seat: *mut objects::wl_seat
) -> *mut objects::wl_keyboard {
    let id = raw::wl_proxy_marshal_constructor(
        wl_seat as *mut objects::wl_proxy,
        WL_SEAT_GET_KEYBOARD,
        &raw::wl_keyboard_interface,
        ptr::null_mut::<c_void>()
    );
    id as *mut objects::wl_keyboard
}

#[inline(always)]
pub unsafe fn wl_seat_get_touch(
    wl_seat: *mut objects::wl_seat
) -> *mut objects::wl_touch {
    let id = raw::wl_proxy_marshal_constructor(
        wl_seat as *mut objects::wl_proxy,
        WL_SEAT_GET_TOUCH,
        &raw::wl_touch_interface,
        ptr::null_mut::<c_void>()
    );
    id as *mut objects::wl_touch
}
