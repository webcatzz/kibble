//! Audio format.

use std::ffi::{c_int, c_uint};

use sdl3_sys::audio::*;

/// Describes the format of audio data.
#[derive(Clone, Copy)]
pub struct AudioFormat {
	/// The format used to represent audio samples.
	pub sample_format: AudioSampleFormat,
	/// The number of samples per second.
	pub sample_rate: u32,
	/// The number of audio channels.
	///
	/// Mono audio uses `1` channel, stereo uses `2`, etc.
	pub channel_count: u32,
}

impl From<SDL_AudioSpec> for AudioFormat {

	fn from(spec: SDL_AudioSpec) -> Self {
		Self {
			sample_format: AudioSampleFormat::from(spec.format),
			sample_rate: spec.freq as u32,
			channel_count: spec.channels as u32,
		}
	}

}

impl Into<SDL_AudioSpec> for AudioFormat {

	fn into(self) -> SDL_AudioSpec {
		SDL_AudioSpec {
			format: self.sample_format.into(),
			channels: self.channel_count as c_int,
			freq: self.sample_rate as c_int,
		}
	}

}

/// A format used to represent audio samples.
#[repr(u32)]
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum AudioSampleFormat {
	/// Samples are represented with unsigned 8-bit integers.
	U8      = SDL_AUDIO_U8.0 as u32,
	/// Samples are represented with signed 8-bit integers.
	I8      = SDL_AUDIO_S8.0 as u32,
	/// Samples are represented with signed 16-bit integers.
	I16     = SDL_AUDIO_S16.0 as u32,
	/// Samples are represented with signed 32-bit integers.
	I32     = SDL_AUDIO_S32.0 as u32,
	/// Samples are represented with signed 32-bit floating-point numbers.
	F32     = SDL_AUDIO_F32.0 as u32,
	/// Unspecified format.
	Unknown = SDL_AUDIO_UNKNOWN.0 as u32,
}

impl From<SDL_AudioFormat> for AudioSampleFormat {

	fn from(value: SDL_AudioFormat) -> Self {
		match value {
			SDL_AUDIO_U8      => Self::U8,
			SDL_AUDIO_S8      => Self::I8,
			SDL_AUDIO_S16     => Self::I16,
			SDL_AUDIO_S32     => Self::I32,
			SDL_AUDIO_F32     => Self::F32,
			SDL_AUDIO_UNKNOWN => Self::Unknown,
			_                 => panic!("Unknown `SDL_AudioFormat` variant"),
		}
	}

}

impl Into<SDL_AudioFormat> for AudioSampleFormat {

	fn into(self) -> SDL_AudioFormat {
		SDL_AudioFormat(self as c_uint)
	}

}