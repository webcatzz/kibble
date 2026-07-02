use std::ffi::{c_float, c_int};
use std::fs::File;
use std::io::{self, Read, Seek};
use std::mem::MaybeUninit;
use std::path::Path;
use std::ptr::{self, NonNull};

use sdl3_sys::render::*;
use sdl3_image_sys::image::*;
use sdl3_sys::surface::{SDL_SCALEMODE_LINEAR, SDL_SCALEMODE_NEAREST, SDL_SCALEMODE_PIXELART, SDL_ScaleMode};

use crate::math::{Color, Rect, Transform, Vec2};
use crate::render::{Frame, Renderer};
use crate::sdl_util::{AsSdlExt, SdlIoStream, sdl_assert, sdl_panic};

/// A drawable texture.
pub struct Texture(NonNull<SDL_Texture>);

impl Texture {

	/// Loads a texture from a file.
	pub fn load(path: impl AsRef<Path>, renderer: &Renderer) -> io::Result<Self> {
		let mut file = File::open(path)?;
		Self::from_bytes(&mut file, renderer)
	}

	/// Reads a texture from bytes.
	pub fn from_bytes(bytes: &mut (impl Read + Seek), renderer: &Renderer) -> io::Result<Self> {
		let stream = SdlIoStream::new_read_seek(bytes);
		let ptr = unsafe { IMG_LoadTexture_IO(renderer.as_sdl(), stream.as_sdl(), false) };
		let Some(non_null) = NonNull::new(ptr) else { sdl_panic!() };
		Ok(Texture::from_sdl_texture(non_null))
	}

	/// Returns the width of the texture, in pixels.
	pub fn width(&self) -> u32 {
		unsafe { self.as_sdl().read().w as u32 }
	}

	/// Returns the height of the texture, in pixels.
	pub fn height(&self) -> u32 {
		unsafe { self.as_sdl().read().h as u32 }
	}

	/// Returns the size of the texture, in pixels.
	pub fn size(&self) -> Vec2<u32> {
		let SDL_Texture { w, h, .. } = unsafe { self.as_sdl().read() };
		Vec2 { x: w as u32, y: h as u32 }
	}

	/// Returns the filter used to sample the texture.
	///
	/// By default, the filter is [`TextureFilter::Linear`].
	pub fn filter(&self) -> TextureFilter {
		let mut scale_mode = MaybeUninit::uninit();
		sdl_assert!(unsafe { SDL_GetTextureScaleMode(self.as_sdl(), scale_mode.as_mut_ptr()) });
		unsafe { scale_mode.assume_init() }.into()
	}

	/// Sets the filter used to sample the texture.
	///
	/// If the filter isn't supported, the closest supported filter is used.
	pub fn set_filter(&mut self, filter: TextureFilter) {
		sdl_assert!(unsafe { SDL_SetTextureScaleMode(self.as_sdl(), filter.into()) });
	}

	/// Copies a portion of the texture to a frame.
	///
	/// If `rect` is [`None`], copies the whole texture.
	pub fn draw(&self, rect: Option<Rect<f32>>, pos: Vec2<f32>, frame: &mut Frame) {
		let size = rect.map_or_else(|| self.size().map(|v| v as f32), |rect| rect.size);
		self.draw_stretched(rect, Some(Rect { pos, size }), frame);
	}

	/// Stretches a portion of the texture to the given rectangle.
	///
	/// - If `src_rect` is [`None`], uses the whole texture.
	/// - If `dst_rect` is [`None`], stretches to the whole frame.
	pub fn draw_stretched(&self, src_rect: Option<Rect<f32>>, dst_rect: Option<Rect<f32>>, frame: &mut Frame) {
		let src_rect = src_rect.map(|rect| rect.map(|v| v as c_float).into());
		let dst_rect = dst_rect.map(|rect| rect.map(|v| v as c_float).into());
		sdl_assert!(unsafe { SDL_RenderTexture(frame.as_sdl(), self.as_sdl(), src_rect.as_ref().map_or(ptr::null(), ptr::from_ref), dst_rect.as_ref().map_or(ptr::null(), ptr::from_ref)) });
	}

	/// Draws the texture to a frame.
	///
	/// - `rect` is the portion of the texture to draw. If [`None`], draws the
	///   whole texture.
	/// - `offset` is a shift applied to the texture. For example, an offset of
	///   zero will draw the texture with its top-left corner at the origin, and
	///   an offset of `self.size() / 2` will draw the texture centered on the
	///   origin.
	/// - `transform` is a 2D transformation applied to the texture.
	pub fn draw_transformed(&self, rect: Option<Rect<f32>>, offset: Vec2<f32>, transform: Transform, frame: &mut Frame) {
		let rem    = rect.map_or(self.size().map(|v| v as f32), |rect| rect.size) + offset;
		let origin = transform.transform(offset);
		let right  = transform.transform(Vec2 { x: rem.x, y: offset.y });
		let down   = transform.transform(Vec2 { x: offset.x, y: rem.y });
		let rect = rect.map(|rect| rect.map(|v| v as c_float).into());
		sdl_assert!(unsafe { SDL_RenderTextureAffine(frame.as_sdl(), self.as_sdl(), rect.as_ref().map_or(ptr::null(), ptr::from_ref), &origin.map(|v| v as c_float).into(), &right.map(|v| v as c_float).into(), &down.map(|v| v as c_float).into()) });
	}

	/// Wraps an `SDL_Texture` pointer in a [`Texture`].
	pub const fn from_sdl_texture(sdl_texture: NonNull<SDL_Texture>) -> Self {
		Self(sdl_texture)
	}

}

impl Clone for Texture {

	fn clone(&self) -> Self {
		unsafe {
			let SDL_Texture { format, w, h, .. } = *self.as_sdl();
			let renderer = SDL_GetRendererFromTexture(self.as_sdl());
			sdl_assert!(!renderer.is_null());
			let texture_ptr = SDL_CreateTexture(renderer, format, SDL_TEXTUREACCESS_TARGET, w, h);
			let Some(texture_non_null) = NonNull::new(texture_ptr) else { sdl_panic!() };
			let texture = Texture::from_sdl_texture(texture_non_null);
			sdl_assert!(SDL_SetRenderTarget(renderer, texture.as_sdl())
				&& SDL_RenderTexture(renderer, self.as_sdl(), ptr::null(), ptr::null())
				&& SDL_SetRenderTarget(renderer, ptr::null_mut()));
			texture
		}
	}

}

impl AsSdlExt for Texture {

	type Sdl = *mut SDL_Texture;

	fn as_sdl(&self) -> Self::Sdl {
		self.0.as_ptr()
	}

}

impl Drop for Texture {

	fn drop(&mut self) {
		unsafe { SDL_DestroyTexture(self.as_sdl()); }
	}

}

unsafe impl Send for Texture {}
unsafe impl Sync for Texture {}

/// A method used to sample a texture.
#[repr(i32)]
#[derive(Clone, Copy)]
pub enum TextureFilter {
	/// Blends between nearby pixels.
	Linear   = SDL_SCALEMODE_LINEAR.0   as i32,
	/// Reads from the nearest pixel.
	Nearest  = SDL_SCALEMODE_NEAREST.0  as i32,
	/// Reads from the nearest pixel, with some fixes for transformed pixel art.
	///
	/// For a detailed explanation, see [Crafting a Better Shader for Pixel Art
	/// Upscaling].
	///
	/// [Crafting a Better Shader for Pixel Art Upscaling]:
	///     https://youtu.be/d6tp43wZqps
	PixelArt = SDL_SCALEMODE_PIXELART.0 as i32,
}

impl From<SDL_ScaleMode> for TextureFilter {

	fn from(value: SDL_ScaleMode) -> Self {
		match value {
			SDL_SCALEMODE_LINEAR   => Self::Linear,
			SDL_SCALEMODE_NEAREST  => Self::Nearest,
			SDL_SCALEMODE_PIXELART => Self::PixelArt,
			_                      => panic!("Unknown `SDL_ScaleMode` variant"),
		}
	}

}

impl Into<SDL_ScaleMode> for TextureFilter {

	fn into(self) -> SDL_ScaleMode {
		SDL_ScaleMode(self as c_int)
	}

}