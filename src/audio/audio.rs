use std::ffi::c_void;
use std::fs::File;
use std::io::{self, Read, Seek};
use std::mem::MaybeUninit;
use std::path::Path;
use std::ptr::NonNull;
use std::slice;

use sdl3_sys::audio::*;
use sdl3_sys::stdinc::SDL_free;

use crate::audio::AudioFormat;
use crate::sdl_util::{AsSdlExt, SdlIoStream, sdl_assert, sdl_panic};

/// Audio data.
pub struct Audio {
	/// The raw audio bytes.
	bytes:  NonNull<u8>,
	/// The number of bytes.
	len:    usize,
	/// The format of the data.
	format: AudioFormat,
}

impl Audio {

	/// Loads audio from a file.
	pub fn load(path: impl AsRef<Path>) -> io::Result<Self> {
		let mut file = File::open(path)?;
		Self::from_bytes(&mut file)
	}

	/// Reads audio from bytes.
	pub fn from_bytes(bytes: &mut (impl Read + Seek)) -> io::Result<Self> {
		let stream = SdlIoStream::new_read_seek(bytes);
		let mut audio_buf = MaybeUninit::uninit();
		let mut audio_len = MaybeUninit::uninit();
		let mut audio_spec = MaybeUninit::uninit();
		// SAFETY: `audio_buf` pointer is freed by drop destructor
		sdl_assert!(unsafe { SDL_LoadWAV_IO(stream.as_sdl(), false, audio_spec.as_mut_ptr(), audio_buf.as_mut_ptr(), audio_len.as_mut_ptr()) });
		let Some(audio_buf_non_null) = NonNull::new(unsafe { audio_buf.assume_init() }) else { sdl_panic!() };
		Ok(Self {
			bytes:  audio_buf_non_null,
			len:    usize::try_from(unsafe { audio_len.assume_init() }).expect("Number of audio bytes should not exceed `usize::MAX`"),
			format: AudioFormat::from(unsafe { audio_spec.assume_init() }),
		})
	}

	/// Returns the format of the audio data.
	pub fn format(&self) -> AudioFormat {
		self.format
	}

	/// Returns a reference to the raw audio data.
	pub fn bytes(&self) -> &[u8] {
		// SAFETY: `bytes` is a valid pointer returned by `Self::from_bytes()`
		unsafe { slice::from_raw_parts(self.bytes.as_ptr(), self.len) }
	}

}

impl Drop for Audio {

	fn drop(&mut self) {
		// SAFETY: `bytes` is a valid SDL pointer returned by `Self::from_bytes()`
		unsafe { SDL_free(self.bytes.as_ptr() as *mut c_void); }
	}

}