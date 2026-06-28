//! Windows.
//!
//! See the [`Window`] documentation.

use std::ffi::{c_int, CStr, CString};
use std::mem::MaybeUninit;
use std::ptr::NonNull;

use sdl3_sys::init::{SDL_InitSubSystem, SDL_QuitSubSystem, SDL_INIT_VIDEO};
use sdl3_sys::keyboard::SDL_ScreenKeyboardShown;
use sdl3_sys::video::*;

use crate::math::Vec2;
use crate::sdl_util::{self, AsSdlExt, sdl_assert, sdl_panic};
use crate::thread;

/// An open window.
///
/// # Examples
///
/// To open a window:
///
/// ```
/// # use kibble::math::Vec2;
/// # use kibble::window::Window;
/// let window = Window::new("title", Vec2 { x: 800, y: 600 });
/// ```
///
/// The window will be automatically closed when dropped.
///
/// To render to the window, use its next [`Frame`]:
///
/// ```
/// # use kibble::math::{Color, Vec2};
/// # use kibble::window::Window;
/// # let mut window = Window::new("title", Vec2 { x: 800, y: 600 });
/// let frame = window.frame();
/// ```
///
/// See the [`Frame`] documentation for examples.
pub struct Window(NonNull<SDL_Window>);

impl Window {

	/// Opens a new window.
	///
	/// # Panics
	///
	/// Panics if called outside the main thread.
	#[doc(alias = "open")]
	pub fn new(title: &str, size: Vec2<u32>) -> Self {
		assert!(thread::is_main(), "`Window::new()` should only be called on the main thread");
		unsafe { Self::new_unchecked(title, size) }
	}

	/// Opens a new window.
	///
	/// # Safety
	///
	/// Should only be called on the main thread.
	pub unsafe fn new_unchecked(title: &str, size: Vec2<u32>) -> Self {
		sdl_assert!(unsafe { SDL_InitSubSystem(SDL_INIT_VIDEO) });
		let Some(sdl_window) = NonNull::new(unsafe { SDL_CreateWindow(CString::new(title).unwrap().as_ptr(), size.x as c_int, size.y as c_int, SDL_WINDOW_HIGH_PIXEL_DENSITY) }) else { sdl_panic!() };
		Self(sdl_window)
	}

	/// Returns the title of the window.
	pub fn title(&self) -> String {
		let ptr = unsafe { SDL_GetWindowTitle(self.as_sdl()) };
		if ptr.is_null() { sdl_panic!(); }
		// SAFETY: `SDL_GetWindowTitle()` always returns valid UTF-8
		let str = unsafe { CStr::from_ptr(ptr).to_str().unwrap_unchecked() };
		str.to_owned()
	}

	/// Sets the title of the window.
	pub fn set_title(&mut self, title: &str) {
		sdl_assert!(unsafe { SDL_SetWindowTitle(self.as_sdl(), CString::new(title).unwrap().as_ptr()) });
	}

	/// Returns the size of the window.
	pub fn size(&self) -> Vec2<u32> {
		let mut w = MaybeUninit::uninit();
		let mut h = MaybeUninit::uninit();
		sdl_assert!(unsafe { SDL_GetWindowSizeInPixels(self.as_sdl(), w.as_mut_ptr(), h.as_mut_ptr()) });
		Vec2 { x: unsafe { w.assume_init() } as u32, y: unsafe { h.assume_init() } as u32 }
	}

	/// Sets the size of the window.
	pub fn set_size(&mut self, size: Vec2<u32>) {
		sdl_assert!(unsafe { SDL_SetWindowSize(self.as_sdl(), size.x as c_int, size.y as c_int) });
	}

	/// Returns the minimum size of the window.
	pub fn min_size(&self) -> Vec2<u32> {
		let mut w = MaybeUninit::uninit();
		let mut h = MaybeUninit::uninit();
		sdl_assert!(unsafe { SDL_GetWindowMinimumSize(self.as_sdl(), w.as_mut_ptr(), h.as_mut_ptr()) });
		Vec2 { x: unsafe { w.assume_init() } as u32, y: unsafe { h.assume_init() } as u32 }
	}

	/// Sets the minimum size of the window.
	pub fn set_min_size(&mut self, min_size: Vec2<u32>) {
		sdl_assert!(unsafe { SDL_SetWindowMinimumSize(self.as_sdl(), min_size.x as c_int, min_size.y as c_int) });
	}

	/// Returns the maximum size of the window.
	pub fn max_size(&self) -> Vec2<u32> {
		let mut w = MaybeUninit::uninit();
		let mut h = MaybeUninit::uninit();
		sdl_assert!(unsafe { SDL_GetWindowMaximumSize(self.as_sdl(), w.as_mut_ptr(), h.as_mut_ptr()) });
		Vec2 { x: unsafe { w.assume_init() } as u32, y: unsafe { h.assume_init() } as u32 }
	}

	/// Sets the maximum size of the window.
	pub fn set_max_size(&mut self, max_size: Vec2<u32>) {
		sdl_assert!(unsafe { SDL_SetWindowMaximumSize(self.as_sdl(), max_size.x as c_int, max_size.y as c_int) });
	}

	/// Returns `true` if the window is visible.
	pub fn visible(&self) -> bool {
		unsafe { SDL_GetWindowFlags(self.as_sdl()) & SDL_WINDOW_HIDDEN == 0 }
	}

	/// Makes the window visible.
	pub fn show(&mut self) {
		sdl_assert!(unsafe { SDL_ShowWindow(self.as_sdl()) });
	}

	/// Hides the window.
	pub fn hide(&mut self) {
		sdl_assert!(unsafe { SDL_HideWindow(self.as_sdl()) });
	}

	/// Centers the window on the screen.
	pub fn center(&mut self) {
		sdl_assert!(unsafe { SDL_SetWindowPosition(self.as_sdl(), SDL_WINDOWPOS_CENTERED, SDL_WINDOWPOS_CENTERED) });
	}

	/// Sets whether the window can be resized by the user.
	pub fn set_resizable(&mut self, resizable: bool) {
		sdl_assert!(unsafe { SDL_SetWindowResizable(self.as_sdl(), resizable) });
	}

	/// Returns `true` if the screen keyboard is shown for the window.
	pub fn is_screen_keyboard_shown(&self) -> bool {
		unsafe { SDL_ScreenKeyboardShown(self.as_sdl()) }
	}

}

impl AsSdlExt for Window {

	type Sdl = *mut SDL_Window;

	fn as_sdl(&self) -> Self::Sdl {
		self.0.as_ptr()
	}

}

impl Drop for Window {

	fn drop(&mut self) {
		unsafe {
			SDL_DestroyWindow(self.as_sdl());
			SDL_QuitSubSystem(SDL_INIT_VIDEO);
			sdl_util::quit_if_unused();
		}
	}

}