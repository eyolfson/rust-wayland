// Copyright 2014 Jonathan Eyolfson

use libc::{c_int, c_void, uint32_t};

use raw;
use raw::types::listeners;
use raw::types::objects;

pub const WL_DATA_DEVICE_START_DRAG: uint32_t = 0;
pub const WL_DATA_DEVICE_SET_SELECTION: uint32_t = 1;
pub const WL_DATA_DEVICE_RELEASE: uint32_t = 2;

#[inline(always)]
pub unsafe fn wl_data_device_add_listener(
    wl_data_device: *mut objects::wl_data_device,
    listener: *const listeners::wl_data_device_listener,
    data: *mut c_void
) -> c_int {
    raw::wl_proxy_add_listener(
        wl_data_device as *mut objects::wl_proxy,
        listener as *mut extern fn(),
        data
    )
}

#[inline(always)]
pub unsafe fn wl_data_device_set_user_data(
    wl_data_device: *mut objects::wl_data_device,
    user_data: *mut c_void
) {
    raw::wl_proxy_set_user_data(
        wl_data_device as *mut objects::wl_proxy,
        user_data
    )
}

#[inline(always)]
pub unsafe fn wl_data_device_get_user_data(
    wl_data_device: *mut objects::wl_data_device
) -> *mut c_void {
    raw::wl_proxy_get_user_data(wl_data_device as *mut objects::wl_proxy)
}

#[inline(always)]
pub unsafe fn wl_data_device_destroy(wl_data_device: *mut objects::wl_data_device) {
    raw::wl_proxy_destroy(wl_data_device as *mut objects::wl_proxy)
}

#[inline(always)]
pub unsafe fn wl_data_device_start_drag(
    wl_data_device: *mut objects::wl_data_device,
    source: *mut objects::wl_data_source,
    origin: *mut objects::wl_surface,
    icon: *mut objects::wl_surface,
    serial: uint32_t
) {
    raw::wl_proxy_marshal(
        wl_data_device as *mut objects::wl_proxy,
        WL_DATA_DEVICE_START_DRAG,
        source,
        origin,
        icon,
        serial
    )
}

#[inline(always)]
pub unsafe fn wl_data_device_set_selection(
    wl_data_device: *mut objects::wl_data_device,
    source: *mut objects::wl_data_source,
    serial: uint32_t
) {
    raw::wl_proxy_marshal(
        wl_data_device as *mut objects::wl_proxy,
        WL_DATA_DEVICE_SET_SELECTION,
        source,
        serial
    )
}

#[inline(always)]
pub unsafe fn wl_data_device_release(
    wl_data_device: *mut objects::wl_data_device
) {
    raw::wl_proxy_marshal(
        wl_data_device as *mut objects::wl_proxy,
        WL_DATA_DEVICE_RELEASE
    );
    raw::wl_proxy_destroy(wl_data_device as *mut objects::wl_proxy)
}
