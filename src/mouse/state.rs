use std::mem::MaybeUninit;

use sdl3_sys::mouse::SDL_GetMouseState;

use crate::math::Vec2;
use crate::mouse::MouseButtons;

/// Returns the current global mouse state.
///
/// For a safe interface, see [`EventQueue::mouse_state()`].
///
/// # Safety
///
/// Should only be called on the main thread.
///
/// [`EventQueue::mouse_state()`]: crate::event::EventQueue::mouse_state
pub unsafe fn state() -> (MouseButtons, Vec2<f32>) {
	let mut x = MaybeUninit::uninit();
	let mut y = MaybeUninit::uninit();
	let flags = unsafe { SDL_GetMouseState(x.as_mut_ptr(), y.as_mut_ptr()) };
	let pos = Vec2 { x: unsafe { x.assume_init() } as f32, y: unsafe { y.assume_init() } as f32 };
	(MouseButtons::from(flags), pos)
}