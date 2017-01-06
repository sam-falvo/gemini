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

use std::{result, vec};


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
    fn draw_point(&mut self, (u16, u16), u8);

    /// Retrieves the current pixel value at a given position.
    fn get_point(&self, (u16, u16)) -> u8;
}


/// This structure represents an SDL2-backed VDI surface (bluntly, a window).
/// The window is fixed in size, emulating the frame buffer of a given size.
/// When the window opens, the state of the frame buffer is completely undefined.
/// You'll need to paint the frame buffer to establish a known image.
pub struct SDL2Vdi<'a> {
    /// The dimensions field allows for a display surface up to 64Kx64K in size.
    dimensions: (u16, u16),

    /// When the window opens, this is the title to assign it.
    title: &'a str,

    /// Borrowed SDL context.
    sdl: &'a sdl2::Sdl,

    /// Borrowed SDL context.
    video: sdl2::VideoSubsystem,

    /// Renderer (from which we can get the window again if we need to)
    renderer: render::Renderer<'static>,

    /// Texture used to contain the frame buffer for the window.
    texture: render::Texture,

    /// Back-buffer to support get_point().
    backbuffer: Vec<u8>,
}


impl<'a> SDL2Vdi<'a> {
    /// Create a new SDL2-backed VDI instance.
    /// This will open a window and
    /// create an appropriately-sized frame buffer to back it.
    /// At present, the bitmap is monochrome: 0s are black, 1s are white.
    ///
    /// width and height are measured in pixels.
    pub fn new(context: &'a sdl2::Sdl, width: u16, height: u16, title: &'a str) ->
                result::Result<SDL2Vdi<'a>, VdiError> {
        let total_pixels = width as usize * height as usize;
        let backbuffer = Vec::with_capacity(total_pixels);

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
            title:      title,
            sdl:        context,
            video:      video_subsystem,
            renderer:   r,
            texture:    t,
            backbuffer: backbuffer,
        })
    }
}


impl<'a> VDI for SDL2Vdi<'a> {
    fn draw_point(&mut self, at: (u16, u16), pen: u8) {
        let (x, y) = at;
    let (width, height) = self.dimensions;

        if (x >= width) || (y >= height) {
            return;
        }

        let p = if pen >= 128 { 255 } else { 0 };

        (&mut self.texture).with_lock(None, |bits: &mut [u8], span: usize| {
            let (x, y) = (x as usize, y as usize);
            let offset = y * span + (4 * x);
            bits[offset+0] = p;
            bits[offset+1] = p;
            bits[offset+2] = p;
            bits[offset+3] = p;
        });
    }

    fn get_point(&self, at: (u16, u16)) -> u8 {
        0
    }
}

impl<'a> Drop for SDL2Vdi<'a> {
    fn drop(&mut self) {
        println!("DROPPING SDL2Vdi");
    }
}

