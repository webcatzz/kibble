//! App interface.
//!
//! Most platforms support user-defined program loops, with some notable
//! exceptions. On web platforms, for example, a loop of any kind will freeze
//! the page. These platforms require programs to be updated by the platform's
//! own callbacks, so this app interface is provided for maximum compatibility.
//!
//! # Examples
//!
//! Instead of writing your own `main()` function, implement [`App`] and use
//! [`run_app`]:
//!
//! ```
//! pub struct Game { /* ... */ }
//!
//! impl App for Game {
//!   /* ... */
//! }
//!
//! fn main() {
//!   run_app!(Game);
//! }
//! ```

use std::ffi::c_int;
use std::mem;

use sdl3_sys::init::{SDL_APP_CONTINUE, SDL_APP_FAILURE, SDL_APP_SUCCESS, SDL_AppResult};
#[doc(hidden)]
pub use sdl3_sys as __sdl3_sys;

use crate::event::Event;
use crate::thread::Mtm;

/// An app interface.
pub trait App: Sized {

	/// Called once at the beginning of the program to initialize the
	/// application.
	///
	/// Always called on the main thread.
	fn init(mtm: Mtm) -> AppStatus<Self>;

	/// Always called on the main thread.
	fn iter(&mut self) -> AppStatus;

	/// May be called outside the main thread.
	unsafe fn listen(&mut self, event: &Event) -> AppStatus;

}

/// A status code returned by an application callback.
#[repr(i32)]
pub enum AppStatus<T = ()> {
	Running(T) = SDL_APP_CONTINUE.0 as i32,
	Succeeded  = SDL_APP_SUCCESS.0  as i32,
	Failed     = SDL_APP_FAILURE.0  as i32,
}

impl From<SDL_AppResult> for AppStatus {

	fn from(value: SDL_AppResult) -> Self {
		// SAFETY: `SDL_AppResult` has same layout and variants as `AppStatus<()>`
		unsafe { mem::transmute(value.0 as i32) }
	}

}

impl Into<SDL_AppResult> for AppStatus {

	fn into(self) -> SDL_AppResult {
		// SAFETY: `AppStatus<()>` has same layout and variants as `SDL_AppResult`
		SDL_AppResult(unsafe { mem::transmute::<Self, i32>(self) } as c_int)
	}

}

/// Runs an [`App`] implementation.
///
/// Should be called as the only line in the program's `main()`. See [Usage] for
/// an example.
///
/// # Usage
///
/// ```
/// # use kibble::app::{App, AppResult, run_app};
/// struct Game { /* ... */ }
///
/// impl App for Game {
/// 	/* ... */
/// }
///
/// fn main() {
///   run_app!(Game);
/// }
/// ```
///
/// [Usage]: #usage
#[macro_export]
#[doc(hidden)]
macro_rules! run_app {
	($app:ty) => {
		pub fn main() {
			use std::ffi::{c_char, c_int, c_void};
			use $crate::app::__sdl3_sys::events::SDL_Event;
			use $crate::app::__sdl3_sys::init::*;
			use $crate::app::__sdl3_sys::main::*;
			use $crate::app::{App, AppStatus};
			use $crate::thread::Mtm;

			unsafe extern "C" fn init<T: App>(appstate: *mut *mut c_void, argc: c_int, argv: *mut *mut c_char) -> SDL_AppResult {
				// SAFETY: this function is only ever called on the main thread
				match <$app>::init(unsafe { Mtm::open_unchecked() }) {
					AppStatus::Running(state) => {
						unsafe { appstate.write(Box::into_raw(Box::new(state)) as *mut c_void); }
						SDL_APP_CONTINUE
					}
					AppStatus::Succeeded => SDL_APP_SUCCESS,
					AppStatus::Failed => SDL_APP_FAILURE,
				}
			}

			unsafe extern "C" fn iter<T: App>(appstate: *mut c_void) -> SDL_AppResult {
				// SAFETY: `appstate` is only accessed by these SDL callbacks
				// SAFETY: `appstate` is non-null since it was leaked from a `Box`
				let state = unsafe { (appstate as *mut $app).as_mut_unchecked() };
				state.iter().into()
			}

			unsafe extern "C" fn event<T: App>(appstate: *mut c_void, event: *mut SDL_Event) -> SDL_AppResult {
				// SAFETY: `appstate` is only accessed by these SDL callbacks
				// SAFETY: `appstate` is non-null since it was leaked from a `Box`
				let state = unsafe { (appstate as *mut $app).as_mut_unchecked() };
				let Ok(event) = unsafe { *event }.try_into() else { return SDL_APP_CONTINUE; };
				unsafe { state.listen(&event) }.into()
			}

			unsafe extern "C" fn quit<T: App>(appstate: *mut c_void, result: SDL_AppResult) {
				// SAFETY: `appstate` was leaked from a `Box`
				// SAFETY: nothing reads `appstate` again; this callback is run right before the program exits
				drop(unsafe { Box::from_raw(appstate as *mut $app) });
			}

			unsafe { SDL_EnterAppMainCallbacks(0, std::ptr::null_mut(), Some(init::<$app>), Some(iter::<$app>), Some(event::<$app>), Some(quit::<$app>)); }
		}
	};
}

#[doc(inline)]
pub use run_app;