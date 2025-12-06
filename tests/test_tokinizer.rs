 use obsidian_sync_lib::{debug::{TESTS, get_count_tests, get_test, result_list}, *};
 
#[test]
fn test_skip_construction()
{
    let result1: bool = obsidian_sync_lib::tokinezed::skip_construction("      { { ", &mut 0, " ", "{{");
    println!("{}",result1);
    test_assert!(result1,true);
   
    let result1: bool = obsidian_sync_lib::tokinezed::skip_construction("       [  ", &mut 0, " ", "[[");
    println!("{}",result1);
    test_assert!(result1,false); 
    
    let result1: bool = obsidian_sync_lib::tokinezed::skip_construction("  {i t e r}", &mut 0, " ", "{iter}");
    println!("{}",result1);
    test_assert!(result1,true); 

    if !result_list()
    {
        std::process::exit(1);
    }
}