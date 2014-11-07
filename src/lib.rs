// Copyright 2014 Jonathan Eyolfson

#![feature(globs)]

extern crate libc;

pub use buffer::Buffer;
pub use compositor::Compositor;
pub use display::Display;
pub use list::List;
pub use region::Region;
pub use registry::Registry;
pub use seat::Seat;
pub use shell::Shell;
pub use shell_surface::ShellSurface;
pub use shm::Shm;
pub use shm_pool::ShmPool;
pub use surface::Surface;

mod buffer;
mod compositor;
mod display;
mod list;
mod region;
mod registry;
mod seat;
mod shell;
mod shell_surface;
mod shm;
mod shm_pool;
mod surface;

pub mod raw;
