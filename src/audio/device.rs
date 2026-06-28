use std::mem::MaybeUninit;
use std::num::NonZeroU32;
use std::ptr;

use sdl3_sys::audio::*;
use sdl3_sys::init::{SDL_INIT_AUDIO, SDL_InitSubSystem, SDL_QuitSubSystem};

use crate::sdl_util::{self, AsSdlExt, sdl_assert, sdl_panic};
use crate::thread;

use super::AudioFormat;

/// A logical audio device.
///
/// # Examples
///
/// To open an audio device for playback:
///
/// ```
/// # use kibble::audio::AudioDevice;
/// # use kibble::audio::AudioDeviceId;
/// let device = AudioDevice::new(AudioDeviceId::DEFAULT_PLAYBACK, None);
/// ```
pub struct AudioDevice(AudioDeviceId);

impl AudioDevice {

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
	///
	/// # Panics
	///
	/// Panics if called outside the main thread.
	pub fn new(id: AudioDeviceId, format: Option<AudioFormat>) -> Self {
		assert!(thread::is_main(), "`AudioDevice::new()` should only be called on the main thread");
		unsafe { Self::new_unchecked(id, format) }
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
	///
	/// # Safety
	///
	/// Should only be called on the main thread.
	pub unsafe fn new_unchecked(id: AudioDeviceId, format: Option<AudioFormat>) -> Self {
		sdl_assert!(unsafe { SDL_InitSubSystem(SDL_INIT_AUDIO) });
		let id = unsafe { SDL_OpenAudioDevice(id.as_sdl(), format.map(Into::into).as_ref().map(ptr::from_ref).unwrap_or_default()) };
		let Some(id) = AudioDeviceId::new(id.0) else { sdl_panic!() };
		Self(id)
	}

	/// Returns the unique ID of the device.
	pub fn id(&self) -> AudioDeviceId {
		self.0
	}

	/// Returns the audio format currently used by the device.
	pub fn format(&self) -> AudioFormat {
		self.0.format()
	}

	/// Sets if an audio device is playing or paused.
	///
	/// Any [`AudioStream`]s bound to a paused device will not progress.
	pub fn set_paused(&mut self, paused: bool) {
		if paused {
			sdl_assert!(unsafe { SDL_PauseAudioDevice(self.as_sdl()) });
		} else {
			sdl_assert!(unsafe { SDL_ResumeAudioDevice(self.as_sdl()) });
		}
	}

	/// Returns true if the audio device is paused.
	///
	/// New devices are paused by default.
	pub fn is_paused(&self) -> bool {
		unsafe { SDL_AudioDevicePaused(self.as_sdl()) }
	}

}

impl AsSdlExt for AudioDevice {

	type Sdl = SDL_AudioDeviceID;

	fn as_sdl(&self) -> Self::Sdl {
		self.0.as_sdl()
	}

}

impl Drop for AudioDevice {

	fn drop(&mut self) {
		unsafe {
			SDL_CloseAudioDevice(self.as_sdl());
			SDL_QuitSubSystem(SDL_INIT_AUDIO);
			sdl_util::quit_if_unused();
		}
	}

}

/// A unique ID for an audio device.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct AudioDeviceId(NonZeroU32);

impl AudioDeviceId {

	/// The ID for the default audio playback device.
	pub const DEFAULT_PLAYBACK:  Self = Self(NonZeroU32::new(SDL_AUDIO_DEVICE_DEFAULT_PLAYBACK.0).unwrap());
	/// The ID for the default audio recording device.
	pub const DEFAULT_RECORDING: Self = Self(NonZeroU32::new(SDL_AUDIO_DEVICE_DEFAULT_RECORDING.0).unwrap());

	/// Returns a new audio device ID.
	///
	/// Returns `None` if `id` is `0`. The zero ID is used by SDL to signify an
	/// invalid or null device.
	pub fn new(id: u32) -> Option<Self> {
		NonZeroU32::new(id).map(Self)
	}

	/// Returns the audio format currently used by the device with this ID if it
	/// has been opened. Otherwise, returns the audio format preferred by the
	/// device, or a reasonable default if it can't be determined.
	pub fn format(self) -> AudioFormat {
		let mut spec = MaybeUninit::uninit();
		sdl_assert!(unsafe { SDL_GetAudioDeviceFormat(self.as_sdl(), spec.as_mut_ptr(), ptr::null_mut()) });
		AudioFormat::from(unsafe { spec.assume_init() })
	}

}

impl AsSdlExt for AudioDeviceId {

	type Sdl = SDL_AudioDeviceID;

	fn as_sdl(&self) -> Self::Sdl {
		SDL_AudioDeviceID(self.0.get())
	}

}