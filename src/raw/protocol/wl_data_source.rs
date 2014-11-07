// Copyright 2014 Jonathan Eyolfson

use libc::{c_char, c_int, c_void, uint32_t};

use raw;
use raw::types::listeners;
use raw::types::objects;

pub const WL_DATA_SOURCE_OFFER: uint32_t = 0;
pub const WL_DATA_SOURCE_DESTROY: uint32_t = 1;

#[inline(always)]
pub unsafe fn wl_data_source_add_listener(
    wl_data_source: *mut objects::wl_data_source,
    listener: *const listeners::wl_data_source_listener,
    data: *mut c_void
) -> c_int {
    raw::wl_proxy_add_listener(
        wl_data_source as *mut objects::wl_proxy,
        listener as *mut extern fn(),
        data
    )
}

#[inline(always)]
pub unsafe fn wl_data_source_set_user_data(
    wl_data_source: *mut objects::wl_data_source,
    user_data: *mut c_void
) {
    raw::wl_proxy_set_user_data(
        wl_data_source as *mut objects::wl_proxy,
        user_data
    )
}

#[inline(always)]
pub unsafe fn wl_data_source_get_user_data(
    wl_data_source: *mut objects::wl_data_source
) -> *mut c_void {
    raw::wl_proxy_get_user_data(wl_data_source as *mut objects::wl_proxy)
}

#[inline(always)]
pub unsafe fn wl_data_source_offer(
    wl_data_source: *mut objects::wl_data_source,
    mime_type: *const c_char
) {
    raw::wl_proxy_marshal(
        wl_data_source as *mut objects::wl_proxy,
        WL_DATA_SOURCE_OFFER,
        mime_type
    )
}

#[inline(always)]
pub unsafe fn wl_data_source_destroy(
    wl_data_source: *mut objects::wl_data_source
) {
    raw::wl_proxy_marshal(
        wl_data_source as *mut objects::wl_proxy,
        WL_DATA_SOURCE_DESTROY
    );
    raw::wl_proxy_destroy(wl_data_source as *mut objects::wl_proxy)
}
