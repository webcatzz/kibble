//! Threads.

use sdl3_sys::init::SDL_IsMainThread;

/// Returns `true` if called on the main thread.
///
/// - On Apple platforms, the main thread is the one that runs the program's
///   entry point.
/// - On other platforms, the main thread is the one that creates a [`Window`],
///   which should usually be the one that runs the program's entry point.
///
/// [`Window`]: crate::window::Window
pub fn is_main() -> bool {
	unsafe { SDL_IsMainThread() }
}