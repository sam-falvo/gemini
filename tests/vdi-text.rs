extern crate sdl2;
extern crate gemini;


use gemini::vdi;

use std::cmp::{max,min};


// This defines a small, 16-character font containing glyphs for the hexadecimal digits.
// The font is 128x8 pixels in size, each glyph being 8x8 pixels.
static FONT_DATA : [u16; 64] = [
    0b0001100000001000, 0b0011110000111100, 0b0000100001111110, 0b0011110001111110,
        0b0011110000111100, 0b0001100001111100, 0b0011110001111000, 0b0111111001111110,
    0b0010010000111000, 0b0100001001000010, 0b0001100001000000, 0b0100001000000010,
        0b0100001001000010, 0b0010010001000010, 0b0100001001000100, 0b0100000001000000,
    0b0100001000001000, 0b0000001000000010, 0b0010100001000000, 0b0100000000000100,
        0b0100001001000010, 0b0010010001000010, 0b0100000001000010, 0b0100000001000000,
    0b0100001000001000, 0b0001110000011100, 0b0010100001111100, 0b0111110000000100,
        0b0011110000111100, 0b0100001001111100, 0b0100000001000010, 0b0111100001111000,
    0b0100001000001000, 0b0010000000000010, 0b0111111000000010, 0b0100001000001000,
        0b0100001000000010, 0b0111111001000010, 0b0100000001000010, 0b0100000001000000,
    0b0010010000001000, 0b0100000001000010, 0b0000100001000010, 0b0100001000001000,
        0b0100001001000010, 0b0100001001000010, 0b0100001001000100, 0b0100000001000000,
    0b0001100000111110, 0b0111111000111100, 0b0000100000111100, 0b0011110000001000,
        0b0011110000111100, 0b0100001001111100, 0b0011110001111000, 0b0111111001000000,
    0b0000000000000000, 0b0000000000000000, 0b0000000000000000, 0b0000000000000000,
        0b0000000000000000, 0b0000000000000000, 0b0000000000000000, 0b0000000000000000,
];


struct Font<'a> {
    bits:           &'a [u16],
    left_edges:     &'a [u16],
    width:          u16,
    ascender:       u16,
    height:         u16,
}


static HEXFONT : Font<'static> = Font {
    bits:           &FONT_DATA,
    left_edges:     &[0*8, 1*8, 2*8, 3*8, 4*8, 5*8, 6*8, 7*8,
                     8*8, 9*8, 10*8, 11*8, 12*8, 13*8, 14*8, 15*8,
                     16*8],
    width:          16*8,
    ascender:       7,
    height:         8
};


static DESKTOP : [u16; 16] = [
    0xAAAA, 0x5555, 0xAAAA, 0x5555,
    0xAAAA, 0x5555, 0xAAAA, 0x5555,
    0xAAAA, 0x5555, 0xAAAA, 0x5555,
    0xAAAA, 0x5555, 0xAAAA, 0x5555,
];


struct TextContext<'a> {
    vdi:            &'a mut vdi::VDI,
    font:           &'a Font<'a>,

    // where next character goes.
    left:           u16,
    baseline:       u16,
    function:       u8,

    // display boundaries.
    left_margin:    u16,
    right_margin:   u16,
    window_top:     u16,
    window_bottom:  u16,
}


impl<'a> TextContext<'a> {
    fn put_char(&mut self, chr: u8) {
        let font = self.font;
        let chr_left = font.left_edges[chr as usize];
        let chr_width = font.left_edges[(chr+1) as usize] - chr_left;
        let vdi = &mut self.vdi;
        let vdi_top = self.baseline - font.ascender;
        let vdi_bottom = vdi_top + font.height;
        let vdi_top_clipped = max(vdi_top, self.window_top);
        let chr_top_clipped = vdi_top_clipped - vdi_top;
        let vdi_bottom_clipped = min(self.window_bottom, vdi_bottom);
        if vdi_top_clipped > vdi_bottom_clipped {
            return;  // outside the visible window; nothing to show.
        }
        let chr_height_clipped = vdi_bottom_clipped - vdi_top_clipped;

        vdi.copy_rect_big_endian(
            (chr_left, chr_top_clipped), font.width as usize, font.bits,
            (self.left, vdi_top_clipped),
            (chr_width, chr_height_clipped),
            self.function,
        );

        self.left += chr_width;
    }
}

fn put_string(t: &mut TextContext, y: u16) {
    t.left = 4;
    t.baseline = t.font.ascender + y*t.font.height + 4;

    for x in 0..16 {
        t.put_char(x as u8);
    }
}

#[test]
fn text() {
    let sdl = sdl2::init().unwrap();
    let vdi : &mut vdi::VDI =
        &mut vdi::SDL2Vdi::new(&sdl, 640, 480, "blah").unwrap();
    let mut t : TextContext = TextContext{
        vdi: vdi,
        font: &HEXFONT,
        left: 4,
        function: 0b0101,
        baseline: 11,
        left_margin: 4,
        right_margin: 128,
        window_top: 8,
        window_bottom: 24,
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
