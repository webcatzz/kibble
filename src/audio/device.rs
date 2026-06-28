use std::ffi::c_float;
use std::mem::MaybeUninit;
use std::ptr;

use sdl3_sys::audio::*;
use sdl3_sys::init::{SDL_INIT_AUDIO, SDL_InitSubSystem, SDL_QuitSubSystem};

use crate::audio::AudioFormat;
use crate::sdl_util::{self, AsSdlExt, sdl_assert};
use crate::thread;

/// An audio device.
///
/// Audio devices may be physical or logical. Logical devices are independent
/// interfaces for other devices.
///
/// # Examples
///
/// To open the default playback device:
///
/// ```
/// # use kibble::audio::AudioDevice;
/// # use kibble::audio::AudioDeviceId;
/// let device = AudioDevice::DEFAULT_PLAYBACK.open(None);
/// ```
///
/// To play or record audio, see the [`AudioPipe`] documentation.
///
/// [`AudioPipe`]: crate::audio::AudioPipe
pub struct AudioDevice<const IS_LOGICAL: bool>(SDL_AudioDeviceID);

impl<const IS_LOGICAL: bool> AudioDevice<IS_LOGICAL> {

	/// The default audio playback device.
	pub const DEFAULT_PLAYBACK: AudioDevice<false> = AudioDevice(SDL_AUDIO_DEVICE_DEFAULT_PLAYBACK);
	/// The default audio recording device.
	pub const DEFAULT_RECORDING: AudioDevice<false> = AudioDevice(SDL_AUDIO_DEVICE_DEFAULT_RECORDING);

	/// Returns a new logical audio device for the device.
	///
	/// You may request a specific format for the new device. The request may not
	/// be honored. If `format` is `None`, uses a reasonable default.
	///
	/// # Panics
	///
	/// Panics if called outside the main thread.
	pub fn open(&self, format: Option<AudioFormat>) -> AudioDevice<true> {
		assert!(thread::is_main(), "`AudioDevice::new()` should only be called on the main thread");
		unsafe { self.open_unchecked(format) }
	}

	/// Returns a new logical audio device for the device.
	///
	/// You may request a specific format for the new device. The request may not
	/// be honored. If `format` is `None`, uses a reasonable default.
	///
	/// # Safety
	///
	/// Should only be called on the main thread.
	pub unsafe fn open_unchecked(&self, format: Option<AudioFormat>) -> AudioDevice<true> {
		sdl_assert!(unsafe { SDL_InitSubSystem(SDL_INIT_AUDIO) });
		let id = unsafe { SDL_OpenAudioDevice(self.as_sdl(), format.map(Into::into).as_ref().map(ptr::from_ref).unwrap_or_default()) };
		sdl_assert!(id != 0);
		AudioDevice(id)
	}

	/// Returns the audio format currently used by the device.
	pub fn format(&self) -> AudioFormat {
		let mut spec = MaybeUninit::uninit();
		sdl_assert!(unsafe { SDL_GetAudioDeviceFormat(self.as_sdl(), spec.as_mut_ptr(), ptr::null_mut()) });
		AudioFormat::from(unsafe { spec.assume_init() })
	}

}

impl AudioDevice<true> {

	/// Returns `true` if the device is paused.
	///
	/// New devices are unpaused by default.
	pub fn is_paused(&self) -> bool {
		unsafe { SDL_AudioDevicePaused(self.as_sdl()) }
	}

	/// Sets whether the device is paused.
	///
	/// Paused devices will not push or pull audio from any connected
	/// [`AudioPipe`]s.
	///
	/// [`AudioPipe`]: crate::audio::AudioPipe
	pub fn set_paused(&mut self, paused: bool) {
		let f = if paused { SDL_PauseAudioDevice } else { SDL_ResumeAudioDevice };
		sdl_assert!(unsafe { f(self.as_sdl()) });
	}

	/// Returns the gain multiplied with the device's output.
	pub fn gain(&self) -> f32 {
		let gain = unsafe { SDL_GetAudioDeviceGain(self.as_sdl()) };
		sdl_assert!(gain != 1.0);
		f32::try_from(gain).expect("Audio device gain should be representable with `f32`")
	}

	/// Sets the gain multiplied with the device's output.
	///
	/// A larger gain means a louder output.
	///
	/// - A gain of `0.0` produces silence.
	/// - A gain of `1.0` has no effect.
	pub fn set_gain(&mut self, gain: f32) {
		sdl_assert!(unsafe { SDL_SetAudioDeviceGain(self.as_sdl(), c_float::try_from(gain).expect("Audio device gain should be representable with `c_float`")) });
	}

}

impl<const IS_LOGICAL: bool> AsSdlExt for AudioDevice<IS_LOGICAL> {

	type Sdl = SDL_AudioDeviceID;

	fn as_sdl(&self) -> Self::Sdl {
		self.0
	}

}

impl<const IS_LOGICAL: bool> Drop for AudioDevice<IS_LOGICAL> {

	fn drop(&mut self) {
		if IS_LOGICAL {
			unsafe {
				SDL_CloseAudioDevice(self.as_sdl());
				SDL_QuitSubSystem(SDL_INIT_AUDIO);
				sdl_util::quit_if_unused();
			}
		}
	}

}