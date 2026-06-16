//! Threads.

use std::marker::PhantomData;

use sdl3_sys::init::SDL_IsMainThread;

use crate::audio::{AudioDevice, AudioDeviceId, AudioFormat};
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

	/// Returns a [`Sys`].
	///
	/// # Panics
	///
	/// Panics if called outside the main thread.
	pub fn new() -> Self {
		assert!(is_main(), "`Mtm::new()` should only be called on the main thread");
		unsafe { Self::new_unchecked() }
	}

	/// Returns a [`Sys`].
	///
	/// # Safety
	///
	/// Should only be called on the main thread.
	pub unsafe fn new_unchecked() -> Self {
		Self(PhantomData)
	}

	/// Opens a new window.
	pub fn open_window(self, title: &str, size: Vec2<u32>) -> Window {
		unsafe { Window::new_unchecked(title, size) }
	}

	/// Returns an interface for the audio device with the given ID.
	///
	/// If you don't need to use a specific device, use the IDs
	/// [`DEFAULT_PLAYBACK`] or [`DEFAULT_RECORDING`] for a reasonable default.
	/// If the default audio device ever changes (e.g. it is plugged in or
	/// unplugged), the interface will seamlessly switch to the new device.
	///
	/// You may request a specific format for the audio device. The device may
	/// not honor the request. If `format` is `None`, uses a reasonable default.
	///
	/// [`DEFAULT_PLAYBACK`]: AudioDeviceId::DEFAULT_PLAYBACK
	/// [`DEFAULT_RECORDING`]: AudioDeviceId::DEFAULT_RECORDING
	pub fn open_audio_device(self, id: AudioDeviceId, format: Option<AudioFormat>) -> AudioDevice {
		unsafe { AudioDevice::new_unchecked(id, format) }
	}

}