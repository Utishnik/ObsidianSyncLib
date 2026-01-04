use std::{
    sync::{
        Arc, Mutex,
        atomic::{AtomicUsize, Ordering},
    },
    thread::sleep,
};

//todo https://github.com/rust-threadpool/rust-threadpool/blob/master/src/lib.rs
pub struct NonIdleBarrier {
    barrier_size: AtomicUsize,
    barrier_out: AtomicUsize,
}

impl NonIdleBarrier {
    fn build(size: usize) -> Self {
        Self {
            barrier_size: AtomicUsize::new(size),
            barrier_out: AtomicUsize::new(0),
        }
    }
}


#[test]
pub fn barrier_non_idle() {
    let size: Arc<usize> = Arc::new(3);
    let barrier: Arc<Mutex<NonIdleBarrier>> = Arc::new(Mutex::new(NonIdleBarrier::build(*size)));
    let clone_size: Arc<usize> = Arc::clone(&size);

    let test_block = Arc::new(move || {
        let barrier_clone: Arc<Mutex<NonIdleBarrier>> = Arc::clone(&barrier);
        /////// сложный код
        let timesleep: std::time::Duration = std::time::Duration::from_millis(1);
        sleep(timesleep);
        /////

        let guard_pack: Result<
            std::sync::MutexGuard<'_, NonIdleBarrier>,
            std::sync::PoisonError<std::sync::MutexGuard<'_, NonIdleBarrier>>,
        > = barrier_clone.lock();
        let guard: std::sync::MutexGuard<'_, NonIdleBarrier> = guard_pack.unwrap();
        guard.barrier_out.fetch_add(1, Ordering::Acquire);
        let mut give_curr: usize = guard.barrier_out.load(Ordering::Acquire);
        while give_curr != *size {
            println!("give curr : {}",give_curr);
            let timesleep: std::time::Duration = std::time::Duration::from_millis(1);
            sleep(timesleep);
            give_curr = guard.barrier_out.load(Ordering::Acquire);
        }
        println!("конец");
    });
    let mut handles: Vec<std::thread::JoinHandle<()>>  = Vec::new();
    for i in 0..*clone_size {
        let test_block_clone = Arc::clone(&test_block);
        handles.push(std::thread::spawn(move || {
            let _ = test_block_clone().clone();
        }));
        println!("поток {}",i);
    }

    for handl in handles.into_iter()
    {
        handl.join().expect("test");
    }
}
