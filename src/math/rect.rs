//! Generic rectangles.

use std::ffi::{c_float, c_int};
use std::ops::{Add, AddAssign, Mul, Neg, Sub, SubAssign};

use num_traits::{ConstOne, ConstZero};
use sdl3_sys::rect::{SDL_FRect, SDL_Rect};

use super::{Axis, Transform, Vec2};

/// A rectangle.
#[derive(Default, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Rect<T, U = T> {
	/// The top-left corner of the rectangle.
	pub pos:  Vec2<T>,
	/// The size of the rectangle.
	pub size: Vec2<U>,
}

impl<T: ConstZero, U: ConstZero> Rect<T, U> {

	/// A rectangle starting at the origin with no size.
	pub const ZERO: Self = Self { pos: Vec2::ZERO, size: Vec2::ZERO };

}

impl<T: ConstZero, U: ConstOne> Rect<T, U> {

	/// A one-by-one rectangle starting at the origin.
	pub const ONE: Self = Self { pos: Vec2::ZERO, size: Vec2::ONE };

}

impl<T: Copy + Add<Output = T>> Rect<T> {

	/// Returns the bottom-right corner of a rect.
	pub fn end(&self) -> Vec2<T> {
		self.pos + self.size
	}

	/// Returns the `x` coordinate of the bottom-right corner of the rectangle.
	pub fn end_x(&self) -> T {
		self.pos.x + self.size.x
	}

	/// Returns the `y` coordinate of the bottom-right corner of the rectangle.
	pub fn end_y(&self) -> T {
		self.pos.y + self.size.y
	}

}

impl<T> Rect<T> {

	/// Returns a rectangle with the results of calling `f` on its coordinates.
	#[inline]
	pub fn map<U>(self, mut f: impl FnMut(T) -> U) -> Rect<U> {
		Rect {
			pos:  Vec2 { x: f(self.pos.x),  y: f(self.pos.y),  },
			size: Vec2 { x: f(self.size.x), y: f(self.size.y), },
		}
	}

	/// Converts a pair of rectangles into a rectangle of pairs.
	#[inline]
	pub fn zip<U>(self, other: Rect<U>) -> Rect<(T, U)> {
		Rect {
			pos:  self.pos.zip(other.pos),
			size: self.size.zip(other.size),
		}
	}

}

impl<T: Copy + Mul<Output = T>> Rect<T> {

	/// Returns the area of the rectangle.
	pub fn area(&self) -> T {
		self.size.x * self.size.y
	}

}

impl<T: Copy + PartialOrd + Add<Output = T>> Rect<T> {

	/// Returns true if the given point is within the rectangle.
	pub fn contains_point(self, point: Vec2<T>) -> bool {
		point >= self.pos && point <= self.end()
	}

}

impl<T: Copy + Ord + Add<Output = T> + Sub<Output = T>> Rect<T> {

	/// Returns a rectangle extended to enclose the given point.
	pub fn extend_to_point(mut self, pos: Vec2<T>) -> Self {
		for axis in [Axis::X, Axis::Y] {
			if pos[axis] < self.pos[axis] {
				self.pos[axis] = pos[axis];
			} else {
				self.size[axis] = self.size[axis].min(pos[axis] - self.pos[axis]);
			}
		}
		self
	}

}

impl<T: Copy + AddAssign + SubAssign + Add<Output = T>> Rect<T> {

	/// Expands the rectangle by the given amount on all sides.
	pub fn grow(mut self, by: T) -> Self {
		let vec = Vec2::of(by);
		self.pos  -= vec;
		self.size += vec + vec;
		self
	}

	/// Expands the rectangle on each side by the given amounts.
	pub fn grow_sides(mut self, top: T, right: T, bottom: T, left: T) -> Self {
		self.pos.x  -= left;
		self.pos.y  -= top;
		self.size.x += left + right;
		self.size.y += top + bottom;
		self
	}

	/// Expands the rectangle on one side by the given amount.
	pub fn grow_side(mut self, dir: Dir, by: T) -> Self {
		match dir {
			Dir::Left  => self.pos.x  -= by,
			Dir::Right => self.size.x += by,
			Dir::Up    => self.pos.y  -= by,
			Dir::Down  => self.size.y += by,
		}
		self
	}

}

impl<T: Copy + SubAssign> Rect<T> {

	/// Splits the rectangle into two at the given position along the given axis.
	pub fn split(mut self, axis: Axis, at: T) -> (Self, Self) {
		let mut other = self;
		self.size[axis] = at;
		other.pos[axis] = at;
		other.size[axis] -= at;
		(self, other)
	}

}

impl Rect<f32> {

	/// Transforms the rectangle.
	pub fn transform(mut self, transform: Transform) -> Self {
		self.pos  = transform.transform(self.pos);
		self.size = transform.multiply(self.size);
		self
	}

}

impl From<SDL_Rect> for Rect<c_int> {

	fn from(value: SDL_Rect) -> Self {
		Self { pos: Vec2 { x: value.x, y: value.y }, size: Vec2 { x: value.w, y: value.h } }
	}

}

impl From<SDL_FRect> for Rect<c_float> {

	fn from(value: SDL_FRect) -> Self {
		Self { pos: Vec2 { x: value.x, y: value.y }, size: Vec2 { x: value.w, y: value.h } }
	}

}

impl Into<SDL_Rect> for Rect<c_int> {

	fn into(self) -> SDL_Rect {
		SDL_Rect { x: self.pos.x, y: self.pos.y, w: self.size.x, h: self.size.y }
	}

}

impl Into<SDL_FRect> for Rect<c_float> {

	fn into(self) -> SDL_FRect {
		SDL_FRect { x: self.pos.x, y: self.pos.y, w: self.size.x, h: self.size.y }
	}

}

/// A cardinal direction.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum Dir {
	Up,
	Down,
	Left,
	Right,
}

impl Dir {

	/// Returns a unit vector representing the direction.
	pub fn unit<T: ConstOne + ConstZero + Neg<Output = T>>(self) -> Vec2<T> {
		match self {
			Self::Up    => Vec2 { x:  T::ZERO, y: -T::ONE  },
			Self::Down  => Vec2 { x:  T::ZERO, y:  T::ONE  },
			Self::Left  => Vec2 { x: -T::ONE,  y:  T::ZERO },
			Self::Right => Vec2 { x:  T::ONE,  y:  T::ZERO },
		}
	}

}