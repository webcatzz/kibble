//! Generic vectors.

use std::ffi::{c_float, c_int};
use std::fmt;
use std::ops::{Add, Sub, Mul, Div, Rem, Neg, AddAssign, SubAssign, MulAssign, DivAssign, RemAssign, Index, IndexMut};

use num_traits::{ConstOne, ConstZero, Float};
use sdl3_sys::rect::{SDL_FPoint, SDL_Point};

/// Defines a generic vector type with the given components.
///
/// # Usage
///
/// ```ignore
/// impl_vector!(Vec2(x, y));
/// ```
macro_rules! impl_vector {
	( $( #[$attr:meta] )* $name:ident($( $comp:ident ),+) $( ; )? ) => {

		$( #[$attr] )*
		#[derive(Default, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
		pub struct $name<T> {
			$( pub $comp: T, )+
		}

		impl<T: ConstZero> $name<T> {

			/// A vector with all components set to zero.
			pub const ZERO: Self = Self { $( $comp: T::ZERO, )+ };

		}

		impl<T: ConstOne> $name<T> {

			/// A vector with all components set to one.
			pub const ONE: Self = Self { $( $comp: T::ONE, )+ };

		}

		impl<T: Copy> $name<T> {

			/// Returns a vector with all components set to the given value.
			#[inline]
			pub const fn of(v: T) -> Self {
				Self { $( $comp: v, )+ }
			}

		}

		impl<T> $name<T> {

			/// Returns a vector with the result of calling `f` on its components.
			#[inline]
			pub fn map<U>(self, mut f: impl FnMut(T) -> U) -> $name<U> {
				$name { $( $comp: f(self.$comp), )+ }
			}

			/// Converts a pair of vectors into a vector of pairs.
			#[inline]
			pub fn zip<U>(self, other: $name<U>) -> $name<(T, U)> {
				$name { $( $comp: (self.$comp, other.$comp), )+ }
			}

		}

		impl<T: Float + ConstZero> $name<T> {

			/// Returns the length (or magnitude) of the vector.
			pub fn length(self) -> T {
				($( self.$comp * self.$comp + )+ T::ZERO).sqrt()
			}

			/// Scales the vector to unit length. Returns [`ZERO`] if the vector's
			/// length is equal to `0.0`.
			///
			/// # Examples
			///
			/// ```
			/// # use kibble::math::Vec2;
			/// assert_eq!(Vec2 { x: 2.0, y: 0.0 }.normalize(), Vec2 { x: 1.0, y: 0.0 });
			/// assert_eq!(Vec2::<f32>::ZERO.normalize(), Vec2::ZERO);
			/// ```
			///
			/// [`ZERO`]: Self::ZERO
			pub fn normalize(self) -> Self {
				match self.length() {
					length if length == T::ZERO => Self::ZERO,
					length                      => self / length,
				}
			}

			/// Linearly interpolates between two vectors by the given weight.
			///
			/// # Examples
			///
			/// ```
			/// # use kibble::math::Vec2;
			/// let vec_a = Vec2::of(0.0);
			/// let vec_b = Vec2::of(1.0);
			/// assert_eq!(vec_a.lerp(vec_b, 0.5), Vec2::of(0.5));
			/// ```
			pub fn lerp(self, to: Self, by: T) -> Self {
				self + (to - self) * by
			}

		}

		impl<T: Add<Output = T>> Add for $name<T> {

			type Output = Self;

			fn add(mut self, rhs: Self) -> Self::Output {
				$( self.$comp = self.$comp + rhs.$comp; )+
				self
			}

		}

		impl<T: Sub<Output = T>> Sub for $name<T> {

			type Output = Self;

			fn sub(mut self, rhs: Self) -> Self::Output {
				$( self.$comp = self.$comp - rhs.$comp; )+
				self
			}

		}

		impl<T: Mul<Output = T>> Mul for $name<T> {

			type Output = Self;

			fn mul(mut self, rhs: Self) -> Self::Output {
				$( self.$comp = self.$comp * rhs.$comp; )+
				self
			}

		}

		impl<T: Mul<Output = T> + Copy> Mul<T> for $name<T> {

			type Output = Self;

			fn mul(mut self, rhs: T) -> Self::Output {
				$( self.$comp = self.$comp * rhs; )+
				self
			}

		}

		impl<T: Div<Output = T>> Div for $name<T> {

			type Output = Self;

			fn div(mut self, rhs: Self) -> Self::Output {
				$( self.$comp = self.$comp / rhs.$comp; )+
				self
			}

		}

		impl<T: Div<Output = T> + Copy> Div<T> for $name<T> {

			type Output = Self;

			fn div(mut self, rhs: T) -> Self::Output {
				$( self.$comp = self.$comp / rhs; )+
				self
			}

		}

		impl<T: Rem<Output = T>> Rem for $name<T> {

			type Output = Self;

			fn rem(mut self, rhs: Self) -> Self::Output {
				$( self.$comp = self.$comp % rhs.$comp; )+
				self
			}

		}

		impl<T: Rem<Output = T> + Copy> Rem<T> for $name<T> {

			type Output = Self;

			fn rem(mut self, rhs: T) -> Self::Output {
				$( self.$comp = self.$comp % rhs; )+
				self
			}

		}

		impl<T: Neg<Output = T>> Neg for $name<T> {

			type Output = Self;

			fn neg(mut self) -> Self::Output {
				$( self.$comp = -self.$comp; )+
				self
			}

		}

		impl<T: AddAssign> AddAssign for $name<T> {

			fn add_assign(&mut self, rhs: Self) {
				$( self.$comp += rhs.$comp; )+
			}

		}

		impl<T: SubAssign> SubAssign for $name<T> {

			fn sub_assign(&mut self, rhs: Self) {
				$( self.$comp -= rhs.$comp; )+
			}

		}

		impl<T: MulAssign> MulAssign for $name<T> {

			fn mul_assign(&mut self, rhs: Self) {
				$( self.$comp *= rhs.$comp; )+
			}

		}

		impl<T: MulAssign + Copy> MulAssign<T> for $name<T> {

			fn mul_assign(&mut self, rhs: T) {
				$( self.$comp *= rhs; )+
			}

		}

		impl<T: DivAssign> DivAssign for $name<T> {

			fn div_assign(&mut self, rhs: Self) {
				$( self.$comp /= rhs.$comp; )+
			}

		}

		impl<T: DivAssign + Copy> DivAssign<T> for $name<T> {

			fn div_assign(&mut self, rhs: T) {
				$( self.$comp /= rhs; )+
			}

		}

		impl<T: RemAssign> RemAssign for $name<T> {

			fn rem_assign(&mut self, rhs: Self) {
				$( self.$comp %= rhs.$comp; )+
			}

		}

		impl<T: RemAssign + Copy> RemAssign<T> for $name<T> {

			fn rem_assign(&mut self, rhs: T) {
				$( self.$comp %= rhs; )+
			}

		}

		impl<T: fmt::Display> fmt::Debug for $name<T> {

			fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
				write!(f, "(")?;
				write_vector!(f, self($( $comp ),+));
				write!(f, ")")
			}

		}

	};
}

/// Implements [`Index`] and [`IndexMut`] with [`Axis`] for a vector type.
///
/// # Usage
///
/// ```ignore
/// impl_index!(Vec2 {
///   Axis::X => x,
///   Axis::Y => y,
/// });
/// ```
macro_rules! impl_axis_index {
	( $name:ident {
		$( $axis:pat => $comp:ident ),* $( , )?
	} ) => {

		impl<T> Index<Axis> for $name<T> {

			type Output = T;

			fn index(&self, index: Axis) -> &Self::Output {
				#[allow(unreachable_patterns)]
				match index {
					$( $axis => &self.$comp, )*
					axis => panic!("Invalid axis: {axis}"),
				}
			}

		}

		impl<T> IndexMut<Axis> for $name<T> {

			fn index_mut(&mut self, index: Axis) -> &mut Self::Output {
				#[allow(unreachable_patterns)]
				match index {
					$( $axis => &mut self.$comp, )*
					axis => panic!("Invalid axis: {axis}"),
				}
			}

		}

	};
}

/// Writes a vector type to a formatter with the form `(x, y, ...)`.
macro_rules! write_vector {
	( $f:ident, $self:ident($head: ident, $( $tail:ident ),+) ) => {
		write!($f, "{}, ", $self.$head)?;
		write_vector!($f, $self($( $tail ),+));
	};
	( $f:ident, $self:ident($comp:ident) ) => {
		write!($f, "{}", $self.$comp)?;
	};
}

impl_vector! {
	/// A two-dimensional vector.
	Vec2(x, y);
}

impl_vector! {
	/// A three-dimensional vector.
	Vec3(x, y, z);
}

impl_vector! {
	/// A four-dimensional vector.
	Vec4(x, y, z, w);
}

impl_axis_index!(Vec2 {
	Axis::X => x,
	Axis::Y => y,
});

impl_axis_index!(Vec3 {
	Axis::X => x,
	Axis::Y => y,
	Axis::Z => z,
});

impl_axis_index!(Vec4 {
	Axis::X => x,
	Axis::Y => y,
	Axis::Z => z,
	Axis::W => w,
});

impl<T: Float> Vec2<T> {

	/// Returns a unit vector with the given angle with respect to the positive X
	/// axis.
	pub fn from_angle(angle: T) -> Self {
		Vec2 { x: angle.cos(), y: angle.sin() }
	}

	/// Returns the angle of the vector with respect to the positive X axis.
	pub fn angle(self) -> T {
		self.y.atan2(self.x)
	}

}

impl<T> From<(T, T)> for Vec2<T> {


	fn from(value: (T, T)) -> Self {
		Self { x: value.0, y: value.1 }
	}

}

impl<T> From<(T, T, T)> for Vec3<T> {

	fn from(value: (T, T, T)) -> Self {
		Self { x: value.0, y: value.1, z: value.2 }
	}

}

impl<T> From<(T, T, T, T)> for Vec4<T> {

	fn from(value: (T, T, T, T)) -> Self {
		Self { x: value.0, y: value.1, z: value.2, w: value.3 }
	}

}

impl From<SDL_Point> for Vec2<c_int> {

	fn from(value: SDL_Point) -> Self {
		Self { x: value.x, y: value.y }
	}

}

impl From<SDL_FPoint> for Vec2<c_float> {

	fn from(value: SDL_FPoint) -> Self {
		Self { x: value.x, y: value.y }
	}

}

impl<T> Into<(T, T)> for Vec2<T> {

	fn into(self) -> (T, T) {
		(self.x, self.y)
	}

}

impl<T> Into<(T, T, T)> for Vec3<T> {

	fn into(self) -> (T, T, T) {
		(self.x, self.y, self.z)
	}

}

impl<T> Into<(T, T, T, T)> for Vec4<T> {

	fn into(self) -> (T, T, T, T) {
		(self.x, self.y, self.z, self.w)
	}

}

impl Into<SDL_Point> for Vec2<c_int> {

	fn into(self) -> SDL_Point {
		SDL_Point { x: self.x, y: self.y }
	}

}

impl Into<SDL_FPoint> for Vec2<c_float> {

	fn into(self) -> SDL_FPoint {
		SDL_FPoint { x: self.x, y: self.y }
	}

}

/// A coordinate axis.
///
/// Used to index vector types, e.g.:
///
/// ```
/// # use kibble::math::{Axis, Vec2};
/// let point = Vec2 { x: 4, y: 3 };
/// assert_eq!(point[Axis::X], point.x);
/// ```
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum Axis { X, Y, Z, W }

impl fmt::Display for Axis {

	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Self::X => write!(f, "X"),
			Self::Y => write!(f, "Y"),
			Self::Z => write!(f, "Z"),
			Self::W => write!(f, "W"),
		}
	}

}