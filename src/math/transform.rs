use super::Vec2;

/// A 2D transformation matrix.
///
/// Transforms are constructed by mutating the identity transform, e.g.:
///
/// ```
/// # use std::f32::consts::PI;
/// # use kibble::math::Transform;
/// let transform = Transform::IDENTITY.rotated(PI);
/// ```
#[derive(Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Transform {
	m11: f32, m12: f32,
	m21: f32, m22: f32,
	pub translation: Vec2<f32>,
}

impl Transform {

	/// The identity transform.
	pub const IDENTITY: Self = Self {
		m11: 1.0, m12: 0.0,
		m21: 0.0, m22: 1.0,
		translation: Vec2::ZERO,
	};

	/// Returns the transform with added rotation, in radians.
	pub fn rotated(mut self, by: f32) -> Self {
		self.m11 *=  by.cos();
		self.m12 *= -by.sin();
		self.m21 *=  by.sin();
		self.m22 *=  by.cos();
		self
	}

	/// Returns the transform with added scale.
	pub fn scaled(mut self, by: Vec2<f32>) -> Self {
		self.m11 *= by.x;
		self.m22 *= by.y;
		self
	}

	/// Returns the transform with added shear.
	pub fn sheared(mut self, by: f32) -> Self {
		self.m12 *= by;
		self
	}

	/// Returns the transform with added translation.
	pub fn translated(mut self, by: Vec2<f32>) -> Self {
		self.translation += by;
		self
	}

	/// Stacks two transforms.
	pub fn stack(self, other: Self) -> Self {
		Self {
			m11: self.m11 * other.m11 + self.m12 * other.m21,
			m12: self.m11 * other.m12 + self.m12 * other.m22,
			m21: self.m21 * other.m11 + self.m22 * other.m21,
			m22: self.m21 * other.m12 + self.m22 * other.m22,
			translation: self.translation + other.translation,
		}
	}

	/// Returns `true` if the transform only encodes a translation.
	///
	/// # Examples
	///
	/// ```
	/// # use kibble::math::{Transform, Vec2};
	/// let transform = Transform::IDENTITY.translated(Vec2::of(4.0));
	/// assert_eq!(transform.is_translation(), true);
	/// assert_eq!(transform.scaled(Vec2::of(2.0)).is_translation(), false);
	/// ```
	pub fn is_translation(self) -> bool {
		self.m11 == 1.0 && self.m12 == 0.0 &&
		self.m21 == 0.0 && self.m22 == 1.0
	}

	/// Transforms a point without translating it.
	pub fn multiply(self, vector: Vec2<f32>) -> Vec2<f32> {
		Vec2 {
			x: self.m11 * vector.x + self.m12 * vector.y,
			y: self.m21 * vector.x + self.m22 * vector.y,
		}
	}

	/// Transforms a point.
	pub fn transform(self, vector: Vec2<f32>) -> Vec2<f32> {
		self.multiply(vector) + self.translation
	}

}

impl Default for Transform {

	fn default() -> Self {
		Self::IDENTITY
	}

}