// Copyright 2014 Jonathan Eyolfson

use libc::{c_char, c_void, int32_t, uint32_t};

use raw;
use raw::types::objects;
use raw::types::utils;

#[repr(C)]
pub struct wl_buffer_listener {
    pub release: extern fn(
        data: *mut c_void,
        wl_buffer: *mut objects::wl_buffer
    )
}

#[repr(C)]
pub struct wl_callback_listener {
    pub done: extern fn(
        data: *mut c_void,
        wl_callback: *mut objects::wl_callback,
        callback_data: uint32_t
    )
}

#[repr(C)]
pub struct wl_data_device_listener {
    pub data_offer: extern fn(
        data: *mut c_void,
        wl_data_device: *mut objects::wl_data_device,
        id: *mut objects::wl_data_offer
    ),
    pub enter: extern fn(
        data: *mut c_void,
        wl_data_device: *mut objects::wl_data_device,
        serial: uint32_t,
        surface: *mut objects::wl_surface,
        x: raw::wl_fixed_t,
        y: raw::wl_fixed_t,
        id: *mut objects::wl_data_offer
    ),
    pub leave: extern fn(
        data: *mut c_void,
        wl_data_device: *mut objects::wl_data_device
    ),
    pub motion: extern fn(
        data: *mut c_void,
        wl_data_device: *mut objects::wl_data_device,
        time: uint32_t,
        x: raw::wl_fixed_t,
        y: raw::wl_fixed_t
    ),
    pub drop: extern fn(
        data: *mut c_void,
        wl_data_device: *mut objects::wl_data_device
    ),
    pub selection: extern fn(
        data: *mut c_void,
        wl_data_device: *mut objects::wl_data_device,
        id: *mut objects::wl_data_offer
    )
}

#[repr(C)]
pub struct wl_data_offer_listener {
    pub offer: extern fn(
        data: *mut c_void,
        wl_data_offer: *mut objects::wl_data_offer,
        mime_type: *const c_char
    )
}

#[repr(C)]
pub struct wl_data_source_listener {
    pub target: extern fn(
        data: *mut c_void,
        wl_data_source: *mut objects::wl_data_source,
        mime_type: *const c_char
    ),
    pub send: extern fn(
        data: *mut c_void,
        wl_data_source: *mut objects::wl_data_source,
        mime_type: *const c_char,
        fd: int32_t
    ),
    pub cancelled: extern fn(
        data: *mut c_void,
        wl_data_source: *mut objects::wl_data_source
    )
}

#[repr(C)]
pub struct wl_display_listener {
    pub error: extern fn(
        data: *mut c_void,
        wl_display: *mut objects::wl_display,
        object_id: *mut c_void,
        code: uint32_t,
        message: *const c_char
    ),
    pub delete_id: extern fn(
        data: *mut c_void,
        wl_display: *mut objects::wl_display,
        id: uint32_t
    )
}

#[repr(C)]
pub struct wl_keyboard_listener {
    pub keymap: extern fn(
        data: *mut c_void,
        wl_keyboard: *mut objects::wl_keyboard,
        format: uint32_t,
        fd: int32_t,
        size: uint32_t
    ),
    pub enter: extern fn(
        data: *mut c_void,
        wl_keyboard: *mut objects::wl_keyboard,
        serial: uint32_t,
        surface: *mut objects::wl_surface,
        keys: *mut utils::wl_array
    ),
    pub leave: extern fn(
        data: *mut c_void,
        wl_keyboard: *mut objects::wl_keyboard,
        serial: uint32_t,
        surface: *mut objects::wl_surface
    ),
    pub key: extern fn(
        data: *mut c_void,
        wl_keyboard: *mut objects::wl_keyboard,
        serial: uint32_t,
        time: uint32_t,
        key: uint32_t,
        state: uint32_t
    ),
    pub modifiers: extern fn(
        data: *mut c_void,
        wl_keyboard: *mut objects::wl_keyboard,
        serial: uint32_t,
        mods_depressed: uint32_t,
        mods_latched: uint32_t,
        mods_locked: uint32_t,
        group: uint32_t
    ),
    pub repeat_info: extern fn(
        data: *mut c_void,
        wl_keyboard: *mut objects::wl_keyboard,
        rate: int32_t,
        delay: int32_t
    )
}

#[repr(C)]
pub struct wl_output_listener {
    pub geometry: extern fn(
        data: *mut c_void,
        wl_output: *mut objects::wl_output,
        x: int32_t,
        y: int32_t,
        physical_width: int32_t,
        physical_height: int32_t,
        subpixel: int32_t,
        make: *const c_char,
        model: *const c_char,
        transform: int32_t
    ),
    pub mode: extern fn(
        data: *mut c_void,
        wl_output: *mut objects::wl_output,
        flags: uint32_t,
        width: int32_t,
        height: int32_t,
        refresh: int32_t
    ),
    pub done: extern fn(
        data: *mut c_void,
        wl_output: *mut objects::wl_output
    ),
    pub scale: extern fn(
        data: *mut c_void,
        wl_output: *mut objects::wl_output,
        factor: int32_t
    )
}

#[repr(C)]
pub struct wl_pointer_listener {
    pub enter: extern fn(
        data: *mut c_void,
        wl_pointer: *mut objects::wl_pointer,
        serial: uint32_t,
        surface: *mut objects::wl_surface,
        surface_x: raw::wl_fixed_t,
        surface_y: raw::wl_fixed_t
    ),
    pub leave: extern fn(
        data: *mut c_void,
        wl_pointer: *mut objects::wl_pointer,
        serial: uint32_t,
        surface: *mut objects::wl_surface
    ),
    pub motion: extern fn(
        data: *mut c_void,
        wl_pointer: *mut objects::wl_pointer,
        time: uint32_t,
        surface_x: raw::wl_fixed_t,
        surface_y: raw::wl_fixed_t
    ),
    pub button: extern fn(
        data: *mut c_void,
        wl_pointer: *mut objects::wl_pointer,
        serial: uint32_t,
        time: uint32_t,
        button: uint32_t,
        state: uint32_t
    ),
    pub axis: extern fn(
        data: *mut c_void,
        wl_pointer: *mut objects::wl_pointer,
        time: uint32_t,
        axis: uint32_t,
        value: raw::wl_fixed_t
    )
}

#[repr(C)]
pub struct wl_registry_listener {
    pub global: extern fn(
        data: *mut c_void,
        wl_registry: *mut objects::wl_registry,
        name: uint32_t,
        interface: *const c_char,
        version: uint32_t
    ),
    pub global_remove: extern fn(
        data: *mut c_void,
        wl_registry: *mut objects::wl_registry,
        name: uint32_t
    )
}

#[repr(C)]
pub struct wl_seat_listener {
    pub capabilities: extern fn(
        data: *mut c_void,
        wl_seat: *mut objects::wl_seat,
        capabilities: uint32_t
    ),
    pub name: extern fn(
        data: *mut c_void,
        wl_seat: *mut objects::wl_seat,
        name: *const c_char
    )
}

#[repr(C)]
pub struct wl_shell_surface_listener {
    pub ping: extern fn(
        data: *mut c_void,
        wl_shell_surface: *mut objects::wl_shell_surface,
        serial: uint32_t
    ),
    pub configure: extern fn(
        data: *mut c_void,
        wl_shell_surface: *mut objects::wl_shell_surface,
        edges: uint32_t,
        width: int32_t,
        height: int32_t
    ),
    pub popup_done: extern fn(
        data: *mut c_void,
        wl_shell_surface: *mut objects::wl_shell_surface,
    )   
}

#[repr(C)]
pub struct wl_shm_listener {
    pub format: extern fn(
        data: *mut c_void,
        wl_shm: *mut objects::wl_shm,
        format: uint32_t
    )
}

#[repr(C)]
pub struct wl_surface_listener {
    pub enter: extern fn(
        data: *mut c_void,
        wl_surface: *mut objects::wl_surface,
        output: *mut objects::wl_output
    ),
    pub leave: extern fn(
        data: *mut c_void,
        wl_surface: *mut objects::wl_surface,
        output: *mut objects::wl_output
    )
}

#[repr(C)]
pub struct wl_touch_listener {
    pub down: extern fn(
        data: *mut c_void,
        wl_touch: *mut objects::wl_touch,
        serial: uint32_t,
        time: uint32_t,
        surface: *mut objects::wl_surface,
        id: int32_t,
        x: raw::wl_fixed_t,
        y: raw::wl_fixed_t
    ),
    pub up: extern fn(
        data: *mut c_void,
        wl_touch: *mut objects::wl_touch,
        serial: uint32_t,
        time: uint32_t,
        id: int32_t
    ),
    pub motion: extern fn(
        data: *mut c_void,
        wl_touch: *mut objects::wl_touch,
        time: uint32_t,
        id: int32_t,
        x: raw::wl_fixed_t,
        y: raw::wl_fixed_t
    ),
    pub frame: extern fn(
        data: *mut c_void,
        wl_touch: *mut objects::wl_touch
    ),
    pub cancel: extern fn(
        data: *mut c_void,
        wl_touch: *mut objects::wl_touch
    )
}
