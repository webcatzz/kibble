use derive_more::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign};
use sdl3_sys::keycode::*;

/// A bitmask of modifier keys.
#[derive(Clone, Copy, PartialEq, Eq, BitAnd, BitOr, BitXor, BitAndAssign, BitOrAssign, BitXorAssign)]
pub struct ModKeys(SDL_Keymod);

impl ModKeys {

	/// An empty mask.
	pub const NONE:      Self = Self(SDL_KMOD_NONE);
	/// A mask for the left and right shift keys.
	pub const SHIFT:     Self = Self(SDL_KMOD_SHIFT);
	/// A mask for the left shift key.
	pub const LSHIFT:    Self = Self(SDL_KMOD_LSHIFT);
	/// A mask for the right shift key.
	pub const RSHIFT:    Self = Self(SDL_KMOD_RSHIFT);
	/// A mask for the left and right ctrl keys.
	pub const CTRL:      Self = Self(SDL_KMOD_CTRL);
	/// A mask for the left ctrl key.
	pub const LCTRL:     Self = Self(SDL_KMOD_LCTRL);
	/// A mask for the right ctrl key.
	pub const RCTRL:     Self = Self(SDL_KMOD_RCTRL);
	/// A mask for the left and right alt keys.
	pub const ALT:       Self = Self(SDL_KMOD_ALT);
	/// A mask for the left alt key.
	pub const LALT:      Self = Self(SDL_KMOD_LALT);
	/// A mask for the right alt key.
	pub const RALT:      Self = Self(SDL_KMOD_RALT);
	/// A mask for the left and right super keys.
	pub const SUPER:     Self = Self(SDL_KMOD_GUI);
	/// A mask for the left super key.
	pub const LSUPER:    Self = Self(SDL_KMOD_LGUI);
	/// A mask for the right super key.
	pub const RSUPER:    Self = Self(SDL_KMOD_RGUI);
	/// A mask for the caps lock key.
	pub const CAPS_LOCK: Self = Self(SDL_KMOD_CAPS);

	/// Returns `true` if either shift key flag is enabled.
	pub const fn has_shift(self) -> bool {
		self.0.0 & Self::SHIFT.0.0 != 0
	}

	/// Returns `true` if the left shift key flag is enabled.
	pub const fn has_lshift(self) -> bool {
		self.0.0 & Self::LSHIFT.0.0 != 0
	}

	/// Returns `true` if the right shift key flag is enabled.
	pub const fn has_rshift(self) -> bool {
		self.0.0 & Self::RSHIFT.0.0 != 0
	}

	/// Returns `true` if either control key flag is enabled.
	pub const fn has_ctrl(self) -> bool {
		self.0.0 & Self::CTRL.0.0 != 0
	}

	/// Returns `true` if the left control key flag is enabled.
	pub const fn has_lctrl(self) -> bool {
		self.0.0 & Self::LCTRL.0.0 != 0
	}

	/// Returns `true` if the right control key flag is enabled.
	pub const fn has_rctrl(self) -> bool {
		self.0.0 & Self::RCTRL.0.0 != 0
	}

	/// Returns `true` if either alt key flag is enabled.
	pub const fn has_alt(self) -> bool {
		self.0.0 & Self::ALT.0.0 != 0
	}

	/// Returns `true` if the left alt key flag is enabled.
	pub const fn has_lalt(self) -> bool {
		self.0.0 & Self::LALT.0.0 != 0
	}

	/// Returns `true` if the right alt key flag is enabled.
	pub const fn has_ralt(self) -> bool {
		self.0.0 & Self::RALT.0.0 != 0
	}

	/// Returns `true` if either super key flag is enabled.
	pub const fn has_super(self) -> bool {
		self.0.0 & Self::SUPER.0.0 != 0
	}

	/// Returns `true` if the left super key flag is enabled.
	pub const fn has_lsuper(self) -> bool {
		self.0.0 & Self::LSUPER.0.0 != 0
	}

	/// Returns `true` if the right super key flag is enabled.
	pub const fn has_rsuper(self) -> bool {
		self.0.0 & Self::RSUPER.0.0 != 0
	}

}

impl From<SDL_Keymod> for ModKeys {

	fn from(keymod: SDL_Keymod) -> Self {
		Self(keymod)
	}

}