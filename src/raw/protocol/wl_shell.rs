// Copyright 2014 Jonathan Eyolfson

use std::ptr;

use libc::{c_void, uint32_t};

use raw;
use raw::types::objects;

pub const WL_SHELL_GET_SHELL_SURFACE: uint32_t = 0;

#[inline(always)]
pub unsafe fn wl_shell_set_user_data(
    wl_shell: *mut objects::wl_shell,
    user_data: *mut c_void
) {
    raw::wl_proxy_set_user_data(
        wl_shell as *mut objects::wl_proxy,
        user_data
    )
}

#[inline(always)]
pub unsafe fn wl_shell_get_user_data(
    wl_shell: *mut objects::wl_shell
) -> *mut c_void {
    raw::wl_proxy_get_user_data(wl_shell as *mut objects::wl_proxy)
}

#[inline(always)]
pub unsafe fn wl_shell_destroy(wl_shell: *mut objects::wl_shell) {
    raw::wl_proxy_destroy(wl_shell as *mut objects::wl_proxy)
}

#[inline(always)]
pub unsafe fn wl_shell_get_shell_surface(
    wl_shell: *mut objects::wl_shell,
    surface: *mut objects::wl_surface
) -> *mut objects::wl_shell_surface {
    let id = raw::wl_proxy_marshal_constructor(
        wl_shell as *mut objects::wl_proxy,
        WL_SHELL_GET_SHELL_SURFACE,
        &raw::wl_shell_surface_interface,
        ptr::null_mut::<c_void>(),
        surface
    );
    id as *mut objects::wl_shell_surface
}
