extern crate sdl2;
extern crate gemini;


use gemini::vdi;


#[test]
fn rect() {
    let sdl = sdl2::init().unwrap();
    let vdi : &mut vdi::VDI =
        &mut vdi::SDL2Vdi::new(&sdl, 640, 480, "blah").unwrap();

    let desktop_pattern : [u16; 16] = [
        0xAAAA, 0x5555, 0xAAAA, 0x5555,
        0xAAAA, 0x5555, 0xAAAA, 0x5555,
        0xAAAA, 0x5555, 0xAAAA, 0x5555,
        0xAAAA, 0x5555, 0xAAAA, 0x5555,
    ];

    let shadow_pattern : [u16; 16] = [
        0x0000, 0x0000, 0x0000, 0x0000,
        0x0000, 0x0000, 0x0000, 0x0000,
        0x0000, 0x0000, 0x0000, 0x0000,
        0x0000, 0x0000, 0x0000, 0x0000,
    ];

    let paper_pattern : [u16; 16] = [
        0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF,
        0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF,
        0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF,
        0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF,
    ];

    vdi.rect((0, 0), (640, 480), &desktop_pattern);
    vdi.rect((162, 102), (482, 302), &shadow_pattern);
    vdi.rect((160, 100), (480, 300), &paper_pattern);
    vdi.frame((160, 100), (480, 300), 0x0000);

    vdi.commit().unwrap();

    {
        use std::{thread, time};
        thread::sleep(time::Duration::new(10,0));
    }
}

