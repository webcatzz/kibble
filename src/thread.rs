//! Threads.

use sdl3_sys::init::SDL_IsMainThread;

/// Returns `true` if called on the main thread.
pub fn is_main() -> bool {
	unsafe { SDL_IsMainThread() }
}