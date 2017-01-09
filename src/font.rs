use std::cmp::{max,min};
use super::vdi;
use super::system_font;

pub struct Font<'a> {
    pub bits:           &'a [u16],
    pub left_edges:     &'a [u16],
    pub width:          u16,
    pub ascender:       u16,
    pub height:         u16,
}


pub struct TextContext<'a> {
    pub vdi:            &'a mut vdi::VDI,
    pub font:           &'a Font<'a>,

    // where next character goes.
    pub left:           u16,
    pub baseline:       u16,
    pub strike_fn:      u8,

    // display boundaries.
    pub left_margin:    u16,
    pub right_margin:   u16,
    pub top_margin:     u16,
    pub bottom_margin:  u16,
}


impl<'a> TextContext<'a> {
    pub fn get_real_size(&self, chr: u8) -> (u16, u16, u16) {
        let font = self.font;
        let chr_left = font.left_edges[chr as usize];
        let chr_right = font.left_edges[(chr+1) as usize];
        let width = chr_right - chr_left;
        let height = font.height;
        let ascender = font.ascender;

        (width, height, ascender)
    }

    pub fn simple_put_char(&mut self, chr: u8) {
        let vdi = &mut self.vdi;
        let font = self.font;

        let chr_left = font.left_edges[chr as usize];
        let vdi_top = self.baseline - font.ascender;
        let vdi_top_clipped = max(vdi_top, self.top_margin);
        let chr_top_clipped = vdi_top_clipped - vdi_top;
        let vdi_bottom = vdi_top + font.height;
        let vdi_bottom_clipped = min(self.bottom_margin, vdi_bottom);
        if vdi_top_clipped >= vdi_bottom_clipped {
            return;  // outside the visible window; nothing to show.
        }
        let chr_height_clipped = vdi_bottom_clipped - vdi_top_clipped;

        let chr_right = font.left_edges[(chr+1) as usize];
        let chr_width = chr_right - chr_left;
        let vdi_left_clipped = max(self.left_margin, self.left);
        let vdi_right_clipped = min(self.right_margin, self.left + chr_width);
        if vdi_left_clipped >= vdi_right_clipped {
            return;  // outside the visible window; nothing to show.
        }
        let delta_x = vdi_left_clipped - self.left;
        let chr_left_clipped = chr_left + delta_x;
        let chr_width_clipped = min(chr_width, vdi_right_clipped - vdi_left_clipped);

        vdi.copy_rect_big_endian(
            (chr_left_clipped, chr_top_clipped), font.width as usize, font.bits,
            (vdi_left_clipped, vdi_top_clipped),
            (chr_width_clipped, chr_height_clipped),
            self.strike_fn,
        );

        self.left += chr_width;
    }
}


pub fn borrow_system_font() -> &'static Font<'static> {
    return &SYSTEM_FONT;
}


pub static SYSTEM_FONT : Font<'static> = Font {
    bits:           &system_font::SYSTEM_GLYPHS,
    left_edges:     &system_font::SYSTEM_LEFT_EDGES,
    width:          256*8,
    ascender:       7,
    height:         8
};

