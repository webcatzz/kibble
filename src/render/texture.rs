use std::ffi::{c_float, c_int};
use std::fs::File;
use std::io::{self, Read, Seek};
use std::mem::MaybeUninit;
use std::path::Path;
use std::ptr::{self, NonNull};

use sdl3_sys::rect::{SDL_FPoint, SDL_FRect};
use sdl3_sys::render::*;
use sdl3_image_sys::image::*;
use sdl3_sys::surface::{SDL_SCALEMODE_LINEAR, SDL_SCALEMODE_NEAREST, SDL_SCALEMODE_PIXELART, SDL_ScaleMode};

use crate::math::{Color, Rect, Transform, Vec2};
use crate::sdl_util::{AsSdlExt, SdlIoStream, sdl_assert, sdl_panic};

use super::Frame;

/// A texture used for rendering.
pub struct Texture(NonNull<SDL_Texture>);

impl Texture {

	/// Loads a texture from a file.
	pub fn load(path: impl AsRef<Path>, frame: &Frame) -> io::Result<Self> {
		let mut file = File::open(path)?;
		Self::from_bytes(&mut file, frame)
	}

	/// Reads a texture from bytes.
	pub fn from_bytes(bytes: &mut (impl Read + Seek), frame: &Frame) -> io::Result<Self> {
		let stream = SdlIoStream::new_read_seek(bytes);
		let ptr = unsafe { IMG_LoadTexture_IO(frame.as_sdl(), stream.as_sdl(), false) };
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

	/// Draws the texture to a frame with the given options.
	pub fn draw(&self, TextureDrawOptions { rect, offset, transform, modulate }: TextureDrawOptions, frame: &mut Frame) {
		let rect   = rect.unwrap_or_else(|| Rect { pos: Vec2::ZERO, size: self.size().map(|v| v as f32) });
		let rem    = rect.size + offset;
		let origin = transform.transform(offset);
		let right  = transform.transform(Vec2 { x: rem.x, y: offset.y });
		let down   = transform.transform(Vec2 { x: offset.x, y: rem.y });
		unsafe { sdl_assert!(SDL_SetTextureColorMod(self.as_sdl(), modulate.r, modulate.g, modulate.b)
			&& SDL_SetTextureAlphaMod(self.as_sdl(), modulate.a)
			&& SDL_RenderTextureAffine(frame.as_sdl(), self.as_sdl(), &SDL_FRect { x: rect.pos.x as c_float, y: rect.pos.y as c_float, w: rect.size.x as c_float, h: rect.size.y as c_float }, &SDL_FPoint { x: origin.x as c_float, y: origin.y as c_float }, &SDL_FPoint { x: right.x as c_float, y: right.y as c_float }, &SDL_FPoint { x: down.x as c_float, y: down.y as c_float })); }
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

impl AsSdlExt<*mut SDL_Texture> for Texture {

	fn as_sdl(&self) -> *mut SDL_Texture {
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

/// Options for drawing a texture.
#[derive(Clone, Copy)]
pub struct TextureDrawOptions {
	/// The portion of the texture to draw. If `None`, uses the full texture.
	pub rect:      Option<Rect<f32>>,
	/// The offset applied to the texture.
	pub offset:    Vec2<f32>,
	/// The transform applied to the texture.
	pub transform: Transform,
	/// The color modulation applied to the texture.
	pub modulate:  Color<u8>,
}

impl Default for TextureDrawOptions {

	fn default() -> Self {
		Self {
			rect:      None,
			offset:    Vec2::ZERO,
			transform: Transform::IDENTITY,
			modulate:  Color::WHITE,
		}
	}

}

/// The method by which a texture is sampled.
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