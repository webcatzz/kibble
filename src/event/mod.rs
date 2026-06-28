//! Events.
//!
//! Events are messages from the system about user input. For example, an event
//! is generated when a key is pressed, a mouse is moved, or the user closes the
//! program.
//!
//! See the [`Event`] enum for a list of events. To handle incoming events, see
//! [`EventQueue`].

mod event;
mod queue;

pub use event::Event;
pub use queue::EventQueue;