// Copyright 2014 Jonathan Eyolfson

use raw;

pub struct Buffer {
    ptr: *mut raw::wl_buffer
}

impl Buffer {
    pub unsafe fn from_ptr(ptr: *mut raw::wl_buffer) -> Buffer {
        Buffer { ptr: ptr }
    }
    pub unsafe fn to_ptr(&mut self) -> *mut raw::wl_buffer {
        self.ptr
    }
}

impl Drop for Buffer {
    fn drop(&mut self) {
        unsafe {
            raw::wl_buffer_destroy(self.ptr)
        }
    }
}
