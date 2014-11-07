// Copyright 2014 Jonathan Eyolfson

use std::ptr;

use libc::{c_void, uint32_t};

use raw;
use raw::types::objects;

pub const WL_SUBCOMPOSITOR_DESTROY: uint32_t = 0;
pub const WL_SUBCOMPOSITOR_GET_SUBSURFACE: uint32_t = 1;

#[inline(always)]
pub unsafe fn wl_subcompositor_set_user_data(
    wl_subcompositor: *mut objects::wl_subcompositor,
    user_data: *mut c_void
) {
    raw::wl_proxy_set_user_data(
        wl_subcompositor as *mut objects::wl_proxy,
        user_data
    )
}

#[inline(always)]
pub unsafe fn wl_subcompositor_get_user_data(
    wl_subcompositor: *mut objects::wl_subcompositor
) -> *mut c_void {
    raw::wl_proxy_get_user_data(wl_subcompositor as *mut objects::wl_proxy)
}

#[inline(always)]
pub unsafe fn wl_subcompositor_destroy(
    wl_subcompositor: *mut objects::wl_subcompositor
) {
    raw::wl_proxy_marshal(
        wl_subcompositor as *mut objects::wl_proxy,
        WL_SUBCOMPOSITOR_DESTROY
    );
    raw::wl_proxy_destroy(wl_subcompositor as *mut objects::wl_proxy)
}

#[inline(always)]
pub unsafe fn wl_subcompositor_get_subsurface(
    wl_subcompositor: *mut objects::wl_subcompositor,
    surface: *mut objects::wl_surface,
    parent: *mut objects::wl_surface
) -> *mut objects::wl_subsurface {
    let id = raw::wl_proxy_marshal_constructor(
        wl_subcompositor as *mut objects::wl_proxy,
        WL_SUBCOMPOSITOR_GET_SUBSURFACE,
        &raw::wl_subsurface_interface,
        ptr::null_mut::<c_void>(),
        surface,
        parent
    );
    id as *mut objects::wl_subsurface
}
