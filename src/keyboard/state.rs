use std::mem::MaybeUninit;
use std::ptr::NonNull;

use sdl3_sys::keyboard::*;

use super::Keycode;

/// Returns the global keyboard state.
///
/// For a safe interface, see [`EventQueue::keyboard_state()`].
///
/// # Safety
///
/// Should only be called on the main thread.
///
/// [`EventQueue::keyboard_state()`]: crate::event::EventQueue::keyboard_state
pub unsafe fn state() -> KeyboardState {
	// SAFETY: array returned by `SDL_GetKeyboardState()` is valid for 'static
	let mut len = MaybeUninit::uninit();
	let ptr = unsafe { SDL_GetKeyboardState(len.as_mut_ptr()) };
	KeyboardState { ptr: unsafe { NonNull::new_unchecked(ptr as *mut bool) }, len: unsafe { len.assume_init() } as usize }
}

/// Resets the global keyboard state.
///
/// Generates key-up events for all pressed keys.
///
/// For a safe interface, see [`EventQueue::reset_keyboard()`].
///
/// # Safety
///
/// Should only be called on the main thread.
///
/// [`EventQueue::reset_keyboard()`]: crate::event::EventQueue::reset_keyboard
pub unsafe fn reset() {
	unsafe { SDL_ResetKeyboard(); }
}

/// The global keyboard state.
///
/// Returned by [`keyboard::state()`].
///
/// [`keyboard::state()`]: state
#[derive(Clone, Copy)]
pub struct KeyboardState {
	ptr: NonNull<bool>,
	len: usize,
}

impl KeyboardState {

	/// Returns `true` if the key with the given keycode is pressed.
	pub fn is_down(self, code: Keycode) -> bool {
		let i = code.0 as usize;
		// SAFETY: short-circuits if `i` is beyond the bounds of the state array
		i < self.len && unsafe { self.ptr.add(i).read() }
	}

}