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
use sdl2::{render, video};

use std::result;


/// Indication of an error somewhere inside the VDI module.
#[derive(Debug)]
pub enum VdiError {
    FromSdl(String),
    Miscellaneous,
}


/// VDI drivers must conform to this interface.
pub trait VDI {
}


/// This structure records the state of the VDI.
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
}

impl<'a> SDL2Vdi<'a> {
    /// Create a new SDL2-backed VDI instance.
    /// This will open a window and
    /// create an appropriately-sized frame buffer to back it.
    /// At present, the bitmap is monochrome: 0s are black, 1s are white.
    pub fn new(context: &'a sdl2::Sdl, width: u16, height: u16, title: &'a str) ->
                result::Result<SDL2Vdi<'a>, VdiError> {
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

        Ok(SDL2Vdi {
            dimensions: (width, height),
            title:      title,
            sdl:        context,
            video:      video_subsystem,
            renderer:   r,
        })
    }
}

impl<'a> VDI for SDL2Vdi<'a> {
}

