use obsidian_sync_lib::{debug::*, tokinezed::construction, *};
use obsidian_sync_lib::argsfmt;

#[test]
fn test()
{
    let mut test: ArgsFmt = argsfmt!("hello",10,"gay","orno",);
    test.debug_print();
}