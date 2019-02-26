//! Provide timer abstraction

// #![allow(missing_docs)]
use crate::peripheral::SYST;
use crate::peripheral::syst::SystClkSource;
use crate::clock::Clocks;
use core::time::Duration;
use core::cmp::min;

/// A `Timer`` trait to represent count down time.
/// This is a typical peripheral that has an internal counter that decrease or increase over time until it reach its limit.
pub trait Timer {

    /// Inner type of the counter
    type U

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
    fn start(mut self) ->  TimerInstant<Self>;

    /// Stop the counting timer.
    /// This method is only used by `TimerInstant` to release the timer.
    fn stop(mut self) -> Self;

    /// Test if the counter has wrapped to its initial value
    fn has_wrapped(&mut self) -> bool;
    fn get_current(&mut self) -> Self::U;
    fn tick(&mut self) -> Duration;


/// Capture an instant from a timer.
pub struct TimerInstant<T>
where T : Timer
{
    delay: T,
}

impl<T> SysTickInstant<T>
where T : Clocks
{
    fn now(delay: SysTickDelay<T>) -> Self {
        SysTickInstant {
            delay,
        }
    }

    pub fn elapsed(&mut self) -> Duration {
        if self.delay.has_wrapped() {
            panic!("Can not tell the elapse time as we have wrapped.")
        }
        self.delay.tick() * (0x0FF_FFFF - self.delay.get_current())
    }

    pub fn stop(self) -> SysTickDelay<T> {
        self.delay.stop()
    }
}

/// Delay base on Systick.
pub struct SysTickDelay<T>
where T : Clocks
{
    syst: SYST,
    clocks: T,
}

/// Delay using the SysTick timer
impl<T> SysTickDelay<T>
where
    T: Clocks
{

    /// Build a new SysTick timer base on external source clock.
    /// External clock is vendor dependent
    pub fn new_external(mut syst: SYST, clocks: T) -> Self {
        syst.set_clock_source(SystClkSource::External);
        SysTickDelay {
            syst,
            clocks
        }
    }

    pub fn start(mut self) ->  SysTickInstant<T> {
        self.syst.set_reload(0x00FF_FFFF);
        self.syst.clear_current();
        self.syst.enable_counter();
        SysTickInstant::now(self)
    }

    pub fn stop(mut self) -> Self {
        self.syst.disable_counter();
        self
    }

    pub fn tick(&mut self) -> Duration {
        self.clocks.get_syst_clock(&mut self.syst).tick()
    }

    pub fn has_wrapped(&mut self) -> bool {
        self.syst.has_wrapped()
    }

    pub fn get_current(&mut self) -> u32 {
        SYST::get_current()
    }
}
