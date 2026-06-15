//! Main loop.
//!
//! See the [`Run`] documentation.

use std::thread;
use std::time::{Duration, Instant};

use crate::event::{Event, EventQueue};

/// A template trait for implementing a main loop.
///
/// [`Run::run()`] runs a main loop, calling [`listen()`], [`update()`], and
/// [`render()`] as necessary. These functions do nothing by default; to define
/// the behavior of your program, you should provide implementations for them.
///
/// Note that while [`Run`] provides a reasonable main loop, it is *not*
/// required to run a loop, and you are encouraged to write your own main loop
/// if you wish. [`Run`] is only provided for convenience.
///
/// # Notes on custom implementations
///
/// If you'd like to write your own main loop, make sure your program quits when
/// requested to by the user, i.e. you should open the event queue and respond
/// to the [`Event::Quit`] event. See the [`event`] documentation for examples.
///
/// [`listen()`]: Self::listen
/// [`update()`]: Self::update
/// [`render()`]: Self::render
/// [`event`]: crate::event
pub trait Run {

	/// How often the main loop should update.
	const UPDATE_FREQ:    Duration = Duration::from_nanos(1_000_000_000 / 30);
	/// How often the main loop should render.
	const RENDER_FREQ:    Duration = Duration::from_nanos(1_000_000_000 / 30);
	/// How long the main loop should sleep between iterations.
	const SLEEP_DURATION: Duration = Duration::ZERO;

	/// Updates based on an event.
	#[allow(unused_variables)]
	fn listen(&mut self, event: &Event) {}

	/// Updates.
	fn update(&mut self) {}

	/// Draws to the screen.
	///
	/// `delta` is a value in `[0.0, 1.0)` representing the time between the last
	/// and next call to [`update()`].
	///
	/// [`update()`]: Self::update
	#[allow(unused_variables)]
	fn render(&mut self, delta: f32) {}

	/// Runs a main loop, blocking until the program is requested to quit.
	///
	/// This function may be overwritten by implementors of [`Run`]. For notes on
	/// the default implementation, see the source code.
	fn run(&mut self) {
		// This implementation does three external calls:
		//
		// - `listen()` to update based on incoming events,
		// - `update()` to update over time, and
		// - `render()` to draw to the screen.
		//
		// 1. Incoming events are passed to `listen()` as quickly as possible to
		//    reduce input latency.
		// 2. `update()` and `render()` are called every `UPDATE_FREQ` and
		//    `RENDER_FREQ` respectively.
		//    - If the loop takes long enough to miss an update, multiple updates
		//      are done consecutively until the game state catches back up.
		//    - The loop will skip render calls as necessary.
		// 3. Finally, the loop will sleep for `SLEEP_DURATION` (zero by default).
		//
		// For design motivation, see [Game Loop] from Game Programming Patterns.
		//
		// [Game Loop]: https://gameprogrammingpatterns.com/game-loop.html
		let mut events = EventQueue::open();
		let mut last_update_time = Instant::now();
		let mut last_render_time = Instant::now();
		let mut update_lag = Duration::ZERO;
		loop {
			// Handles incoming events
			while let Some(event) = events.next() {
				self.listen(&event);
				// Exits the loop if a quit event is received
				if matches!(event, Event::Quit) {
					println!("Quitting on request...");
					return;
				}
			}
			// Updates
			update_lag += last_update_time.elapsed();
			last_update_time = Instant::now();
			while update_lag >= Self::UPDATE_FREQ {
				self.update();
				update_lag -= Self::UPDATE_FREQ;
			}
			// Renders
			if last_render_time.elapsed() >= Self::RENDER_FREQ {
				last_render_time = Instant::now();
				self.render(update_lag.div_duration_f32(Self::UPDATE_FREQ));
			}
			// Sleeps
			if !Self::SLEEP_DURATION.is_zero() { // Compiles away if `SLEEP_DURATION` is zero
				thread::sleep(Self::SLEEP_DURATION);
			}
		}
	}

}