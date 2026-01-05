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
use crate::zerotrait::{self, Zero};

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
//todo перенести zero utils и c битами в micro utils crate или типо того
#[inline]
#[must_use]
fn check_bits<T,TT>(val: T,mask: T) -> bool
where T: core::ops::BitAnd,
TT: Sized + Zero,
<T as BitAnd>::Output: PartialEq<TT>,
{
    if (val & mask) != TT::zero(){
        true
    }
    else {
        false
    }
}
#[inline]
fn add_bits<T>(val: &mut T,mask: T)
where T: core::ops::BitOrAssign,
{
    *val |= mask;
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
            add_bits(&mut self.msg, Self::IS_LIMIT_EXCEEDED);
            let park_state: bool = check_bits(self.msg, Self::PARK);
        }
    }
}
