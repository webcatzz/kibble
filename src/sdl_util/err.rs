//! SDL error handling.

use std::ffi::CStr;
use std::fmt;

use sdl3_sys::error::SDL_GetError;

/// Represents SDL's current error message.
///
/// Note that any error message will be overwritten by subsequent SDL errors.
///
/// Implements [`Display`] to print the current SDL error message.
///
/// [`Display`]: fmt::Display
pub struct SdlErr;

impl fmt::Display for SdlErr {

	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		writeln!(f, "SDL error: {:?}", unsafe { CStr::from_ptr(SDL_GetError()) })
	}

}

impl fmt::Debug for SdlErr {

	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		fmt::Display::fmt(self, f)
	}

}

impl std::error::Error for SdlErr {}

/// Panics with the current SDL error message.
///
/// # Usage
///
/// ```should_panic
/// # use kibble::sdl_util::sdl_panic;
/// sdl_panic!(); // Panics and prints the current SDL error message
/// ```
#[macro_export]
#[doc(hidden)]
macro_rules! sdl_panic {
	() => {
		panic!("{}", crate::sdl_util::SdlErr)
	};
}

/// Asserts a condition, panicking with the current SDL error message on
/// failure.
///
/// # Usage
///
/// ```
/// # use kibble::sdl_util::sdl_assert;
/// sdl_assert!(true);
/// ```
///
/// ```should_panic
/// # use kibble::sdl_util::sdl_assert;
/// sdl_assert!(false); // Panics and prints the current SDL error message
/// ```
#[macro_export]
#[doc(hidden)]
macro_rules! sdl_assert {
	($cond:expr) => {
		assert!($cond, "{}", crate::sdl_util::SdlErr)
	};
}

pub use sdl_panic;
pub use sdl_assert;