// Copyright 2014 Jonathan Eyolfson

use libc::{c_void, int32_t, uint32_t};

use raw;
use raw::types::objects;

pub const WL_SUBSURFACE_DESTROY: uint32_t = 0;
pub const WL_SUBSURFACE_SET_POSITION: uint32_t = 1;
pub const WL_SUBSURFACE_PLACE_ABOVE: uint32_t = 2;
pub const WL_SUBSURFACE_PLACE_BELOW: uint32_t = 3;
pub const WL_SUBSURFACE_SET_SYNC: uint32_t = 4;
pub const WL_SUBSURFACE_SET_DESYNC: uint32_t = 5;

#[inline(always)]
pub unsafe fn wl_subsurface_set_user_data(
    wl_subsurface: *mut objects::wl_subsurface,
    user_data: *mut c_void
) {
    raw::wl_proxy_set_user_data(
        wl_subsurface as *mut objects::wl_proxy,
        user_data
    )
}

#[inline(always)]
pub unsafe fn wl_subsurface_get_user_data(
    wl_subsurface: *mut objects::wl_subsurface
) -> *mut c_void {
    raw::wl_proxy_get_user_data(wl_subsurface as *mut objects::wl_proxy)
}

#[inline(always)]
pub unsafe fn wl_subsurface_destroy(
    wl_subsurface: *mut objects::wl_subsurface
) {
    raw::wl_proxy_marshal(
        wl_subsurface as *mut objects::wl_proxy,
        WL_SUBSURFACE_DESTROY
    );
    raw::wl_proxy_destroy(wl_subsurface as *mut objects::wl_proxy)
}

#[inline(always)]
pub unsafe fn wl_subsurface_set_position(
    wl_subsurface: *mut objects::wl_subsurface,
    x: int32_t,
    y: int32_t
) {
    raw::wl_proxy_marshal(
        wl_subsurface as *mut objects::wl_proxy,
        WL_SUBSURFACE_SET_POSITION,
        x,
        y
    )
}

#[inline(always)]
pub unsafe fn wl_subsurface_place_above(
    wl_subsurface: *mut objects::wl_subsurface,
    sibling: *mut objects::wl_surface
) {
    raw::wl_proxy_marshal(
        wl_subsurface as *mut objects::wl_proxy,
        WL_SUBSURFACE_PLACE_ABOVE,
        sibling
    )
}

#[inline(always)]
pub unsafe fn wl_subsurface_place_below(
    wl_subsurface: *mut objects::wl_subsurface,
    sibling: *mut objects::wl_surface
) {
    raw::wl_proxy_marshal(
        wl_subsurface as *mut objects::wl_proxy,
        WL_SUBSURFACE_PLACE_BELOW,
        sibling
    )
}

#[inline(always)]
pub unsafe fn wl_subsurface_set_sync(
    wl_subsurface: *mut objects::wl_subsurface
) {
    raw::wl_proxy_marshal(
        wl_subsurface as *mut objects::wl_proxy,
        WL_SUBSURFACE_SET_SYNC
    )
}

#[inline(always)]
pub unsafe fn wl_subsurface_set_desync(
    wl_subsurface: *mut objects::wl_subsurface
) {
    raw::wl_proxy_marshal(
        wl_subsurface as *mut objects::wl_proxy,
        WL_SUBSURFACE_SET_DESYNC
    )
}
