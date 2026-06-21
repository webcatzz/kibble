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
		#[allow(unused_unsafe)]
		{
			use std::ffi::CStr;
			use $crate::sdl_util::_SDL_GetError;
			panic!("SDL error: {:?}", unsafe { CStr::from_ptr(_SDL_GetError()) });
		}
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
		#[allow(unused_unsafe)]
		{
			use std::ffi::CStr;
			use $crate::sdl_util::_SDL_GetError;
			assert!($cond, "SDL error: {:?}", unsafe { CStr::from_ptr(_SDL_GetError()) });
		}
	};
}

pub use sdl_panic;
pub use sdl_assert;