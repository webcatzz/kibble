//! A little game toolkit.
//!
//! Built on top of [Simple DirectMedia Layer] (SDL), a cross-platform
//! development library.
//!
//! # Why Kibble?
//!
//! - Kibble only provides you tools. You architect as you think best.
//! - Kibble is small-by-design. No long compilations, no unneeded features.
//! - Kibble is modular. Don't want it? Don't need it.
//! - Kibble is built on [SDL], a popular cross-platform library. [Anywhere SDL
//!   can run], Kibble can run too.
//! - Kibble is extensible. Underlying SDL types are exposed through the
//!   [`AsSdlExt`] trait.
//!
//! # Getting started
//!
//! The [`Run`] trait provides a reasonable main loop through [`Run::run()`],
//! which may be useful to you:
//!
//! ```no_run
//! # use kibble::event::Event;
//! # use kibble::run::Run;
//! #
//! struct Game { /* ... */ }
//! #
//! # impl Game {
//! # fn new() -> Self { Self { } }
//! # }
//!
//! impl Run for Game {
//!
//!   fn listen(&mut self, event: &Event) { /* handle events... */ }
//!
//!   fn update(&mut self) { /* update the game state... */ }
//!
//!   fn render(&mut self, delta: f32) { /* render the game... */ }
//!
//! }
//!
//! fn main() {
//!   Game::new().run();
//! }
//! ```
//!
//! Note that [`Run`] is only provided for convenience. You are encouraged to
//! write your own main loop if you wish!
//!
//! For more, skim the modules below.
//!
//! [Simple DirectMedia Layer]: https://www.libsdl.org/
//! [SDL]: https://www.libsdl.org/
//! [Anywhere SDL can run]: https://wiki.libsdl.org/SDL3/README-platforms
//! [`AsSdlExt`]: sdl_util::AsSdlExt
//! [`Run`]: run::Run
//! [`Run::run()`]: run::Run::run

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