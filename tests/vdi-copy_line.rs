extern crate sdl2;
extern crate gemini;


use gemini::vdi;


static MOUSE_IOR : [u16; 16] = [
    0b1100000000000000,
    0b1111000000000000,
    0b0111110000000000,
    0b0111111100000000,
    0b0011111111000000,
    0b0011111111110000,
    0b0001111111111000,
    0b0001111111110000,
    0b0000111111100000,
    0b0000111111110000,
    0b0000011111111000,
    0b0000011101111100,
    0b0000001000111110,
    0b0000000000011110,
    0b0000000000001100,
    0b0000000000000000
];

static MOUSE_XOR : [u16; 16] = [
    0b0000000000000000,
    0b0100000000000000,
    0b0011000000000000,
    0b0011110000000000,
    0b0001111100000000,
    0b0001111111000000,
    0b0000111111110000,
    0b0000111111100000,
    0b0000011111000000,
    0b0000011111100000,
    0b0000001101110000,
    0b0000001000111000,
    0b0000000000011100,
    0b0000000000001100,
    0b0000000000000000,
    0b0000000000000000
];


static DESKTOP : [u16; 16] = [
    0xAAAA, 0x5555, 0xAAAA, 0x5555,
    0xAAAA, 0x5555, 0xAAAA, 0x5555,
    0xAAAA, 0x5555, 0xAAAA, 0x5555,
    0xAAAA, 0x5555, 0xAAAA, 0x5555,
];

#[test]
fn copy_line() {
    let sdl = sdl2::init().unwrap();
    let vdi : &mut vdi::VDI =
        &mut vdi::SDL2Vdi::new(&sdl, 640, 480, "blah").unwrap();
    
    vdi.rect((0, 0), (640, 480), &DESKTOP);

    for x in 0..640 {
        vdi.rect((0, 0), (640, 64), &DESKTOP);
        for y in 0..16 {
            vdi.copy_line(
                (0, y),     // left/top edge of source
                16,         // Source is 16 bits wide.
                &MOUSE_IOR, // source bits

                (x, y),     // left edge of destination

                16,         // Move 16 pixels.

                0xEE        // logical OR operation.
            );
            vdi.copy_line(
                (0, y),     // left/top edge of source
                16,         // Source is 16 bits wide.
                &MOUSE_XOR, // source bits

                (x, y),     // left edge of destination

                16,         // Move 16 pixels.

                0x66        // logical OR operation.
            );
            vdi.copy_line_big_endian(
                (0, y),     // left/top edge of source
                16,         // Source is 16 bits wide.
                &MOUSE_IOR, // source bits

                (x, y + 32),     // left edge of destination

                16,         // Move 16 pixels.

                0xEE        // logical OR operation.
            );
            vdi.copy_line_big_endian(
                (0, y),     // left/top edge of source
                16,         // Source is 16 bits wide.
                &MOUSE_XOR, // source bits

                (x, y + 32),     // left edge of destination

                16,         // Move 16 pixels.

                0x66        // logical OR operation.
            );
        }
        vdi.commit().unwrap();
    }
}
