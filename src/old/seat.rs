// Copyright 2014-2015 Jonathan Eyolfson

use raw;

pub struct Seat {
    ptr: *mut raw::wl_seat
}

impl Seat {
    pub unsafe fn from_ptr(ptr: *mut raw::wl_seat) -> Seat {
        Seat { ptr: ptr }
    }
}

impl Drop for Seat {
    fn drop(&mut self) {
        unsafe {
            raw::wl_seat_destroy(self.ptr)
        }
    }
}
