#![cfg(test)]

extern crate gemini;
extern crate sdl2;

use gemini::vdi;
use gemini::vdi::*;

use sdl2::*;

#[test]
fn creation() {
    let sdl_context = sdl2::init().unwrap();
    let vdi : &vdi::VDI = &vdi::SDL2Vdi::new(&sdl_context, 640, 480);
}
