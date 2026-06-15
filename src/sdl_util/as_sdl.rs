/// An extension trait that allows access to underlying SDL representations.
///
/// These representations are usually hidden from the user; to access them,
/// bring this trait into scope.
///
/// # Examples
///
/// ```
/// # use kibble::math::Vec2;
/// # use kibble::sdl_util::sdl_assert;
/// # use kibble::window::Window;
/// # use sdl3_sys::video::SDL_HideWindow;
/// use kibble::sdl_util::AsSdlExt;
///
/// let mut window = Window::new("Hello world!", Vec2 { x: 400, y: 300 });
/// assert_eq!(window.visible(), true);
/// sdl_assert!(unsafe { SDL_HideWindow(window.as_sdl()) });
/// assert_eq!(window.visible(), false);
/// ```
pub trait AsSdlExt<Sdl> {

	/// Returns the underlying SDL representation.
	///
	/// Note that interfacing with SDL, especially types underlying Kibble types,
	/// is by nature unsafe.
	fn as_sdl(&self) -> Sdl;

}