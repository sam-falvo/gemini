extern crate sdl2;
extern crate gemini;


use gemini::vdi;
use std::{thread, time};

#[test]
fn draw_point() {
    let sdl = sdl2::init().unwrap();
    let vdi : &mut vdi::VDI =
        &mut vdi::SDL2Vdi::new(&sdl, 640, 480, "blah").unwrap();

println!("---------------------------------------------------------------");
    for x in 0..64 {
        for y in 0..64 {
            vdi.draw_point((x,y), (2*(x+y) & 0xFF) as u8);
        }
    }
println!("---------------------------------------------------------------");
    vdi.commit().unwrap();
println!("---------------------------------------------------------------");
    thread::sleep(time::Duration::new(20, 0));
}
