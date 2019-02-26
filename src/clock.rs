//! Clock trait for Cortex-M.
//!
//! Access the SysTick peripheral and provide timing abstraction

#![allow(missing_docs)]
use core::ops::Div;
use core::time::Duration;
use crate::peripheral::syst::SystClkSource;
use crate::peripheral::SYST;


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
/// Using the frequency we can calculate the counter for some delay
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Frequency {
    pub (crate) resolution: FreqRange,
    pub (crate) numerator: u32,
    pub (crate) denominator: u32,
}

impl Frequency {

    fn new(value: u32, resolution: FreqRange) -> Frequency {
        Frequency {
            resolution,
            numerator: value,
            denominator: 1,
        }
    }

    pub const fn tick(&self) -> Duration {
        Duration::from_nanos(1_000_000_000_000u64 * self.denominator as u64 / (self.numerator as u64 * self.resolution as u64))
    }

    pub const fn ticks_in(&self, d: Duration) -> u64 {
        (d.as_secs() * 1_000_000_000u64 + d.subsec_nanos() as u64) * self.resolution as u64 * self.numerator as u64 / (self.denominator as u64 * 1_000_000_000_000u64)
    }

    pub fn into_hertz(&self) -> Frequency {
        Frequency {
            resolution: FreqRange::Hertz,
            numerator: (self.resolution as u32 * self.numerator) / (FreqRange::Hertz as u32 * self.denominator),
            denominator: 1,
        }
    }

    pub fn into_kilo(&self) -> Frequency {
        Frequency {
            resolution: FreqRange::KiloHertz,
            numerator: (self.resolution as u32 * self.numerator) / (FreqRange::KiloHertz as u32 * self.denominator),
            denominator: 1,
        }
    }

    pub fn into_mega(&self) -> Frequency {
        Frequency {
            resolution: FreqRange::MegaHertz,
            numerator: (self.resolution as u32 * self.numerator) / (FreqRange::MegaHertz as u32 * self.denominator),
            denominator: 1,
        }
    }

    pub fn into_milli(&self) -> Frequency {
        Frequency {
            resolution: FreqRange::MilliHertz,
            numerator: (self.resolution as u32 * self.numerator) / (FreqRange::MilliHertz as u32 * self.denominator),
            denominator: 1,
        }
    }
}

impl Div<u32> for Frequency {
    type Output = Frequency;

    fn div(self, rhs: u32) -> Frequency {
        assert!(rhs != 0);
        Frequency {
            resolution: self.resolution,
            numerator: self.numerator,
            denominator: self.denominator * rhs
        }
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
}
