// Copyright 2014 Jonathan Eyolfson

use raw;

pub struct Region {
    ptr: *mut raw::wl_region
}

impl Region {
    pub unsafe fn from_ptr(ptr: *mut raw::wl_region) -> Region {
        Region { ptr: ptr }
    }
}

impl Drop for Region {
    fn drop(&mut self) {
        unsafe {
            raw::wl_region_destroy(self.ptr)
        }
    }
}
