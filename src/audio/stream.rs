use std::ffi::{c_int, c_void};
use std::mem::MaybeUninit;
use std::ptr::{self, NonNull};

use sdl3_sys::audio::*;

use crate::sdl_util::{AsSdlExt, sdl_assert, sdl_panic};

use super::{AudioDevice, AudioDeviceId, AudioFormat};

#[repr(transparent)]
pub struct AudioStream(NonNull<SDL_AudioStream>);

impl AudioStream {

	/// Returns a new audio stream with the given input and output formats.
	pub fn new(in_format: Option<AudioFormat>, out_format: Option<AudioFormat>) -> Self {
		let ptr = unsafe { SDL_CreateAudioStream(in_format.map(Into::into).as_ref().map(ptr::from_ref).unwrap_or_default(), out_format.map(Into::into).as_ref().map(ptr::from_ref).unwrap_or_default()) };
		let Some(non_null) = NonNull::new(ptr) else { sdl_panic!() };
		Self(non_null)
	}

	/// Connects the audio stream to the given audio device.
	///
	/// If the device is for playback, it will play back any audio put into the
	/// stream. If it is a recording device, it will add recorded audio to the
	/// stream.
	pub fn connect(&mut self, device: &AudioDevice) {
		sdl_assert!(unsafe { SDL_BindAudioStream(device.as_sdl(), self.as_sdl()) });
	}

	/// Disconnects the audio stream from its audio device.
	pub fn disconnect(&mut self) {
		unsafe { SDL_UnbindAudioStream(self.as_sdl()); }
	}

	/// Reads data from the audio stream, returning the number of bytes read.
	///
	/// The data is in the format specified when creating the stream.
	pub fn get(&mut self, data: &mut [u8]) -> usize {
		let bytes_read = unsafe { SDL_GetAudioStreamData(self.as_sdl(), data.as_mut_ptr() as *mut c_void, data.len() as c_int) };
		sdl_assert!(bytes_read != -1);
		bytes_read as usize
	}

	/// Adds data to the audio stream.
	///
	/// Panics if the length of `data` exceeds [`c_int::MAX`].
	pub fn put(&mut self, data: &[u8]) {
		sdl_assert!(unsafe { SDL_PutAudioStreamData(self.as_sdl(), data.as_ptr() as *const c_void, c_int::try_from(data.len()).expect("the length of `data` should not exceed `c_int::MAX`")) });
	}

	/// Clears any data in the audio stream.
	pub fn clear(&mut self) {
		sdl_assert!(unsafe { SDL_ClearAudioStream(self.as_sdl()) });
	}

	/// Signals that no more data is incoming, and that any data being buffered
	/// should be made available immediately.
	pub fn flush(&mut self) {
		sdl_assert!(unsafe { SDL_FlushAudioStream(self.as_sdl()) });
	}

	/// Returns the number of bytes available to be read from the stream.
	///
	/// The stream may buffer data until it has enough to resample correctly, so
	/// this number may be lower than expected or even zero. Add more data or
	/// flush the stream if you need the data now.
	///
	/// The return value is clamped to [`c_int::MAX`].
	pub fn bytes_available(&self) -> usize {
		let bytes_available = unsafe { SDL_GetAudioStreamAvailable(self.as_sdl()) };
		sdl_assert!(bytes_available != -1);
		bytes_available as usize
	}

	/// Returns the number of bytes put into the stream.
	pub fn bytes_queued(&self) -> usize {
		let bytes_queued = unsafe { SDL_GetAudioStreamQueued(self.as_sdl()) };
		sdl_assert!(bytes_queued != -1);
		bytes_queued as usize
	}

	/// Returns the input and output audio format of the stream.
	pub fn format(&self) -> (AudioFormat, AudioFormat) {
		let mut src_spec = MaybeUninit::uninit();
		let mut dst_spec = MaybeUninit::uninit();
		sdl_assert!(unsafe { SDL_GetAudioStreamFormat(self.as_sdl(), src_spec.as_mut_ptr(), dst_spec.as_mut_ptr()) });
		(AudioFormat::from(unsafe { src_spec.assume_init() }), AudioFormat::from(unsafe { dst_spec.assume_init() }))
	}

	/// Returns the frequency ratio of the stream.
	pub fn frequency_ratio(&self) -> f32 {
		let ratio = unsafe { SDL_GetAudioStreamFrequencyRatio(self.as_sdl()) };
		sdl_assert!(ratio != 0.0);
		ratio
	}

	/// Returns the unique ID of the audio device connected to the stream, if
	/// any.
	pub fn device_id(&self) -> Option<AudioDeviceId> {
		AudioDeviceId::new(unsafe { SDL_GetAudioStreamDevice(self.as_sdl()).0 })
	}

	/// Returns `true` if there is an audio device connected to the stream and it
	/// is paused.
	pub fn is_device_paused(&self) -> bool {
		unsafe { SDL_AudioStreamDevicePaused(self.as_sdl()) }
	}

}

impl AsSdlExt for AudioStream {

	type Sdl = *mut SDL_AudioStream;

	fn as_sdl(&self) -> Self::Sdl {
		self.0.as_ptr()
	}

}

impl Drop for AudioStream {

	fn drop(&mut self) {
		unsafe { SDL_DestroyAudioStream(self.as_sdl()); }
	}

}