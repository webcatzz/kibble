use std::fmt;

use sdl3_sys::keycode::*;

/// A representation of a key on a keyboard, based on the current keyboard
/// layout.
///
/// - Keys that produce Unicode characters are represented with the
///   [`KeyLabel::Char`] variant. Note that they are represented in "lowercase",
///   with the character they would produce if no modifier keys were held.
/// - Keys that do not produce Unicode characters are represented with their own
///   variants.
///
/// Thanks to niche filling, [`KeyLabel`] is the same size as [`char`].
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum KeyLabel {
	/// A key that produces a Unicode character.
	Char(char),
	/// An unknown key.
	Unknown,
	// Control keys
	CapsLock,
	Down,
	F1,
	F10,
	F11,
	F12,
	F2,
	F3,
	F4,
	F5,
	F6,
	F7,
	F8,
	F9,
	LAlt,
	LCtrl,
	Left,
	LShift,
	LSuper,
	RAlt,
	RCtrl,
	Return,
	Right,
	RShift,
	RSuper,
	Up,
}

impl fmt::Display for KeyLabel {

	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Self::Char(c) => write!(f, "[{c}]"),
			Self::CapsLock => write!(f, "[CapsLock]"),
			Self::Down     => write!(f, "[ArrowDown]"),
			Self::F1       => write!(f, "[F1]"),
			Self::F10      => write!(f, "[F10]"),
			Self::F11      => write!(f, "[F11]"),
			Self::F12      => write!(f, "[F12]"),
			Self::F2       => write!(f, "[F2]"),
			Self::F3       => write!(f, "[F3]"),
			Self::F4       => write!(f, "[F4]"),
			Self::F5       => write!(f, "[F5]"),
			Self::F6       => write!(f, "[F6]"),
			Self::F7       => write!(f, "[F7]"),
			Self::F8       => write!(f, "[F8]"),
			Self::F9       => write!(f, "[F9]"),
			Self::LAlt     => write!(f, "[LAlt]"),
			Self::LCtrl    => write!(f, "[LCtrl]"),
			Self::Left     => write!(f, "[ArrowLeft]"),
			Self::LShift   => write!(f, "[LShift]"),
			Self::LSuper   => write!(f, "[LSuper]"),
			Self::RAlt     => write!(f, "[RAlt]"),
			Self::RCtrl    => write!(f, "[RCtrl]"),
			Self::Return   => write!(f, "[Return]"),
			Self::Right    => write!(f, "[ArrowRight]"),
			Self::RShift   => write!(f, "[RShift]"),
			Self::RSuper   => write!(f, "[RSuper]"),
			Self::Up       => write!(f, "[ArrowUp]"),
			Self::Unknown  => write!(f, "[Unknown]"),
		}
	}

}

impl From<SDL_Keycode> for KeyLabel {

	fn from(keycode: SDL_Keycode) -> Self {
		match keycode {
			SDLK_A             |
			SDLK_B             |
			SDLK_C             |
			SDLK_D             |
			SDLK_E             |
			SDLK_F             |
			SDLK_G             |
			SDLK_H             |
			SDLK_I             |
			SDLK_J             |
			SDLK_K             |
			SDLK_L             |
			SDLK_M             |
			SDLK_N             |
			SDLK_O             |
			SDLK_P             |
			SDLK_Q             |
			SDLK_R             |
			SDLK_S             |
			SDLK_T             |
			SDLK_U             |
			SDLK_V             |
			SDLK_W             |
			SDLK_X             |
			SDLK_Y             |
			SDLK_Z             |
			SDLK_0             |
			SDLK_1             |
			SDLK_2             |
			SDLK_3             |
			SDLK_4             |
			SDLK_5             |
			SDLK_6             |
			SDLK_7             |
			SDLK_8             |
			SDLK_9             |
			SDLK_AMPERSAND     |
			SDLK_APOSTROPHE    |
			SDLK_ASTERISK      |
			SDLK_AT            |
			SDLK_BACKSLASH     |
			SDLK_CARET         |
			SDLK_COLON         |
			SDLK_COMMA         |
			SDLK_DBLAPOSTROPHE |
			SDLK_DOLLAR        |
			SDLK_EQUALS        |
			SDLK_EXCLAIM       |
			SDLK_GRAVE         |
			SDLK_GREATER       |
			SDLK_HASH          |
			SDLK_LEFTBRACE     |
			SDLK_LEFTBRACKET   |
			SDLK_LEFTPAREN     |
			SDLK_LESS          |
			SDLK_MINUS         |
			SDLK_PERCENT       |
			SDLK_PERIOD        |
			SDLK_PIPE          |
			SDLK_PLUS          |
			SDLK_PLUSMINUS     |
			SDLK_QUESTION      |
			SDLK_RIGHTBRACE    |
			SDLK_RIGHTBRACKET  |
			SDLK_RIGHTPAREN    |
			SDLK_SEMICOLON     |
			SDLK_SLASH         |
			SDLK_TILDE         |
			SDLK_UNDERSCORE    |
			SDLK_BACKSPACE     |
			SDLK_DELETE        |
			SDLK_ESCAPE        |
			SDLK_SPACE         |
			SDLK_TAB           => Self::Char(char::try_from(keycode.0).unwrap()),
			SDLK_CAPSLOCK      => Self::CapsLock,
			SDLK_DOWN          => Self::Down,
			SDLK_F1            => Self::F1,
			SDLK_F10           => Self::F10,
			SDLK_F11           => Self::F11,
			SDLK_F12           => Self::F12,
			SDLK_F2            => Self::F2,
			SDLK_F3            => Self::F3,
			SDLK_F4            => Self::F4,
			SDLK_F5            => Self::F5,
			SDLK_F6            => Self::F6,
			SDLK_F7            => Self::F7,
			SDLK_F8            => Self::F8,
			SDLK_F9            => Self::F9,
			SDLK_LALT          => Self::LAlt,
			SDLK_LCTRL         => Self::LCtrl,
			SDLK_LEFT          => Self::Left,
			SDLK_LGUI          => Self::LSuper,
			SDLK_LSHIFT        => Self::LShift,
			SDLK_RALT          => Self::RAlt,
			SDLK_RCTRL         => Self::RCtrl,
			SDLK_RETURN        => Self::Return,
			SDLK_RGUI          => Self::RSuper,
			SDLK_RIGHT         => Self::Right,
			SDLK_RSHIFT        => Self::RShift,
			SDLK_UP            => Self::Up,
			_                  => Self::Unknown,
		}
	}

}