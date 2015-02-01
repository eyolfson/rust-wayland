// Copyright 2014-2015 Jonathan Eyolfson

use raw;

use super::Buffer;

pub struct Surface {
    ptr: *mut raw::wl_surface
}

impl Surface {
    pub unsafe fn from_ptr(ptr: *mut raw::wl_surface) -> Surface {
        Surface { ptr: ptr }
    }
    pub unsafe fn to_ptr(&mut self) -> *mut raw::wl_surface {
        self.ptr
    }
    pub fn attach(&mut self, buffer: &mut Buffer, x: i32, y: i32) {
        unsafe {
            raw::wl_surface_attach(self.ptr, buffer.to_ptr(), x, y);
        }
    }
    pub fn commit(&mut self) {
        unsafe {
            raw::wl_surface_commit(self.ptr);
        }
    }
}

impl Drop for Surface {
    fn drop(&mut self) {
        unsafe {
            raw::wl_surface_destroy(self.ptr)
        }
    }
}
