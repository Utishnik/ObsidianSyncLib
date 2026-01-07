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
use crate::zerotrait::{self, Zero};
use core::ops::BitAnd;

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
#[derive(Debug)]
//step надо сделать атомарными!!!
pub struct Backoff {
    step: u8,
    msg: u8,
    yield_limit: u8,
}

impl Default for Backoff {
    fn default() -> Self {
        Self {
            step: 0,
            msg: 0,
            yield_limit: Self::YIELD_LIMIT,
        }
    }
}

impl Backoff {
    fn reset_step(&mut self) {
        self.step = 0;
    }
    fn step_set(&mut self, step: u8) {
        self.step = step;
    }
    pub fn get_step(&self) -> u8 {
        self.step
    }
}

impl Backoff {
    const UNPARK: u8 = 0xf0;
    const IS_LIMIT_EXCEEDED: u8 = 0x0f;
    const YIELD_LIMIT: u8 = 10;
}
//todo перенести zero utils и c битами в micro utils crate или типо того
#[inline]
#[must_use]
fn check_bits<T, TT>(val: T, mask: T) -> bool
where
    T: core::ops::BitAnd,
    TT: Sized + Zero,
    <T as BitAnd>::Output: PartialEq<TT>,
{
    if (val & mask) != TT::zero() {
        true
    } else {
        false
    }
}
#[inline]
fn add_bits<T>(val: &mut T, mask: T)
where
    T: core::ops::BitOrAssign,
{
    *val |= mask;
}

impl Relax for Backoff {
    #[inline]
    fn relax(&mut self) {
        for _ in 0..1_u16 << self.step {
            //надо после того как сделаю step атомарным проверять если ее значение изменилось проверить
            //дошли до 1024 а измненили на 256 смотрим а в цикле мы на 310 итерации значит break
            let unpark_state: bool = check_bits(self.msg, Self::UNPARK);
            if unpark_state {
                break;
            }
            core::hint::spin_loop();
        }
        if self.step <= self.yield_limit {
            self.step += 1;
        } else {
            add_bits(&mut self.msg, Self::IS_LIMIT_EXCEEDED);
        }
    }
}
