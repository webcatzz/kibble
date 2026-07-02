use std::mem::MaybeUninit;

use sdl3_sys::mouse::SDL_GetMouseState;

use crate::math::Vec2;
use crate::thread;

use super::MouseButtons;

/// Returns the current global mouse state.
///
/// # Panics
///
/// Panics if called outside the main thread.
pub fn mouse_state() -> MouseState {
	assert!(thread::is_main(), "`mouse_state()` should only be called on the main thread");
	unsafe { mouse_state_unchecked() }
}

/// Returns the current global mouse state.
///
/// # Safety
///
/// Should only be called on the main thread.
pub unsafe fn mouse_state_unchecked() -> MouseState {
	let mut x = MaybeUninit::uninit();
	let mut y = MaybeUninit::uninit();
	let flags = unsafe { SDL_GetMouseState(x.as_mut_ptr(), y.as_mut_ptr()) };
	let pos = Vec2 { x: unsafe { x.assume_init() } as f32, y: unsafe { y.assume_init() } as f32 };
	MouseState { buttons: MouseButtons::from(flags), pos }
}

/// Mouse state.
#[derive(Default, Clone, Copy)]
pub struct MouseState {
	/// The currently pressed buttons.
	pub buttons: MouseButtons,
	/// The position of the cursor, relative to the focused window.
	pub pos:     Vec2<f32>,
}