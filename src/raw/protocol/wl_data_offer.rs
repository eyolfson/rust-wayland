// Copyright 2014 Jonathan Eyolfson

use libc::{c_char, c_int, c_void, int32_t, uint32_t};

use raw;
use raw::types::listeners;
use raw::types::objects;

pub const WL_DATA_OFFER_ACCEPT: uint32_t = 0;
pub const WL_DATA_OFFER_RECEIVE: uint32_t = 1;
pub const WL_DATA_OFFER_DESTROY: uint32_t = 2;

#[inline(always)]
pub unsafe fn wl_data_offer_add_listener(
    wl_data_offer: *mut objects::wl_data_offer,
    listener: *const listeners::wl_data_offer_listener,
    data: *mut c_void
) -> c_int {
    raw::wl_proxy_add_listener(
        wl_data_offer as *mut objects::wl_proxy,
        listener as *mut extern fn(),
        data
    )
}

#[inline(always)]
pub unsafe fn wl_data_offer_set_user_data(
    wl_data_offer: *mut objects::wl_data_offer,
    user_data: *mut c_void
) {
    raw::wl_proxy_set_user_data(
        wl_data_offer as *mut objects::wl_proxy,
        user_data
    )
}

#[inline(always)]
pub unsafe fn wl_data_offer_get_user_data(
    wl_data_offer: *mut objects::wl_data_offer
) -> *mut c_void {
    raw::wl_proxy_get_user_data(wl_data_offer as *mut objects::wl_proxy)
}

#[inline(always)]
pub unsafe fn wl_data_offer_accept(
    wl_data_offer: *mut objects::wl_data_offer,
    serial: uint32_t,
    mime_type: *const c_char
) {
    raw::wl_proxy_marshal(
        wl_data_offer as *mut objects::wl_proxy,
        WL_DATA_OFFER_ACCEPT,
        serial,
        mime_type
    )
}

#[inline(always)]
pub unsafe fn wl_data_offer_receive(
    wl_data_offer: *mut objects::wl_data_offer,
    mime_type: *const c_char,
    fd: int32_t
) {
    raw::wl_proxy_marshal(
        wl_data_offer as *mut objects::wl_proxy,
        WL_DATA_OFFER_RECEIVE,
        mime_type,
        fd
    )
}

#[inline(always)]
pub unsafe fn wl_data_offer_destroy(
    wl_data_offer: *mut objects::wl_data_offer
) {
    raw::wl_proxy_marshal(
        wl_data_offer as *mut objects::wl_proxy,
        WL_DATA_OFFER_DESTROY
    );
    raw::wl_proxy_destroy(wl_data_offer as *mut objects::wl_proxy)
}
