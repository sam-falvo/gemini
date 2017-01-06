extern crate sdl2;
extern crate gemini;


use gemini::vdi;


#[test]
fn hline() {
    let sdl = sdl2::init().unwrap();
    let vdi : &mut vdi::VDI =
        &mut vdi::SDL2Vdi::new(&sdl, 512, 512, "blah").unwrap();

    for i in 0..512 {
        vdi.hline((i, i), 512, 0xFFFF);
        vdi.hline((0, i), i, 0);
    }

    vdi.commit().unwrap();

    for y in 0..512 {
        for x in 0..512 {
            let p = vdi.get_point((x, y));
            if x < y {
                assert_eq!(p, 0, "Point ({}, {}) = {}", x, y, p);
            }
            if x >= y {
                assert_eq!(p, 255, "Point ({}, {}) = {}", x, y, p);
            }
        }
    }
}

