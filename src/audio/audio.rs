use std::fs::File;
use std::io::{self, Read, Seek};
use std::mem::MaybeUninit;
use std::path::Path;
use std::slice;

use sdl3_sys::audio::*;
use sdl3_sys::stdinc::SDL_free;

use crate::sdl_util::sdl_assert;

use super::AudioFormat;

/// Audio data.
pub struct Audio {
	/// The raw audio bytes.
	bytes:  Vec<u8>,
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
		// TODO: use custom io interface
		let mut buf = Vec::new();
		bytes.read_to_end(&mut buf).unwrap();
		let mut audio_spec = MaybeUninit::uninit();
		let mut audio_buf = MaybeUninit::uninit();
		let mut audio_len = MaybeUninit::uninit();
		unsafe {
			let stream = sdl3_sys::iostream::SDL_IOFromConstMem(buf.as_mut_ptr() as *const _, buf.len());
			sdl_assert!(SDL_LoadWAV_IO(stream, true, audio_spec.as_mut_ptr(), audio_buf.as_mut_ptr(), audio_len.as_mut_ptr()));
			let slice = slice::from_raw_parts_mut(audio_buf.assume_init(), audio_len.assume_init() as usize);
			let audio = Audio {
				format: audio_spec.assume_init().into(),
				bytes:  slice.to_vec(),
			};
			SDL_free(audio_buf.assume_init() as *mut _);
			Ok(audio)
		}

		// let stream = SdlIoStream::new_read_seek(bytes);
		// let mut audio_spec = MaybeUninit::uninit();
		// let mut audio_buf = MaybeUninit::uninit();
		// let mut audio_len = MaybeUninit::uninit();
		// unsafe {
		// 	sdl_assert!(SDL_LoadWAV_IO(stream.sdl_stream(), false, audio_spec.as_mut_ptr(), audio_buf.as_mut_ptr(), audio_len.as_mut_ptr()));
		// 	let slice = slice::from_raw_parts_mut(audio_buf.assume_init(), audio_len.assume_init() as usize);
		// 	let audio = Audio {
		// 		spec: audio_spec.assume_init().into(),
		// 		data: slice.to_vec(),
		// 	};
		// 	SDL_free(audio_buf.assume_init() as *mut _);
		// 	audio
		// }
	}

	/// Returns the format of the audio data.
	pub fn format(&self) -> AudioFormat {
		self.format
	}

	/// Returns a reference to the raw audio data.
	pub fn bytes(&self) -> &[u8] {
		self.bytes.as_slice()
	}

}