extern crate sdl2;
extern crate gemini;


use gemini::vdi;


#[test]
fn creation() {
    let sdl = sdl2::init().unwrap();
    let vdi : &mut vdi::VDI =
        &mut vdi::SDL2Vdi::new(&sdl, 640, 480, "blah").unwrap();
}
