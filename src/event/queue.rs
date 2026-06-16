use std::marker::PhantomData;
use std::mem::MaybeUninit;

use sdl3_sys::events::{SDL_Event, SDL_PollEvent, SDL_PushEvent, SDL_QuitEvent, SDL_EVENT_QUIT};
use sdl3_sys::init::{SDL_INIT_EVENTS, SDL_InitSubSystem, SDL_QuitSubSystem};
use sdl3_sys::timer::SDL_GetTicksNS;

use crate::sdl_util::{self, sdl_assert};
use crate::thread;

use super::Event;

// The event queue could be accessed just by a function, as it is in SDL.
// Instead, we use a zero-sized type that keeps the event subsystem initialized
// while in scope, to avoid potentially rapidly initializing and deinitializing
// the events subsystem each time such a function would be called.

/// A handle for the system event queue.
///
/// Implements [`Iterator`], allowing events to be filtered, mapped, and counted.
///
/// # Examples
///
/// To open the queue:
///
/// ```no_run
/// # use kibble::event::EventQueue;
/// let mut event_queue = EventQueue::open();
/// ```
///
/// To consume the next event:
///
/// ```no_run
/// # use kibble::event::{Event, EventQueue};
/// # let mut event_queue = EventQueue::open();
/// // Consumes the next event
/// let event = event_queue.next();
/// // Events can be pattern-matched!
/// if let Some(Event::Key { down: true, .. }) = event {
///   println!("You pressed a key!");
/// }
/// ```
pub struct EventQueue(PhantomData<*const ()>); // Phantom pointer for !Send and !Sync

impl EventQueue {

	/// Returns a new handle for the system event queue.
	///
	/// Note that all handles share the same event queue.
	///
	/// # Panics
	///
	/// Panics if called outside the main thread.
	#[doc(alias = "new")]
	pub fn open() -> Self {
		assert!(thread::is_main(), "`EventQueue::open()` should only be called on the main thread");
		unsafe { Self::open_unchecked() }
	}

	/// Returns a new handle for the system event queue.
	///
	/// Note that all handles share the same event queue.
	///
	/// # Safety
	///
	/// Should only be called on the main thread.
	pub unsafe fn open_unchecked() -> Self {
		sdl_assert!(unsafe { SDL_InitSubSystem(SDL_INIT_EVENTS) });
		Self(PhantomData)
	}

	/// Pushes [`Event::Quit`] onto the event queue.
	///
	/// Can be used to quit the program, if it responds to the event.
	pub fn push_quit() {
		sdl_assert!(unsafe { SDL_PushEvent(&mut SDL_Event {
			quit: SDL_QuitEvent {
				r#type: SDL_EVENT_QUIT,
				timestamp: SDL_GetTicksNS(),
				reserved: 0,
			}
		}) });
	}

}

impl Iterator for EventQueue {

	type Item = Event;

	/// Pops the next event off the event queue, if any, and returns it.
	fn next(&mut self) -> Option<Event> {
		let mut event = MaybeUninit::uninit();
		if unsafe { SDL_PollEvent(event.as_mut_ptr()) } {
			Event::try_from(unsafe { event.assume_init() }).ok()
		} else {
			None
		}
	}

}

impl Drop for EventQueue {

	fn drop(&mut self) {
		unsafe {
			SDL_QuitSubSystem(SDL_INIT_EVENTS);
			sdl_util::quit_if_unused();
		}
	}

}