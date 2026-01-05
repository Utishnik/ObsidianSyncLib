//todo https://github.com/rust-threadpool/rust-threadpool/blob/master/src/lib.rs

pub mod std_mutex {
    use std::{
        sync::{
            Arc, Mutex,
            atomic::{AtomicBool, AtomicUsize, Ordering},
        },
        thread::sleep,
    };

    use rand::random;

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

    pub fn barrier_non_idle() {
        let size: Arc<usize> = Arc::new(3);
        let barrier: Arc<Mutex<NonIdleBarrier>> =
            Arc::new(Mutex::new(NonIdleBarrier::build(*size)));
        let clone_size: Arc<usize> = Arc::clone(&size);

        let test_block = Arc::new(move |x: u64| {
            let barrier_clone: Arc<Mutex<NonIdleBarrier>> = Arc::clone(&barrier);
            /////// сложный код
            let timesleep: std::time::Duration = std::time::Duration::from_millis(x);
            sleep(timesleep);
            /////
            let guard_pack: Result<
                std::sync::MutexGuard<'_, NonIdleBarrier>,
                std::sync::PoisonError<std::sync::MutexGuard<'_, NonIdleBarrier>>,
            > = barrier_clone.lock();
            let guard: std::sync::MutexGuard<'_, NonIdleBarrier> = guard_pack.unwrap();
            guard.barrier_out.fetch_add(1, Ordering::Release);
            let mut give_curr: usize = guard.barrier_out.load(Ordering::Acquire);
            drop(guard);
            while give_curr != *size {
                println!("give curr : {}  size : {}", give_curr, *size);
                let timesleep: std::time::Duration = std::time::Duration::from_millis(1);
                sleep(timesleep);
                let guard_pack: Result<
                    std::sync::MutexGuard<'_, NonIdleBarrier>,
                    std::sync::PoisonError<std::sync::MutexGuard<'_, NonIdleBarrier>>,
                > = barrier_clone.lock();
                let guard: std::sync::MutexGuard<'_, NonIdleBarrier> = guard_pack.unwrap();
                give_curr = guard.barrier_out.load(Ordering::Acquire);
                drop(guard);
            }
            println!("конец");
        });
        let mut handles: Vec<std::thread::JoinHandle<()>> = Vec::new();
        for i in 0..*clone_size {
            let test_block_clone = Arc::clone(&test_block);
            let rnd: u64 = random::<u64>();
            let rnd_rem: u64 = rnd % 2;
            println!("rnd: {}", rnd_rem);
            handles.push(std::thread::spawn(move || {
                let _ = test_block_clone(rnd_rem).clone();
            }));
            println!("поток {}", i);
        }

        for handl in handles.into_iter() {
            handl.join().expect("test");
        }
    }
}

pub mod spin_lock_mutex {
    use std::{
        sync::{
            Arc,
            atomic::{AtomicBool, AtomicUsize, Ordering},
        },
        thread::sleep,
    };

    use rand::random;

    #[derive(Default)]
    pub struct SpinLock {
        lock: AtomicBool,
        thread_id: usize,
    }

    //pub struct SpinL

    impl SpinLock {
        fn lock(&mut self) {
            if !self.lock.load(Ordering::Acquire) {
                self.lock.store(true, Ordering::Release);
            }
            else {
                while self.lock.load(Ordering::Acquire) {
                    core::hint::spin_loop();
                }
                self.lock.store(true, Ordering::Release);
                //todo проверка отравленого
            }
        }
    }

    fn barrier_non_idle() {}
}
