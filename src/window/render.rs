use std::ffi::{c_float, c_int};
use std::mem::MaybeUninit;
use std::num::NonZero;
use std::ptr::{self, NonNull};

use sdl3_sys::render::*;

use crate::math::Vec2;
use crate::render::{Frame, TextureFilter};
use crate::sdl_util::{AsSdlExt, sdl_assert, sdl_panic};

use super::Window;

impl Window {

	/// Creates an SDL renderer for the window if none exists.
	fn create_sdl_renderer_if_none(&mut self) {
		if self.sdl_renderer.is_none() {
			let Some(sdl_renderer) = NonNull::new(unsafe { SDL_CreateRenderer(self.as_sdl(), ptr::null()) }) else { sdl_panic!() };
			self.sdl_renderer = Some(sdl_renderer);
		}
	}

	/// Returns the next frame of the window for rendering.
	pub fn frame<'a>(&'a mut self) -> Frame<'a> {
		self.create_sdl_renderer_if_none();
		sdl_assert!(unsafe { SDL_SetRenderDrawColor(self.as_sdl(), 0, 0, 0, 0)
			&& SDL_RenderClear(self.as_sdl()) });
		Frame::from_sdl_renderer(self.sdl_renderer.unwrap())
	}

	/// Sets the filter used by newly created textures.
	pub fn set_default_texture_filter(&mut self, filter: TextureFilter) {
		self.create_sdl_renderer_if_none();
		sdl_assert!(unsafe { SDL_SetDefaultTextureScaleMode(self.as_sdl(), filter.into()) });
	}

	/// Sets the viewport displayed by the window.
	pub fn set_viewport(&mut self, viewport: Option<Viewport>) {
		self.create_sdl_renderer_if_none();
		let w;
		let h;
		let mode;
		if let Some(viewport) = viewport {
			w = c_int::try_from(viewport.size.x).expect("Window width should be representable with `c_int`");
			h = c_int::try_from(viewport.size.y).expect("Window height should be representable with `c_int`");
			mode = viewport.fit.into();
		} else {
			w = 0;
			h = 0;
			mode = SDL_LOGICAL_PRESENTATION_DISABLED;
		}
		sdl_assert!(unsafe { SDL_SetRenderLogicalPresentation(self.as_sdl(), w, h, mode) });
	}

	/// Returns the scale applied to the window's contents.
	pub fn scale(&self) -> Vec2<f32> {
		let mut w = MaybeUninit::uninit();
		let mut h = MaybeUninit::uninit();
		sdl_assert!(unsafe { SDL_GetRenderScale(self.as_sdl(), w.as_mut_ptr(), h.as_mut_ptr()) });
		Vec2 { x: unsafe { w.assume_init() } as f32, y: unsafe { h.assume_init() } as f32 }
	}

	/// Sets the scale applied to the window's contents.
	pub fn set_scale(&mut self, scale: Vec2<f32>) {
		self.create_sdl_renderer_if_none();
		sdl_assert!(unsafe { SDL_SetRenderScale(self.as_sdl(), scale.x as c_float, scale.y as c_float) });
	}

	/// Sets the vertical syncing applied when rendering.
	///
	/// Note that not every value is supported by every driver.
	pub fn set_vsync(&mut self, vsync: VSync) {
		self.create_sdl_renderer_if_none();
		sdl_assert!(unsafe { SDL_SetRenderVSync(self.as_sdl(), vsync.into()) });
	}

}

impl AsSdlExt<*mut SDL_Renderer> for Window {

	fn as_sdl(&self) -> *mut SDL_Renderer {
		self.sdl_renderer.map_or(ptr::null_mut(), NonNull::as_ptr)
	}

}

#[derive(Clone, Copy)]
pub struct Viewport {
	/// The size of the viewport.
	pub size: Vec2<u32>,
	/// The method by which the viewport is scaled to the window.
	pub fit:  ViewportFit,
}

/// The method by which window contents are scaled to the window's resolution.
#[repr(i32)]
#[derive(Clone, Copy)]
pub enum ViewportFit {
	/// The window contents are stretched to the window resolution.
	Stretch = SDL_LOGICAL_PRESENTATION_STRETCH.0       as i32,
	/// The window contents are fit to the largest dimension.
	Contain = SDL_LOGICAL_PRESENTATION_LETTERBOX.0     as i32,
	/// The window contents are fit to the smallest dimension.
	Cover   = SDL_LOGICAL_PRESENTATION_OVERSCAN.0      as i32,
	/// The window contents are scaled up by integer multiples to fit the output
	/// resolution.
	Integer = SDL_LOGICAL_PRESENTATION_INTEGER_SCALE.0 as i32,
}

impl Into<SDL_RendererLogicalPresentation> for ViewportFit {

	fn into(self) -> SDL_RendererLogicalPresentation {
		SDL_RendererLogicalPresentation(self as c_int)
	}

}

#[derive(Clone, Copy)]
pub enum VSync {
	Adaptive,
	Every(NonZero<u16>),
}

impl Into<c_int> for VSync {

	fn into(self) -> c_int {
		match self {
			Self::Adaptive => SDL_RENDERER_VSYNC_ADAPTIVE,
			Self::Every(n) => c_int::try_from(n.get()).expect("V-sync frequency should be representable with `c_int`"),
		}
	}

}