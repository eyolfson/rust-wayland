// Copyright 2014-2015 Jonathan Eyolfson

use raw;

use super::Buffer;

pub struct ShmPool {
    ptr: *mut raw::wl_shm_pool
}

impl ShmPool {
    pub unsafe fn from_ptr(ptr: *mut raw::wl_shm_pool) -> ShmPool {
        ShmPool { ptr: ptr }
    }
    pub fn create_buffer(
        &mut self,
        offset: i32,
        width: i32,
        height: i32,
        stride: i32,
        format: u32
    ) -> Buffer {
        unsafe {
            let ptr = raw::wl_shm_pool_create_buffer(
                self.ptr, offset, width, height, stride, format);
            Buffer::from_ptr(ptr)
        }
    }
}

impl Drop for ShmPool {
    fn drop(&mut self) {
        unsafe {
            raw::wl_shm_pool_destroy(self.ptr)
        }
    }
}
