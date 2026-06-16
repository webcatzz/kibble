use std::cmp::Ordering;
use std::ffi::c_int;

use sdl3_sys::scancode::SDL_Scancode;

/// A physical location of a key on a keyboard.
///
/// Based on the [USB HID keyboard/keypad usage page]. See its footnotes for
/// notes on usage.
///
/// [USB HID keyboard/keypad usage page]:
///     https://www.usb.org/sites/default/files/hut1_7.pdf#page=90
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Keycode(pub u8);

impl Keycode {

	/// An unknown keycode.
	pub const UNKNOWN:             Self = Self(0x00);
	/// Keyboard 'a' and 'A'.
	pub const A:                   Self = Self(0x04);
	/// Keyboard 'b' and 'B'.
	pub const B:                   Self = Self(0x05);
	/// Keyboard 'c' and 'C'.
	pub const C:                   Self = Self(0x06);
	/// Keyboard 'd' and 'D'.
	pub const D:                   Self = Self(0x07);
	/// Keyboard 'e' and 'E'.
	pub const E:                   Self = Self(0x08);
	/// Keyboard 'f' and 'F'.
	pub const F:                   Self = Self(0x09);
	/// Keyboard 'g' and 'G'.
	pub const G:                   Self = Self(0x0A);
	/// Keyboard 'h' and 'H'.
	pub const H:                   Self = Self(0x0B);
	/// Keyboard 'i' and 'I'.
	pub const I:                   Self = Self(0x0C);
	/// Keyboard 'j' and 'J'.
	pub const J:                   Self = Self(0x0D);
	/// Keyboard 'k' and 'K'.
	pub const K:                   Self = Self(0x0E);
	/// Keyboard 'l' and 'L'.
	pub const L:                   Self = Self(0x0F);
	/// Keyboard 'm' and 'M'.
	pub const M:                   Self = Self(0x10);
	/// Keyboard 'n' and 'N'.
	pub const N:                   Self = Self(0x11);
	/// Keyboard 'o' and 'O'.
	pub const O:                   Self = Self(0x12);
	/// Keyboard 'p' and 'P'.
	pub const P:                   Self = Self(0x13);
	/// Keyboard 'q' and 'Q'.
	pub const Q:                   Self = Self(0x14);
	/// Keyboard 'r' and 'R'.
	pub const R:                   Self = Self(0x15);
	/// Keyboard 's' and 'S'.
	pub const S:                   Self = Self(0x16);
	/// Keyboard 't' and 'T'.
	pub const T:                   Self = Self(0x17);
	/// Keyboard 'u' and 'U'.
	pub const U:                   Self = Self(0x18);
	/// Keyboard 'v' and 'V'.
	pub const V:                   Self = Self(0x19);
	/// Keyboard 'w' and 'W'.
	pub const W:                   Self = Self(0x1A);
	/// Keyboard 'x' and 'X'.
	pub const X:                   Self = Self(0x1B);
	/// Keyboard 'y' and 'Y'.
	pub const Y:                   Self = Self(0x1C);
	/// Keyboard 'z' and 'Z'.
	pub const Z:                   Self = Self(0x1D);
	/// Keyboard '1' and '!'.
	pub const NUM_1:               Self = Self(0x1E);
	/// Keyboard '2' and '@'.
	pub const NUM_2:               Self = Self(0x1F);
	/// Keyboard '3' and '#'.
	pub const NUM_3:               Self = Self(0x20);
	/// Keyboard '4' and '$'.
	pub const NUM_4:               Self = Self(0x21);
	/// Keyboard '5' and '%'.
	pub const NUM_5:               Self = Self(0x22);
	/// Keyboard '6' and '∧'.
	pub const NUM_6:               Self = Self(0x23);
	/// Keyboard '7' and '&'.
	pub const NUM_7:               Self = Self(0x24);
	/// Keyboard '8' and '*'.
	pub const NUM_8:               Self = Self(0x25);
	/// Keyboard '9' and '('.
	pub const NUM_9:               Self = Self(0x26);
	/// Keyboard '0' and ')'.
	pub const NUM_0:               Self = Self(0x27);
	/// Keyboard 'return'/'enter'.
	pub const RETURN:              Self = Self(0x28);
	/// Keyboard 'escape'.
	pub const ESCAPE:              Self = Self(0x29);
	/// Keyboard 'delete'/'backspace'.
	pub const DELETE:              Self = Self(0x2A);
	/// Keyboard 'tab'.
	pub const TAB:                 Self = Self(0x2B);
	/// Keyboard 'spacebar'.
	pub const SPACE:               Self = Self(0x2C);
	/// Keyboard '-' and '_'.
	pub const MINUS:               Self = Self(0x2D);
	/// Keyboard '=' and '+'.
	pub const EQUALS:              Self = Self(0x2E);
	/// Keyboard '[' and '{'.
	pub const L_BRACE:             Self = Self(0x2F);
	/// Keyboard ']' and '}'.
	pub const R_BRACE:             Self = Self(0x30);
	/// Keyboard '\' and '|'.
	pub const BACKSLASH:           Self = Self(0x31);
	/// Keyboard 'Non-US #' and '˜'.
	pub const NON_US_HASH:         Self = Self(0x32);
	/// Keyboard ';' and ':'.
	pub const SEMICOLON:           Self = Self(0x33);
	/// Keyboard ''' (apostrophe) and '"'.
	pub const APOSTROPHE:          Self = Self(0x34);
	/// Keyboard '`' and '~'.
	pub const GRAVE:               Self = Self(0x35);
	/// Keyboard ',' and '<'.
	pub const COMMA:               Self = Self(0x36);
	/// Keyboard '.' and '>'.
	pub const PERIOD:              Self = Self(0x37);
	/// Keyboard '/' and '?'.
	pub const SLASH:               Self = Self(0x38);
	/// Keyboard 'caps lock'.
	pub const CAPS_LOCK:           Self = Self(0x39);
	/// Keyboard 'F1'.
	pub const F1:                  Self = Self(0x3A);
	/// Keyboard 'F2'.
	pub const F2:                  Self = Self(0x3B);
	/// Keyboard 'F3'.
	pub const F3:                  Self = Self(0x3C);
	/// Keyboard 'F4'.
	pub const F4:                  Self = Self(0x3D);
	/// Keyboard 'F5'.
	pub const F5:                  Self = Self(0x3E);
	/// Keyboard 'F6'.
	pub const F6:                  Self = Self(0x3F);
	/// Keyboard 'F7'.
	pub const F7:                  Self = Self(0x40);
	/// Keyboard 'F8'.
	pub const F8:                  Self = Self(0x41);
	/// Keyboard 'F9'.
	pub const F9:                  Self = Self(0x42);
	/// Keyboard 'F10'.
	pub const F10:                 Self = Self(0x43);
	/// Keyboard 'F11'.
	pub const F11:                 Self = Self(0x44);
	/// Keyboard 'F12'.
	pub const F12:                 Self = Self(0x45);
	/// Keyboard 'print screen'.
	pub const PRINT_SCREEN:        Self = Self(0x46);
	/// Keyboard 'scroll lock'.
	pub const SCROLL_LOCK:         Self = Self(0x47);
	/// Keyboard 'pause'.
	pub const PAUSE:               Self = Self(0x48);
	/// Keyboard 'insert'.
	pub const INSERT:              Self = Self(0x49);
	/// Keyboard 'home'.
	pub const HOME:                Self = Self(0x4A);
	/// Keyboard 'page up'.
	pub const PAGE_UP:             Self = Self(0x4B);
	/// Keyboard 'delete forward'.
	pub const DELETE_FORWARD:      Self = Self(0x4C);
	/// Keyboard 'end'.
	pub const END:                 Self = Self(0x4D);
	/// Keyboard 'page down'.
	pub const PAGE_DOWN:           Self = Self(0x4E);
	/// Keyboard 'right arrow'.
	pub const RIGHT:               Self = Self(0x4F);
	/// Keyboard 'left arrow'.
	pub const LEFT:                Self = Self(0x50);
	/// Keyboard 'down arrow'.
	pub const DOWN:                Self = Self(0x51);
	/// Keyboard 'up arrow'.
	pub const UP:                  Self = Self(0x52);
	/// Keypad 'num lock' and 'clear'.
	pub const KP_NUM_LOCK:         Self = Self(0x53);
	/// Keypad '/'.
	pub const KP_DIV:              Self = Self(0x54);
	/// Keypad '*'.
	pub const KP_MUL:              Self = Self(0x55);
	/// Keypad '-'.
	pub const KP_SUB:              Self = Self(0x56);
	/// Keypad '+'.
	pub const KP_ADD:              Self = Self(0x57);
	/// Keypad 'enter'.
	pub const KP_ENTER:            Self = Self(0x58);
	/// Keypad '1' and 'end'.
	pub const KP_1:                Self = Self(0x59);
	/// Keypad '2' and 'down arrow'.
	pub const KP_2:                Self = Self(0x5A);
	/// Keypad '3' and 'page down'.
	pub const KP_3:                Self = Self(0x5B);
	/// Keypad '4' and 'left arrow'.
	pub const KP_4:                Self = Self(0x5C);
	/// Keypad '5'.
	pub const KP_5:                Self = Self(0x5D);
	/// Keypad '6' and 'right arrow'.
	pub const KP_6:                Self = Self(0x5E);
	/// Keypad '7' and 'home'.
	pub const KP_7:                Self = Self(0x5F);
	/// Keypad '8' and 'up arrow'.
	pub const KP_8:                Self = Self(0x60);
	/// Keypad '9' and 'page up'.
	pub const KP_9:                Self = Self(0x61);
	/// Keypad '0' and 'insert'.
	pub const KP_0:                Self = Self(0x62);
	/// Keypad '.' and 'delete'.
	pub const KP_PERIOD:           Self = Self(0x63);
	/// Keyboard 'non-US \' and '|'.
	pub const NON_US_BACKSLASH:    Self = Self(0x64);
	/// Keyboard 'application'.
	pub const APPLICATION:         Self = Self(0x65);
	/// Keyboard 'power'.
	pub const POWER:               Self = Self(0x66);
	/// Keypad '='.
	pub const KP_EQUALS:           Self = Self(0x67);
	/// Keyboard 'F13'.
	pub const F13:                 Self = Self(0x68);
	/// Keyboard 'F14'.
	pub const F14:                 Self = Self(0x69);
	/// Keyboard 'F15'.
	pub const F15:                 Self = Self(0x6A);
	/// Keyboard 'F16'.
	pub const F16:                 Self = Self(0x6B);
	/// Keyboard 'F17'.
	pub const F17:                 Self = Self(0x6C);
	/// Keyboard 'F18'.
	pub const F18:                 Self = Self(0x6D);
	/// Keyboard 'F19'.
	pub const F19:                 Self = Self(0x6E);
	/// Keyboard 'F20'.
	pub const F20:                 Self = Self(0x6F);
	/// Keyboard 'F21'.
	pub const F21:                 Self = Self(0x70);
	/// Keyboard 'F22'.
	pub const F22:                 Self = Self(0x71);
	/// Keyboard 'F23'.
	pub const F23:                 Self = Self(0x72);
	/// Keyboard 'F24'.
	pub const F24:                 Self = Self(0x73);
	/// Keyboard 'execute'.
	pub const EXECUTE:             Self = Self(0x74);
	/// Keyboard 'help'.
	pub const HELP:                Self = Self(0x75);
	/// Keyboard 'menu'.
	pub const MENU:                Self = Self(0x76);
	/// Keyboard 'select'.
	pub const SELECT:              Self = Self(0x77);
	/// Keyboard 'stop'.
	pub const STOP:                Self = Self(0x78);
	/// Keyboard 'again'.
	pub const AGAIN:               Self = Self(0x79);
	/// Keyboard 'undo'.
	pub const UNDO:                Self = Self(0x7A);
	/// Keyboard 'cut'.
	pub const CUT:                 Self = Self(0x7B);
	/// Keyboard 'copy'.
	pub const COPY:                Self = Self(0x7C);
	/// Keyboard 'paste'.
	pub const PASTE:               Self = Self(0x7D);
	/// Keyboard 'find'.
	pub const FIND:                Self = Self(0x7E);
	/// Keyboard 'mute'.
	pub const MUTE:                Self = Self(0x7F);
	/// Keyboard 'volume up'.
	pub const VOLUME_UP:           Self = Self(0x80);
	/// Keyboard 'volume down'.
	pub const VOLUME_DOWN:         Self = Self(0x81);
	/// Keyboard 'locking caps lock'.
	pub const LOCKING_CAPS_LOCK:   Self = Self(0x82);
	/// Keyboard 'locking num lock'.
	pub const LOCKING_NUM_LOCK:    Self = Self(0x83);
	/// Keyboard 'locking scroll lock'.
	pub const LOCKING_SCROLL_LOCK: Self = Self(0x84);
	/// Keypad 'comma'.
	pub const KP_COMMA:            Self = Self(0x85);
	/// Keypad 'equal sign'.
	pub const KP_EQUAL_SIGN:       Self = Self(0x86);
	/// Keyboard 'international 1'.
	pub const INTERNATIONAL_1:     Self = Self(0x87);
	/// Keyboard 'international 2'.
	pub const INTERNATIONAL_2:     Self = Self(0x88);
	/// Keyboard 'international 3'.
	pub const INTERNATIONAL_3:     Self = Self(0x89);
	/// Keyboard 'international 4'.
	pub const INTERNATIONAL_4:     Self = Self(0x8A);
	/// Keyboard 'international 5'.
	pub const INTERNATIONAL_5:     Self = Self(0x8B);
	/// Keyboard 'international 6'.
	pub const INTERNATIONAL_6:     Self = Self(0x8C);
	/// Keyboard 'international 7'.
	pub const INTERNATIONAL_7:     Self = Self(0x8D);
	/// Keyboard 'international 8'.
	pub const INTERNATIONAL_8:     Self = Self(0x8E);
	/// Keyboard 'international 9'.
	pub const INTERNATIONAL_9:     Self = Self(0x8F);
	/// Keyboard 'lang 1'.
	pub const LANG_1:              Self = Self(0x90);
	/// Keyboard 'lang 2'.
	pub const LANG_2:              Self = Self(0x91);
	/// Keyboard 'lang 3'.
	pub const LANG_3:              Self = Self(0x92);
	/// Keyboard 'lang 4'.
	pub const LANG_4:              Self = Self(0x93);
	/// Keyboard 'lang 5'.
	pub const LANG_5:              Self = Self(0x94);
	/// Keyboard 'lang 6'.
	pub const LANG_6:              Self = Self(0x95);
	/// Keyboard 'lang 7'.
	pub const LANG_7:              Self = Self(0x96);
	/// Keyboard 'lang 8'.
	pub const LANG_8:              Self = Self(0x97);
	/// Keyboard 'lang 9'.
	pub const LANG_9:              Self = Self(0x98);
	/// Keyboard 'alternate erase'.
	pub const ALT_ERASE:           Self = Self(0x99);
	/// Keyboard 'sys req'/'attention'.
	pub const SYS_REQ:             Self = Self(0x9A);
	/// Keyboard 'cancel'.
	pub const CANCEL:              Self = Self(0x9B);
	/// Keyboard 'clear'.
	pub const CLEAR:               Self = Self(0x9C);
	/// Keyboard 'prior'.
	pub const PRIOR:               Self = Self(0x9D);
	/// Keyboard 'return'.
	pub const RETURN_2:            Self = Self(0x9E);
	/// Keyboard 'separator'.
	pub const SEPARATOR:           Self = Self(0x9F);
	/// Keyboard 'out'.
	pub const OUT:                 Self = Self(0xA0);
	/// Keyboard 'oper'.
	pub const OPER:                Self = Self(0xA1);
	/// Keyboard 'clear'/'again'.
	pub const CLEAR_2:             Self = Self(0xA2);
	/// Keyboard 'cr sel'/'props'.
	pub const CR_SEL:              Self = Self(0xA3);
	/// Keyboard 'ex sel'.
	pub const EX_SEL:              Self = Self(0xA4);
	/// Keypad '00'.
	pub const KP_00:               Self = Self(0xB0);
	/// Keypad '000'.
	pub const KP_000:              Self = Self(0xB1);
	/// 'Thousands separator'.
	pub const THOUSANDS_SEPARATOR: Self = Self(0xB2);
	/// 'Decimal separator'.
	pub const DECIMAL_SEPARATOR:   Self = Self(0xB3);
	/// 'Currency unit'.
	pub const CURRENCY_UNIT:       Self = Self(0xB4);
	/// 'Currency sub-unit'.
	pub const CURRENCY_SUBUNIT:    Self = Self(0xB5);
	/// Keypad '('.
	pub const KP_L_PAREN:          Self = Self(0xB6);
	/// Keypad ')'.
	pub const KP_R_PAREN:          Self = Self(0xB7);
	/// Keypad '{'.
	pub const KP_L_BRACE:          Self = Self(0xB8);
	/// Keypad '}'.
	pub const KP_R_BRACE:          Self = Self(0xB9);
	/// Keypad 'tab'.
	pub const KP_TAB:              Self = Self(0xBA);
	/// Keypad 'backspace'.
	pub const KP_BACKSPACE:        Self = Self(0xBB);
	/// Keypad 'A'.
	pub const KP_A:                Self = Self(0xBC);
	/// Keypad 'B'.
	pub const KP_B:                Self = Self(0xBD);
	/// Keypad 'C'.
	pub const KP_C:                Self = Self(0xBE);
	/// Keypad 'D'.
	pub const KP_D:                Self = Self(0xBF);
	/// Keypad 'E'.
	pub const KP_E:                Self = Self(0xC0);
	/// Keypad 'F'.
	pub const KP_F:                Self = Self(0xC1);
	/// Keypad 'XOR'.
	pub const KP_XOR:              Self = Self(0xC2);
	/// Keypad '∧'.
	pub const KP_CARET:            Self = Self(0xC3);
	/// Keypad '%'.
	pub const KP_PERCENT:          Self = Self(0xC4);
	/// Keypad '<'.
	pub const KP_LESS:             Self = Self(0xC5);
	/// Keypad '>'.
	pub const KP_GREATER:          Self = Self(0xC6);
	/// Keypad '&'.
	pub const KP_AMPERSAND:        Self = Self(0xC7);
	/// Keypad '&&'.
	pub const KP_DOUBLE_AMPERSAND: Self = Self(0xC8);
	/// Keypad '|'.
	pub const KP_PIPE:             Self = Self(0xC9);
	/// Keypad '||'.
	pub const KP_DOUBLE_PIPE:      Self = Self(0xCA);
	/// Keypad ':'.
	pub const KP_COLON:            Self = Self(0xCB);
	/// Keypad '#'.
	pub const KP_HASH:             Self = Self(0xCC);
	/// Keypad 'space'.
	pub const KP_SPACE:            Self = Self(0xCD);
	/// Keypad '@'.
	pub const KP_AT:               Self = Self(0xCE);
	/// Keypad '!'.
	pub const KP_EXCLAMATION:      Self = Self(0xCF);
	/// Keypad 'memory store'.
	pub const KP_MEM_STORE:        Self = Self(0xD0);
	/// Keypad 'memory recall'.
	pub const KP_MEM_RECALL:       Self = Self(0xD1);
	/// Keypad 'memory clear'.
	pub const KP_MEM_CLEAR:        Self = Self(0xD2);
	/// Keypad 'memory add'.
	pub const KP_MEM_ADD:          Self = Self(0xD3);
	/// Keypad 'memory subtract'.
	pub const KP_MEM_SUB:          Self = Self(0xD4);
	/// Keypad 'memory multiply'.
	pub const KP_MEM_MUL:          Self = Self(0xD5);
	/// Keypad 'memory divide'.
	pub const KP_MEM_DIV:          Self = Self(0xD6);
	/// Keypad '+/-'.
	pub const KP_PLUS_MINUS:       Self = Self(0xD7);
	/// Keypad 'clear'.
	pub const KP_CLEAR:            Self = Self(0xD8);
	/// Keypad 'clear entry'.
	pub const KP_CLEAR_ENTRY:      Self = Self(0xD9);
	/// Keypad 'binary'.
	pub const KP_BINARY:           Self = Self(0xDA);
	/// Keypad 'octal'.
	pub const KP_OCTAL:            Self = Self(0xDB);
	/// Keypad 'decimal'.
	pub const KP_DECIMAL:          Self = Self(0xDC);
	/// Keypad 'hexadecimal'.
	pub const KP_HEXADECIMAL:      Self = Self(0xDD);
	/// Keyboard 'left control'.
	pub const L_CTRL:              Self = Self(0xE0);
	/// Keyboard 'left shift'.
	pub const L_SHIFT:             Self = Self(0xE1);
	/// Keyboard 'left alt'.
	///
	/// The left Alt key on Windows, or Option key on Mac.
	pub const L_ALT:               Self = Self(0xE2);
	/// Keyboard 'left GUI'.
	///
	/// The left Windows key on Windows, Command key on Mac, or other meta key.
	pub const L_GUI:               Self = Self(0xE3);
	/// Keyboard 'right control'.
	pub const R_CTRL:              Self = Self(0xE4);
	/// Keyboard 'right shift'.
	pub const R_SHIFT:             Self = Self(0xE5);
	/// Keyboard 'right alt'.
	///
	/// The right Alt key on Windows, or Option key on Mac.
	pub const R_ALT:               Self = Self(0xE6);
	/// Keyboard 'right GUI'.
	///
	/// The right Windows key on Windows, Command key on Mac, or other meta key.
	pub const R_GUI:               Self = Self(0xE7);

}

impl PartialOrd for Keycode {

	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		match (self, other) {
			// Unknown keycodes can't be ordered
			(&Keycode::UNKNOWN, _) | (_, &Keycode::UNKNOWN) => None,
			// Otherwise, falls back to `u8` ordering
			_ => Some(self.0.cmp(&other.0)),
		}
	}

}

impl From<SDL_Scancode> for Keycode {

	fn from(scancode: SDL_Scancode) -> Self {
		Self(scancode.0 as u8)
	}

}

impl Into<SDL_Scancode> for Keycode {

	fn into(self) -> SDL_Scancode {
		SDL_Scancode(self.0 as c_int)
	}

}