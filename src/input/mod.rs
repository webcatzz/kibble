//! Input.

#[cfg(feature = "keyboard")]
mod keyboard;
#[cfg(feature = "mouse")]
mod mouse;

#[cfg(feature = "keyboard")]
pub use keyboard::*;
#[cfg(feature = "mouse")]
pub use mouse::*;