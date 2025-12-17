use obsidian_sync_lib::argsfmt;
use obsidian_sync_lib::call_functions;
use obsidian_sync_lib::debug_multi_thread::TypedResult;
use obsidian_sync_lib::{debug::*, tokinezed::Construction, *};
use std::any::Any;

fn test_add(a: i32, b: i32) -> i32 {
    a + b
}

#[test]
fn test() {
    let res: Vec<TypedResult> = call_functions!( test_add => (5, 3) -> i32);
}
