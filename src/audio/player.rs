use std::borrow::Borrow;

use super::{Audio, AudioFormat, AudioSource, AudioStream};

/// Sources audio from [`Audio`].
#[non_exhaustive]
pub struct AudioPlayer<T> {
	/// Source audio.
	pub audio:  T,
	/// Current position in the source audio.
	pub pos:    usize,
	/// Whether audio should loop.
	pub repeat: bool,
}

impl<T: Borrow<Audio>> AudioPlayer<T> {

	/// Returns a new player for the given audio.
	pub fn new(audio: T, repeat: bool) -> Self {
		Self {
			audio,
			pos: 0,
			repeat,
		}
	}

}

impl<T: Borrow<Audio>> AudioSource for AudioPlayer<T> {

	fn format(&self) -> AudioFormat {
		self.audio.borrow().format()
	}

	fn put(&mut self, stream: &mut AudioStream, count: usize) {
		if self.pos >= self.audio.borrow().bytes().len() {
			if self.repeat {
				// Moves back to the start of the audio if the player should repeat
				self.pos = 0;
			} else {
				// Returns early if the player should not repeat
				return;
			}
		}
		// Puts data into the stream
		let bytes = self.audio.borrow().bytes();
		let bytes = &bytes[self.pos..(self.pos + count).min(bytes.len())];
		self.pos += bytes.len();
		stream.put(bytes);
	}

}