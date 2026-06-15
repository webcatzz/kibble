//! Events.
//!
//! Events are messages from the system about user input. For example, an event
//! is generated when a key is pressed, a mouse is moved, or the user closes the
//! program.
//!
//! See the [`Event`] enum for a list of events.
//!
//! # Examples
//!
//! To handle incoming events, open the [`EventQueue`]:
//!
//! ```no_run
//! # use kibble::event::{Event, EventQueue};
//! # use kibble::keyboard::Keycode;
//! // Opens the system event queue
//! let mut events = EventQueue::open();
//! // Consumes incoming events
//! while let Some(event) = events.next() {
//!   match event {
//!      // Detects when the space key is pressed
//!      Event::Key { code: Keycode::SPACE, down: true, .. } => println!("Pressed space!"),
//!      // Quits if requested
//!      Event::Quit => break,
//!      // Ignores other events
//!      _ => {}
//!   }
//! }
//! ```
//!
//! [`Run`]: crate::run::Run

mod event;
mod queue;

pub use event::Event;
pub use queue::EventQueue;