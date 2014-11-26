// Copyright 2014 Jonathan Eyolfson

#![feature(asm)]

extern crate wayland;

extern crate libc;

use std::ptr;

static SYSCALL_MEMFD_CREATE: u32 = 319;
static MFD_CLOEXEC: u32 = 1;
static MFD_ALLOW_SEALING: u32 = 2;

// const WIDTH: i32 = 300;
// const HEIGHT: i32 = 200;
// const STRIDE: i32 = WIDTH * 4;

struct ShmBuffer {
    fd: i32,
    ptr: *mut libc::c_void,
    width: i32,
    height: i32,
    capacity: uint,
}

impl ShmBuffer {
    pub fn new() -> ShmBuffer {
        unsafe {
            let mut fd: i32;
            let name = b"rust-wayland-shm\x00";
            asm!("syscall"
                : "={rax}"(fd)
                 : "{rax}"(SYSCALL_MEMFD_CREATE),
                   "{rdi}"(name.as_ptr()),
                   "{rsi}"(MFD_CLOEXEC | MFD_ALLOW_SEALING)
                : "rcx", "r11", "memory"
                : "volatile"
            );
            assert!(fd >= 0);
            let width = 300i32;
            let height = 200i32;
            let capacity = width as uint
                * height as uint
                * std::mem::size_of::<u32>();
            assert!(libc::ftruncate(fd, capacity as i64) != -1);
            let ptr = libc::mmap(ptr::null_mut(), capacity as u64,
                                 libc::PROT_WRITE | libc::PROT_READ, 1, fd, 0);
            assert!(ptr != libc::MAP_FAILED);
            for i in range(0, width * height) {
                let p: *mut u32 = (ptr as *mut u32).offset(i as int);
                let x = i % width;
                let y = i / width;
                match x {
                    0...5 => std::ptr::write(&mut *p, 0x7FFF0000),
                    294...299 => std::ptr::write(&mut *p, 0x7F0000FF),
                    _ => match y {
                        0...5 => std::ptr::write(&mut *p, 0x7F00FF00),
                        194...199 => std::ptr::write(&mut *p, 0x7FFF00FF),
                        _ =>  std::ptr::write(&mut *p, 0x7F000000),
                    }
                }
            }
            ShmBuffer {
                fd: fd,
                ptr: ptr,
                width: width,
                height: height,
                capacity: capacity,
            }
        }
    }
    pub fn resize(&mut self, width: i32, height: i32) {
        if (width * height) > (self.width * self.height) {
            println!("TODO: actual");
        }
        self.width = width;
        self.height = height;
    }
    pub fn fd(&self) -> i32 {
        self.fd
    }
    pub fn capacity(&self) -> uint { self.capacity }
    pub fn width(&self) -> i32 { self.width }
    pub fn height(&self) -> i32 { self.height }
    pub fn stride(&self) -> i32 {
        self.width * std::mem::size_of::<u32>() as i32
    }
}

impl Drop for ShmBuffer {
    fn drop(&mut self) {
        unsafe {
            libc::munmap(self.ptr, self.capacity as u64);
            libc::close(self.fd);
        }
    }
}

fn main() {
    let mut display = wayland::Display::connect_to_env_or_default();
    let mut registry = wayland::Registry::new(&mut display);
    let mut shm_buffer = ShmBuffer::new();
    shm_buffer.resize(300, 200);
    let mut pool = registry.shm().create_pool(shm_buffer.fd(),
                                              shm_buffer.capacity() as i32);
    let mut surface = registry.compositor().create_surface();
    let mut buffer = pool.create_buffer(
        0, shm_buffer.width(), shm_buffer.height(), shm_buffer.stride(),
        wayland::raw::WL_SHM_FORMAT_ARGB8888
    );
    let mut shell_surface = registry.shell().get_shell_surface(&mut surface);
    shell_surface.set_toplevel();
    surface.attach(&mut buffer, 0, 0);
    surface.commit();
    loop {
        display.dispatch();
    }
}
