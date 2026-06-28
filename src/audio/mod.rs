//! Audio.

mod audio;
mod device;
mod format;
mod pipe;

pub use audio::Audio;
pub use device::AudioDevice;
pub use format::{AudioFormat, AudioSampleFormat};
pub use pipe::AudioPipe;