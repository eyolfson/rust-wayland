// Copyright 2014 Jonathan Eyolfson

use libc::{c_char, c_int, c_void, int32_t, uint32_t};

use raw;
use raw::types::listeners;
use raw::types::objects;

pub const WL_SHELL_SURFACE_PONG: uint32_t = 0;
pub const WL_SHELL_SURFACE_MOVE: uint32_t = 1;
pub const WL_SHELL_SURFACE_RESIZE: uint32_t = 2;
pub const WL_SHELL_SURFACE_SET_TOPLEVEL: uint32_t = 3;
pub const WL_SHELL_SURFACE_SET_TRANSIENT: uint32_t = 4;
pub const WL_SHELL_SURFACE_SET_FULLSCREEN: uint32_t = 5;
pub const WL_SHELL_SURFACE_SET_POPUP: uint32_t = 6;
pub const WL_SHELL_SURFACE_SET_MAXIMIZED: uint32_t = 7;
pub const WL_SHELL_SURFACE_SET_TITLE: uint32_t = 8;
pub const WL_SHELL_SURFACE_SET_CLASS: uint32_t = 9;

#[repr(C)]
pub struct wl_shell_surface_listener {
    pub ping: extern fn(
        data: *mut c_void,
        wl_shell_surface: *mut objects::wl_shell_surface,
        serial: uint32_t
    ),
    pub configure: extern fn(
        data: *mut c_void,
        wl_shell_surface: *mut objects::wl_shell_surface,
        edges: uint32_t,
        width: int32_t,
        height: int32_t
    ),
    pub popup_done: extern fn(
        data: *mut c_void,
        wl_shell_surface: *mut objects::wl_shell_surface
    )
}

#[inline(always)]
pub unsafe fn wl_shell_surface_add_listener(
    wl_shell_surface: *mut objects::wl_shell_surface,
    listener: *const listeners::wl_shell_surface_listener,
    data: *mut c_void
) -> c_int {
    raw::wl_proxy_add_listener(
        wl_shell_surface as *mut objects::wl_proxy,
        listener as *mut extern fn(),
        data
    )
}

#[inline(always)]
pub unsafe fn wl_shell_surface_set_user_data(
    wl_shell_surface: *mut objects::wl_shell_surface,
    user_data: *mut c_void
) {
    raw::wl_proxy_set_user_data(
        wl_shell_surface as *mut objects::wl_proxy,
        user_data
    )
}

#[inline(always)]
pub unsafe fn wl_shell_surface_get_user_data(
    wl_shell_surface: *mut objects::wl_shell_surface
) -> *mut c_void {
    raw::wl_proxy_get_user_data(wl_shell_surface as *mut objects::wl_proxy)
}

#[inline(always)]
pub unsafe fn wl_shell_surface_destroy(wl_shell_surface: *mut objects::wl_shell_surface) {
    raw::wl_proxy_destroy(wl_shell_surface as *mut objects::wl_proxy)
}

#[inline(always)]
pub unsafe fn wl_shell_surface_pong(
    wl_shell_surface: *mut objects::wl_shell_surface,
    serial: uint32_t
) {
    raw::wl_proxy_marshal(
        wl_shell_surface as *mut objects::wl_proxy,
        WL_SHELL_SURFACE_PONG,
        serial
    )
}

#[inline(always)]
pub unsafe fn wl_shell_surface_move(
    wl_shell_surface: *mut objects::wl_shell_surface,
    seat: *mut objects::wl_seat,
    serial: uint32_t
) {
    raw::wl_proxy_marshal(
        wl_shell_surface as *mut objects::wl_proxy,
        WL_SHELL_SURFACE_MOVE,
        seat,
        serial
    )
}

#[inline(always)]
pub unsafe fn wl_shell_surface_resize(
    wl_shell_surface: *mut objects::wl_shell_surface,
    seat: *mut objects::wl_seat,
    serial: uint32_t,
    edges: uint32_t
) {
    raw::wl_proxy_marshal(
        wl_shell_surface as *mut objects::wl_proxy,
        WL_SHELL_SURFACE_RESIZE,
        seat,
        serial,
        edges
    )
}

#[inline(always)]
pub unsafe fn wl_shell_surface_set_toplevel(
    wl_shell_surface: *mut objects::wl_shell_surface
) {
    raw::wl_proxy_marshal(
        wl_shell_surface as *mut objects::wl_proxy,
        WL_SHELL_SURFACE_SET_TOPLEVEL
    )
}

#[inline(always)]
pub unsafe fn wl_shell_surface_set_transient(
    wl_shell_surface: *mut objects::wl_shell_surface,
    parent: *mut objects::wl_surface,
    x: int32_t,
    y: int32_t,
    flags: uint32_t
) {
    raw::wl_proxy_marshal(
        wl_shell_surface as *mut objects::wl_proxy,
        WL_SHELL_SURFACE_SET_TRANSIENT,
        parent,
        x,
        y,
        flags
    )
}

#[inline(always)]
pub unsafe fn wl_shell_surface_set_fullscreen(
    wl_shell_surface: *mut objects::wl_shell_surface,
    method: uint32_t,
    framerate: uint32_t,
    output: *mut objects::wl_output
) {
    raw::wl_proxy_marshal(
        wl_shell_surface as *mut objects::wl_proxy,
        WL_SHELL_SURFACE_SET_FULLSCREEN,
        method,
        framerate,
        output
    )
}

#[inline(always)]
pub unsafe fn wl_shell_surface_set_popup(
    wl_shell_surface: *mut objects::wl_shell_surface,
    seat: *mut objects::wl_seat,
    serial: uint32_t,
    parent: *mut objects::wl_surface,
    x: int32_t,
    y: int32_t,
    flags: uint32_t
) {
    raw::wl_proxy_marshal(
        wl_shell_surface as *mut objects::wl_proxy,
        WL_SHELL_SURFACE_SET_POPUP,
        seat,
        serial,
        parent,
        x,
        y,
        flags
    )
}

#[inline(always)]
pub unsafe fn wl_shell_surface_set_maximized(
    wl_shell_surface: *mut objects::wl_shell_surface,
    output: *mut objects::wl_output
) {
    raw::wl_proxy_marshal(
        wl_shell_surface as *mut objects::wl_proxy,
        WL_SHELL_SURFACE_SET_MAXIMIZED,
        output
    )
}

#[inline(always)]
pub unsafe fn wl_shell_surface_set_title(
    wl_shell_surface: *mut objects::wl_shell_surface,
    title: *const c_char
) {
    raw::wl_proxy_marshal(
        wl_shell_surface as *mut objects::wl_proxy,
        WL_SHELL_SURFACE_SET_TITLE,
        title
    )
}

#[inline(always)]
pub unsafe fn wl_shell_surface_set_class(
    wl_shell_surface: *mut objects::wl_shell_surface,
    class_: *const c_char
) {
    raw::wl_proxy_marshal(
        wl_shell_surface as *mut objects::wl_proxy,
        WL_SHELL_SURFACE_SET_CLASS,
        class_
    )
}
