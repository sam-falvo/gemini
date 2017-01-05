#![cfg(test)]

extern crate gemini;
extern crate sdl2;

use std::{thread, time};
use gemini::vdi;

#[test]
fn creation() {
    let sdl_context = sdl2::init().unwrap();
    let vdi : &mut vdi::VDI = &mut vdi::SDL2Vdi::new(&sdl_context, 640, 480, "Fake VGA").unwrap();
    let vdi2 : &mut vdi::VDI = &mut vdi::SDL2Vdi::new(&sdl_context, 512, 384, "Fake Mac").unwrap();
    thread::sleep(time::Duration::new(20,0));
}
