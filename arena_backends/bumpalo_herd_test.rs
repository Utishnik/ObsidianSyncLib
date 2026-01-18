//TODO spin mutex and parking
// We disable stuff on that platform and are lazy to disable all the imports too, this is shorter.
#[cfg_attr(all(miri, target_os = "windows"), allow(unused_imports))]
mod tests {
    use crate::bumpalo_herd::Herd;
    use std::sync::Mutex;
    use std::thread;

    // Doesn't test much in ordinary tests, but miri can check it
    #[test]
    #[cfg(not(all(miri, target_os = "windows")))]
    fn alloc_miri() {
        let mut herd: Herd = Herd::new();

        let v: Mutex<Vec<_>> = Mutex::new(Vec::new());

        thread::scope(|s: &thread::Scope<'_, '_>| {
            s.spawn(|| {
                let bump = herd.get();
                v.lock().unwrap().push(bump.alloc(42));
            });
        });

        let sum: u32 = v.into_inner().unwrap().iter().map(|i| **i).sum();
        assert_eq!(42, sum);

        herd.reset();

        let hello: &mut str = herd.get().alloc_str("hello");
        assert_eq!("hello", hello);
    }
}
