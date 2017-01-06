//! # VDI
//! This module corresponds to the Video Driver Interface (VDI).
//! It provides basic primitives for displaying simple graphics.
//! It is expressly not intended to replace something like Cairo.
//!
//! Influenced more by GEOS than by GEM's VDI, this module allows
//! applications to scribble on the entire display surface.  No
//! support for clipping exists (except for the edges of the display
//! surface of course).
//!
//! Inspirations:
//! - GEM -- its namesake in GEM is more powerful, but otherwise fills
//!          the same role.
//! - GEOS -- Pretty much the entirety of GEOS is one big graphics driver
//!           for a monochrome bitmap, which is (for now at least) what
//!           this driver module is intended to replicate in order to keep
//!           things very simple.


use sdl2;
use sdl2::{pixels, render, video};

use std::{mem, result};


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

    /// Commit tells the sends the current contents of the VDI frame buffer
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
}


/// This structure represents an SDL2-backed VDI surface (bluntly, a window).
/// The window is fixed in size, emulating the frame buffer of a given size.
/// When the window opens, the state of the frame buffer is completely undefined.
/// You'll need to paint the frame buffer to establish a known image.
pub struct SDL2Vdi {
    /// The dimensions field allows for a display surface up to 64Kx64K in size.
    dimensions: (u16, u16),

    /// Renderer (from which we can get the window again if we need to)
    renderer: render::Renderer<'static>,

    /// Texture used to contain the frame buffer for the window.
    texture: render::Texture,

    /// Back-buffer to support get_point().
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
}

impl Drop for SDL2Vdi {
    fn drop(&mut self) {
        println!("DROPPING SDL2Vdi");
    }
}

