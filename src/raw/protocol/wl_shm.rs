// Copyright 2014 Jonathan Eyolfson

use std::ptr;

use libc::{c_int, c_void, int32_t, uint32_t};

use raw;
use raw::types::objects;
use raw::types::listeners;

pub const WL_SHM_CREATE_POOL: uint32_t = 0;

#[inline(always)]
pub unsafe fn wl_shm_add_listener(
    wl_shm: *mut objects::wl_shm,
    listener: *const listeners::wl_shm_listener,
    data: *mut c_void
) -> c_int {
    raw::wl_proxy_add_listener(
        wl_shm as *mut objects::wl_proxy,
        listener as *mut extern fn(),
        data
    )
}

#[inline(always)]
pub unsafe fn wl_shm_set_user_data(
    wl_shm: *mut objects::wl_shm,
    user_data: *mut c_void
) {
    raw::wl_proxy_set_user_data(
        wl_shm as *mut objects::wl_proxy,
        user_data
    )
}

#[inline(always)]
pub unsafe fn wl_shm_get_user_data(
    wl_shm: *mut objects::wl_shm
) -> *mut c_void {
    raw::wl_proxy_get_user_data(wl_shm as *mut objects::wl_proxy)
}

#[inline(always)]
pub unsafe fn wl_shm_destroy(wl_shm: *mut objects::wl_shm) {
    raw::wl_proxy_destroy(wl_shm as *mut objects::wl_proxy)
}

#[inline(always)]
pub unsafe fn wl_shm_create_pool(
    wl_shm: *mut objects::wl_shm,
    fd: int32_t,
    size: int32_t
) -> *mut objects::wl_shm_pool {
    let id = raw::wl_proxy_marshal_constructor(
        wl_shm as *mut objects::wl_proxy,
        WL_SHM_CREATE_POOL,
        &raw::wl_shm_pool_interface,
        ptr::null_mut::<c_void>(),
        fd,
        size
    );
    id as *mut objects::wl_shm_pool
}
