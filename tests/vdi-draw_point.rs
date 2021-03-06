extern crate sdl2;
extern crate gemini;


use gemini::vdi;


#[test]
fn draw_point() {
    let sdl = sdl2::init().unwrap();
    let vdi : &mut vdi::VDI =
        &mut vdi::SDL2Vdi::new(&sdl, 640, 480, "blah").unwrap();

    for i in 0..128 {
        vdi.draw_point((0, 0), i);
        assert_eq!(vdi.get_point((0, 0)), 0);
    }

    for i in 128..256 {
        vdi.draw_point((0, 0), i as u8);
        assert_eq!(vdi.get_point((0, 0)), 255);
    }
}
