use super::{AudioFormat, AudioStream};

/// Types that can produce audio data.
pub trait AudioSource {

	/// Returns the format of audio data produced by the source.
	fn format(&self) -> AudioFormat;

	/// Puts at most the next `count` bytes from the source into the given
	/// stream.
	fn put(&mut self, stream: &mut AudioStream, count: usize);

}