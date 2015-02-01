// Copyright 2014-2015 Jonathan Eyolfson

use std::ptr;

use raw;

pub struct List {
    link: raw::wl_list
}

impl List {
    pub fn new() -> List {
        unsafe {
            let mut l = List {
                link: raw::wl_list {
                    prev: ptr::null_mut(),
                    next: ptr::null_mut()
                }
            };
            raw::wl_list_init(&mut l.link);
            l
        }
    }
}
