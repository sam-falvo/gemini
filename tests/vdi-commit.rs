extern crate sdl2;
extern crate gemini;


use gemini::vdi;


#[test]
fn draw_point() {
    let sdl = sdl2::init().unwrap();
    let vdi : &mut vdi::VDI =
        &mut vdi::SDL2Vdi::new(&sdl, 640, 480, "blah").unwrap();

    for x in 0..640 {
        for y in 0..480 {
            vdi.draw_point((x,y), (2*(x+y) & 0xFF) as u8);
        }
    }
    vdi.commit().unwrap();
}
