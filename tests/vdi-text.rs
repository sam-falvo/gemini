extern crate sdl2;
extern crate gemini;


use gemini::{vdi, font};


static DESKTOP : [u16; 16] = [
    0xAAAA, 0x5555, 0xAAAA, 0x5555,
    0xAAAA, 0x5555, 0xAAAA, 0x5555,
    0xAAAA, 0x5555, 0xAAAA, 0x5555,
    0xAAAA, 0x5555, 0xAAAA, 0x5555,
];


fn put_string(t: &mut font::TextContext, y: u16) {
    t.left = 4;
    t.baseline = t.font.ascender + y*t.font.height + 4;

    for x in 0..16 {
        t.simple_put_char((x+64) as u8);

        // We assume in this test that the system font is an 8x8
        // fixed width font.
        let (width, height, ascender) = t.get_real_size((x+64) as u8);
        assert_eq!(width, 8);
        assert_eq!(height, 8);
        assert_eq!(ascender, 7);
    }
}

#[test]
fn text() {
    let sdl = sdl2::init().unwrap();
    let vdi : &mut vdi::VDI =
        &mut vdi::SDL2Vdi::new(&sdl, 640, 480, "blah").unwrap();
    let mut t = font::TextContext{
        vdi: vdi,
        font: font::borrow_system_font(),
        left: 0,
        baseline: 0,
        strike_fn: 0b0101,
        left_margin: 8,
        right_margin: 128,
        top_margin: 8,
        bottom_margin: 24,
    };

    t.vdi.rect((0, 0), (640, 480), &DESKTOP);
    put_string(&mut t, 0);
    put_string(&mut t, 1);
    put_string(&mut t, 2);
    put_string(&mut t, 3);
    t.vdi.commit().unwrap();

{
use std::{thread,time};
thread::sleep(time::Duration::new(15,0));
}
}
