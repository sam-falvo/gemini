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


extern crate sdl2;
use sdl2::*;


/// VDI drivers must conform to this interface.
pub trait VDI {
}

/// This structure records the state of the VDI.
#[derive(Debug)]
pub struct SDL2Vdi<'a> {
    /// The dimensions field allows for a display surface up to 64Kx64K in size.
    dimensions: (u16, u16),

    /// Borrowed SDL context.
    sdl: &'a sdl2::Sdl,
}

impl<'a> SDL2Vdi<'a> {
    /// Create a new SDL2-backed VDI instance.
    /// This will open a window and
    /// create an appropriately-sized frame buffer to back it.
    /// At present, the bitmap is monochrome: 0s are black, 1s are white.
    pub fn new(context: &'a sdl2::Sdl, width: u16, height: u16) -> Self {
        SDL2Vdi {
            dimensions: (width, height),
            sdl: context,
        }
    }
}

impl<'a> VDI for SDL2Vdi<'a> {
}

