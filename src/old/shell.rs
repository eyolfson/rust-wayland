// Copyright 2014-2015 Jonathan Eyolfson

use raw;

use super::ShellSurface;
use super::Surface;

pub struct Shell {
    ptr: *mut raw::wl_shell
}

impl Shell {
    pub unsafe fn from_ptr(ptr: *mut raw::wl_shell) -> Shell {
        Shell { ptr: ptr }
    }
    pub fn get_shell_surface(&mut self, surface: &mut Surface) -> ShellSurface {
        unsafe {
            let ptr = raw::wl_shell_get_shell_surface(
                self.ptr,
                surface.to_ptr()
            );
            ShellSurface::from_ptr(ptr)
        }
    }
}

impl Drop for Shell {
    fn drop(&mut self) {
        unsafe {
            raw::wl_shell_destroy(self.ptr)
        }
    }
}
