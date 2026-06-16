//! a little game toolkit.
//!
//! built on top of [simple directmedia layer] (sdl), a cross-platform
//! development library.
//!
//! # why kibble?
//!
//! - kibble only provides you tools. you architect as you think best.
//! - kibble is small-by-design. no long compilations, no unneeded features.
//! - kibble is modular. don't want it? don't need it.
//! - kibble is built on [sdl], a popular cross-platform library. [anywhere sdl
//!   can run], kibble can run too.
//! - kibble is extensible. underlying sdl types are exposed through the
//!   [`AsSdlExt`] trait.
//!
//! # getting started
//!
//! implementing your program's main loop is up to you! all you have to do is
//! make sure your program exits when told. see the [`event`] documentation for
//! an example.
//!
//! for more, skim the modules below.
//!
//! [simple directmedia layer]: https://www.libsdl.org/
//! [sdl]: https://www.libsdl.org/
//! [anywhere sdl can run]: https://wiki.libsdl.org/sdl3/readme-platforms
//! [`assdlext`]: sdl_util::assdlext
//! [`run`]: run::run
//! [`run::run()`]: run::run::run

pub mod app;
pub mod input;
pub mod math;
pub mod thread;

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