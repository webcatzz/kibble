use std::ffi::{c_float, c_int, c_void};
use std::mem::MaybeUninit;
use std::ptr::{self, NonNull};

use sdl3_sys::audio::*;

use crate::audio::{AudioDevice, AudioFormat};
use crate::sdl_util::{AsSdlExt, sdl_assert, sdl_panic};

/// An audio sampler.
///
/// An audio pipe may be connected to an [`AudioDevice`], which will push or
/// pull audio from the pipe.
///
/// # Examples
///
/// To play back audio:
///
/// ```
/// # use kibble::audio::{AudioDevice, AudioPipe, AudioSampleFormat};
/// # let audio_format = AudioFormat { sample_format: AudioSampleFormat::F32, sample_rate: 44100, channel_count: 1 };
/// # let audio_data = &[];
/// // Opens a playback device
/// let audio_device = AudioDevice::DEFAULT_PLAYBACK.new(None);
/// // Creates an audio pipe
/// let mut audio_pipe = AudioPipe::new(Some(audio_format), None);
/// // Connects the pipe to the device
/// audio_pipe.connect(&audio_device);
/// // Pushes audio into the pipe for playback
/// audio_pipe.push(&audio_data);
/// ```
pub struct AudioPipe(NonNull<SDL_AudioStream>);

impl AudioPipe {

	/// Returns a new audio pipe with the given input and output formats.
	///
	/// If either format is `None`, attempting to push or pull audio from the
	/// pipe will fail. However, if `None`, the input or output format can be
	/// automatically detected when the pipe is connected to a recording or
	/// playback device, respectively.
	pub fn new(input_format: Option<AudioFormat>, output_format: Option<AudioFormat>) -> Self {
		let ptr = unsafe { SDL_CreateAudioStream(input_format.map(Into::into).as_ref().map(ptr::from_ref).unwrap_or_default(), output_format.map(Into::into).as_ref().map(ptr::from_ref).unwrap_or_default()) };
		let Some(non_null) = NonNull::new(ptr) else { sdl_panic!() };
		Self(non_null)
	}

	/// Connects the audio pipe to the given audio device.
	///
	/// - Playback devices will pull audio from the pipe while playing.
	/// - Recording devices will push audio into the pipe while recording.
	///
	/// An audio pipe may only be connected to a single device at a time. Note
	/// that a single device may be connected to multiple audio pipes.
	///
	/// If unset, the input or output format of the pipe will be automatically
	/// detected from the given recording or playback device, respectively.
	pub fn connect(&mut self, device: &AudioDevice<true>) {
		sdl_assert!(unsafe { SDL_BindAudioStream(device.as_sdl(), self.as_sdl()) });
	}

	/// Disconnects the audio pipe from its audio device.
	pub fn disconnect(&mut self) {
		unsafe { SDL_UnbindAudioStream(self.as_sdl()); }
	}

	/// Pulls data from the audio pipe, returning the number of bytes read.
	///
	/// The data is interpreted in the output format returned by
	/// [`AudioPipe::format()`].
	pub fn pull(&mut self, data: &mut [u8]) -> usize {
		let bytes_read = unsafe { SDL_GetAudioStreamData(self.as_sdl(), data.as_mut_ptr() as *mut c_void, data.len() as c_int) };
		sdl_assert!(bytes_read != -1);
		bytes_read as usize
	}

	/// Pushes data into the audio pipe.
	///
	/// The data is interpreted in the input format returned by
	/// [`AudioPipe::format()`].
	pub fn push(&mut self, data: &[u8]) {
		sdl_assert!(unsafe { SDL_PutAudioStreamData(self.as_sdl(), data.as_ptr() as *const c_void, c_int::try_from(data.len()).expect("the length of `data` should not exceed `c_int::MAX`")) });
	}

	/// Clears any data in the audio pipe.
	pub fn clear(&mut self) {
		sdl_assert!(unsafe { SDL_ClearAudioStream(self.as_sdl()) });
	}

	/// Signals that no more data is incoming, and that any data being buffered
	/// should be made available immediately.
	pub fn flush(&mut self) {
		sdl_assert!(unsafe { SDL_FlushAudioStream(self.as_sdl()) });
	}

	/// Returns the number of bytes available to be read from the pipe.
	///
	/// The pipe may buffer data until it has enough to resample correctly, so
	/// this number may be lower than expected or even zero. Add more data or
	/// flush the pipe if you need the data now.
	///
	/// The return value is clamped to [`c_int::MAX`].
	pub fn bytes_available(&self) -> usize {
		let bytes_available = unsafe { SDL_GetAudioStreamAvailable(self.as_sdl()) };
		sdl_assert!(bytes_available != -1);
		bytes_available as usize
	}

	/// Returns the number of bytes queued in the pipe.
	pub fn bytes_queued(&self) -> usize {
		let bytes_queued = unsafe { SDL_GetAudioStreamQueued(self.as_sdl()) };
		sdl_assert!(bytes_queued != -1);
		bytes_queued as usize
	}

	/// Returns the input and output formats of the pipe.
	pub fn format(&self) -> (AudioFormat, AudioFormat) {
		let mut src_spec = MaybeUninit::uninit();
		let mut dst_spec = MaybeUninit::uninit();
		sdl_assert!(unsafe { SDL_GetAudioStreamFormat(self.as_sdl(), src_spec.as_mut_ptr(), dst_spec.as_mut_ptr()) });
		(AudioFormat::from(unsafe { src_spec.assume_init() }), AudioFormat::from(unsafe { dst_spec.assume_init() }))
	}

	/// Sets the input and output formats of the pipe.
	pub fn set_format(&mut self, input: Option<AudioFormat>, output: Option<AudioFormat>) {
		let src = input.map(Into::into);
		let dst = output.map(Into::into);
		sdl_assert!(unsafe { SDL_SetAudioStreamFormat(self.as_sdl(), src.map_or(ptr::null(), |format| &format), dst.map_or(ptr::null(), |format| &format)) });
	}

	/// Returns the rate at which the pipe's device will play back audio data.
	pub fn frequency_ratio(&self) -> f32 {
		let frequency_ratio = unsafe { SDL_GetAudioStreamFrequencyRatio(self.as_sdl()) };
		sdl_assert!(frequency_ratio != 0.0);
		f32::try_from(frequency_ratio).expect("Audio pipe frequency ratio should be representable with `f32`")
	}

	/// Sets the rate at which the pipe's device will play back audio data.
	///
	/// - A value greater than `1.0` plays audio faster and at a higher pitch.
	/// - A value less than `1.0` plays audio slower and at a lower pitch.
	pub fn set_frequency_ratio(&mut self, frequency_ratio: f32) {
		sdl_assert!(unsafe { SDL_SetAudioStreamFrequencyRatio(self.as_sdl(), c_float::try_from(frequency_ratio).expect("Audio pipe frequency ratio should be representable with `c_float`")) });
	}

	/// Returns `true` if there is an audio device connected to the pipe and it
	/// is paused.
	pub fn is_device_paused(&self) -> bool {
		unsafe { SDL_AudioStreamDevicePaused(self.as_sdl()) }
	}

}

impl AsSdlExt for AudioPipe {

	type Sdl = *mut SDL_AudioStream;

	fn as_sdl(&self) -> Self::Sdl {
		self.0.as_ptr()
	}

}

impl Drop for AudioPipe {

	fn drop(&mut self) {
		unsafe { SDL_DestroyAudioStream(self.as_sdl()); }
	}

}