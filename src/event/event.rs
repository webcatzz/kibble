//! System events.

use std::ffi::CStr;

use sdl3_sys::events::*;

use crate::input::{ModKeys, Keycode, KeyLabel, MouseButton, MouseButtons};
use crate::math::Vec2;

/// A system event.
///
/// See the [`event`] documentation for usage examples.
///
/// # Notable events
///
/// - Keyboard events are represented by the [`Key`] variant.
/// - Mouse events are represented by the [`MouseButton`] and [`MouseMotion`]
///   variants.
///
/// [`event`]: super
/// [`Key`]: Self::Key
/// [`MouseButton`]: Self::MouseButton
/// [`MouseMotion`]: Self::MouseMotion
#[derive(Clone)]
pub enum Event {
	/// The program is requested to quit.
	Quit,
	/// A key is pressed or released.
	#[cfg(feature = "keyboard")]
	Key {
		/// The label of the key.
		label:     KeyLabel,
		/// The physical location of the key.
		code:      Keycode,
		/// If `true`, the key is pressed, otherwise, it is released.
		down:      bool,
		/// A set of modifier keys that are pressed concurrently.
		modifiers: ModKeys,
	},
	/// Text is input.
	Text {
		text: String,
	},
	/// A mouse button is pressed or released.
	#[cfg(feature = "mouse")]
	MouseButton {
		/// The button that is pressed or released.
		button: MouseButton,
		/// If `true`, the button is pressed, otherwise, it is released.
		down:   bool,
		/// The position of the cursor, relative to the window.
		pos:    Vec2<f32>,
	},
	/// A mouse is moved.
	#[cfg(feature = "mouse")]
	MouseMotion {
		/// The position of the cursor, relative to the window.
		pos:     Vec2<f32>,
		/// The motion of the cursor, relative to its last position.
		motion:  Vec2<f32>,
		/// A set of mouse buttons that are pressed concurrently.
		buttons: MouseButtons,
	},
	/// A window is resized.
	#[cfg(feature = "window")]
	WindowResize {
		/// The new size of the window.
		size: Vec2<u32>,
	}
}

impl Event {

	/// Returns `Some(true)` if the event is in an active state.
	pub fn is_down(&self) -> Option<bool> {
		match self {
			#[cfg(feature = "keyboard")]
			Self::Key         { down, .. } => Some(*down),
			#[cfg(feature = "mouse")]
			Self::MouseButton { down, .. } => Some(*down),
			_                                     => None,
		}
	}

	/// Returns the position of the event if it is a mouse event and records the
	/// given mouse button as down.
	#[cfg(feature = "mouse")]
	pub fn mouse_down_pos(&self, button: MouseButton) -> Option<Vec2<f32>> {
		match self {
			Self::MouseButton { button: b, down: true, pos } if *b == button =>
				Some(*pos),
			Self::MouseMotion { pos, buttons, .. } if buttons.is_down(button) =>
				Some(*pos),
			_ => None,
		}
	}

	/// Maps the event's mouse position, if any, with the given function.
	#[cfg(feature = "mouse")]
	pub fn map_mouse_pos(&mut self, mut f: impl FnMut(Vec2<f32>) -> Vec2<f32>) {
		match self {
			Self::MouseButton { pos, .. } |
			Self::MouseMotion { pos, .. } =>
				*pos = f(*pos),
			_ => {}
		}
	}

}

impl TryFrom<SDL_Event> for Event {

	type Error = String;

	fn try_from(event: SDL_Event) -> Result<Self, Self::Error> {
		unsafe {
			match SDL_EventType(event.r#type) {
				SDL_EVENT_QUIT => Ok(Self::Quit),
				#[cfg(feature = "keyboard")]
				SDL_EVENT_KEY_DOWN | SDL_EVENT_KEY_UP => Ok(Self::Key {
					label:     KeyLabel::from(event.key.key),
					code:      Keycode::from(event.key.scancode),
					down:      event.key.down,
					modifiers: ModKeys::from(event.key.r#mod),
				}),
				SDL_EVENT_TEXT_INPUT => Ok(Self::Text {
					text: CStr::from_ptr(event.text.text).to_str().unwrap().to_string(),
				}),
				#[cfg(feature = "mouse")]
				SDL_EVENT_MOUSE_BUTTON_DOWN | SDL_EVENT_MOUSE_BUTTON_UP => Ok(Self::MouseButton {
					button: MouseButton::from_sdl_index(event.button.button),
					down:   event.button.down,
					pos:    Vec2 { x: event.button.x as f32, y: event.button.y as f32 },
				}),
				#[cfg(feature = "mouse")]
				SDL_EVENT_MOUSE_MOTION => Ok(Self::MouseMotion {
					pos:     Vec2 { x: event.motion.x as f32, y: event.motion.y as f32 },
					motion:  Vec2 { x: event.motion.xrel as f32, y: event.motion.yrel as f32 },
					buttons: event.motion.state.into(),
				}),
				#[cfg(feature = "window")]
				SDL_EVENT_WINDOW_RESIZED => Ok(Self::WindowResize {
					size: Vec2 { x: event.window.data1 as u32, y: event.window.data2 as u32 },
				}),
				_ => Err(format!("No `Event` representation for SDL event of type: {}", event.r#type)),
			}
		}
	}

}