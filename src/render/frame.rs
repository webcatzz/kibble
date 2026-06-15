use std::ffi::{c_float, c_int};
use std::marker::PhantomData;
use std::mem::MaybeUninit;
use std::ops::{Deref, DerefMut};
use std::ptr::{self, NonNull};

use sdl3_sys::rect::{SDL_FRect, SDL_Rect};
use sdl3_sys::render::*;

use crate::math::{Color, Rect, Vec2};
use crate::sdl_util::{AsSdlExt, sdl_assert};

/// A rendering canvas.
///
/// # Examples
///
/// To get a frame, first open a [`Window`]:
///
/// ```
/// # use kibble::math::Vec2;
/// # use kibble::window::Window;
/// let mut window = Window::new("title", Vec2::ZERO);
/// let mut frame = window.frame();
/// ```
///
/// To draw to a frame:
///
/// ```
/// # use kibble::math::{Color, Vec2};
/// # use kibble::window::Window;
/// # let mut window = Window::new("title", Vec2::ZERO);
/// let mut frame = window.frame();
/// // Clears the frame to black
/// frame.clear(Color::BLACK);
/// // Fills a circle in the top-left corner
/// frame.fill_circle(Vec2::ZERO, 16.0, Color::WHITE);
/// // Presents the frame in its window
/// frame.present();
/// ```
///
/// [`Window`]: crate::window::Window
pub struct Frame<'a> {
	sdl_renderer: NonNull<SDL_Renderer>,
	phantom:      PhantomData<&'a ()>,
}

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
		for pos in BresenhamCircle::new(radius as i32) {
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
		for pos in BresenhamCircle::new(radius as i32) {
			let pos = pos.map(|v| v as f32);
			self.draw_line(Vec2 { x: center.x - pos.x, y: center.y - pos.y }, Vec2 { x: center.x + pos.x, y: center.y - pos.y }, color);
			self.draw_line(Vec2 { x: center.x - pos.x, y: center.y + pos.y }, Vec2 { x: center.x + pos.x, y: center.y + pos.y }, color);
			self.draw_line(Vec2 { x: center.x - pos.y, y: center.y + pos.x }, Vec2 { x: center.x + pos.y, y: center.y + pos.x }, color);
			self.draw_line(Vec2 { x: center.x - pos.y, y: center.y - pos.x }, Vec2 { x: center.x + pos.y, y: center.y - pos.x }, color);
		}
	}

	/// Returns a clipped area of the frame for rendering.
	pub fn clip<'r>(&'r mut self, rect: Rect<u32>) -> ClippedFrame<'r, 'a> {
		sdl_assert!(unsafe { SDL_SetRenderClipRect(self.as_sdl(), &SDL_Rect { x: rect.pos.x as c_int, y: rect.pos.y as c_int, w: rect.size.x as c_int, h: rect.size.y as c_int }) });
		ClippedFrame(self)
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
		Self { sdl_renderer, phantom: PhantomData }
	}

}

impl<'a> AsSdlExt<*mut SDL_Renderer> for Frame<'a> {

	fn as_sdl(&self) -> *mut SDL_Renderer {
		self.sdl_renderer.as_ptr()
	}

}

pub struct ClippedFrame<'r, 'a>(&'r mut Frame<'a>);

impl<'r, 'a> Deref for ClippedFrame<'r, 'a> {

	type Target = Frame<'a>;

	fn deref(&self) -> &Self::Target {
		self.0
	}

}

impl<'r, 'a> DerefMut for ClippedFrame<'r, 'a> {

	fn deref_mut(&mut self) -> &mut Self::Target {
		self.0
	}

}

impl<'r, 'a> Drop for ClippedFrame<'r, 'a> {

	fn drop(&mut self) {
		sdl_assert!(unsafe { SDL_SetRenderClipRect(self.as_sdl(), ptr::null()) })
	}

}

/// An iterator over points in a Bresenham circle.
///
/// See the [midpoint circle algorithm].
///
/// [midpoint circle algorithm]:
///     https://en.wikipedia.org/wiki/Midpoint_circle_algorithm
struct BresenhamCircle {
	point: Vec2<i32>,
	t1:    i32,
}

impl BresenhamCircle {

	/// Starts a new Bresenham circle with the given radius.
	fn new(radius: i32) -> Self {
		Self {
			point: Vec2 { x: radius, y: 0 },
			t1:    radius / 16,
		}
	}

}

impl Iterator for BresenhamCircle {

	type Item = Vec2<i32>;

	fn next(&mut self) -> Option<Self::Item> {
		if self.point.x >= self.point.y {
			let last_point = self.point;
			self.point.y += 1;
			self.t1 += self.point.y;
			let t2 = self.t1 - self.point.x;
			if t2 >= 0 {
				self.t1 = t2;
				self.point.x -= 1;
			}
			Some(last_point)
		} else {
			None
		}
	}

}