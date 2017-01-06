extern crate sdl2;
extern crate gemini;


use gemini::vdi;


#[test]
fn invert() {
    let sdl = sdl2::init().unwrap();
    let vdi : &mut vdi::VDI =
        &mut vdi::SDL2Vdi::new(&sdl, 512, 512, "blah").unwrap();

    let paper : [u16; 16] = [
        0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF,
        0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF,
        0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF,
        0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF,
    ];

    vdi.rect((0, 0), (512, 512), &paper);
    vdi.commit().unwrap();

    for x in 0..512 {
        for y in 0..512 {
            assert_eq!(vdi.get_point((x,y)), 255);
        }
    }

    vdi.invert_rect((0, 0), (512, 512));
    vdi.commit().unwrap();

    for x in 0..512 {
        for y in 0..512 {
            assert_eq!(vdi.get_point((x,y)), 0);
        }
    }

    for i in 0..512 {
        vdi.invert_line((0, i), i);
    }
    vdi.commit().unwrap();

    for x in 0..512 {
        for y in 0..512 {
            assert_eq!(
                vdi.get_point((x,y)),
                if x >= y { 0 } else { 255 }
            );
        }
    }

}
