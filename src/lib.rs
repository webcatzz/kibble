//! a little game toolkit.
//!
//! built on top of [simple directmedia layer] (SDL), a cross-platform
//! development library.
//!
//! # why kibble?
//!
//! - kibble only provides the basics. you architect as you think best.
//! - kibble is small-by-design. no long compilations, no unneeded features.
//! - kibble is built on [SDL], a popular cross-platform library. [anywhere SDL
//!   can run], you can run too.
//! - kibble is extensible. underlying SDL types can be exposed.
//!
//! # getting started
//!
//! implementing your program's main loop is up to you! all you have to do is
//! make sure your program exits when told (i.e. when it recieves the
//! [`Event::Quit`] event). see the [`EventQueue`] documentation for examples.
//!
//! for more, skim the modules below.
//!
//! [simple directmedia layer]: https://www.libsdl.org/
//! [SDL]: https://www.libsdl.org/
//! [anywhere SDL can run]: https://wiki.libsdl.org/sdl3/readme-platforms
//! [`Event::Quit`]: event::Event::Quit
//! [`EventQueue`]: event::EventQueue

pub mod input;
pub mod math;
pub mod thread;

#[cfg(feature = "app")]
pub mod app;
#[cfg(feature = "audio")]
pub mod audio;
#[cfg(feature = "event")]
pub mod event;
#[cfg(feature = "meta")]
pub mod meta;
#[cfg(feature = "render")]
pub mod render;
#[cfg(feature = "window")]
pub mod window;

#[cfg(feature = "sdl_util")]
pub mod sdl_util;
#[cfg(not(feature = "sdl_util"))]
mod sdl_util;