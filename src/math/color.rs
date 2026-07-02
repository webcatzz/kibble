//! Generic colors.

use std::fmt;
use std::num::ParseIntError;

use num_traits::Float;

/// An RGBA color.
#[derive(Default, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Color<C> {
	/// The red component of the color.
	pub r: C,
	/// The green component of the color.
	pub g: C,
	/// The blue component of the color.
	pub b: C,
	/// The alpha component of the color.
	pub a: C,
}

impl<C: ColorComponent> Color<C> {

	/// Absolute black.
	pub const BLACK:   Self = Self::from_rgb(C::MIN, C::MIN, C::MIN);
	/// Absolute white.
	pub const WHITE:   Self = Self::from_rgb(C::MAX, C::MAX, C::MAX);
	/// Absolute red.
	pub const RED:     Self = Self::from_rgb(C::MAX, C::MIN, C::MIN);
	/// Absolute green.
	pub const GREEN:   Self = Self::from_rgb(C::MIN, C::MAX, C::MIN);
	/// Absolute blue.
	pub const BLUE:    Self = Self::from_rgb(C::MIN, C::MIN, C::MAX);
	/// Absolute yellow.
	pub const YELLOW:  Self = Self::from_rgb(C::MAX, C::MAX, C::MIN);
	/// Absolute cyan.
	pub const CYAN:    Self = Self::from_rgb(C::MIN, C::MAX, C::MAX);
	/// Absolute magenta.
	pub const MAGENTA: Self = Self::from_rgb(C::MAX, C::MIN, C::MAX);

	/// Returns an opaque color with its `r`, `g`, and `b` components set.
	pub const fn from_rgb(r: C, g: C, b: C) -> Self {
		Self { r, g, b, a: C::MAX }
	}

	/// Returns a color with its `r`, `g`, `b`, and `a` components set.
	pub const fn from_rgba(r: C, g: C, b: C, a: C) -> Self {
		Self { r, g, b, a }
	}

	/// Returns a grayscale color with the given value.
	pub const fn from_value(v: C) -> Self {
		Self { r: v, g: v, b: v, a: C::MAX }
	}

	/// Returns a white color with the given alpha.
	pub const fn from_alpha(a: C) -> Self {
		Self { r: C::MAX, g: C::MAX, b: C::MAX, a }
	}

}

impl Color<u8> {

	/// Converts a hexcode of the form `0xRRGGBB` to a color.
	pub const fn from_hex_rgb(v: u32) -> Self {
		Color {
			r: (v >> 16) as u8,
			g: (v >> 8)  as u8,
			b: v         as u8,
			a: u8::MAX,
		}
	}

	/// Converts a hexcode of the form `0xRRGGBBAA` to a color.
	pub const fn from_hex_rgba(v: u32) -> Self {
		Color {
			r: (v >> 24) as u8,
			g: (v >> 16) as u8,
			b: (v >> 8)  as u8,
			a: v         as u8,
		}
	}

	/// Converts a hexcode of the form `"RRGGBB"` or `"RRGGBBAA"` to a color.
	pub fn from_hex_str(str: &str) -> Result<Self, ParseIntError> {
		let r = u8::from_str_radix(&str[0..2], 16)?;
		let g = u8::from_str_radix(&str[2..4], 16)?;
		let b = u8::from_str_radix(&str[4..6], 16)?;
		let a = match str.get(6..8) {
			Some(src) => u8::from_str_radix(src, 16)?,
			None => u8::MAX,
		};
		Ok(Color { r, g, b, a })
	}

}

impl<F: Float> Color<F> {

	/// Linearly interpolates between two colors by the given weight.
	pub fn lerp(self, to: Self, by: F) -> Self {
		Self {
			r: self.r + (to.r - self.r) * by,
			g: self.g + (to.g - self.g) * by,
			b: self.b + (to.b - self.b) * by,
			a: self.a + (to.a - self.a) * by,
		}
	}

}

impl<C: ColorComponent + fmt::Display> fmt::Debug for Color<C> {

	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "Color({}, {}, {}, {})", self.r, self.g, self.b, self.a)
	}

}

/// Types that can be components of a color.
pub trait ColorComponent: Copy {

	/// The minimum value of the component.
	const MIN: Self;
	/// The maximum value of the component.
	const MAX: Self;

}

impl ColorComponent for u8 {

	const MIN: Self = 0;
	const MAX: Self = 255;

}

impl ColorComponent for f32 {

	const MIN: Self = 0.0;
	const MAX: Self = 1.0;

}

impl ColorComponent for f64 {

	const MIN: Self = 0.0;
	const MAX: Self = 1.0;

}