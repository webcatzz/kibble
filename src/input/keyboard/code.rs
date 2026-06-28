use std::cmp::Ordering;
use std::ffi::c_int;
use std::fmt::{self, Write};

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
	pub const L_SUPER:             Self = Self(0xE3);
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
	pub const R_SUPER:             Self = Self(0xE7);

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

impl fmt::Debug for Keycode {

	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match *self {
			Self::A                   => f.write_char('A'),
			Self::B                   => f.write_char('B'),
			Self::C                   => f.write_char('C'),
			Self::D                   => f.write_char('D'),
			Self::E                   => f.write_char('E'),
			Self::F                   => f.write_char('F'),
			Self::G                   => f.write_char('G'),
			Self::H                   => f.write_char('H'),
			Self::I                   => f.write_char('I'),
			Self::J                   => f.write_char('J'),
			Self::K                   => f.write_char('K'),
			Self::L                   => f.write_char('L'),
			Self::M                   => f.write_char('M'),
			Self::N                   => f.write_char('N'),
			Self::O                   => f.write_char('O'),
			Self::P                   => f.write_char('P'),
			Self::Q                   => f.write_char('Q'),
			Self::R                   => f.write_char('R'),
			Self::S                   => f.write_char('S'),
			Self::T                   => f.write_char('T'),
			Self::U                   => f.write_char('U'),
			Self::V                   => f.write_char('V'),
			Self::W                   => f.write_char('W'),
			Self::X                   => f.write_char('X'),
			Self::Y                   => f.write_char('Y'),
			Self::Z                   => f.write_char('Z'),
			Self::NUM_1               => f.write_char('1'),
			Self::NUM_2               => f.write_char('2'),
			Self::NUM_3               => f.write_char('3'),
			Self::NUM_4               => f.write_char('4'),
			Self::NUM_5               => f.write_char('5'),
			Self::NUM_6               => f.write_char('6'),
			Self::NUM_7               => f.write_char('7'),
			Self::NUM_8               => f.write_char('8'),
			Self::NUM_9               => f.write_char('9'),
			Self::NUM_0               => f.write_char('0'),
			Self::RETURN              => f.write_str("Return"),
			Self::ESCAPE              => f.write_str("Escape"),
			Self::DELETE              => f.write_str("Delete"),
			Self::TAB                 => f.write_str("Tab"),
			Self::SPACE               => f.write_str("Space"),
			Self::MINUS               => f.write_char('-'),
			Self::EQUALS              => f.write_char('='),
			Self::L_BRACE             => f.write_char('['),
			Self::R_BRACE             => f.write_char(']'),
			Self::BACKSLASH           => f.write_char('\\'),
			Self::NON_US_HASH         => f.write_str("Non-US hash"),
			Self::SEMICOLON           => f.write_char(';'),
			Self::APOSTROPHE          => f.write_char('\''),
			Self::GRAVE               => f.write_char('`'),
			Self::COMMA               => f.write_char(','),
			Self::PERIOD              => f.write_char('.'),
			Self::SLASH               => f.write_char('/'),
			Self::CAPS_LOCK           => f.write_str("Caps lock"),
			Self::F1                  => f.write_str("F1"),
			Self::F2                  => f.write_str("F2"),
			Self::F3                  => f.write_str("F3"),
			Self::F4                  => f.write_str("F4"),
			Self::F5                  => f.write_str("F5"),
			Self::F6                  => f.write_str("F6"),
			Self::F7                  => f.write_str("F7"),
			Self::F8                  => f.write_str("F8"),
			Self::F9                  => f.write_str("F9"),
			Self::F10                 => f.write_str("F10"),
			Self::F11                 => f.write_str("F11"),
			Self::F12                 => f.write_str("F12"),
			Self::PRINT_SCREEN        => f.write_str("Print screen"),
			Self::SCROLL_LOCK         => f.write_str("Scroll lock"),
			Self::PAUSE               => f.write_str("Pause"),
			Self::INSERT              => f.write_str("Insert"),
			Self::HOME                => f.write_str("Home"),
			Self::PAGE_UP             => f.write_str("Page up"),
			Self::DELETE_FORWARD      => f.write_str("Delete forward"),
			Self::END                 => f.write_str("End"),
			Self::PAGE_DOWN           => f.write_str("Page down"),
			Self::RIGHT               => f.write_str("Right"),
			Self::LEFT                => f.write_str("Left"),
			Self::DOWN                => f.write_str("Down"),
			Self::UP                  => f.write_str("Up"),
			Self::KP_NUM_LOCK         => f.write_str("Keypad num lock"),
			Self::KP_DIV              => f.write_str("Keypad divide"),
			Self::KP_MUL              => f.write_str("Keypad multiply"),
			Self::KP_SUB              => f.write_str("Keypad subtract"),
			Self::KP_ADD              => f.write_str("Keypad add"),
			Self::KP_ENTER            => f.write_str("Keypad enter"),
			Self::KP_1                => f.write_str("Keypad 1"),
			Self::KP_2                => f.write_str("Keypad 2"),
			Self::KP_3                => f.write_str("Keypad 3"),
			Self::KP_4                => f.write_str("Keypad 4"),
			Self::KP_5                => f.write_str("Keypad 5"),
			Self::KP_6                => f.write_str("Keypad 6"),
			Self::KP_7                => f.write_str("Keypad 7"),
			Self::KP_8                => f.write_str("Keypad 8"),
			Self::KP_9                => f.write_str("Keypad 9"),
			Self::KP_0                => f.write_str("Keypad 0"),
			Self::KP_PERIOD           => f.write_str("Keypad period"),
			Self::NON_US_BACKSLASH    => f.write_str("Non-US backslash"),
			Self::APPLICATION         => f.write_str("Application"),
			Self::POWER               => f.write_str("Power"),
			Self::KP_EQUALS           => f.write_str("Keypad equals"),
			Self::F13                 => f.write_str("F13"),
			Self::F14                 => f.write_str("F14"),
			Self::F15                 => f.write_str("F15"),
			Self::F16                 => f.write_str("F16"),
			Self::F17                 => f.write_str("F17"),
			Self::F18                 => f.write_str("F18"),
			Self::F19                 => f.write_str("F19"),
			Self::F20                 => f.write_str("F20"),
			Self::F21                 => f.write_str("F21"),
			Self::F22                 => f.write_str("F22"),
			Self::F23                 => f.write_str("F23"),
			Self::F24                 => f.write_str("F24"),
			Self::EXECUTE             => f.write_str("Execute"),
			Self::HELP                => f.write_str("Help"),
			Self::MENU                => f.write_str("Menu"),
			Self::SELECT              => f.write_str("Select"),
			Self::STOP                => f.write_str("Stop"),
			Self::AGAIN               => f.write_str("Again"),
			Self::UNDO                => f.write_str("Undo"),
			Self::CUT                 => f.write_str("Cut"),
			Self::COPY                => f.write_str("Copy"),
			Self::PASTE               => f.write_str("Paste"),
			Self::FIND                => f.write_str("Find"),
			Self::MUTE                => f.write_str("Mute"),
			Self::VOLUME_UP           => f.write_str("Volume up"),
			Self::VOLUME_DOWN         => f.write_str("Volume down"),
			Self::LOCKING_CAPS_LOCK   => f.write_str("Locking caps lock"),
			Self::LOCKING_NUM_LOCK    => f.write_str("Locking num lock"),
			Self::LOCKING_SCROLL_LOCK => f.write_str("Locking scroll lock"),
			Self::KP_COMMA            => f.write_str("Keypad comma"),
			Self::KP_EQUAL_SIGN       => f.write_str("Keypad equal sign"),
			Self::INTERNATIONAL_1     => f.write_str("International 1"),
			Self::INTERNATIONAL_2     => f.write_str("International 2"),
			Self::INTERNATIONAL_3     => f.write_str("International 3"),
			Self::INTERNATIONAL_4     => f.write_str("International 4"),
			Self::INTERNATIONAL_5     => f.write_str("International 5"),
			Self::INTERNATIONAL_6     => f.write_str("International 6"),
			Self::INTERNATIONAL_7     => f.write_str("International 7"),
			Self::INTERNATIONAL_8     => f.write_str("International 8"),
			Self::INTERNATIONAL_9     => f.write_str("International 9"),
			Self::LANG_1              => f.write_str("Lang 1"),
			Self::LANG_2              => f.write_str("Lang 2"),
			Self::LANG_3              => f.write_str("Lang 3"),
			Self::LANG_4              => f.write_str("Lang 4"),
			Self::LANG_5              => f.write_str("Lang 5"),
			Self::LANG_6              => f.write_str("Lang 6"),
			Self::LANG_7              => f.write_str("Lang 7"),
			Self::LANG_8              => f.write_str("Lang 8"),
			Self::LANG_9              => f.write_str("Lang 9"),
			Self::ALT_ERASE           => f.write_str("Alt erase"),
			Self::SYS_REQ             => f.write_str("Sys req"),
			Self::CANCEL              => f.write_str("Cancel"),
			Self::CLEAR               => f.write_str("Clear"),
			Self::PRIOR               => f.write_str("Prior"),
			Self::RETURN_2            => f.write_str("Return"),
			Self::SEPARATOR           => f.write_str("Separator"),
			Self::OUT                 => f.write_str("Out"),
			Self::OPER                => f.write_str("Oper"),
			Self::CLEAR_2             => f.write_str("Clear"),
			Self::CR_SEL              => f.write_str("CrSel"),
			Self::EX_SEL              => f.write_str("ExSel"),
			Self::KP_00               => f.write_str("Keypad 00"),
			Self::KP_000              => f.write_str("Keypad 000"),
			Self::THOUSANDS_SEPARATOR => f.write_str("Thousands separator"),
			Self::DECIMAL_SEPARATOR   => f.write_str("Decimal separator"),
			Self::CURRENCY_UNIT       => f.write_str("Currency unit"),
			Self::CURRENCY_SUBUNIT    => f.write_str("Currency sub-unit"),
			Self::KP_L_PAREN          => f.write_str("Keypad ("),
			Self::KP_R_PAREN          => f.write_str("Keypad )"),
			Self::KP_L_BRACE          => f.write_str("Keypad ["),
			Self::KP_R_BRACE          => f.write_str("Keypad ]"),
			Self::KP_TAB              => f.write_str("Keypad tab"),
			Self::KP_BACKSPACE        => f.write_str("Keypad backspace"),
			Self::KP_A                => f.write_str("Keypad A"),
			Self::KP_B                => f.write_str("Keypad B"),
			Self::KP_C                => f.write_str("Keypad C"),
			Self::KP_D                => f.write_str("Keypad D"),
			Self::KP_E                => f.write_str("Keypad E"),
			Self::KP_F                => f.write_str("Keypad F"),
			Self::KP_XOR              => f.write_str("Keypad XOR"),
			Self::KP_CARET            => f.write_str("Keypad ^"),
			Self::KP_PERCENT          => f.write_str("Keypad %"),
			Self::KP_LESS             => f.write_str("Keypad <"),
			Self::KP_GREATER          => f.write_str("Keypad >"),
			Self::KP_AMPERSAND        => f.write_str("Keypad &"),
			Self::KP_DOUBLE_AMPERSAND => f.write_str("Keypad &&"),
			Self::KP_PIPE             => f.write_str("Keypad |"),
			Self::KP_DOUBLE_PIPE      => f.write_str("Keypad ||"),
			Self::KP_COLON            => f.write_str("Keypad :"),
			Self::KP_HASH             => f.write_str("Keypad #"),
			Self::KP_SPACE            => f.write_str("Keypad space"),
			Self::KP_AT               => f.write_str("Keypad @"),
			Self::KP_EXCLAMATION      => f.write_str("Keypad !"),
			Self::KP_MEM_STORE        => f.write_str("Keypad memory store"),
			Self::KP_MEM_RECALL       => f.write_str("Keypad memory recall"),
			Self::KP_MEM_CLEAR        => f.write_str("Keypad memory clear"),
			Self::KP_MEM_ADD          => f.write_str("Keypad memory add"),
			Self::KP_MEM_SUB          => f.write_str("Keypad memory subtract"),
			Self::KP_MEM_MUL          => f.write_str("Keypad memory multiply"),
			Self::KP_MEM_DIV          => f.write_str("Keypad memory divide"),
			Self::KP_PLUS_MINUS       => f.write_str("Keypad plus-minus"),
			Self::KP_CLEAR            => f.write_str("Keypad clear"),
			Self::KP_CLEAR_ENTRY      => f.write_str("Keypad clear entry"),
			Self::KP_BINARY           => f.write_str("Keypad binary"),
			Self::KP_OCTAL            => f.write_str("Keypad octal"),
			Self::KP_DECIMAL          => f.write_str("Keypad decimal"),
			Self::KP_HEXADECIMAL      => f.write_str("Keypad hexadecimal"),
			Self::L_CTRL              => f.write_str("Left control"),
			Self::L_SHIFT             => f.write_str("Left shift"),
			Self::L_ALT               => f.write_str("Left alt"),
			Self::L_SUPER             => f.write_str("Left super"),
			Self::R_CTRL              => f.write_str("Right control"),
			Self::R_SHIFT             => f.write_str("Right shift"),
			Self::R_ALT               => f.write_str("Right alt"),
			Self::R_SUPER             => f.write_str("Right super"),
			_ => f                   .write_str("Unknown"),
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