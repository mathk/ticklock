//! Provide timer abstraction

use core::time::Duration;
use core::ops::Sub;
use core::convert::Into;

/// A `Timer` trait to represent count down / up time.
/// This is a typical peripheral that has an internal counter that decrease or increase over time until it reach its limit.
pub trait Timer {

    /// Inner type of the counter
    type U : Sub<Output = Self::U> + Into<u32>;

    /// Pause the execution for Duration.
    fn delay(&mut self, d: Duration);

    /// Pause execution assuming interrupt is enabled
    /// and correctly handler.
    fn delay_with_interrupt(&mut self, d: Duration) {
        // By default is a not optimal delay.
        self.delay(d);
    }

    /// Start a timer counter
    /// The timer is being move and dedicated
    /// to the instant needs.
    fn start<T: Timer>(self) ->  TimerInstant<T>;

    /// Stop the counting timer.
    /// This method is only used by `TimerInstant` to release the timer.
    fn stop(self) -> Self;

    /// Test if the counter has wrapped to its initial value
    fn has_wrapped(&mut self) -> bool;

    /// The maximum / minimum value.
    /// For count down timer this should be the maximum value. Or the reload value.
    /// For count up limit_value should return 0.
    fn limit_value(&self) -> Self::U;

    /// Return the current counter value.
    fn get_current(&mut self) -> Self::U;

    /// Return the duration between 2 counted value.
    fn tick(&mut self) -> Duration;
}


/// Capture an instant from a timer.
pub struct TimerInstant<T>
where T : Timer
{
    delay: T,
}

impl<T> TimerInstant<T>
where T : Timer
{
    /// Capture an Instant with a given timer.
    pub fn now(delay: T) -> Self {
        TimerInstant {
            delay,
        }
    }

    /// Give the elapsed time from the Instant.
    pub fn elapsed(&mut self) -> Duration {
        if self.delay.has_wrapped() {
            panic!("Can not tell the elapse time as we have wrapped.")
        }
        self.delay.tick() * (self.delay.limit_value() - self.delay.get_current()).into()
    }

    /// Release the instant and stop the timer
    pub fn stop(self) -> T {
        self.delay.stop()
    }
}
