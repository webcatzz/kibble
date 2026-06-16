//! SDL utilities.
//!
//! Kibble only provides a subset of SDL's capabilities. If you'd like to use
//! more of SDL, you should interface with it directly. Note that interfacing
//! with SDL, especially types underlying Kibble types, is by nature unsafe.
//!
//! This module provides utilities from Kibble's implementation:
//!
//! - Panicking with SDL error messages, through [`sdl_panic!`] and
//!   [`sdl_assert!`],
//! - Access to the SDL representations underlying Kibble's types, through the
//!   [`AsSdlExt`] extension trait.
//!
//! Kibble uses the [`sdl3_sys`] Rust bindings.
//!
//! # Notes on memory management
//!
//! SDL requires explicit destructors to release its memory, which Kibble
//! manages automatically through Rust lifetimes. If you'd like to create your
//! own wrapper types around SDL, it's best to match Kibble's design pattern for
//! managing SDL deinitialization.
//!
//! SDL is organized neatly into subsystems, each of which can be individually
//! initialized and deinitialized. Kibble initializes individual subsystems when
//! values of types such as [`Window`], [`EventQueue`], and [`AudioDevice`] are
//! created, and deinititalizes them when those values are dropped. Since SDL
//! subsystem initializations are reference-counted, they can be "initialized"
//! multiple times, and just need to be "deinitialized" that many times to be
//! cleaned up.
//!
//! However, even if all its subsystems are cleaned up, SDL *still* requires a
//! global destructor to be run to clean up remaining memory. Kibble calls this
//! destructor exactly once all SDL subsytems have been deinitialized.
//!
//! Here's [`Window`]'s drop implementation, for example:
//!
//! ```ignore
//! fn drop(&mut self) {
//!   // ...
//!   SDL_QuitSubSystem(SDL_INIT_VIDEO);
//!   sdl_util::quit_if_unused();
//!   // ...
//! }
//! ```
//!
//! Importantly, the drop implementation deinitializes a subsytem, then calls
//! [`quit_if_unused()`] to clean up SDL if no more subsystems are initialized.
//! If you wrap subsystem initializations, you should also call
//! [`quit_if_unused()`] once you deinitialize them to clean up SDL's remaining
//! memory.
//!
//! [`Window`]: crate::window::Window
//! [`EventQueue`]: crate::event::EventQueue
//! [`AudioDevice`]: crate::audio::AudioDevice

mod as_sdl;
mod panic;
mod quit;

pub use as_sdl::AsSdlExt;
pub use quit::quit_if_unused;

#[doc(inline)]
pub use panic::{sdl_assert, sdl_panic};

#[doc(hidden)]
pub use sdl3_sys::error::SDL_GetError as _SDL_GetError; // Exposed for panic macros