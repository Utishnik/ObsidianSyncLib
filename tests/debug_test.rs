use obsidian_sync_lib::argsfmt;
use obsidian_sync_lib::debug::debug_and_test_utils::ArgsFmt;

#[test]
fn test() {
    let mut test: ArgsFmt = argsfmt!("hello", 10, "gay", "orno",);
    test.debug_print();
}
