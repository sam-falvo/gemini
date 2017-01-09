//! Gemini is a graphical user interface library for the Rust programming language.
//! The name derives from GEM, a user interface that I wish to model in the longer term.
//! For now, however, Gemini borrows from a UI library that I'm much more familiar with,
//! the Commodore 64 version of GEOS.
//!
//! Currently, Gemini models a monochrome user interface (black and white only).
//! Compared to other user interface libraries and toolkits, this significantly reduces the learning curve.
//! While more primitive,
//! getting an application up and running with Gemini has proven to be much quicker than with other GUI libraries.
//! Support for color is not planned for the immediate future,
//! but longer-term,
//! color support will become necessary.
//! That bridge will be crossed when we get there.
//!
//! One of the reasons Gemini is more productive is probably because
//! Gemini does not intend to become a *framework*.
//! I'm a strong believer in "libraries over frameworks" philosophy, and have built Gemini to be a library.
//! This makes Gemini substantially easier to integrate with other environments if/when it becomes necessary.
//! Currently, though, Gemini is built around the rust-sdl2 library.


extern crate sdl2;


pub mod vdi;
pub mod font;


mod system_font;
