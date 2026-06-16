//! Threads.

use std::marker::PhantomData;

use sdl3_sys::init::SDL_IsMainThread;

use crate::audio::{AudioDevice, AudioDeviceId, AudioFormat};
use crate::input::{self, KeyboardState, MouseState};
use crate::math::Vec2;
use crate::window::Window;

/// Returns `true` if called on the main thread.
pub fn is_main() -> bool {
	unsafe { SDL_IsMainThread() }
}

/// A main thread marker.
///
/// A zero-sized type that can only exist on the main thread. Provides a safe
/// interface for constructing thread-unsafe types.
#[derive(Clone, Copy)]
pub struct Mtm(PhantomData<*const ()>); // Phantom pointer for `!Send` and `!Sync`

impl Mtm {

	/// Returns a main thread marker.
	///
	/// # Panics
	///
	/// Panics if called outside the main thread.
	pub fn new() -> Self {
		assert!(is_main(), "`Mtm::new()` should only be called on the main thread");
		unsafe { Self::new_unchecked() }
	}

	/// Returns a main thread marker.
	///
	/// # Safety
	///
	/// Should only be called on the main thread.
	pub unsafe fn new_unchecked() -> Self {
		Self(PhantomData)
	}

	/// Safe interface for [`Window::new_unchecked()`].
	pub fn open_window(self, title: &str, size: Vec2<u32>) -> Window {
		unsafe { Window::new_unchecked(title, size) }
	}

	/// Safe interface for [`AudioDevice::new_unchecked()`].
	pub fn open_audio_device(self, id: AudioDeviceId, format: Option<AudioFormat>) -> AudioDevice {
		unsafe { AudioDevice::new_unchecked(id, format) }
	}

	/// Safe interface for [`input::keyboard_state()`].
	pub fn keyboard_state(self) -> KeyboardState {
		unsafe { input::keyboard_state() }
	}

	/// Safe interface for [`input::reset_keyboard()`].
	pub fn reset_keyboard(self) {
		unsafe { input::reset_keyboard(); }
	}

	/// Safe interface for [`input::mouse_state()`].
	pub fn mouse_state(self) -> MouseState {
		unsafe { input::mouse_state() }
	}

}