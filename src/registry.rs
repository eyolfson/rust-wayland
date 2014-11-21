// Copyright 2014 Jonathan Eyolfson

use std::c_str;
use std::mem;

use libc::{c_char, c_void, strcmp, uint32_t};

use raw;

use Compositor;
use Display;
use Seat;
use Shell;
use Shm;

pub struct Registry {
    ptr: *mut raw::wl_registry,
    compositor: Option<Compositor>,
    seat: Option<Seat>,
    shell: Option<Shell>,
    shm: Option<Shm>,
}

#[allow(unused_variables)]
extern fn global(
    data: *mut c_void,
    registry: *mut raw::wl_registry,
    name: uint32_t,
    interface: *const c_char,
    version: uint32_t
) {
    unsafe {
        let r: &mut Registry = mem::transmute(data);
        if strcmp(interface, raw::wl_compositor_interface.name) == 0 {
            let ptr = raw::wl_registry_bind(
                registry,
                name,
                & raw::wl_compositor_interface,
                version
            );
            let compositor = Compositor::from_ptr(
                ptr as *mut raw::wl_compositor
            );
            r.compositor = Some(compositor);
        }
        else if strcmp(interface, raw::wl_seat_interface.name) == 0 {
            let ptr = raw::wl_registry_bind(
                registry,
                name,
                & raw::wl_seat_interface,
                version
            );
            let seat = Seat::from_ptr(
                ptr as *mut raw::wl_seat
            );
            r.seat = Some(seat);
        }
        else if strcmp(interface, raw::wl_shell_interface.name) == 0 {
            let ptr = raw::wl_registry_bind(
                registry,
                name,
                & raw::wl_shell_interface,
                version
            );
            let shell = Shell::from_ptr(
                ptr as *mut raw::wl_shell
            );
            r.shell = Some(shell);
        }
        else if strcmp(interface, raw::wl_shm_interface.name) == 0 {
            let ptr = raw::wl_registry_bind(
                registry,
                name,
                & raw::wl_shm_interface,
                version
            );
            let shm = Shm::from_ptr(
                ptr as *mut raw::wl_shm
            );
            r.shm = Some(shm);
        }
        let interface_c_str = c_str::CString::new(interface, false);
        let interface_str = interface_c_str.as_str().unwrap();
        println!("wl_registry.global name={} interface={} version={}",
                 name, interface_str, version);
    }
}

#[allow(unused_variables)]
extern fn global_remove(
    data: *mut c_void,
    registry: *mut raw::wl_registry,
    name: uint32_t
) {
    println!("wl_registry.global_remove name = {}", name);
    unimplemented!();
}

static LISTENER: raw::wl_registry_listener = raw::wl_registry_listener {
    global: global,
    global_remove: global_remove
};

impl Registry {
    pub fn new(display: &mut Display) -> Registry {
        unsafe {
            let ptr = raw::wl_display_get_registry(display.to_ptr());
            let mut r = Registry {
                ptr: ptr,
                compositor: None,
                seat: None,
                shell: None,
                shm: None,
            };
            raw::wl_registry_add_listener(
                ptr,
                &LISTENER,
                mem::transmute(&mut r)
            );
            display.roundtrip();
            r
        }
    }
    pub fn compositor(&mut self) -> &mut Compositor {
        match self.compositor {
            Some(ref mut c) => c,
            None => panic!("compositor not set"),
        }
    }
    pub fn seat(&mut self) -> &mut Seat {
        match self.seat {
            Some(ref mut s) => s,
            None => panic!("seat not set"),
        }
    }
    pub fn shell(&mut self) -> &mut Shell {
        match self.shell {
            Some(ref mut s) => s,
            None => panic!("shell not set"),
        }
    }
    pub fn shm(&mut self) -> &mut Shm {
        match self.shm {
            Some(ref mut s) => s,
            None => panic!("shell not set"),
        }
    }
}

impl Drop for Registry {
    fn drop(&mut self) {
        unsafe {
            raw::wl_registry_destroy(self.ptr);
        }
    }
}
