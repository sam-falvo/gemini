//! # VDI
//!
//! This module corresponds to the Video Driver Interface (VDI).
//! It provides basic primitives for displaying simple graphics.
//!
//! Influenced more by GEOS than by GEM's VDI, this module allows
//! applications to scribble on the entire display surface.  No
//! support for clipping yet exists,
//! except for the edges of the display surface of course.


use sdl2;
use sdl2::{pixels, render, video};

use std::{mem, result};
use std::cmp::min;


/// Indication of an error somewhere inside the VDI module.
#[derive(Debug)]
pub enum VdiError {
    FromSdl(String),
    Miscellaneous,
}


/// VDI drivers must conform to this interface.
///
/// A word about color indices.  Currently, only two indices are supported.
/// Indices 0...127 corresponds to black, while indices 128...255 corresponds
/// to white.  For future compatibility, use index 255 to refer to white.
pub trait VDI {
    /// Draw a single point at the provided coordinates.  Attempts to draw beyond
    /// the edge of the surface will simply be ignored.
    fn draw_point(&mut self, at: (u16, u16), pen: u8);

    /// Retrieves the current pixel value at a given position.
    fn get_point(&self, at: (u16, u16)) -> u8;

    /// Commit sends the current contents of the VDI frame buffer
    /// to the attached display.  Typically, a program would draw into the
    /// frame buffer, and then call `commit` to make the drawing visible to
    /// the user.  Note that this procedure updates the entire frame buffer.
    fn commit(&mut self) -> result::Result<(), VdiError>;

    /// Draw a horizontal line on the VDI surface using the provided pattern.
    /// Coordinates are clipped to the edges of the surface only.
    /// The pattern is naturally aligned with the left edge of the surface,
    /// so that pixel 0 aligns with bit 0 of the pattern, pixel 1 with bit 1,
    /// pixel 15 with bit 15, pixel 16 with bit 0 again, and so forth.
    /// In this way, a horizontal line can be drawn in several segments if
    /// desired, and the pattern will be continuous.
    ///
    /// The `at` coordinate specifies where to start drawing the line.
    /// The `to` coordinate specifies only the horizontal coordinate of where
    /// to end the line.  The horizontal range covered is `[at.x, to)`.
    fn hline(&mut self, at: (u16, u16), to: u16, pattern: u16);

    /// Draw a vertical line on the VDI surface using the provided pattern.
    /// Coordinates are clipped to the edges of the surface only.
    /// The pattern is naturally aligned with the top edge of the surface,
    /// so that pixel 0 aligns with bit 0 of the pattern, pixel 1 with bit 1,
    /// pixel 15 with bit 15, pixel 16 with bit 0 again, and so forth.
    /// In this way, a vertical line can be drawn in several segments if
    /// desired, and the pattern will be continuous.
    ///
    /// The `at` coordinate specifies where to start drawing the line.
    /// The `to` coordinate specifies only the vertical coordinate of where
    /// to end the line.  The vertical range covered is `[at.y, to)`.
    fn vline(&mut self, at: (u16, u16), to: u16, pattern: u16);

    /// Draw a filled rectangle starting at `at`, and extending to `to`.
    /// Use the supplied pattern.
    ///
    /// The pattern is aligned to the left and top edge of the VDI surface.
    /// You can draw several overlapping and/or adjacent filled rectangles,
    /// and the pattern will be continuous.
    fn rect(&mut self, at: (u16, u16), to: (u16, u16), pattern: &[u16; 16]);

    /// Draw an unfilled rectangular frame starting at `at` and extending to `to`.
    /// Use the supplied line pattern.
    fn frame(&mut self, at: (u16, u16), to: (u16, u16), pattern: u16);

    /// Invert a horizontal line.
    fn invert_line(&mut self, at: (u16, u16), to: u16);

    /// Invert a rectangle.
    fn invert_rect(&mut self, at: (u16, u16), to: (u16, u16));

    /// Copy a single row of pixels from a source bitmap into the VDI surface.
    ///
    /// `from` specifies where, in the source bitmap, to start reading bits to
    /// copy.  These coordinates cannot exceed the boundaries of the source bitmap.
    /// `src_width` specifies how wide the bitmap is in pixels.
    /// `from_bits` tells where to find the vector of `u16`s containing the bitmap
    /// itself.  Each row of `u16`s are just big enough to hold `src_width` pixels.
    /// For example, a 24-pixel wide image occupies two `u16`s per row.
    ///
    /// `to` specifies where in the VDI surface to place the bitmap image.
    ///
    /// `width` specifies the desired number of pixels to move.
    /// The actual number of pixels moved may be fewer;
    /// this procedure will clip the blitted image if it falls off the right-hand
    /// edge of the screen.
    /// 
    /// The `function` parameter specifies how to mix the source and destination
    /// pixels:
    ///
    /// |   3   |   2    |    1   |    0    |
    /// |:-----:|:------:|:------:|:-------:|
    /// | D & S | D & !S | !D & S | !D & !S |
    ///
    /// where **S** refers to the source (bitmap) pixel,
    /// and **D** refers to the corresponding destination (VDI) pixel.
    fn copy_line(
        &mut self,
        from: (u16, u16),
        src_width: usize,
        from_bits: &[u16],
        to: (u16, u16),
        width: usize,
        function: u8
    );

    // As with `copy_line`, but the source bitmap data is stored with big-endian
    // bit ordering.
    fn copy_line_big_endian(
        &mut self,
        from: (u16, u16),
        src_width: usize,
        from_bits: &[u16],
        to: (u16, u16),
        width: usize,
        function: u8
    );

    /// Copy a rectangular arrangement of pixels from a source bitmap into the VDI surface.
    ///
    /// `from` specifies where, in the source bitmap, to start reading bits to
    /// copy.  These coordinates cannot exceed the boundaries of the source bitmap.
    /// `src_width` specifies how wide the bitmap is in pixels.
    /// `from_bits` tells where to find the vector of `u16`s containing the bitmap
    /// itself.  Each row of `u16`s are just big enough to hold `src_width` pixels.
    /// For example, a 24-pixel wide image occupies two `u16`s per row.
    ///
    /// `to` specifies where in the VDI surface to place the bitmap image.
    ///
    /// `dimensions` specifies the desired number of pixels to move.
    /// The width component sets the maximum number of pixels to move in the horziontal
    /// axis, while the height component does the same for the vertical axis.
    /// The actual number of pixels moved may be fewer;
    /// this procedure will clip the blitted image if it falls off the right-hand
    /// and/or bottom edge of the screen.
    /// 
    /// The `function` parameter specifies how to mix the source and destination
    /// pixels:
    ///
    /// |   3   |   2    |    1   |    0    |
    /// |:-----:|:------:|:------:|:-------:|
    /// | D & S | D & !S | !D & S | !D & !S |
    ///
    /// where **S** refers to the source (bitmap) pixel,
    /// and **D** refers to the corresponding destination (VDI) pixel.
    fn copy_rect(
        &mut self,
        from: (u16, u16),
        src_width: usize,
        from_bits: &[u16],
        to: (u16, u16),
        dimensions: (u16, u16),
        function: u8
    );

    /// As with `copy_rect`, but with big-endian formatted source bitmaps.
    fn copy_rect_big_endian(
        &mut self,
        from: (u16, u16),
        src_width: usize,
        from_bits: &[u16],
        to: (u16, u16),
        dimensions: (u16, u16),
        function: u8
    );
}


/// This structure represents an SDL2-backed VDI surface (bluntly, a window).
/// The window is fixed in size, emulating the frame buffer of a given size.
/// When the window opens, the state of the frame buffer is completely undefined.
/// You'll need to paint the frame buffer to establish a known image.
/// For example:
///
/// ```text
/// let sdl = sdl2::init().unwrap();
/// let vdi : &mut vdi::VDI =
///     &mut vdi::SDL2Vdi::new(&sdl, 640, 480, "blah").unwrap();
///
/// let desktop_pattern : [u16; 16] = [
///     0xAAAA, 0x5555, 0xAAAA, 0x5555,
///     0xAAAA, 0x5555, 0xAAAA, 0x5555,
///     0xAAAA, 0x5555, 0xAAAA, 0x5555,
///     0xAAAA, 0x5555, 0xAAAA, 0x5555,
/// ];
///
/// vdi.rect((0, 0), (640, 480), &desktop_pattern);
/// vdi.commit().unwrap();
///
/// // At this point, the frame buffer on-screen and in backing store match.
/// ```
pub struct SDL2Vdi {
    /// The dimensions field allows for a display surface up to 64Kx64K in size.
    dimensions: (u16, u16),

    /// SDL2 Renderer (from which we can get the window again if we need to)
    renderer: render::Renderer<'static>,

    /// SDL2 Texture used to contain the frame buffer for the window.
    texture: render::Texture,

    /// Back-buffer to draw into and support `get_point` with.
    /// **Implementation detail:**
    /// When invoking `commit`, this backbuffer is color-expanded into pixels
    /// that SDL2 can understand, and then submitted to SDL for rendering.
    backbuffer: Vec<u8>,
}


impl SDL2Vdi {
    /// Create a new SDL2-backed VDI instance.
    /// This will open a window and
    /// create an appropriately-sized frame buffer to back it.
    /// At present, the bitmap is monochrome: 0s are black, 1s are white.
    ///
    /// width and height are measured in pixels.
    pub fn new(context: & sdl2::Sdl, width: u16, height: u16, title: & str) ->
                result::Result<SDL2Vdi, VdiError> {
        let total_pixels = width as usize * height as usize;
        let mut backbuffer = Vec::with_capacity(total_pixels);
        (&mut backbuffer).resize(total_pixels, 0);

        let video_subsystem = match context.video() {
            Err(e) =>
                return Err(VdiError::FromSdl(e)),

            Ok(subsys) =>
                subsys
        };

        let w : video::Window = match video::WindowBuilder::new(
                    &video_subsystem, title,
                    width as u32, height as u32
                )
                .resizable()
                .build() {
            Err(video::WindowBuildError::HeightOverflows(_)) =>
                return Err(VdiError::FromSdl(String::from("Height overflows"))),

            Err(video::WindowBuildError::WidthOverflows(_)) =>
                return Err(VdiError::FromSdl(String::from("Width overflows"))),

            Err(video::WindowBuildError::InvalidTitle(_)) =>
                return Err(VdiError::FromSdl(String::from("Invalid title"))),

            Err(video::WindowBuildError::SdlError(s)) =>
                return Err(VdiError::FromSdl(s)),

            Ok(w) =>
                w
        };

        let r : render::Renderer = match w.renderer().build() {
            Err(sdl2::IntegerOrSdlError::IntegerOverflows(s, n)) =>
                return Err(VdiError::FromSdl(String::from(format!("Integer overflows: {}:{}", s, n)))),

            Err(sdl2::IntegerOrSdlError::SdlError(s)) =>
                return Err(VdiError::FromSdl(s)),

            Ok(r) =>
               r
        };

        let mut t : render::Texture = match (&r).create_texture(
                pixels::PixelFormatEnum::ARGB8888,
                render::TextureAccess::Streaming,
                width as u32, height as u32
        ) {
            Err(render::TextureValueError::WidthOverflows(_)) =>
                return Err(VdiError::FromSdl(String::from("Width overflow"))),

            Err(render::TextureValueError::HeightOverflows(_)) =>
                return Err(VdiError::FromSdl(String::from("Height overflow"))),

            Err(render::TextureValueError::WidthMustBeMultipleOfTwoForFormat(_, _)) =>
                return Err(VdiError::FromSdl(String::from("Texture width must be a power of two."))),

            Err(render::TextureValueError::SdlError(s)) =>
                return Err(VdiError::FromSdl(s)),

            Ok(t) =>
                t,
        };

        (&mut t).set_blend_mode(render::BlendMode::None);

        Ok(SDL2Vdi {
            dimensions: (width, height),
            renderer:   r,
            texture:    t,
            backbuffer: backbuffer,
        })
    }
}


impl VDI for SDL2Vdi {
    fn draw_point(&mut self, at: (u16, u16), pen: u8) {
        let (x, y) = at;
        let (x, y) = (x as usize, y as usize);
        let (width, height) = self.dimensions;
        let (width, height) = (width as usize, height as usize);
        let backbuf = &mut self.backbuffer;

        if (x >= width) || (y >= height) {
            return;
        }

        let p = if pen >= 128 { 255 } else { 0 };

        backbuf[(y * width + x) as usize] = p;
    }

    fn get_point(&self, at: (u16, u16)) -> u8 {
        let (x, y) = at;
        let (x, y) = (x as usize, y as usize);
        let (width, height) = self.dimensions;
        let (width, height) = (width as usize, height as usize);

        if (x >= width) || (y >= height) {
            0
        }
        else {
            let offset = y * width + x;
            self.backbuffer[offset]
        }
    }

    fn commit(&mut self) -> result::Result<(), VdiError> {
        let (width, height) = self.dimensions;
        let (width, height) = (width as usize, height as usize);
        let backbuf = &mut self.backbuffer; 
    let r = &mut self.renderer;
        let t = &mut self.texture;

        t.with_lock(None, |bits: &mut [u8], span: usize| {
            let mut source_offset = 0;
            let mut dest_offset = 0;

            for _ in 0..height {
                for x in 0..width {
                    let pen = backbuf[source_offset];
                    source_offset += 1;

                    let x4 = dest_offset + x * 4;
                    bits[x4+0] = pen;
                    bits[x4+1] = pen;
                    bits[x4+2] = pen;
                    bits[x4+3] = pen;
                }
                dest_offset += span;
            }
        }).and_then(|_| r.copy(t, None, None))
        .map_err(|e| VdiError::FromSdl(e))
        .and_then(|_| -> result::Result<(), VdiError> {
            r.present();
            Ok(())
        })
    }

    fn hline(&mut self, at: (u16, u16), to: u16, pattern: u16) {
        let (left, y) = at;
        let mut left = left as usize;
        let mut right = to as usize;
        let y = y as usize;
        let backbuf = &mut self.backbuffer;

        let width = self.dimensions.0 as usize;

        if left >= right {
            mem::swap(&mut left, &mut right);
        }

        if left >= width {
            left = width;
        }

        if right >= width {
            right = width;
        }

        let mut offset = y * width + left;
        let mut p = pattern.rotate_right((left & 15) as u32);

        for _ in left..right {
            backbuf[offset] = if (p & 1) != 0 { 255 } else { 0 };
            p = p.rotate_right(1);
            offset += 1;
        }
    }

    fn vline(&mut self, at: (u16, u16), to: u16, pattern: u16) {
        let left = at.0 as usize;
        let mut top = at.1 as usize;
        let mut bottom = to as usize;
        let width = self.dimensions.0 as usize;
        let height = self.dimensions.1 as usize;

        if left >= width {
            return; // off surface; nothing to draw.
        }

        if top >= bottom {
            mem::swap(&mut top, &mut bottom);
        }

        if top >= height {
            top = height;
        }

        if bottom >= height {
            bottom = height;
        }

        let mut backbuf = &mut self.backbuffer;
        let mut offset = top * width + left;
        let mut p = pattern.rotate_right((top & 15) as u32);

        for _ in top..bottom {
            backbuf[offset] = if (p & 1) != 0 { 255 } else { 0 };
            p = p.rotate_right(1);
            offset += width;
        }
    }

    fn rect(&mut self, at: (u16, u16), to: (u16, u16), pattern: &[u16; 16]) {
        let mut top = at.1;
        let mut bottom = to.1;

        if top >= bottom {
            mem::swap(&mut top, &mut bottom);
        }

        for y in top..bottom {
            self.hline((at.0, y), to.0, pattern[(y & 15) as usize]);
        }
    }

    fn frame(&mut self, at: (u16, u16), to: (u16, u16), pattern: u16) {
        let mut left = at.0;
        let mut top = at.1;
        let mut right = to.0;
        let mut bottom = to.1;

        if left > right {
            mem::swap(&mut left, &mut right);
        }

        if top > bottom {
            mem::swap(&mut top, &mut bottom);
        }

        self.hline((left, top), right, pattern);
        self.hline((left, bottom - 1), right, pattern);
        self.vline((left, top), bottom, pattern);
        self.vline((right-1, top), bottom, pattern);
    }

    fn invert_line(&mut self, at: (u16, u16), to: u16) {
        let mut left = at.0 as usize;
        let y = at.1 as usize;
        let mut right = to as usize;
        let backbuf = &mut self.backbuffer;

        let width = self.dimensions.0 as usize;

        if left >= right {
            mem::swap(&mut left, &mut right);
        }

        if left >= width {
            left = width;
        }

        if right >= width {
            right = width;
        }

        let mut offset = y * width + left;

        for _ in left..right {
            backbuf[offset] = backbuf[offset] ^ 0xFF;
            offset += 1;
        }
    }

    fn invert_rect(&mut self, at: (u16, u16), to: (u16, u16)) {
        let mut top = at.1;
        let mut bottom = to.1;

        if top >= bottom {
            mem::swap(&mut top, &mut bottom);
        }

        for y in top..bottom {
            self.invert_line((at.0, y), to.0);
        }
    }

    fn copy_line(
        &mut self,
        from: (u16, u16),
        src_width: usize,
        from_bits: &[u16],
        to: (u16, u16),
        width: usize,
        function: u8
    ) {
        // First, expand the pen lookup table implied by `function`
        // into something we can index conveniently.
        // Index bit 1 maps to the source bit, while bit 0 maps to the destination bit.
        let mut pens : Vec<u8> = vec!(0, 0, 0, 0);
        for i in 0..4 {
            pens[i] = if (function & (1 << i)) == 0 { 0 } else { 255 };
        }

        // Source preparation.

        let src_left = from.0 as usize;
        let src_width_u16 = (src_width + 15) / 16;
        let mut soffset = ((from.1 as usize) * src_width_u16) + (src_left / 16);
        let mut ix = src_left & 15;
        let mut src_word = from_bits[soffset] >> ix;
        let src_width_adjusted = min(width, src_width - src_left);
        let largest_offset = from_bits.len();

        // Destination preparation.

        let mut doffset = ((to.1 as usize) * (self.dimensions.0 as usize)) + (to.0 as usize);
        let backbuf : &mut [u8] = &mut self.backbuffer;
        let dst_width_adjusted = min(width, (self.dimensions.0 - to.0) as usize);

        // Copy loop.

        let mut index : usize;
        for _ in 0..min(src_width_adjusted, dst_width_adjusted) {
            index = ((src_word & 1) as usize) | ((backbuf[doffset] & 2) as usize);
            backbuf[doffset] = pens[index];
            doffset += 1;

            if ix == 15 {
                ix = 0;
                soffset += 1;
                if soffset == largest_offset {
                    break;
                }
            }
            else {
                ix += 1;
            }
            src_word = if ix != 0 { src_word >> 1 } else { from_bits[soffset] };
        }
    }

    fn copy_line_big_endian(
        &mut self,
        from: (u16, u16),
        src_width: usize,
        from_bits: &[u16],
        to: (u16, u16),
        width: usize,
        function: u8
    ) {
        // First, expand the pen lookup table implied by `function`
        // into something we can index conveniently.
        // Index bit 1 maps to the source bit, while bit 0 maps to the destination bit.
        let mut pens : Vec<u8> = vec!(0, 0, 0, 0);
        for i in 0..4 {
            pens[i] = if (function & (1 << i)) == 0 { 0 } else { 255 };
        }

        // Source preparation.

        let src_left = from.0 as usize;
        let src_width_u16 = (src_width + 15) / 16;
        let mut soffset = ((from.1 as usize) * src_width_u16) + (src_left / 16);
        let mut ix = src_left & 15;
        let mut src_word = from_bits[soffset] << ix;
        let src_width_adjusted = min(width, src_width - src_left);
        let largest_offset = from_bits.len();

        // Destination preparation.

        let mut doffset = ((to.1 as usize) * (self.dimensions.0 as usize)) + (to.0 as usize);
        let backbuf : &mut [u8] = &mut self.backbuffer;
        let dst_width_adjusted = min(width, (self.dimensions.0 - to.0) as usize);

        // Copy loop.

        let mut index : usize;
        for _ in 0..min(src_width_adjusted, dst_width_adjusted) {
            index = (((src_word & 0x8000) >> 15) as usize) | ((backbuf[doffset] & 2) as usize);
            backbuf[doffset] = pens[index];
            doffset += 1;

            if ix == 15 {
                ix = 0;
                soffset += 1;
                if soffset == largest_offset {
                    break;
                }
            }
            else {
                ix += 1;
            }
            src_word = if ix != 0 { src_word << 1 } else { from_bits[soffset] };
        }
    }

    fn copy_rect(
        &mut self,
        from: (u16, u16),
        src_width: usize,
        from_bits: &[u16],
        to: (u16, u16),
        dimensions: (u16, u16),
        function: u8
    ) {
        if to.0 >= self.dimensions.0 {
            return;
        }

        if to.1 >= self.dimensions.1 {
            return;
        }

        let adjusted_bottom = min(to.1 + dimensions.1, self.dimensions.1);
        let adjusted_height = adjusted_bottom - to.1;

        for y in 0..adjusted_height {
            self.copy_line(
                (from.0, from.1 + y), src_width, from_bits,
                (to.0, to.1 + y), dimensions.0 as usize,
                function
            );
        }
    }

    fn copy_rect_big_endian(
        &mut self,
        from: (u16, u16),
        src_width: usize,
        from_bits: &[u16],
        to: (u16, u16),
        dimensions: (u16, u16),
        function: u8
    ) {
        if to.0 >= self.dimensions.0 {
            return;
        }

        if to.1 >= self.dimensions.1 {
            return;
        }

        let adjusted_bottom = min(to.1 + dimensions.1, self.dimensions.1);
        let adjusted_height = adjusted_bottom - to.1;

        for y in 0..adjusted_height {
            self.copy_line_big_endian(
                (from.0, from.1 + y), src_width, from_bits,
                (to.0, to.1 + y), dimensions.0 as usize,
                function
            );
        }
    }
}

