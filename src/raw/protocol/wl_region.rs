// Copyright 2014 Jonathan Eyolfson

use libc::{c_void, int32_t, uint32_t};

use raw;
use raw::types::objects;

pub const WL_REGION_DESTROY: uint32_t = 0;
pub const WL_REGION_ADD: uint32_t = 1;
pub const WL_REGION_SUBTRACT: uint32_t = 2;

#[inline(always)]
pub unsafe fn wl_region_set_user_data(
    wl_region: *mut objects::wl_region,
    user_data: *mut c_void
) {
    raw::wl_proxy_set_user_data(
        wl_region as *mut objects::wl_proxy,
        user_data
    )
}

#[inline(always)]
pub unsafe fn wl_region_get_user_data(
    wl_region: *mut objects::wl_region
) -> *mut c_void {
    raw::wl_proxy_get_user_data(wl_region as *mut objects::wl_proxy)
}

#[inline(always)]
pub unsafe fn wl_region_destroy(
    wl_region: *mut objects::wl_region
) {
    raw::wl_proxy_marshal(
        wl_region as *mut objects::wl_proxy,
        WL_REGION_DESTROY
    );
    raw::wl_proxy_destroy(wl_region as *mut objects::wl_proxy)
}

#[inline(always)]
pub unsafe fn wl_region_add(
    wl_region: *mut objects::wl_region,
    x: int32_t,
    y: int32_t,
    width: int32_t,
    height: int32_t
) {
    raw::wl_proxy_marshal(
        wl_region as *mut objects::wl_proxy,
        WL_REGION_ADD,
        x,
        y,
        width,
        height
    )
}

#[inline(always)]
pub unsafe fn wl_region_subtract(
    wl_region: *mut objects::wl_region,
    x: int32_t,
    y: int32_t,
    width: int32_t,
    height: int32_t
) {
    raw::wl_proxy_marshal(
        wl_region as *mut objects::wl_proxy,
        WL_REGION_SUBTRACT,
        x,
        y,
        width,
        height
    )
}
