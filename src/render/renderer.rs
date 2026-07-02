use std::ffi::{c_float, c_int};
use std::mem::MaybeUninit;
use std::num::NonZero;
use std::ptr::{self, NonNull};

use sdl3_sys::render::*;

use crate::math::{Color, Vec2};
use crate::render::{Frame, TextureFilter};
use crate::sdl_util::{AsSdlExt, sdl_assert, sdl_panic};
use crate::window::Window;

/// A renderer.
///
/// # Examples
///
/// To create a renderer:
///
/// ```
/// # use kibble::math::Vec2;
/// # use kibble::render::Renderer;
/// # use kibble::window::Window;
/// let window = Window::new("title", Vec2 { x: 800, y: 600 });
/// let renderer = Renderer::new(&window);
/// ```
///
/// To draw to the next frame:
///
/// ```
/// # use kibble::math::{Color, Vec2};
/// # use kibble::render::{Frame, Renderer};
/// # use kibble::window::Window;
/// # let window = Window::new("title", Vec2 { x: 800, y: 600 });
/// let mut renderer = Renderer::new(&window);
/// let mut frame = renderer.frame();
/// frame.clear(Color::WHITE);
/// frame.present();
/// ```
///
/// See the [`Frame`] documentation for more.
pub struct Renderer(NonNull<SDL_Renderer>);

impl Renderer {

	/// Returns a new renderer for the given window.
	pub fn new(window: &Window) -> Self {
		match NonNull::new(unsafe { SDL_CreateRenderer(window.as_sdl(), ptr::null()) }) {
			Some(sdl_renderer) => Self(sdl_renderer),
			None => sdl_panic!(),
		}
	}

	/// Returns the next frame for rendering.
	pub fn frame<'a>(&'a mut self) -> Frame<'a> {
		let mut frame = Frame::from_sdl_renderer(self.0);
		frame.clear(Color::BLACK);
		frame
	}

	/// Sets the filter used by newly created textures.
	pub fn set_default_texture_filter(&mut self, filter: TextureFilter) {
		sdl_assert!(unsafe { SDL_SetDefaultTextureScaleMode(self.as_sdl(), filter.into()) });
	}

	/// Returns the viewport rendered to by the renderer.
	pub fn viewport(&self) -> Option<Viewport> {
		let mut w = MaybeUninit::uninit();
		let mut h = MaybeUninit::uninit();
		let mut mode = MaybeUninit::uninit();
		sdl_assert!(unsafe { SDL_GetRenderLogicalPresentation(self.as_sdl(), w.as_mut_ptr(), h.as_mut_ptr(), mode.as_mut_ptr()) });
		let fit = match unsafe { mode.assume_init() } {
			SDL_LOGICAL_PRESENTATION_DISABLED      => return None,
			SDL_LOGICAL_PRESENTATION_STRETCH       => ViewportFit::Stretch,
			SDL_LOGICAL_PRESENTATION_LETTERBOX     => ViewportFit::Contain,
			SDL_LOGICAL_PRESENTATION_OVERSCAN      => ViewportFit::Cover,
			SDL_LOGICAL_PRESENTATION_INTEGER_SCALE => ViewportFit::Integer,
			SDL_RendererLogicalPresentation(v) => panic!("Unknown `SDL_RendererLogicalPresentation` variant: {v}"),
		};
		Some(Viewport {
			size: Vec2 { x: u32::try_from(unsafe { w.assume_init() }).expect("Viewport width should be representable with `u32`"), y: u32::try_from(unsafe { h.assume_init() }).expect("Viewport height should be representable with `u32`") },
			fit,
		})
	}

	/// Sets the viewport rendered to by the renderer.
	///
	/// When a viewport is set, the renderer will "pretend" to render at the
	/// viewport resolution, scaling any coordinates to the actual output
	/// resolution.
	pub fn set_viewport(&mut self, viewport: Option<Viewport>) {
		let w;
		let h;
		let mode;
		if let Some(viewport) = viewport {
			w = c_int::try_from(viewport.size.x).expect("Viewport width should be representable with `c_int`");
			h = c_int::try_from(viewport.size.y).expect("Viewport height should be representable with `c_int`");
			mode = viewport.fit.into();
		} else {
			w = 0;
			h = 0;
			mode = SDL_LOGICAL_PRESENTATION_DISABLED;
		}
		sdl_assert!(unsafe { SDL_SetRenderLogicalPresentation(self.as_sdl(), w, h, mode) });
	}

	/// Returns the scale applied to renderer coordinates.
	pub fn scale(&self) -> Vec2<f32> {
		let mut w = MaybeUninit::uninit();
		let mut h = MaybeUninit::uninit();
		sdl_assert!(unsafe { SDL_GetRenderScale(self.as_sdl(), w.as_mut_ptr(), h.as_mut_ptr()) });
		Vec2 { x: unsafe { w.assume_init() } as f32, y: unsafe { h.assume_init() } as f32 }
	}

	/// Sets the scale applied to renderer coordinates.
	pub fn set_scale(&mut self, scale: Vec2<f32>) {
		sdl_assert!(unsafe { SDL_SetRenderScale(self.as_sdl(), scale.x as c_float, scale.y as c_float) });
	}

	/// Returns the vertical syncing used by the renderer.
	pub fn vsync(&self) -> VSync {
		let mut vsync = MaybeUninit::uninit();
		sdl_assert!(unsafe { SDL_GetRenderVSync(self.as_sdl(), vsync.as_mut_ptr()) });
		unsafe { vsync.assume_init() }.into()
	}

	/// Sets the vertical syncing used by the renderer.
	///
	/// Note that not every value may be supported by the driver.
	pub fn set_vsync(&mut self, vsync: VSync) {
		sdl_assert!(unsafe { SDL_SetRenderVSync(self.as_sdl(), vsync.into()) });
	}

	/// Converts from window coordinates to render coordinates.
	pub fn render_coords_from_window_coords(&self, window_coords: Vec2<f32>) -> Vec2<f32> {
		let mut x = MaybeUninit::uninit();
		let mut y = MaybeUninit::uninit();
		sdl_assert!(unsafe { SDL_RenderCoordinatesFromWindow(self.as_sdl(), window_coords.x as c_float, window_coords.y as c_float, x.as_mut_ptr(), y.as_mut_ptr()) });
		Vec2 { x: unsafe { x.assume_init() } as f32, y: unsafe { y.assume_init() } as f32 }
	}

	/// Converts from render coordinates to window coordinates.
	pub fn window_coords_from_render_coords(&self, render_coords: Vec2<f32>) -> Vec2<f32> {
		let mut x = MaybeUninit::uninit();
		let mut y = MaybeUninit::uninit();
		sdl_assert!(unsafe { SDL_RenderCoordinatesToWindow(self.as_sdl(), render_coords.x as c_float, render_coords.y as c_float, x.as_mut_ptr(), y.as_mut_ptr()) });
		Vec2 { x: unsafe { x.assume_init() } as f32, y: unsafe { y.assume_init() } as f32 }
	}

}

impl AsSdlExt for Renderer {

	type Sdl = *mut SDL_Renderer;

	fn as_sdl(&self) -> Self::Sdl {
		self.0.as_ptr()
	}

}

impl Drop for Renderer {

	fn drop(&mut self) {
		unsafe { SDL_DestroyRenderer(self.as_sdl()); }
	}

}

/// A logical output size for a renderer.
///
/// By default, a renderer renders directly to its output resolution. However,
/// if a viewport is set with [`Renderer::set_viewport()`], the renderer will
/// "pretend" to render at the viewport resolution, scaling any coordinates to
/// the actual output resolution. This is useful for programs that want to
/// render at a fixed size but scale as needed.
#[derive(Clone, Copy)]
pub struct Viewport {
	/// The size of the viewport.
	pub size: Vec2<u32>,
	/// The method used to scale the viewport to the output resolution.
	pub fit:  ViewportFit,
}

/// A method used to scale a viewport to the output resolution.
#[repr(i32)]
#[derive(Clone, Copy)]
pub enum ViewportFit {
	/// The viewport is stretched to the output resolution.
	Stretch = SDL_LOGICAL_PRESENTATION_STRETCH.0       as i32,
	/// The viewport is fit to the largest dimension.
	Contain = SDL_LOGICAL_PRESENTATION_LETTERBOX.0     as i32,
	/// The viewport is fit to the smallest dimension.
	Cover   = SDL_LOGICAL_PRESENTATION_OVERSCAN.0      as i32,
	/// The viewport is scaled up by integer multiples to fit the output
	/// resolution.
	Integer = SDL_LOGICAL_PRESENTATION_INTEGER_SCALE.0 as i32,
}

impl Into<SDL_RendererLogicalPresentation> for ViewportFit {

	fn into(self) -> SDL_RendererLogicalPresentation {
		SDL_RendererLogicalPresentation(self as c_int)
	}

}

/// Vertical syncing used when rendering.
///
/// Vertical syncing prevents screen tearing by waiting for the display to
/// finish drawing the current frame before supplying it with the next frame.
#[derive(Clone, Copy)]
pub enum VSync {
	/// Does not sync.
	Disabled,
	/// Syncs only when the frame rate is above the display refresh rate.
	Adaptive,
	/// Syncs every given number of frames.
	Every(NonZero<u16>),
}

impl From<c_int> for VSync {

	fn from(value: c_int) -> Self {
		match value {
			SDL_RENDERER_VSYNC_DISABLED => VSync::Disabled,
			SDL_RENDERER_VSYNC_ADAPTIVE => VSync::Adaptive,
			n => match NonZero::new(u16::try_from(n).expect("VSync interval should be representable with `u16`")) {
				Some(n) => VSync::Every(n),
				None => unreachable!(),
			}
		}
	}

}

impl Into<c_int> for VSync {

	fn into(self) -> c_int {
		match self {
			Self::Disabled => SDL_RENDERER_VSYNC_DISABLED,
			Self::Adaptive => SDL_RENDERER_VSYNC_ADAPTIVE,
			Self::Every(n) => c_int::try_from(n.get()).expect("VSync interval should be representable with `c_int`"),
		}
	}

}