use obsidian_sync_lib::argsfmt;
use obsidian_sync_lib::utils::*;
use obsidian_sync_lib::{debug::*, tokinezed::construction, *};

#[test]
fn test() {
    let rt1: String = unique_sym_to_str("[[{", "]]}}{[[");
    let rt2: String = unique_sym_to_str("66{", "]667");
    let rt3: String = unique_sym_to_str("ппрп575ююлааааалллллkkkkkkkk", "ааааапппр55559001ёёё");
    println!("rt1:  {}\trt2:  {}\trt3:  {}\t", rt1, rt2, remove_duplicate_chars_simple_nm(&rt3));
    println!("rt3 2: {}",remove_duplicate_chars_simple_n(&rt3));
    let strstest = vec!["test1", "test2", "zxct", "663", "12333"];
    let ostrstest = convert_vec_to_owned(strstest);
    let rt4: String = unique_sym_to_vec_str(&ostrstest);
    println!("rt4:  {}", rt4);
}
