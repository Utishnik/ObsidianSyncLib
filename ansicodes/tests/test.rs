use std::{
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    },
    thread::sleep,
};

//todo https://github.com/rust-threadpool/rust-threadpool/blob/master/src/lib.rs
pub struct NonIdleBarrier {
    barrier_size: AtomicUsize,
    barrier_sleep: AtomicUsize,
}

impl NonIdleBarrier {
    fn build(size: usize) -> Self {
        Self {
            barrier_size: AtomicUsize::new(size),
            barrier_sleep: AtomicUsize::new(0),
        }
    }
}

#[test]
pub fn barrier_non_idle() {
    let size: usize=3;
    let barrier: NonIdleBarrier = NonIdleBarrier::build(size);
    let test_block = || {
        let give_curr: usize=barrier.barrier_sleep.load(Ordering::Acquire);
        /////// сложный код
        let timesleep: std::time::Duration = std::time::Duration::from_millis(1);
        sleep(timesleep);
        /////
        while give_curr!=size{   
            let timesleep: std::time::Duration = std::time::Duration::from_millis(1);
            sleep(timesleep);
        }
    };
    
    
    
    std::thread::spawn(|| {
        let timesleep: std::time::Duration = std::time::Duration::from_millis(50);
        sleep(timesleep);
    });
}
