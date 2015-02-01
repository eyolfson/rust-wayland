// Copyright 2015 Jonathan Eyolfson

extern crate wayland;

use wayland::new::*;

fn main() {
    let mut display = Display::connect().unwrap();
    display.get_registry().unwrap();
    for _ in 0..17 {
        display.read_message().unwrap();
    }
}
