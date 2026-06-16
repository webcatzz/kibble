use std::mem::MaybeUninit;

use sdl3_sys::mouse::SDL_GetMouseState;

use crate::math::Vec2;

use super::MouseButtons;

/// Returns the current global mouse state.
///
/// For a safe interface, see [`EventQueue::mouse_state()`].
///
/// # Safety
///
/// Should only be called on the main thread.
///
/// [`EventQueue::mouse_state()`]: crate::event::EventQueue::mouse_state
pub unsafe fn mouse_state() -> MouseState {
	let mut x = MaybeUninit::uninit();
	let mut y = MaybeUninit::uninit();
	let flags = unsafe { SDL_GetMouseState(x.as_mut_ptr(), y.as_mut_ptr()) };
	let pos = Vec2 { x: unsafe { x.assume_init() } as f32, y: unsafe { y.assume_init() } as f32 };
	MouseState { buttons: MouseButtons::from(flags), pos }
}

/// Mouse state.
pub struct MouseState {
	/// The currently pressed buttons.
	pub buttons: MouseButtons,
	/// The position of the cursor, relative to the focused window.
	pub pos:     Vec2<f32>,
}