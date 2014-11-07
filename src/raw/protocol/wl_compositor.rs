// Copyright 2014 Jonathan Eyolfson

use std::ptr;

use libc::{c_void, uint32_t};

use raw;
use raw::types::objects;

pub const WL_COMPOSITOR_CREATE_SURFACE: uint32_t = 0;
pub const WL_COMPOSITOR_CREATE_REGION: uint32_t = 1;

#[inline(always)]
pub unsafe fn wl_compositor_set_user_data(
    wl_compositor: *mut objects::wl_compositor,
    user_data: *mut c_void
) {
    raw::wl_proxy_set_user_data(
        wl_compositor as *mut objects::wl_proxy,
        user_data
    )
}

#[inline(always)]
pub unsafe fn wl_compositor_get_user_data(
    wl_compositor: *mut objects::wl_compositor
) -> *mut c_void {
    raw::wl_proxy_get_user_data(wl_compositor as *mut objects::wl_proxy)
}

#[inline(always)]
pub unsafe fn wl_compositor_destroy(wl_compositor: *mut objects::wl_compositor) {
    raw::wl_proxy_destroy(wl_compositor as *mut objects::wl_proxy)
}

#[inline(always)]
pub unsafe fn wl_compositor_create_surface(
    wl_compositor: *mut objects::wl_compositor
) -> *mut objects::wl_surface {
    let id = raw::wl_proxy_marshal_constructor(
        wl_compositor as *mut objects::wl_proxy,
        WL_COMPOSITOR_CREATE_SURFACE,
        &raw::wl_surface_interface,
        ptr::null_mut::<c_void>()
    );
    id as *mut objects::wl_surface
}

#[inline(always)]
pub unsafe fn wl_compositor_create_region(
    wl_compositor: *mut objects::wl_compositor
) -> *mut objects::wl_region {
    let id = raw::wl_proxy_marshal_constructor(
        wl_compositor as *mut objects::wl_proxy,
        WL_COMPOSITOR_CREATE_REGION,
        &raw::wl_region_interface,
        ptr::null_mut::<c_void>()
    );
    id as *mut objects::wl_region
}
