// Copyright 2014 Jonathan Eyolfson

use libc::uint32_t;

#[repr(C)]
pub type wl_data_device_error = uint32_t;
pub const WL_DATA_DEVICE_ERROR_ROLE: wl_data_device_error = 0;

#[repr(C)]
pub type wl_display_error = uint32_t;
pub const WL_DISPLAY_ERROR_INVALID_OBJECT: wl_display_error = 0;
pub const WL_DISPLAY_ERROR_INVALID_METHOD: wl_display_error = 1;
pub const WL_DISPLAY_ERROR_NO_MEMORY: wl_display_error = 2;

#[repr(C)]
pub type wl_keyboard_key_state = uint32_t;
pub const WL_KEYBOARD_KEY_STATE_RELEASED: wl_keyboard_key_state = 0;
pub const WL_KEYBOARD_KEY_STATE_PRESSED: wl_keyboard_key_state = 1;

#[repr(C)]
pub type wl_keyboard_keymap_format = uint32_t;
pub const WL_KEYBOARD_KEYMAP_FORMAT_NO_KEYMAP: wl_keyboard_keymap_format = 0;
pub const WL_KEYBOARD_KEYMAP_FORMAT_XKB_V1: wl_keyboard_keymap_format = 1;

#[repr(C)]
pub type wl_output_mode = uint32_t;
pub const WL_OUTPUT_MODE_CURRENT: wl_output_mode = 0x1;
pub const WL_OUTPUT_MODE_PREFERRED: wl_output_mode = 0x2;

#[repr(C)]
pub type wl_output_subpixel = uint32_t;
pub const WL_OUTPUT_SUBPIXEL_UNKNOWN: wl_output_subpixel = 0;
pub const WL_OUTPUT_SUBPIXEL_NONE: wl_output_subpixel = 1;
pub const WL_OUTPUT_SUBPIXEL_HORIZONTAL_RGB: wl_output_subpixel = 2;
pub const WL_OUTPUT_SUBPIXEL_HORIZONTAL_BGR: wl_output_subpixel = 3;
pub const WL_OUTPUT_SUBPIXEL_VERTICAL_RGB: wl_output_subpixel = 4;
pub const WL_OUTPUT_SUBPIXEL_VERTICAL_BGR: wl_output_subpixel = 5;

#[repr(C)]
pub type wl_output_transform = uint32_t;
pub const WL_OUTPUT_TRANSFORM_NORMAL: wl_output_transform = 0;
pub const WL_OUTPUT_TRANSFORM_90: wl_output_transform = 1;
pub const WL_OUTPUT_TRANSFORM_180: wl_output_transform = 2;
pub const WL_OUTPUT_TRANSFORM_270: wl_output_transform = 3;
pub const WL_OUTPUT_TRANSFORM_FLIPPED: wl_output_transform = 4;
pub const WL_OUTPUT_TRANSFORM_FLIPPED_90: wl_output_transform = 5;
pub const WL_OUTPUT_TRANSFORM_FLIPPED_180: wl_output_transform = 6;
pub const WL_OUTPUT_TRANSFORM_FLIPPED_270: wl_output_transform = 7;

#[repr(C)]
pub type wl_pointer_axis = uint32_t;
pub const WL_POINTER_AXIS_VERTICAL_SCROLL: wl_pointer_axis = 0;
pub const WL_POINTER_AXIS_HORIZONTAL_SCROLL: wl_pointer_axis = 1;

#[repr(C)]
pub type wl_pointer_button_state = uint32_t;
pub const WL_POINTER_BUTTON_STATE_RELEASED: wl_pointer_button_state = 0;
pub const WL_POINTER_BUTTON_STATE_PRESSED: wl_pointer_button_state = 1;

#[repr(C)]
pub type wl_pointer_error = uint32_t;
pub const WL_POINTER_ERROR_ROLE: wl_pointer_error = 0;

#[repr(C)]
pub type wl_seat_capability = uint32_t;
pub const WL_SEAT_CAPABILITY_POINTER: wl_seat_capability = 1;
pub const WL_SEAT_CAPABILITY_KEYBOARD: wl_seat_capability = 2;
pub const WL_SEAT_CAPABILITY_TOUCH: wl_seat_capability = 4;

#[repr(C)]
pub type wl_shell_error = uint32_t;
pub const WL_SHELL_ERROR_ROLE: wl_shell_error = 0;

#[repr(C)]
pub type wl_shell_surface_fullscreen_method = uint32_t;
pub const WL_SHELL_SURFACE_FULLSCREEN_METHOD_DEFAULT:
    wl_shell_surface_fullscreen_method = 0;
pub const WL_SHELL_SURFACE_FULLSCREEN_METHOD_SCALE:
    wl_shell_surface_fullscreen_method = 1;
pub const WL_SHELL_SURFACE_FULLSCREEN_METHOD_DRIVER:
    wl_shell_surface_fullscreen_method = 2;
pub const WL_SHELL_SURFACE_FULLSCREEN_METHOD_FILL:
    wl_shell_surface_fullscreen_method = 3;

#[repr(C)]
pub type wl_shell_surface_resize = uint32_t;
pub const WL_SHELL_SURFACE_RESIZE_NONE: wl_shell_surface_resize = 0;
pub const WL_SHELL_SURFACE_RESIZE_TOP: wl_shell_surface_resize = 1;
pub const WL_SHELL_SURFACE_RESIZE_BOTTOM: wl_shell_surface_resize = 2;
pub const WL_SHELL_SURFACE_RESIZE_LEFT: wl_shell_surface_resize = 4;
pub const WL_SHELL_SURFACE_RESIZE_TOP_LEFT: wl_shell_surface_resize = 5;
pub const WL_SHELL_SURFACE_RESIZE_BOTTOM_LEFT: wl_shell_surface_resize = 6;
pub const WL_SHELL_SURFACE_RESIZE_RIGHT: wl_shell_surface_resize = 8;
pub const WL_SHELL_SURFACE_RESIZE_TOP_RIGHT: wl_shell_surface_resize = 9;
pub const WL_SHELL_SURFACE_RESIZE_BOTTOM_RIGHT: wl_shell_surface_resize = 10;

#[repr(C)]
pub type wl_shell_surface_transient = uint32_t;
pub const WL_SHELL_SURFACE_TRANSIENT_INACTIVE: wl_shell_surface_transient = 0x1;

#[repr(C)]
pub type wl_shm_error = uint32_t;
pub const WL_SHM_ERROR_INVALID_FORMAT: wl_shm_error = 0;
pub const WL_SHM_ERROR_INVALID_STRIDE: wl_shm_error = 1;
pub const WL_SHM_ERROR_INVALID_FD: wl_shm_error = 2;

#[repr(C)]
pub type wl_shm_format = uint32_t;
pub const WL_SHM_FORMAT_ARGB8888: wl_shm_format = 0;
pub const WL_SHM_FORMAT_XRGB8888: wl_shm_format = 1;
pub const WL_SHM_FORMAT_C8: wl_shm_format = 0x20203843;
pub const WL_SHM_FORMAT_RGB332: wl_shm_format = 0x38424752;
pub const WL_SHM_FORMAT_BGR233: wl_shm_format = 0x38524742;
pub const WL_SHM_FORMAT_XRGB4444: wl_shm_format = 0x32315258;
pub const WL_SHM_FORMAT_XBGR4444: wl_shm_format = 0x32314258;
pub const WL_SHM_FORMAT_RGBX4444: wl_shm_format = 0x32315852;
pub const WL_SHM_FORMAT_BGRX4444: wl_shm_format = 0x32315842;
pub const WL_SHM_FORMAT_ARGB4444: wl_shm_format = 0x32315241;
pub const WL_SHM_FORMAT_ABGR4444: wl_shm_format = 0x32314241;
pub const WL_SHM_FORMAT_RGBA4444: wl_shm_format = 0x32314152;
pub const WL_SHM_FORMAT_BGRA4444: wl_shm_format = 0x32314142;
pub const WL_SHM_FORMAT_XRGB1555: wl_shm_format = 0x35315258;
pub const WL_SHM_FORMAT_XBGR1555: wl_shm_format = 0x35314258;
pub const WL_SHM_FORMAT_RGBX5551: wl_shm_format = 0x35315852;
pub const WL_SHM_FORMAT_BGRX5551: wl_shm_format = 0x35315842;
pub const WL_SHM_FORMAT_ARGB1555: wl_shm_format = 0x35315241;
pub const WL_SHM_FORMAT_ABGR1555: wl_shm_format = 0x35314241;
pub const WL_SHM_FORMAT_RGBA5551: wl_shm_format = 0x35314152;
pub const WL_SHM_FORMAT_BGRA5551: wl_shm_format = 0x35314142;
pub const WL_SHM_FORMAT_RGB565: wl_shm_format = 0x36314752;
pub const WL_SHM_FORMAT_BGR565: wl_shm_format = 0x36314742;
pub const WL_SHM_FORMAT_RGB888: wl_shm_format = 0x34324752;
pub const WL_SHM_FORMAT_BGR888: wl_shm_format = 0x34324742;
pub const WL_SHM_FORMAT_XBGR8888: wl_shm_format = 0x34324258;
pub const WL_SHM_FORMAT_RGBX8888: wl_shm_format = 0x34325852;
pub const WL_SHM_FORMAT_BGRX8888: wl_shm_format = 0x34325842;
pub const WL_SHM_FORMAT_ABGR8888: wl_shm_format = 0x34324241;
pub const WL_SHM_FORMAT_RGBA8888: wl_shm_format = 0x34324152;
pub const WL_SHM_FORMAT_BGRA8888: wl_shm_format = 0x34324142;
pub const WL_SHM_FORMAT_XRGB2101010: wl_shm_format = 0x30335258;
pub const WL_SHM_FORMAT_XBGR2101010: wl_shm_format = 0x30334258;
pub const WL_SHM_FORMAT_RGBX1010102: wl_shm_format = 0x30335852;
pub const WL_SHM_FORMAT_BGRX1010102: wl_shm_format = 0x30335842;
pub const WL_SHM_FORMAT_ARGB2101010: wl_shm_format = 0x30335241;
pub const WL_SHM_FORMAT_ABGR2101010: wl_shm_format = 0x30334241;
pub const WL_SHM_FORMAT_RGBA1010102: wl_shm_format = 0x30334152;
pub const WL_SHM_FORMAT_BGRA1010102: wl_shm_format = 0x30334142;
pub const WL_SHM_FORMAT_YUYV: wl_shm_format = 0x56595559;
pub const WL_SHM_FORMAT_YVYU: wl_shm_format = 0x55595659;
pub const WL_SHM_FORMAT_UYVY: wl_shm_format = 0x59565955;
pub const WL_SHM_FORMAT_VYUY: wl_shm_format = 0x59555956;
pub const WL_SHM_FORMAT_AYUV: wl_shm_format = 0x56555941;
pub const WL_SHM_FORMAT_NV12: wl_shm_format = 0x3231564e;
pub const WL_SHM_FORMAT_NV21: wl_shm_format = 0x3132564e;
pub const WL_SHM_FORMAT_NV16: wl_shm_format = 0x3631564e;
pub const WL_SHM_FORMAT_NV61: wl_shm_format = 0x3136564e;
pub const WL_SHM_FORMAT_YUV410: wl_shm_format = 0x39565559;
pub const WL_SHM_FORMAT_YVU410: wl_shm_format = 0x39555659;
pub const WL_SHM_FORMAT_YUV411: wl_shm_format = 0x31315559;
pub const WL_SHM_FORMAT_YVU411: wl_shm_format = 0x31315659;
pub const WL_SHM_FORMAT_YUV420: wl_shm_format = 0x32315559;
pub const WL_SHM_FORMAT_YVU420: wl_shm_format = 0x32315659;
pub const WL_SHM_FORMAT_YUV422: wl_shm_format = 0x36315559;
pub const WL_SHM_FORMAT_YVU422: wl_shm_format = 0x36315659;
pub const WL_SHM_FORMAT_YUV444: wl_shm_format = 0x34325559;
pub const WL_SHM_FORMAT_YVU444: wl_shm_format = 0x34325659;

#[repr(C)]
pub type wl_subcompositor_error = uint32_t;
pub const WL_SUBCOMPOSITOR_ERROR_BAD_SURFACE: wl_subcompositor_error = 0;

#[repr(C)]
pub type wl_subsurface_error = uint32_t;
pub const WL_SUBSURFACE_ERROR_BAD_SURFACE: wl_subsurface_error = 0;

#[repr(C)]
pub type wl_surface_error = uint32_t;
pub const WL_SURFACE_ERROR_INVALID_SCALE: wl_surface_error = 0;
pub const WL_SURFACE_ERROR_INVALID_TRANSFORM: wl_surface_error = 1;
