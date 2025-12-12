use obsidian_sync_lib::argsfmt;
use obsidian_sync_lib::{debug::*, tokinezed::construction, *};

#[test]
fn test() {
    let mut test: ArgsFmt = argsfmt!("hello", 10, "gay", "orno",);
    test.debug_print();
}
