// Copyright 2014-2015 Jonathan Eyolfson

use raw;

use super::Surface;

pub struct Compositor {
    ptr: *mut raw::wl_compositor
}

impl Compositor {
    pub unsafe fn from_ptr(ptr: *mut raw::wl_compositor) -> Compositor {
        Compositor { ptr: ptr }
    }
    pub fn create_surface(&mut self) -> Surface {
        unsafe {
            let ptr = raw::wl_compositor_create_surface(self.ptr);
            Surface::from_ptr(ptr)
        }
    }
}

impl Drop for Compositor {
    fn drop(&mut self) {
        unsafe {
            raw::wl_compositor_destroy(self.ptr);
        }
    }
}
