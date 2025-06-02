use std::time::{Duration, Instant}; // ? For measuring execution time

// * Variable to keep track of the timer
static mut TIMER: Option<Instant> = None;

// * Starts the timer
pub fn start() {
    unsafe {
        TIMER = Some(Instant::now());
    }
}

// * Stops the timer and returns the elapsed duration
pub fn stop() -> Duration {
    unsafe {
        if let Some(start) = TIMER {
            return start.elapsed();
        }
    }
    Duration::new(0, 0) // Return zero duration if timer was not started
}
