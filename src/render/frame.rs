use std::ffi::c_float;
use std::iter;
use std::marker::PhantomData;
use std::mem::MaybeUninit;
use std::ops::Deref;
use std::ptr::NonNull;

use sdl3_sys::rect::SDL_FRect;
use sdl3_sys::render::*;

use crate::math::{Color, Rect, Vec2};
use crate::render::Renderer;
use crate::sdl_util::{AsSdlExt, sdl_assert};

/// A frame of a renderer.
///
/// # Examples
///
/// To get a frame, first create a [`Renderer`]:
///
/// ```
/// # use kibble::math::Vec2;
/// # use kibble::render::Renderer;
/// # use kibble::window::Window;
/// let window = Window::new("title", Vec2::ZERO);
/// let mut renderer = Renderer::new(&window);
/// let mut frame = renderer.frame();
/// ```
///
/// To draw to a frame:
///
/// ```
/// # use kibble::math::{Color, Vec2};
/// # use kibble::render::Renderer;
/// # use kibble::window::Window;
/// # let window = Window::new("title", Vec2::ZERO);
/// # let mut renderer = Renderer::new(&window);
/// let mut frame = window.frame();
/// // Clears the frame to black
/// frame.clear(Color::BLACK);
/// // Fills a circle in the top-left corner
/// frame.fill_circle(Vec2::ZERO, 16.0, Color::WHITE);
/// // Presents the frame in its window
/// frame.present();
/// ```
pub struct Frame<'a>(NonNull<SDL_Renderer>, PhantomData<&'a mut Renderer>);

impl<'a> Frame<'a> {

	/// Returns the size of the frame, in pixels.
	pub fn size(&self) -> Vec2<u32> {
		let mut w = MaybeUninit::uninit();
		let mut h = MaybeUninit::uninit();
		sdl_assert!(unsafe { SDL_GetCurrentRenderOutputSize(self.as_sdl(), w.as_mut_ptr(), h.as_mut_ptr()) });
		Vec2 { x: unsafe { w.assume_init() } as u32, y: unsafe { h.assume_init() } as u32 }
	}

	/// Fills a pixel.
	pub fn draw_point(&mut self, pos: Vec2<f32>, color: Color<u8>) {
		sdl_assert!(unsafe { SDL_SetRenderDrawColor(self.as_sdl(), color.r, color.g, color.b, color.a)
			&& SDL_RenderPoint(self.as_sdl(), pos.x as c_float, pos.y as c_float) });
	}

	/// Draws a line from one point to another.
	pub fn draw_line(&mut self, a: Vec2<f32>, b: Vec2<f32>, color: Color<u8>) {
		sdl_assert!(unsafe { SDL_SetRenderDrawColor(self.as_sdl(), color.r, color.g, color.b, color.a)
			&& SDL_RenderLine(self.as_sdl(), a.x as c_float, a.y as c_float, b.x as c_float, b.y as c_float) });
	}

	/// Outlines a rectangle.
	pub fn draw_rect(&mut self, rect: Rect<f32>, color: Color<u8>) {
		sdl_assert!(unsafe { SDL_SetRenderDrawColor(self.as_sdl(), color.r, color.g, color.b, color.a)
			&& SDL_RenderRect(self.as_sdl(), &SDL_FRect { x: rect.pos.x as c_float, y: rect.pos.y as c_float, w: rect.size.x as c_float, h: rect.size.y as c_float }) });
	}

	/// Fills a rectangle.
	pub fn fill_rect(&mut self, rect: Rect<f32>, color: Color<u8>) {
		sdl_assert!(unsafe { SDL_SetRenderDrawColor(self.as_sdl(), color.r, color.g, color.b, color.a)
			&& SDL_RenderFillRect(self.as_sdl(), &SDL_FRect { x: rect.pos.x as c_float, y: rect.pos.y as c_float, w: rect.size.x as c_float, h: rect.size.y as c_float }) });
	}

	/// Outlines a circle.
	pub fn draw_circle(&mut self, center: Vec2<f32>, radius: f32, color: Color<u8>) {
		for pos in bresenham_circle(radius as i32) {
			let pos = pos.map(|v| v as f32);
			self.draw_point(Vec2 { x: center.x + pos.x, y: center.y - pos.y }, color);
			self.draw_point(Vec2 { x: center.x + pos.x, y: center.y + pos.y }, color);
			self.draw_point(Vec2 { x: center.x - pos.x, y: center.y + pos.y }, color);
			self.draw_point(Vec2 { x: center.x - pos.x, y: center.y - pos.y }, color);
			self.draw_point(Vec2 { x: center.x + pos.y, y: center.y - pos.x }, color);
			self.draw_point(Vec2 { x: center.x + pos.y, y: center.y + pos.x }, color);
			self.draw_point(Vec2 { x: center.x - pos.y, y: center.y + pos.x }, color);
			self.draw_point(Vec2 { x: center.x - pos.y, y: center.y - pos.x }, color);
		}
	}

	/// Fills a circle.
	pub fn fill_circle(&mut self, center: Vec2<f32>, radius: f32, color: Color<u8>) {
		for pos in bresenham_circle(radius as i32) {
			let pos = pos.map(|v| v as f32);
			self.draw_line(Vec2 { x: center.x - pos.x, y: center.y - pos.y }, Vec2 { x: center.x + pos.x, y: center.y - pos.y }, color);
			self.draw_line(Vec2 { x: center.x - pos.x, y: center.y + pos.y }, Vec2 { x: center.x + pos.x, y: center.y + pos.y }, color);
			self.draw_line(Vec2 { x: center.x - pos.y, y: center.y + pos.x }, Vec2 { x: center.x + pos.y, y: center.y + pos.x }, color);
			self.draw_line(Vec2 { x: center.x - pos.y, y: center.y - pos.x }, Vec2 { x: center.x + pos.y, y: center.y - pos.x }, color);
		}
	}

	/// Clears the frame with a solid color.
	pub fn clear(&mut self, color: Color<u8>) {
		sdl_assert!(unsafe { SDL_SetRenderDrawColor(self.as_sdl(), color.r, color.g, color.b, color.a)
			&& SDL_RenderClear(self.as_sdl()) });
	}

	/// Displays the frame in its window.
	pub fn present(self) {
		sdl_assert!(unsafe { SDL_RenderPresent(self.as_sdl()) });
	}

	/// Wraps an `SDL_Renderer` pointer in a [`Frame`].
	pub fn from_sdl_renderer(sdl_renderer: NonNull<SDL_Renderer>) -> Self {
		Self(sdl_renderer, PhantomData)
	}

}

impl<'a> Deref for Frame<'a> {

	type Target = Renderer;

	fn deref(&self) -> &Self::Target {
		// SAFETY: `Renderer` is a `repr(transparent)` wrapper around a single
		// `NonNull<SDL_Renderer>`, so a pointer to the `NonNull<SDL_Renderer>`
		// inside `Frame` is the same as a pointer to a `Renderer`.
		unsafe { (&raw const self.0 as *const Renderer).as_ref_unchecked() }
	}

}

impl<'a> AsSdlExt for Frame<'a> {

	type Sdl = *mut SDL_Renderer;

	fn as_sdl(&self) -> Self::Sdl {
		self.0.as_ptr()
	}

}

/// Returns an iterator over points in a Bresenham circle in the arc \[0°, -45°].
///
/// See the [midpoint circle algorithm].
///
/// [midpoint circle algorithm]:
///     https://en.wikipedia.org/wiki/Midpoint_circle_algorithm
fn bresenham_circle(radius: i32) -> impl Iterator<Item = Vec2<i32>> {
	let mut point = Vec2 { x: radius, y: 0 };
	let mut t1 = radius / 16;
	iter::from_fn(move || if point.x >= point.y {
		let last_point = point;
		point.y += 1;
		t1 += point.y;
		let t2 = t1 - point.x;
		if t2 >= 0 {
			t1 = t2;
			point.x -= 1;
		}
		Some(last_point)
	} else {
		None
	})
}