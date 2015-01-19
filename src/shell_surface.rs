// Copyright 2014 Jonathan Eyolfson

use std::ffi;
use std::ptr;

use libc::{c_void, int32_t, uint32_t};

use raw;

#[allow(unused_variables)]
extern fn ping(
    data: *mut c_void,
    wl_shell_surface: *mut raw::wl_shell_surface,
    serial: uint32_t
) {
    println!("wl_shell_surface.ping serial = {}", serial);
    unsafe {
        raw::wl_shell_surface_pong(wl_shell_surface, serial);
    }
}

#[allow(unused_variables)]
extern fn configure(
    data: *mut c_void,
    wl_shell_surface: *mut raw::wl_shell_surface,
    edges: uint32_t,
    width: int32_t,
    height: int32_t
) {
    println!("wl_shell_surface.configure: {}, {}, {}", edges, width, height);
}

#[allow(unused_variables)]
extern fn popup_done(
    data: *mut c_void,
    wl_shell_surface: *mut raw::wl_shell_surface,
) {
    println!("wl_shell_surface.popup_done");
}

static LISTENER: raw::wl_shell_surface_listener =
    raw::wl_shell_surface_listener {
        ping: ping,
        configure: configure,
        popup_done: popup_done
    };

pub struct ShellSurface {
    ptr: *mut raw::wl_shell_surface
}

impl ShellSurface {
    pub unsafe fn from_ptr(ptr: *mut raw::wl_shell_surface) -> ShellSurface {
        raw::wl_shell_surface_add_listener(
            ptr,
            &LISTENER,
            ptr::null_mut()
        );
        ShellSurface { ptr: ptr }
    }
    pub fn set_title(&mut self, title: &str) {
        let c_str = ffi::CString::from_slice(title.as_bytes());
        unsafe {
            raw::wl_shell_surface_set_title(self.ptr, c_str.as_ptr());
        }
    }
    pub fn set_toplevel(&mut self) {
        unsafe {
            raw::wl_shell_surface_set_toplevel(self.ptr);
        }
    }
}

impl Drop for ShellSurface {
    fn drop(&mut self) {
        unsafe {
            raw::wl_shell_surface_destroy(self.ptr)
        }
    }
}
