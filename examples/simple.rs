// Copyright 2014-2015 Jonathan Eyolfson

#![allow(dead_code)]

#![feature(asm)]
#![feature(core)]
#![feature(libc)]

extern crate libc;
extern crate wayland;

use std::ptr;
use wayland::old::*;

const SYSCALL_MEMFD_CREATE: u32 = 319;

const MFD_CLOEXEC: u32 = 1;
const MFD_ALLOW_SEALING: u32 = 2;

const F_SEAL_SEAL: u32 = 1;
const F_SEAL_SHRINK: u32 = 2;
const F_SEAL_GROW: u32 = 4;
const F_SEAL_WRITE: u32 = 8;

const F_LINUX_SPECIFIC_BASE: u32 = 1024;
const F_ADD_SEALS: u32 = F_LINUX_SPECIFIC_BASE + 9;
const F_GET_SEALS: u32 = F_LINUX_SPECIFIC_BASE + 10;

const MAP_SHARED: i32 = 1;

#[derive(Show)]
struct LinuxError {
    pub desc: &'static str,
}

struct MemFd {
    pub index: i32
}

impl MemFd {
    pub fn create(name: &[u8], flags: u32) -> Result<MemFd, LinuxError> {
        let mut index: i32;
        unsafe {
            asm!("syscall"
                : "={rax}"(index)
                : "{rax}"(SYSCALL_MEMFD_CREATE), "{rdi}"(name.as_ptr()), "{rsi}"(flags)
                : "rcx", "r11", "memory"
                : "volatile"
            );
        }
        if index >= 0 {
            Ok(MemFd{index: index})
        }
        else {
            Err(LinuxError{desc: "Function not implemented"})
        }
    }
}

struct ShmBuffer {
    fd: i32,
    ptr: *mut libc::c_void,
    width: i32,
    height: i32,
    capacity: usize,
}

impl ShmBuffer {
    pub fn create(width: i32, height: i32) -> ShmBuffer {
        unsafe {
            let name = b"rust-wayland-shm\x00";
            let fd = MemFd::create(name, MFD_CLOEXEC | MFD_ALLOW_SEALING)
                     .unwrap();
            let capacity = width as usize
                * height as usize
                * std::mem::size_of::<u32>();
            assert!(libc::ftruncate(fd.index, capacity as i64) != -1);
            let ptr = libc::mmap(ptr::null_mut(),
                                 capacity as u64,
                                 libc::PROT_WRITE | libc::PROT_READ,
                                 MAP_SHARED,
                                 fd.index,
                                 0);
            assert!(ptr != libc::MAP_FAILED);
            for i in range(0, width * height) {
                let p: *mut u32 = (ptr as *mut u32).offset(i as isize);
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
                fd: fd.index,
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
    pub fn capacity(&self) -> usize { self.capacity }
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
    let mut display = Display::connect_to_env_or_default();
    let mut registry = Registry::new(&mut display);
    // Create the shell surface
    let mut surface = registry.compositor().create_surface();
    let mut shell_surface = registry.shell().get_shell_surface(&mut surface);
    shell_surface.set_toplevel();
    // Create the buffer
    let mut shm_buffer = ShmBuffer::create(300, 200);
    shm_buffer.resize(300, 200);
    let mut pool = registry.shm().create_pool(shm_buffer.fd(),
                                              shm_buffer.capacity() as i32);
    let mut buffer = pool.create_buffer(
        0, shm_buffer.width(), shm_buffer.height(), shm_buffer.stride(),
        wayland::raw::WL_SHM_FORMAT_ARGB8888
    );
    surface.attach(&mut buffer, 0, 0);
    surface.commit();
    loop {
        display.dispatch();
    }
}
