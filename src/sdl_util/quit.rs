use sdl3_sys::init::{SDL_InitFlags, SDL_Quit, SDL_WasInit};

/// Cleans up all SDL-allocated memory if no SDL systems are initialized.
/// Otherwise, does nothing.
///
/// # Safety
///
/// Should only be called on the main thread.
///
/// # Examples
///
/// ```no_run
/// # use sdl3_sys::init::*;
/// # use kibble::sdl_util::quit_if_unused;
/// # unsafe {
/// SDL_InitSubSystem(SDL_INIT_VIDEO | SDL_INIT_AUDIO);
/// SDL_QuitSubSystem(SDL_INIT_VIDEO);
/// quit_if_unused(); // Does nothing
/// SDL_QuitSubSystem(SDL_INIT_AUDIO);
/// quit_if_unused(); // No systems initialized, cleans up SDL
/// # }
/// ```
pub unsafe fn quit_if_unused() {
	if unsafe { SDL_WasInit(SDL_InitFlags(0)) } == 0 {
		unsafe { SDL_Quit(); }
	}
}