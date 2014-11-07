// Copyright 2014 Jonathan Eyolfson

use std::ptr;

use raw;

pub struct Display {
    ptr: *mut raw::wl_display
}

impl Display {
    pub fn new() -> Display {
        unsafe {
            let ptr = raw::wl_display_connect(ptr::null());
            assert!(!ptr.is_null());
            Display { ptr: ptr }
        }
    }
    pub unsafe fn to_ptr(&mut self) -> *mut raw::wl_display { self.ptr }
    pub fn dispatch(&mut self) {
        unsafe {
            let ret = raw::wl_display_dispatch(self.ptr);
            assert!(ret >= 0);
        }
    }
    pub fn roundtrip(&mut self) {
        unsafe {
            let ret = raw::wl_display_roundtrip(self.ptr);
            assert!(ret >= 0);
        }
    }
    pub fn flush(&mut self) {
        unsafe {
            let ret = raw::wl_display_flush(self.ptr);
            assert!(ret >= 0);
        }
    }
}

impl Drop for Display {
    fn drop(&mut self) {
        unsafe {
            raw::wl_display_disconnect(self.ptr);
        }
    }
}
