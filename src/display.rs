// Copyright 2014 Jonathan Eyolfson

use std::ptr;

use raw;

pub struct Display {
    ptr: *mut raw::wl_display
}

impl Display {
    pub fn connect_to_env_or_default() -> Display {
        unsafe {
            let ptr = raw::wl_display_connect(ptr::null());
            assert!(!ptr.is_null(), "wl_display_connect failed");
            Display { ptr: ptr }
        }
    }
    pub fn connect(name: &str) -> Display {
        let c_str = name.to_c_str();
        unsafe {
            let ptr = raw::wl_display_connect(c_str.as_ptr());
            assert!(!ptr.is_null(), "wl_display_connect failed");
            Display { ptr: ptr }
        }
    }
    pub fn roundtrip(&mut self) -> i32 {
        unsafe {
            let r = raw::wl_display_roundtrip(self.ptr);
            assert!(r != -1, "wl_display_roundtrip failed");
            r
        }
    }
    pub fn read_events(&mut self) {
        unsafe {
            let r = raw::wl_display_read_events(self.ptr);
            assert!(r != -1, "wl_display_read_events failed");
        }
    }
    pub fn prepare_read(&mut self) {
        unsafe {
            let r = raw::wl_display_prepare_read(self.ptr);
            assert!(r != -1, "wl_display_prepare_read failed");
        }
    }
    pub fn cancel_read(&mut self) {
        unsafe {
            raw::wl_display_cancel_read(self.ptr);
        }
    }
    pub fn dispatch(&mut self) -> i32 {
        unsafe {
            let r = raw::wl_display_dispatch(self.ptr);
            assert!(r != -1, "wl_display_dispatch failed");
            r
        }
    }
    pub fn flush(&mut self) -> i32 {
        unsafe {
            let r = raw::wl_display_flush(self.ptr);
            assert!(r != -1, "wl_display_flush failed");
            r
        }
    }
    pub unsafe fn to_ptr(&mut self) -> *mut raw::wl_display { self.ptr }
}

impl Drop for Display {
    fn drop(&mut self) {
        unsafe {
            raw::wl_display_disconnect(self.ptr);
        }
    }
}
