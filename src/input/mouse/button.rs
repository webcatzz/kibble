use derive_more::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign};
use sdl3_sys::mouse::*;

/// Represents a mouse button.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct MouseButton(u8);

impl MouseButton {

	/// Represents the left mouse button.
	pub const LEFT:   Self = Self(0);
	/// Represents the right mouse button.
	pub const RIGHT:  Self = Self(1);
	/// Represents the middle mouse button.
	pub const MIDDLE: Self = Self(2);

	/// Returns a [`MouseButton`] representing the `n`th additional mouse button,
	/// starting from 0.
	pub const fn additional(n: u8) -> Self {
		Self(n + 3)
	}

	/// Returns the [`MouseButton`] value corresponding to the given SDL mouse
	/// button index.
	pub(crate) fn from_sdl_index(index: u8) -> Self {
		match i32::from(index) {
			SDL_BUTTON_LEFT   => Self::LEFT,
			SDL_BUTTON_RIGHT  => Self::RIGHT,
			SDL_BUTTON_MIDDLE => Self::MIDDLE,
			SDL_BUTTON_X1     => Self::additional(0),
			SDL_BUTTON_X2     => Self::additional(1),
			i => Self::additional((i - SDL_BUTTON_X1) as u8)
		}
	}

}

/// A bitmask of mouse buttons.
#[derive(Clone, Copy, PartialEq, Eq, BitAnd, BitOr, BitXor, BitAndAssign, BitOrAssign, BitXorAssign)]
pub struct MouseButtons(u8);

impl MouseButtons {

	/// An empty mask.
	pub const NONE:   Self = Self(0);
	/// A mask for the left mouse button.
	pub const LEFT:   Self = Self(1 << MouseButton::LEFT.0);
	/// A mask for the right mouse button.
	pub const RIGHT:  Self = Self(1 << MouseButton::RIGHT.0);
	/// A mask for the middle mouse button.
	pub const MIDDLE: Self = Self(1 << MouseButton::MIDDLE.0);

	/// Returns `true` if the flag for the given mouse button is enabled.
	pub const fn is_down(self, button: MouseButton) -> bool {
		self.0 & (1 << button.0) != 0
	}

}

impl From<MouseButton> for MouseButtons {

	/// Returns a mask for the given button.
	fn from(value: MouseButton) -> Self {
		Self(1 << value.0)
	}

}

impl From<SDL_MouseButtonFlags> for MouseButtons {

	fn from(flags: SDL_MouseButtonFlags) -> Self {
		if flags == 0 { MouseButtons::NONE } else {
			let mut mask = MouseButtons::NONE;
			if flags & SDL_BUTTON_LMASK  != 0 { mask |= MouseButtons::LEFT; }
			if flags & SDL_BUTTON_RMASK  != 0 { mask |= MouseButtons::RIGHT; }
			if flags & SDL_BUTTON_MMASK  != 0 { mask |= MouseButtons::MIDDLE; }
			if flags & SDL_BUTTON_X1MASK != 0 { mask |= MouseButtons::from(MouseButton::additional(0)); }
			if flags & SDL_BUTTON_X2MASK != 0 { mask |= MouseButtons::from(MouseButton::additional(1)); }
			mask
		}
	}

}