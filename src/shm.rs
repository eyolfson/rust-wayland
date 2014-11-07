// Copyright 2014 Jonathan Eyolfson

use raw;

use ShmPool;

pub struct Shm {
    ptr: *mut raw::wl_shm
}

impl Shm {
    pub unsafe fn from_ptr(ptr: *mut raw::wl_shm) -> Shm {
        Shm { ptr: ptr }
    }
    pub fn create_pool(&mut self, fd: i32, size: i32) -> ShmPool {
        unsafe {
            let ptr = raw::wl_shm_create_pool(self.ptr, fd, size);
            ShmPool::from_ptr(ptr)
        }
    }
}

impl Drop for Shm {
    fn drop(&mut self) {
        unsafe {
            raw::wl_shm_destroy(self.ptr)
        }
    }
}
