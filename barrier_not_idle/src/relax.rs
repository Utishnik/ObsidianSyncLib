//! Relax strategies.
//!
//! Relax strategies are used when the thread cannot acquire a spinlock.
/// A relax strategy.
///
/// `Relax` types are used to relax the current thread during contention.
pub trait Relax: Default {
    /// Relaxes the current thread.
    fn relax(&mut self);
}
use core::ops::BitAnd;
use crate::zerotrait;

/// Rapid spinning.
///
/// This emits [`core::hint::spin_loop`].
#[derive(Default, Debug)]
pub struct Spin;

impl Relax for Spin {
    #[inline]
    fn relax(&mut self) {
        core::hint::spin_loop();
    }
}

/// Exponential backoff.
///
/// This performs exponential backoff to avoid unnecessarily stressing the cache.
//Adapted from <https://github.com/crossbeam-rs/crossbeam/blob/crossbeam-utils-0.8.16/crossbeam-utils/src/backoff.rs>.
#[derive(Default, Debug)]
pub struct Backoff {
    step: u8,
    msg: u8,
}

impl Backoff {
    const YIELD_LIMIT: u8 = 10;
    const PARK: u8 = 0xf0;
    const IS_LIMIT_EXCEEDED: u8 = 0x0f;
}

fn check_bits<T,TT>(val: T,mask: T) -> bool
where T: core::ops::BitAnd,
TT: Sized,
<T as BitAnd>::Output: PartialEq<TT>,
{
    if (val & mask) != 0 {
        true
    }
    else {
        false
    }
}

impl Relax for Backoff {
    #[inline]
    fn relax(&mut self) {
        for _ in 0..1_u16 << self.step {
            core::hint::spin_loop();
        }

        if self.step <= Self::YIELD_LIMIT {
            self.step += 1;
        }
        else{
            self.msg |= Self::IS_LIMIT_EXCEEDED;
            check_bits(self.msg, Self::PARK);
        }
    }
}
