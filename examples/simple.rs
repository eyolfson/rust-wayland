// Copyright 2014 Jonathan Eyolfson

extern crate wayland;

extern crate libc;

use std::ptr;

#[link(name = "rt")]
extern { }

const WIDTH: i32 = 300;
const HEIGHT: i32 = 200;
const STRIDE: i32 = WIDTH * 4;
const PIXELS: i32 = WIDTH * HEIGHT;
const SIZE: i32 = PIXELS * 4;

struct ShmFd {
    fd: i32,
    ptr: *mut libc::c_void
}

impl ShmFd {
    pub fn new() -> ShmFd {
        unsafe {
            let fd = libc::funcs::posix88::mman::shm_open(
                "/eyl-hello-world".to_c_str().as_ptr(),
                libc::O_CREAT | libc::O_RDWR,
                libc::S_IRUSR | libc::S_IWUSR
            );
            assert!(fd >= 0);
            assert!(libc::ftruncate(fd, SIZE as i64) != -1);
            let ptr = libc::mmap(ptr::null_mut(), SIZE as u64,
                                 libc::PROT_WRITE | libc::PROT_READ, 1, fd, 0);
            assert!(ptr != libc::MAP_FAILED);
            for i in range(0, PIXELS) {
                let p: *mut u32 = (ptr as *mut u32).offset(i as int);
                let x = i % WIDTH;
                let y = i / WIDTH;
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
            ShmFd { fd: fd, ptr: ptr }
        }
    }
    pub fn fd(&self) -> i32 {
        self.fd
    }
}

impl Drop for ShmFd {
    fn drop(&mut self) {
        unsafe {
            libc::munmap(self.ptr, SIZE as u64);
            libc::funcs::posix88::mman::shm_unlink(
                "/eyl-hello-world".to_c_str().as_ptr()
            );
        }
    }
}

fn main() {
    let mut display = wayland::Display::connect_to_env_or_default();
    let mut registry = wayland::Registry::new(&mut display);
    let shm_fd = ShmFd::new();
    let mut pool = registry.shm().create_pool(shm_fd.fd(), SIZE);
    let mut surface = registry.compositor().create_surface();
    let mut buffer = pool.create_buffer(
        0, WIDTH, HEIGHT, STRIDE, wayland::raw::WL_SHM_FORMAT_ARGB8888
    );
    let mut shell_surface = registry.shell().get_shell_surface(&mut surface);
    shell_surface.set_toplevel();
    surface.attach(&mut buffer, 0, 0);
    surface.commit();
    loop {
        display.dispatch();
    }
}
