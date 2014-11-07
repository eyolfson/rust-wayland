// Copyright 2014 Jonathan Eyolfson

use std::ptr;

use libc::{c_void, uint32_t};

use raw;
use raw::types::objects;

pub const WL_DATA_DEVICE_MANAGER_CREATE_DATA_SOURCE: uint32_t = 0;
pub const WL_DATA_DEVICE_MANAGER_GET_DATA_DEVICE: uint32_t = 1;

#[inline(always)]
pub unsafe fn wl_data_device_manager_set_user_data(
    wl_data_device_manager: *mut objects::wl_data_device_manager,
    user_data: *mut c_void
) {
    raw::wl_proxy_set_user_data(
        wl_data_device_manager as *mut objects::wl_proxy,
        user_data
    )
}

#[inline(always)]
pub unsafe fn wl_data_device_manager_get_user_data(
    wl_data_device_manager: *mut objects::wl_data_device_manager
) -> *mut c_void {
    raw::wl_proxy_get_user_data(wl_data_device_manager as *mut objects::wl_proxy)
}

#[inline(always)]
pub unsafe fn wl_data_device_manager_destroy(wl_data_device_manager: *mut objects::wl_data_device_manager) {
    raw::wl_proxy_destroy(wl_data_device_manager as *mut objects::wl_proxy)
}

#[inline(always)]
pub unsafe fn wl_data_device_manager_create_data_source(
    wl_data_device_manager: *mut objects::wl_data_device_manager
) -> *mut objects::wl_data_source {
    let id = raw::wl_proxy_marshal_constructor(
        wl_data_device_manager as *mut objects::wl_proxy,
        WL_DATA_DEVICE_MANAGER_CREATE_DATA_SOURCE,
        &raw::wl_data_source_interface,
        ptr::null_mut::<c_void>()
    );
    id as *mut objects::wl_data_source
}

#[inline(always)]
pub unsafe fn wl_data_device_manager_get_data_device(
    wl_data_device_manager: *mut objects::wl_data_device_manager,
    seat: *mut objects::wl_seat
) -> *mut objects::wl_data_device {
    let id = raw::wl_proxy_marshal_constructor(
        wl_data_device_manager as *mut objects::wl_proxy,
        WL_DATA_DEVICE_MANAGER_GET_DATA_DEVICE,
        &raw::wl_data_device_interface,
        ptr::null_mut::<c_void>(),
        seat
    );
    id as *mut objects::wl_data_device
}
