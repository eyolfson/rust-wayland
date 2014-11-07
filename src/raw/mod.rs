// Copyright 2014 Jonathan Eyolfson

pub use raw::protocol::wl_buffer::*;
pub use raw::protocol::wl_callback::*;
pub use raw::protocol::wl_compositor::*;
pub use raw::protocol::wl_data_device::*;
pub use raw::protocol::wl_data_device_manager::*;
pub use raw::protocol::wl_data_offer::*;
pub use raw::protocol::wl_data_source::*;
pub use raw::protocol::wl_display::*;
pub use raw::protocol::wl_keyboard::*;
pub use raw::protocol::wl_output::*;
pub use raw::protocol::wl_pointer::*;
pub use raw::protocol::wl_region::*;
pub use raw::protocol::wl_registry::*;
pub use raw::protocol::wl_seat::*;
pub use raw::protocol::wl_shell::*;
pub use raw::protocol::wl_shell_surface::*;
pub use raw::protocol::wl_shm::*;
pub use raw::protocol::wl_shm_pool::*;
pub use raw::protocol::wl_subcompositor::*;
pub use raw::protocol::wl_subsurface::*;
pub use raw::protocol::wl_surface::*;
pub use raw::protocol::wl_touch::*;
pub use raw::types::enums::*;
pub use raw::types::listeners::*;
pub use raw::types::objects::*;
pub use raw::types::utils::*;

use libc::{c_char, c_int, c_void, size_t, uint32_t};

pub mod protocol;
pub mod types;

#[link(name = "wayland-client")]
extern {
    pub static wl_buffer_interface: wl_interface;
    pub static wl_callback_interface: wl_interface;
    pub static wl_compositor_interface: wl_interface;
    pub static wl_data_device_interface: wl_interface;
    pub static wl_data_device_manager_interface: wl_interface;
    pub static wl_data_offer_interface: wl_interface;
    pub static wl_data_source_interface: wl_interface;
    pub static wl_display_interface: wl_interface;
    pub static wl_keyboard_interface: wl_interface;
    pub static wl_output_interface: wl_interface;
    pub static wl_pointer_interface: wl_interface;
    pub static wl_region_interface: wl_interface;
    pub static wl_registry_interface: wl_interface;
    pub static wl_seat_interface: wl_interface;
    pub static wl_shell_interface: wl_interface;
    pub static wl_shell_surface_interface: wl_interface;
    pub static wl_shm_interface: wl_interface;
    pub static wl_shm_pool_interface: wl_interface;
    pub static wl_subcompositor_interface: wl_interface;
    pub static wl_subsurface_interface: wl_interface;
    pub static wl_surface_interface: wl_interface;
    pub static wl_touch_interface: wl_interface;

    pub fn wl_event_queue_destroy(queue: *mut wl_event_queue);

    pub fn wl_proxy_marshal(p: *mut wl_proxy, opcode: uint32_t, ...);
    pub fn wl_proxy_marshal_array(p: *mut wl_proxy,
                                  opcode: uint32_t,
                                  args: *mut wl_argument);
    pub fn wl_proxy_create(factroy: *mut wl_proxy,
                           interface: *const wl_interface) -> *mut wl_proxy;
    pub fn wl_proxy_marshal_constructor(proxy: *mut wl_proxy,
                                        opcode: uint32_t,
                                        interface: *const wl_interface,
                                        ...) -> *mut wl_proxy;
    pub fn wl_proxy_marshal_array_constructor(
        proxy: *mut wl_proxy,
        opcode: uint32_t,
        args: *mut wl_argument,
        interface: *const wl_interface) -> *mut wl_proxy;
    pub fn wl_proxy_destroy(proxy: *mut wl_proxy);
    pub fn wl_proxy_add_listener(proxy: *mut wl_proxy,
                                 implementation: *mut extern fn(),
                                 data: *mut c_void) -> c_int;
    pub fn wl_proxy_get_listener(proxy: *mut wl_proxy) -> *const c_void;
    pub fn wl_proxy_add_dispatcher(proxy: *mut wl_proxy,
                                   dispatcher_func: wl_dispatcher_func_t,
                                   dispatcher_data: *const c_void,
                                   data: *mut c_void) -> c_int;
    pub fn wl_proxy_set_user_data(proxy: *mut wl_proxy, user_data: *mut c_void);
    pub fn wl_proxy_get_user_data(proxy: *mut wl_proxy) -> *mut c_void;
    pub fn wl_proxy_get_id(proxy: *mut wl_proxy) -> uint32_t;
    pub fn wl_proxy_get_class(proxy: *mut wl_proxy) -> *const c_char;
    pub fn wl_proxy_set_queue(proxy: *mut wl_proxy, queue: *mut wl_event_queue);

    pub fn wl_display_connect(name: *const c_char) -> *mut wl_display;
    pub fn wl_display_connect_to_fd(fd: c_int) -> *mut wl_display;
    pub fn wl_display_disconnect(display: *mut wl_display);
    pub fn wl_display_get_fd(display: *mut wl_display) -> c_int;
    pub fn wl_display_dispatch(display: *mut wl_display) -> c_int;
    pub fn wl_display_dispatch_queue(display: *mut wl_display,
                                     queue: *mut wl_event_queue) -> c_int;
    pub fn wl_display_dispatch_queue_pending(
        display: *mut wl_display,
        queue: *mut wl_event_queue) -> c_int;
    pub fn wl_display_dispatch_pending(display: *mut wl_display) -> c_int;
    pub fn wl_display_get_error(display: *mut wl_display) -> c_int;
    pub fn wl_display_get_protocol_error(display: *mut wl_display,
                                         interface: *mut *const wl_interface,
                                         id: *mut uint32_t) -> uint32_t;
    pub fn wl_display_flush(display: *mut wl_display) -> c_int;
    pub fn wl_display_roundtrip_queue(display: *mut wl_display,
                                      queue: *mut wl_event_queue) -> c_int;
    pub fn wl_display_roundtrip(display: *mut wl_display) -> c_int;
    pub fn wl_display_create_queue(
        display: *mut wl_display) -> *mut wl_event_queue;
    pub fn wl_display_prepare_read_queue(display: *mut wl_display,
                                         queue: *mut wl_event_queue) -> c_int;
    pub fn wl_display_prepare_read(display: *mut wl_display) -> c_int;
    pub fn wl_display_cancel_read(display: *mut wl_display);
    pub fn wl_display_read_events(display: *mut wl_display) -> c_int;

    pub fn wl_log_set_handler_client(handler: wl_log_func_t);
    
    pub fn wl_list_init(list: *mut wl_list);
    pub fn wl_list_insert(list: *mut wl_list, elm: *mut wl_list);
    pub fn wl_list_remove(elm: *mut wl_list);
    pub fn wl_list_length(elm: *const wl_list) -> c_int;
    pub fn wl_list_empty(elm: *const wl_list) -> c_int;
    pub fn wl_list_insert_list(list: *mut wl_list, other: *mut wl_list);

    pub fn wl_array_init(array: *mut wl_array);
    pub fn wl_array_release(array: *mut wl_array);
    pub fn wl_array_add(array: *mut wl_array, size: size_t) -> *mut c_void;
    pub fn wl_array_copy(array: *mut wl_array, source: *mut wl_array) -> c_int;
}
