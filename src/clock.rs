//! Clock trait for Cortex-M.
//!
//! Access the SysTick peripheral and provide timing abstraction

use core::cmp;
use core::ops::{Div, Mul};
use core::time::Duration;

/// Represent frequency range magnitude
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u64)]
pub (crate) enum FreqRange {
    MegaHertz = 1_000_000_000,
    KiloHertz = 1_000_000,
    Hertz = 1_000,
    MilliHertz = 1,
}

/// Frequency abstraction
///
/// Using the frequency we can calculate duration
#[derive(Clone, Copy, Debug)]
pub struct Frequency {
    pub (crate) resolution: FreqRange,
    pub (crate) numerator: u32,
    pub (crate) denominator: u32,
}

impl PartialEq for Frequency {
    fn eq(&self, other: &Frequency) -> bool {
        let left = self.into_hertz();
        let other = other.into_hertz();
        left.numerator == other.numerator
    }
}

impl Eq for Frequency {}


impl Ord for Frequency {
    fn cmp(&self, other: &Frequency) -> cmp::Ordering {
        let left = self.into_hertz();
        let other = other.into_hertz();
        left.numerator.cmp(&other.numerator)
    }
}

impl PartialOrd for Frequency {
    fn partial_cmp(&self, other: &Frequency) -> Option<cmp::Ordering> {
        Some(self.cmp(&other))
    }
}

macro_rules! into_x {
    ($name:ident, $range:ident) => {
        /// Change the frequency range in $range.
        /// This is useful only for printing.
        pub fn $name(&self) -> Frequency {
            Frequency {
                resolution: FreqRange::$range,
                numerator: (self.resolution as u32 * self.numerator) / (FreqRange::$range as u32 * self.denominator),
                denominator: 1,
            }
        }
    }
}

impl Frequency {

    fn new(value: u32, resolution: FreqRange) -> Frequency {
        Frequency {
            resolution,
            numerator: value,
            denominator: 1,
        }
    }

    /// Return the duration of single clock pulse.
    pub const fn tick(&self) -> Duration {
        Duration::from_nanos(1_000_000_000_000u64 * self.denominator as u64 / (self.numerator as u64 * self.resolution as u64))
    }

    /// Return the number of entire clock pulse within a duration
    pub const fn ticks_in(&self, d: Duration) -> u64 {
        (d.as_secs() * 1_000_000_000u64 + d.subsec_nanos() as u64) * self.resolution as u64 * self.numerator as u64 / (self.denominator as u64 * 1_000_000_000_000u64)
    }

    into_x!(into_hertz, Hertz);
    into_x!(into_kilo, KiloHertz);
    into_x!(into_mega, MegaHertz);
    into_x!(into_milli, MilliHertz);

}

impl Div<u32> for Frequency {
    type Output = Frequency;

    /// Allow to scale down a frequency
    fn div(self, rhs: u32) -> Frequency {
        assert!(rhs != 0);
        Frequency {
            resolution: self.resolution,
            numerator: self.numerator,
            denominator: self.denominator * rhs
        }
    }
}

impl Mul<u32> for Frequency {
    type Output = Frequency;

    /// Allow to scale down a frequency
    fn mul(self, rhs: u32) -> Frequency {
        Frequency {
            resolution: self.resolution,
            numerator: self.numerator * rhs,
            denominator: self.denominator
        }
    }
}

impl Div<Frequency> for Frequency {
    type Output = u32;

    /// Give the scale between 2 frequency.
    /// Particularly useful for selecting pre-scale value on MCU.
    fn div(self, rhs: Frequency) -> u32 {
        assert!(rhs.numerator != 0);
        (self.resolution as u32 / rhs.resolution as u32) * (self.numerator * rhs.denominator) / (self.denominator * rhs.numerator)
    }
}


/// Extension trait that adds convenience methods to the `u32` type
pub trait U32Ext {

    /// Wrap in Frequency
    fn hz(self) -> Frequency;

    /// Wrap in Frequency
    fn khz(self) -> Frequency;

    /// Wrap in Frequency
    fn mhz(self) -> Frequency;

    /// Wrap in millisecond Duration
    fn ms(self) -> Duration;

    /// Wrap in microsecond Duration
    fn us(self) -> Duration;

    /// Wrap in microsecond Duration
    fn s(self) -> Duration;

    /// Make the value stay in between 2 bounds
    fn clamp(self, min: u32, max: u32) -> u32;
}

impl U32Ext for u32 {

    fn hz(self) -> Frequency {
        Frequency::new(self, FreqRange::Hertz)
    }

    fn khz(self) -> Frequency {
        Frequency::new(self, FreqRange::KiloHertz)
    }

    fn mhz(self) -> Frequency {
        Frequency::new(self, FreqRange::MegaHertz)
    }

    fn s(self) -> Duration {
        Duration::from_secs(self as u64)
    }

    fn ms(self) -> Duration {
        Duration::from_millis(self as u64)
    }

    fn us(self) -> Duration {
        Duration::from_micros(self as u64)
    }

    fn clamp(self, min: u32, max: u32) -> u32 {
        cmp::min(cmp::max(self, min), max)
    }
}


mod test {

    #[allow(unused_imports)]
    use super::{FreqRange, U32Ext};

    #[test]
    fn multiply() {
        assert_eq!((1.mhz() * 2).numerator, 2);
        assert_eq!((1.mhz() * 8000).numerator, 8000);
        assert_eq!((1.khz() * 80000).into_mega().resolution, FreqRange::MegaHertz);
        assert_eq!((1.khz() * 80000).into_mega().numerator, 80);

    }

    #[test]
    fn divide() {
        assert_eq!((1.mhz() / 2).into_kilo().numerator, 500);
        assert_eq!((1.mhz() / 8000).into_hertz().resolution, FreqRange::Hertz);
        assert_eq!((1.mhz() / 8000).into_hertz().numerator, 125);
        assert_eq!((1.mhz() / 80000).into_hertz().resolution, FreqRange::Hertz);
        assert_eq!((1.mhz() / 80000).into_hertz().numerator, 12);

    }

    #[test]
    fn tick() {
        assert_eq!(1.mhz().tick(), 1.us());
        assert_eq!(1.khz().tick(), 1.ms());
        assert_eq!(1.hz().tick(), 1.s());
        assert_eq!(1.hz().ticks_in(2.s()), 2);
    }

    #[test]
    fn ticks_in() {
        assert_eq!(1.khz().ticks_in(2.s()), 2_000);
        assert_eq!(1.mhz().ticks_in(2.s()), 2_000_000);
        assert_eq!(2.mhz().ticks_in(2.s()), 4_000_000);
        assert_eq!(2.mhz().ticks_in(2.us()), 4);
    }

    #[test]
    fn scale() {
        assert_eq!(1.mhz() / 500.khz(), 2);
        assert_eq!(1.mhz() / 1000.khz(), 1);
        assert_eq!(1.mhz() / 10000.khz(), 0);
    }

    #[test]
    fn clamp() {
        assert_eq!(2.clamp(1, 3), 2);
        assert_eq!(20.clamp(1, 3), 3);
        assert_eq!(0.clamp(1, 3), 1);
    }

    #[test]
    fn comp() {
        assert!(1.mhz() < 2000.khz());
        assert!(1.mhz() == 1000.khz());
        assert!(1.mhz() >= 1000.khz());
        assert!(1.mhz() <= 1000.khz());
    }

}
