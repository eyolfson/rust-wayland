// Copyright 2014 Jonathan Eyolfson

use std::ptr;

use libc::{c_void, int32_t, uint32_t};

use raw;
use raw::types::objects;

pub const WL_SHM_POOL_CREATE_BUFFER: uint32_t = 0;
pub const WL_SHM_POOL_DESTROY: uint32_t = 1;
pub const WL_SHM_POOL_RESIZE: uint32_t = 2;

#[inline(always)]
pub unsafe fn wl_shm_pool_set_user_data(
    wl_shm_pool: *mut objects::wl_shm_pool,
    user_data: *mut c_void
) {
    raw::wl_proxy_set_user_data(
        wl_shm_pool as *mut objects::wl_proxy,
        user_data
    )
}

#[inline(always)]
pub unsafe fn wl_shm_pool_get_user_data(
    wl_shm_pool: *mut objects::wl_shm_pool
) -> *mut c_void {
    raw::wl_proxy_get_user_data(wl_shm_pool as *mut objects::wl_proxy)
}

#[inline(always)]
pub unsafe fn wl_shm_pool_create_buffer(
    wl_shm_pool: *mut objects::wl_shm_pool,
    offset: int32_t,
    width: int32_t,
    height: int32_t,
    stride: int32_t,
    format: uint32_t
) -> *mut objects::wl_buffer {
    let id = raw::wl_proxy_marshal_constructor(
        wl_shm_pool as *mut objects::wl_proxy,
        WL_SHM_POOL_CREATE_BUFFER,
        &raw::wl_buffer_interface,
        ptr::null_mut::<c_void>(),
        offset,
        width,
        height,
        stride,
        format
    );
    id as *mut objects::wl_buffer
}

#[inline(always)]
pub unsafe fn wl_shm_pool_destroy(
    wl_shm_pool: *mut objects::wl_shm_pool
) {
    raw::wl_proxy_marshal(
        wl_shm_pool as *mut objects::wl_proxy,
        WL_SHM_POOL_DESTROY
    );
    raw::wl_proxy_destroy(wl_shm_pool as *mut objects::wl_proxy)
}

#[inline(always)]
pub unsafe fn wl_shm_pool_resize(
    wl_shm_pool: *mut objects::wl_shm_pool,
    size: int32_t
) {
    raw::wl_proxy_marshal(
        wl_shm_pool as *mut objects::wl_proxy,
        WL_SHM_POOL_RESIZE,
        size
    )
}
